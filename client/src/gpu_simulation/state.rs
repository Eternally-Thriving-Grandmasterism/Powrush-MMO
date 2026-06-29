/*!
 * Update GpuSimulationStatePlugin to use locally defined functions
 */

pub struct GpuSimulationStatePlugin;

impl Plugin for GpuSimulationStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GpuSimulationState>()
            .add_systems(Startup, (
                setup_gpu_simulation_buffer,
                setup_gpu_simulation_storage_buffer,
            ))
            .add_systems(Update, (
                upload_gpu_simulation_state,
                upload_gpu_simulation_state_storage,
            ));
    }
}