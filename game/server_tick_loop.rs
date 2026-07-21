// game/server_tick_loop.rs
// Powrush-MMO v21.79.0 — NonSend Bevy wiring + dual multi-realm bridge payloads
// AG-SML v1.0 | Mercy-aligned authoritative GPU foresight
// Thunder locked in. Yoi ⚡

use crate::game::resource_nodes::ResourceNodeManager;
use crate::game::rbe::ServerInventoryComponent;
use crate::game::multi_realm_bridge::{
    AbundanceBridgePayload, OriginBridgePayload, DualBridgePayload,
    collect_abundance_payload, collect_origin_from_inventories,
};
use crate::engine::gpu_patsagi_bridge::{
    GpuPatsagiBridge, MockGpuPatsagiBridge, GpuPatsagiRequest, GpuPatsagiResponse,
    ComputeIntensity, RealGpuPatsagiBridge,
};
use shared::protocol::ServerMessage;
use std::time::{Duration, Instant};
use std::collections::HashMap;

#[derive(Default)]
pub struct GpuPerformanceMetrics {
    pub last_submit_ms: u64,
    pub last_cull_ms: u64,
    pub last_readback_ms: u64,
    pub last_policy_ms: u64,
    pub avg_frame_gpu_ms: f32,
    pub samples: u32,
}

/// Authoritative game tick loop — intended as Bevy `NonSend` (not Send/Sync).
pub struct ServerTickLoop {
    pub resource_nodes: ResourceNodeManager,
    gpu_bridge: Box<dyn GpuPatsagiBridge>,
    last_gpu_update: Instant,
    gpu_update_interval: Duration,
    pub pending_gpu_updates: Vec<ServerMessage>,

    in_flight_gpu_query: Option<PendingReadback>,
    pending_gpu_response: Option<GpuPatsagiResponse>,

    pub perf: GpuPerformanceMetrics,

    pub last_abundance_payload: AbundanceBridgePayload,
    pub last_origin_payload: OriginBridgePayload,
    last_bridge_collect: Instant,
    bridge_collect_interval: Duration,

    /// Soft tick counter for cohost observability.
    pub tick_count: u64,
    pub enabled: bool,
}

impl ServerTickLoop {
    /// Async constructor (real GPU path when feature enabled).
    pub async fn new() -> Self {
        #[cfg(feature = "gpu")]
        let gpu_bridge: Box<dyn GpuPatsagiBridge> = Box::new(RealGpuPatsagiBridge::new(/* device, queue */));

        #[cfg(not(feature = "gpu"))]
        let gpu_bridge: Box<dyn GpuPatsagiBridge> = Box::new(MockGpuPatsagiBridge);

        Self::from_bridge(gpu_bridge)
    }

    /// Sync constructor for NonSend Bevy Startup (always mock GPU unless host swaps bridge).
    pub fn new_sync() -> Self {
        Self::from_bridge(Box::new(MockGpuPatsagiBridge))
    }

    fn from_bridge(gpu_bridge: Box<dyn GpuPatsagiBridge>) -> Self {
        Self {
            resource_nodes: ResourceNodeManager::new(),
            gpu_bridge,
            last_gpu_update: Instant::now(),
            gpu_update_interval: Duration::from_secs(30),
            pending_gpu_updates: Vec::new(),
            in_flight_gpu_query: None,
            pending_gpu_response: None,
            perf: GpuPerformanceMetrics::default(),
            last_abundance_payload: AbundanceBridgePayload::default(),
            last_origin_payload: OriginBridgePayload::default(),
            last_bridge_collect: Instant::now(),
            bridge_collect_interval: Duration::from_secs(2),
            tick_count: 0,
            enabled: true,
        }
    }

    /// Main server tick. now_ms is authoritative game time (milliseconds).
    pub fn tick(&mut self, dt: f32, now_ms: u64) {
        if !self.enabled {
            return;
        }
        let _ = dt;
        let frame_start = Instant::now();
        self.tick_count = self.tick_count.saturating_add(1);

        self.resource_nodes.tick_regen(now_ms);

        if self.last_bridge_collect.elapsed() >= self.bridge_collect_interval {
            self.last_abundance_payload =
                collect_abundance_payload(&self.resource_nodes, now_ms);
            self.last_bridge_collect = Instant::now();
        }

        if self.last_gpu_update.elapsed() >= self.gpu_update_interval {
            if self.in_flight_gpu_query.is_none() {
                let start = Instant::now();
                let node_ids: Vec<u64> = self.resource_nodes.nodes.keys().cloned().collect();
                let request = GpuPatsagiRequest {
                    query: "periodic optimization + culling".to_string(),
                    intensity: ComputeIntensity::High,
                    context: HashMap::from([("node_count".to_string(), node_ids.len() as f32)]),
                    node_ids: node_ids.clone(),
                    harvesting_pressure: None,
                };

                if let Ok(query_id) = self.gpu_bridge.submit_query(request) {
                    self.in_flight_gpu_query = Some(PendingReadback {
                        query_id,
                        submitted_at: Instant::now(),
                        node_ids,
                    });
                    self.perf.last_submit_ms = start.elapsed().as_millis() as u64;
                }
                self.last_gpu_update = Instant::now();
            }
        }

        if let Some(pending) = &self.in_flight_gpu_query {
            let read_start = Instant::now();

            if let Some(response) = self.gpu_bridge.get_result(pending.query_id) {
                self.perf.last_readback_ms = read_start.elapsed().as_millis() as u64;

                let policy_start = Instant::now();
                self.resource_nodes.apply_gpu_policy_update(&response, now_ms);

                let update_msg = self.resource_nodes.build_gpu_update_message(&response);
                self.pending_gpu_updates.push(update_msg);

                self.pending_gpu_response = Some(response);
                self.in_flight_gpu_query = None;

                self.perf.last_policy_ms = policy_start.elapsed().as_millis() as u64;
            } else if pending.submitted_at.elapsed() > Duration::from_secs(8) {
                tracing::warn!("[ServerTick] GPU query {} timed out after 8s", pending.query_id);
                self.in_flight_gpu_query = None;
            }
        }

        let frame_gpu_ms = frame_start.elapsed().as_secs_f32() * 1000.0;
        self.perf.avg_frame_gpu_ms = if self.perf.samples == 0 {
            frame_gpu_ms
        } else {
            (self.perf.avg_frame_gpu_ms * self.perf.samples as f32 + frame_gpu_ms)
                / ((self.perf.samples + 1) as f32)
        };
        self.perf.samples = (self.perf.samples + 1).min(120);

        if self.perf.samples % 60 == 0 {
            tracing::info!(
                "[GPU Perf] avg={:.2}ms submit={} readback={} policy={} | bridge_a={} bridge_o={} ticks={}",
                self.perf.avg_frame_gpu_ms,
                self.perf.last_submit_ms,
                self.perf.last_readback_ms,
                self.perf.last_policy_ms,
                self.last_abundance_payload.realm_count(),
                self.last_origin_payload.realm_count(),
                self.tick_count
            );
        }

        if self.pending_gpu_response.is_some() {
            self.pending_gpu_response = None;
        }
    }

    pub fn refresh_origin_from_inventories<'a>(
        &mut self,
        inventories: impl IntoIterator<Item = &'a ServerInventoryComponent>,
        now_ms: u64,
    ) {
        self.last_origin_payload = collect_origin_from_inventories(inventories, now_ms);
    }

    pub fn get_pending_gpu_updates(&mut self) -> Vec<ServerMessage> {
        std::mem::take(&mut self.pending_gpu_updates)
    }

    pub fn take_abundance_payload(&mut self) -> AbundanceBridgePayload {
        std::mem::take(&mut self.last_abundance_payload)
    }

    pub fn take_origin_payload(&mut self) -> OriginBridgePayload {
        std::mem::take(&mut self.last_origin_payload)
    }

    pub fn abundance_payload(&self) -> &AbundanceBridgePayload {
        &self.last_abundance_payload
    }

    pub fn origin_payload(&self) -> &OriginBridgePayload {
        &self.last_origin_payload
    }

    pub fn dual_payload(&self) -> DualBridgePayload {
        DualBridgePayload {
            abundance: self.last_abundance_payload.clone(),
            origin: self.last_origin_payload.clone(),
        }
    }
}

#[derive(Debug)]
struct PendingReadback {
    query_id: u64,
    submitted_at: Instant,
    node_ids: Vec<u64>,
}

// =============================================================================
// Bevy NonSend wiring (when game package is fully in a Bevy App)
// =============================================================================

/// Insert `NonSend<ServerTickLoop>` and run soft ticks each frame.
/// Host: `.add_plugins(ServerTickLoopPlugin)` after game package is wired.
pub struct ServerTickLoopPlugin;

#[cfg(feature = "bevy_tick")]
mod bevy_nonsend {
    use super::*;
    use bevy::prelude::*;

    impl Plugin for super::ServerTickLoopPlugin {
        fn build(&self, app: &mut App) {
            app.insert_non_send_resource(ServerTickLoop::new_sync())
                .add_systems(Update, nonsend_server_tick_system);
            info!("ServerTickLoopPlugin — NonSend ServerTickLoop active");
        }
    }

    fn nonsend_server_tick_system(
        time: Res<Time>,
        mut loop_res: NonSendMut<ServerTickLoop>,
    ) {
        let dt = time.delta_seconds();
        let now_ms = (time.elapsed_seconds_f64() * 1000.0) as u64;
        loop_res.tick(dt, now_ms);
    }
}

/// Feature-free registration helper for hosts that already own a Bevy App.
/// Call from host binary when `bevy_tick` feature is unavailable:
/// ```ignore
/// app.insert_non_send_resource(ServerTickLoop::new_sync());
/// app.add_systems(Update, |time: Res<Time>, mut t: NonSendMut<ServerTickLoop>| {
///     t.tick(time.delta_seconds(), (time.elapsed_seconds_f64() * 1000.0) as u64);
/// });
/// ```
pub fn register_nonsend_server_tick_loop(app: &mut bevy::app::App) {
    app.insert_non_send_resource(ServerTickLoop::new_sync());
}

// Thunder locked in. NonSend ServerTickLoop ready when game package wired.
// Yoi ⚡
