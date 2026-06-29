/*!
 * Improved dirty-checking strategies for GpuSimulationState
 */

/// Version counter for more granular dirty checking.
/// Increment this when you manually update sections of the state.
#[derive(Resource, Default)]
pub struct GpuSimulationStateVersion {
    pub version: u64,
}

/// Call this after making significant manual changes to GpuSimulationState
/// if you want fine-grained control over GPU uploads.
pub fn mark_gpu_state_dirty(
    mut version: ResMut<GpuSimulationStateVersion>,
) {
    version.version = version.version.wrapping_add(1);
}

// Alternative: Use Bevy's built-in change detection on specific sections.
// The current is_changed() on GpuSimulationState already works well
// because we update it in a single system (sync_gpu_simulation_state).
// For even better performance, you can split the state into multiple
// smaller resources (e.g. GpuHotbarState, GpuCouncilState, etc.).