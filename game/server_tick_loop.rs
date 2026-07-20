// game/server_tick_loop.rs
// Powrush-MMO v21.56.0 — multi-realm abundance bridge payload collection
// Previous: v16.5.57 now_ms wiring + async/multi-frame polish
// AG-SML v1.0 | Mercy-aligned authoritative GPU foresight
// Thunder locked in. Yoi ⚡

use crate::game::resource_nodes::ResourceNodeManager;
use crate::game::multi_realm_bridge::{AbundanceBridgePayload, collect_abundance_payload};
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

pub struct ServerTickLoop {
    pub resource_nodes: ResourceNodeManager,
    gpu_bridge: Box<dyn GpuPatsagiBridge>,
    last_gpu_update: Instant,
    gpu_update_interval: Duration,
    pub pending_gpu_updates: Vec<ServerMessage>,

    in_flight_gpu_query: Option<PendingReadback>,
    pending_gpu_response: Option<GpuPatsagiResponse>,

    pub perf: GpuPerformanceMetrics,

    /// Last collected multi-realm abundance payload (v21.56 bridge).
    /// External Bevy/simulation consumers can read this and emit AbundanceIngestEvent.
    pub last_abundance_payload: AbundanceBridgePayload,
    last_bridge_collect: Instant,
    bridge_collect_interval: Duration,
}

impl ServerTickLoop {
    pub async fn new() -> Self {
        #[cfg(feature = "gpu")]
        let gpu_bridge: Box<dyn GpuPatsagiBridge> = Box::new(RealGpuPatsagiBridge::new(/* device, queue */));

        #[cfg(not(feature = "gpu"))]
        let gpu_bridge: Box<dyn GpuPatsagiBridge> = Box::new(MockGpuPatsagiBridge);

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
            last_bridge_collect: Instant::now(),
            // Soft cadence — keeps bridge fresh without per-tick cost
            bridge_collect_interval: Duration::from_secs(2),
        }
    }

    /// Main server tick. now_ms is authoritative game time (milliseconds).
    pub fn tick(&mut self, dt: f32, now_ms: u64) {
        let _ = dt;
        let frame_start = Instant::now();

        // Always run regeneration first (uses now_ms for restriction expiry)
        self.resource_nodes.tick_regen(now_ms);

        // === Multi-Realm Abundance Bridge (v21.56) ===
        // Soft collect for external simulation EventWriter consumers.
        if self.last_bridge_collect.elapsed() >= self.bridge_collect_interval {
            self.last_abundance_payload =
                collect_abundance_payload(&self.resource_nodes, now_ms);
            self.last_bridge_collect = Instant::now();
        }

        // === GPU Query Submission (periodic) ===
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

        // === Multi-frame GPU Result Handling + Policy Application ===
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
                "[GPU Perf] avg={:.2}ms submit={} readback={} policy={} | bridge_realms={}",
                self.perf.avg_frame_gpu_ms,
                self.perf.last_submit_ms,
                self.perf.last_readback_ms,
                self.perf.last_policy_ms,
                self.last_abundance_payload.realm_count()
            );
        }

        if self.pending_gpu_response.is_some() {
            self.pending_gpu_response = None;
        }
    }

    pub fn get_pending_gpu_updates(&mut self) -> Vec<ServerMessage> {
        std::mem::take(&mut self.pending_gpu_updates)
    }

    /// Take the latest abundance bridge payload (for Bevy EventWriter consumers).
    pub fn take_abundance_payload(&mut self) -> AbundanceBridgePayload {
        std::mem::take(&mut self.last_abundance_payload)
    }

    /// Peek without clearing.
    pub fn abundance_payload(&self) -> &AbundanceBridgePayload {
        &self.last_abundance_payload
    }
}

#[derive(Debug)]
struct PendingReadback {
    query_id: u64,
    submitted_at: Instant,
    node_ids: Vec<u64>,
}

// Thunder locked in. Bridge payload collected every ~2s.
// Yoi ⚡
