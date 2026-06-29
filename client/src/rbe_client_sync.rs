/*!
 * GpuSimulationStatePlugin
 * 
 * Clean, reusable plugin that manages:
 * - GpuSimulationState resource
 * - GPU Uniform + Storage buffers
 * - Dirty-checked upload systems
 * 
 * Usage:
 *   app.add_plugins(GpuSimulationStatePlugin);
 */

pub struct GpuSimulationStatePlugin;

impl Plugin for GpuSimulationStatePlugin {
    fn build(&self, app: &mut App) {
        app
            // CPU-side state
            .init_resource::<GpuSimulationState>()

            // GPU resources
            .add_systems(Startup, (
                setup_gpu_simulation_buffer,
                setup_gpu_simulation_storage_buffer,
            ))

            // Dirty-checked uploads (only when data changes)
            .add_systems(Update, (
                upload_gpu_simulation_state,
                upload_gpu_simulation_state_storage,
            ));
    }
}

// After adding the plugin, you can access:
// - Res<GpuSimulationState>          (CPU side)
// - Res<GpuSimulationStateBuffer>     (Uniform)
// - Res<GpuSimulationStateStorageBuffer> (Storage)