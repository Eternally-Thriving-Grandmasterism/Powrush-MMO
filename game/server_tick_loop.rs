// game/server_tick_loop.rs
// Powrush-MMO v16.5.34 — Multi-Frame GPU Update Handling in Server Tick
// Spread GPU simulation work across frames for better responsiveness.
// AG-SML v1.0

use crate::game::resource_nodes::ResourceNodeManager;
use crate::engine::gpu_patsagi_bridge::{GpuPatsagiBridge, MockGpuPatsagiBridge, GpuPatsagiResponse};
#[cfg(feature = "gpu")]
use crate::engine::wgpu_patsagi_bridge::WgpuPatsagiBridge;
use shared::protocol::ServerMessage;
use std::time::{Duration, Instant};
use std::collections::HashMap;

pub struct ServerTickLoop {
    pub resource_nodes: ResourceNodeManager,
    gpu_bridge: Box<dyn GpuPatsagiBridge>,
    last_gpu_update: Instant,
    gpu_update_interval: Duration,
    pub pending_gpu_updates: Vec<ServerMessage>,

    // Multi-frame GPU state
    in_flight_gpu_query: Option<u64>,
    pending_gpu_response: Option<GpuPatsagiResponse>,
}

impl ServerTickLoop {
    pub async fn new() -> Self {
        #[cfg(feature = "gpu")]
        let gpu_bridge: Box<dyn GpuPatsagiBridge> = Box::new(WgpuPatsagiBridge::new().await);

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
        }
    }

    pub fn tick(&mut self, dt: f32, now_ms: u64) {
        self.resource_nodes.tick_regen(now_ms);

        // === Multi-frame GPU update handling ===
        if self.last_gpu_update.elapsed() >= self.gpu_update_interval {
            if self.in_flight_gpu_query.is_none() && self.pending_gpu_response.is_none() {
                // Start a new GPU query
                let request = crate::engine::gpu_patsagi_bridge::GpuPatsagiRequest {
                    query: "periodic long-term optimization".to_string(),
                    intensity: crate::engine::gpu_patsagi_bridge::ComputeIntensity::Medium,
                    context: HashMap::from([("node_count".to_string(), self.resource_nodes.nodes.len() as f32)]),
                    node_ids: self.resource_nodes.nodes.keys().cloned().collect(),
                };

                if let Ok(query_id) = self.gpu_bridge.submit_query(request) {
                    self.in_flight_gpu_query = Some(query_id);
                    tracing::info!("[ServerTick] Started multi-frame GPU query {}", query_id);
                }
                self.last_gpu_update = Instant::now();
            }
        }

        // Check for completed GPU result (can happen over multiple frames)
        if let Some(query_id) = self.in_flight_gpu_query {
            if let Some(response) = self.gpu_bridge.get_result(query_id) {
                // Apply results
                self.resource_nodes.apply_gpu_policy_update(&response);

                // Generate broadcast message
                let update_msg = self.resource_nodes.build_gpu_update_message(&response);
                self.pending_gpu_updates.push(update_msg);

                self.pending_gpu_response = Some(response);
                self.in_flight_gpu_query = None;

                tracing::info!("[ServerTick] Completed multi-frame GPU update and applied policy");
            }
        }

        // Clear one-frame-old response
        if self.pending_gpu_response.is_some() {
            self.pending_gpu_response = None;
        }
    }

    pub fn get_pending_gpu_updates(&mut self) -> Vec<ServerMessage> {
        std::mem::take(&mut self.pending_gpu_updates)
    }

    pub fn get_resource_node_manager(&self) -> &ResourceNodeManager {
        &self.resource_nodes
    }

    pub fn get_resource_node_manager_mut(&mut self) -> &mut ResourceNodeManager {
        &mut self.resource_nodes
    }
}