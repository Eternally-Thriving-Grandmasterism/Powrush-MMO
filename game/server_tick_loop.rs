// game/server_tick_loop.rs
// Powrush-MMO v16.5.16 — Server Tick Loop with Real WGPU PATSAGi Integration
// Now uses the real WgpuPatsagiBridge when the `gpu` feature is enabled.
// All prior tick logic and periodic update structure preserved.
// AG-SML v1.0

use crate::game::resource_nodes::ResourceNodeManager;
use crate::engine::gpu_patsagi_bridge::{GpuPatsagiBridge, MockGpuPatsagiBridge};
#[cfg(feature = "gpu")]
use crate::engine::wgpu_patsagi_bridge::WgpuPatsagiBridge;
use std::time::{Duration, Instant};

pub struct ServerTickLoop {
    pub resource_nodes: ResourceNodeManager,
    gpu_bridge: Box<dyn GpuPatsagiBridge>,
    last_gpu_update: Instant,
    gpu_update_interval: Duration,
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
        }
    }

    /// Main server tick
    pub fn tick(&mut self, dt: f32, now_ms: u64) {
        // 1. Regenerate resource nodes
        self.resource_nodes.tick_regen(now_ms);

        // 2. Periodic GPU PATSAGi policy update (now using real backend when available)
        if self.last_gpu_update.elapsed() >= self.gpu_update_interval {
            match self.resource_nodes.request_and_apply_gpu_update(self.gpu_bridge.as_ref()) {
                Ok(msg) => {
                    tracing::info!("[ServerTick] GPU PATSAGi policy applied: {}", msg);
                }
                Err(e) => {
                    tracing::warn!("[ServerTick] GPU PATSAGi update failed: {}", e);
                }
            }
            self.last_gpu_update = Instant::now();
        }

        // Future systems...
    }

    pub fn get_resource_node_manager(&self) -> &ResourceNodeManager {
        &self.resource_nodes
    }

    pub fn get_resource_node_manager_mut(&mut self) -> &mut ResourceNodeManager {
        &mut self.resource_nodes
    }
}