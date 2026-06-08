// game/server_tick_loop.rs
// Powrush-MMO v16.5.23 — Server Tick now broadcasts GpuPatsagiUpdate messages
// After GPU policy updates, the server generates and can broadcast results to clients.
// AG-SML v1.0

use crate::game::resource_nodes::ResourceNodeManager;
use crate::engine::gpu_patsagi_bridge::{GpuPatsagiBridge, MockGpuPatsagiBridge};
#[cfg(feature = "gpu")]
use crate::engine::wgpu_patsagi_bridge::WgpuPatsagiBridge;
use shared::protocol::ServerMessage;
use std::time::{Duration, Instant};

pub struct ServerTickLoop {
    pub resource_nodes: ResourceNodeManager,
    gpu_bridge: Box<dyn GpuPatsagiBridge>,
    last_gpu_update: Instant,
    gpu_update_interval: Duration,
    pub pending_gpu_updates: Vec<ServerMessage>, // Messages ready to be broadcast
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
        }
    }

    pub fn tick(&mut self, dt: f32, now_ms: u64) {
        self.resource_nodes.tick_regen(now_ms);

        if self.last_gpu_update.elapsed() >= self.gpu_update_interval {
            if let Ok(response_notes) = self.resource_nodes.request_and_apply_gpu_update(self.gpu_bridge.as_ref()) {
                // Generate broadcast message for clients
                // In a real implementation we would get the actual GpuPatsagiResponse
                // For now we create a representative update
                let update_message = self.resource_nodes.build_gpu_update_message(&crate::engine::gpu_patsagi_bridge::GpuPatsagiResponse {
                    recommended_regen_rates: HashMap::new(),
                    predicted_depletion: HashMap::new(),
                    sustainability_adjustments: HashMap::new(),
                    confidence: 0.92,
                    notes: response_notes,
                });

                self.pending_gpu_updates.push(update_message);
                tracing::info!("[ServerTick] Generated GpuPatsagiUpdate for broadcast");
            }
            self.last_gpu_update = Instant::now();
        }
    }

    pub fn get_pending_gpu_updates(&mut self) -> Vec<ServerMessage> {
        std::mem::take(&mut self.pending_gpu_updates)
    }
}