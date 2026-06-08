// game/server_tick_loop.rs
// Powrush-MMO v16.5.57 — now_ms wiring + async/multi-frame polish
// - Passes real now_ms to apply_gpu_policy_update (required after restoration merge)
// - Cleaner multi-frame GPU result handling + timeout logic
// - Preserves all v16.5.53 performance metrics, tracing, and end-to-end timing
// - Follows RESTORATION_AND_MERGE_PROTOCOL.md (history sanity check passed via auditor)
// AG-SML v1.0 | Mercy-aligned authoritative GPU foresight

use crate::game::resource_nodes::ResourceNodeManager;
use crate::engine::gpu_patsagi_bridge::{GpuPatsagiBridge, MockGpuPatsagiBridge, GpuPatsagiRequest, GpuPatsagiResponse, ComputeIntensity, RealGpuPatsagiBridge};
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
    // pending_gpu_response kept for potential future use (e.g. last known good state)
    pending_gpu_response: Option<GpuPatsagiResponse>,

    pub perf: GpuPerformanceMetrics,
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
        }
    }

    /// Main server tick. now_ms is authoritative game time (milliseconds since start or epoch).
    pub fn tick(&mut self, dt: f32, now_ms: u64) {
        let frame_start = Instant::now();

        // Always run regeneration first (uses now_ms for restriction expiry)
        self.resource_nodes.tick_regen(now_ms);

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

                // === KEY FIX: pass real now_ms to policy (required after v16.5.54 restoration) ===
                let policy_start = Instant::now();
                self.resource_nodes.apply_gpu_policy_update(&response, now_ms);

                // Optional hook for render-world culling (kept as comment for future integration)
                // self.trigger_gpu_culling_dispatch(...);

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

        // Rolling average GPU frame time (for monitoring)
        let frame_gpu_ms = frame_start.elapsed().as_secs_f32() * 1000.0;
        self.perf.avg_frame_gpu_ms = if self.perf.samples == 0 {
            frame_gpu_ms
        } else {
            (self.perf.avg_frame_gpu_ms * self.perf.samples as f32 + frame_gpu_ms) / ((self.perf.samples + 1) as f32)
        };
        self.perf.samples = (self.perf.samples + 1).min(120);

        if self.perf.samples % 60 == 0 {
            tracing::info!(
                "[GPU Perf] avg={:.2}ms submit={} readback={} policy={}",
                self.perf.avg_frame_gpu_ms,
                self.perf.last_submit_ms,
                self.perf.last_readback_ms,
                self.perf.last_policy_ms
            );
        }

        // Clear one-shot response holder (kept for potential future last-known-good use)
        if self.pending_gpu_response.is_some() {
            self.pending_gpu_response = None;
        }
    }

    pub fn get_pending_gpu_updates(&mut self) -> Vec<ServerMessage> {
        std::mem::take(&mut self.pending_gpu_updates)
    }
}

// Internal helper for pending GPU work
#[derive(Debug)]
struct PendingReadback {
    query_id: u64,
    submitted_at: Instant,
    node_ids: Vec<u64>,
}

// Tuning Notes (v16.5.57)
// - gpu_update_interval: higher = less GPU load, lower = fresher foresight
// - ComputeIntensity: High gives better long-term abundance predictions
// - The automated auditor (tools/audit_file_history.py) was run before this change
// - All policy now receives authoritative now_ms for correct restriction timestamps
