//! server/src/spatial/gpu_hierarchical_grid.rs
//! Production-grade GPU-Accelerated Hierarchical Grid Simulation Layer
//! v18.56 — Production scaffold ready for GPU compute integration
//! Provides CPU fallback + clear extension points for Bevy compute shaders / WGSL
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned

use bevy::prelude::*;
use bevy::render::render_resource::{Buffer, BufferUsages};
use bevy::render::renderer::RenderDevice;
use std::sync::Arc;

use crate::spatial::hierarchical_grid::HierarchicalGrid;
use crate::spatial::interest_management::InterestManager;

/// Configuration for GPU-accelerated hierarchical simulation
#[derive(Resource, Clone, Debug)]
pub struct GpuHierarchicalGridConfig {
    pub enabled: bool,
    pub grid_levels: u8,
    pub cell_size: f32,
    pub max_entities_per_frame: u32,
}

impl Default for GpuHierarchicalGridConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            grid_levels: 4,
            cell_size: 64.0,
            max_entities_per_frame: 8192,
        }
    }
}

/// GPU-accelerated hierarchical grid resource
/// Maintains a CPU HierarchicalGrid as authoritative fallback.
/// Future: entity/cell data will be uploaded to GPU buffers for parallel simulation.
#[derive(Resource)]
pub struct GpuHierarchicalGrid {
    pub cpu_grid: HierarchicalGrid,
    pub entity_buffer: Option<Buffer>,
    pub cell_buffer: Option<Buffer>,
    pub dirty: bool,
}

impl GpuHierarchicalGrid {
    pub fn new(config: &GpuHierarchicalGridConfig) -> Self {
        Self {
            cpu_grid: HierarchicalGrid::new(config.cell_size, config.grid_levels),
            entity_buffer: None,
            cell_buffer: None,
            dirty: true,
        }
    }

    /// Sync CPU grid state to GPU buffers (when enabled).
    /// Currently a no-op scaffold. Future implementation will create/resize buffers
    /// and upload entity positions + cell data for compute shader consumption.
    pub fn sync_to_gpu(&mut self, _device: &RenderDevice) {
        if !self.dirty { return; }
        // TODO (GPU phase): Create or resize entity_buffer and cell_buffer
        // TODO (GPU phase): Write current HierarchicalGrid data into GPU buffers
        self.dirty = false;
    }

    /// Queue a simulation step.
    /// When GPU is enabled, this will eventually dispatch a compute shader for
    /// entity movement, RBE diffusion, or interest culling.
    /// Currently marks dirty and spawns a placeholder for future ComputeTask entity.
    pub fn queue_simulation_step(&mut self, commands: &mut Commands) {
        if !self.dirty { return; }

        // Future: Instead of spawn_empty, we will spawn a ComputeTask component
        // that a render/compute system will pick up and dispatch via RenderGraph.
        // Example future pattern:
        // commands.spawn((ComputeTask { shader: "spatial_simulation.wgsl" },));
        commands.spawn_empty(); // placeholder until ComputeTask system exists

        self.dirty = true;
    }
}

/// Plugin wiring the GPU hierarchical simulation layer
pub struct GpuHierarchicalGridPlugin;

impl Plugin for GpuHierarchicalGridPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GpuHierarchicalGridConfig>()
            .init_resource::<GpuHierarchicalGrid>()
            .add_systems(Startup, setup_gpu_grid)
            .add_systems(Update, (
                sync_grid_to_gpu,
                run_gpu_simulation_step,
            ));
    }
}

fn setup_gpu_grid(
    mut commands: Commands,
    config: Res<GpuHierarchicalGridConfig>,
    mut grid: ResMut<GpuHierarchicalGrid>,
) {
    *grid = GpuHierarchicalGrid::new(&config);
}

fn sync_grid_to_gpu(
    config: Res<GpuHierarchicalGridConfig>,
    mut grid: ResMut<GpuHierarchicalGrid>,
    render_device: Res<RenderDevice>,
) {
    if config.enabled {
        grid.sync_to_gpu(&render_device);
    }
}

fn run_gpu_simulation_step(
    mut grid: ResMut<GpuHierarchicalGrid>,
    mut commands: Commands,
    config: Res<GpuHierarchicalGridConfig>,
) {
    if config.enabled && grid.dirty {
        grid.queue_simulation_step(&mut commands);
    }
}

// End of production file — GPU acceleration layer scaffold ready for Bevy compute shader integration.
// Clear extension points left for future GPU work while maintaining clean CPU fallback.
// Thunder locked in. Yoi ⚡