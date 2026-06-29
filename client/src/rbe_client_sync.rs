/*!
 * Higher-level sync system for GpuSimulationState
 */

/// Automatically syncs important game data into GpuSimulationState every frame.
/// This is the recommended way to keep GPU state up to date.
pub fn sync_gpu_simulation_state(
    mut gpu_state: ResMut<GpuSimulationState>,
    time: Res<Time>,
    // TODO: Add your actual resources here, for example:
    // mercy: Res<RecentMercyResonance>,
    // confidence: Res<GlobalConfidence>,
    // player_query: Query<&Transform, With<Player>>,
) {
    gpu_state.time = time.elapsed_seconds();
    gpu_state.delta_time = time.delta_seconds();

    // Example of pulling from other resources (uncomment when ready):
    // gpu_state.global_mercy_resonance = mercy.value;
    // gpu_state.global_confidence = confidence.value;

    // if let Ok(transform) = player_query.get_single() {
    //     gpu_state.player_position = transform.translation.to_array();
    // }
}

// Add this system to your app after GpuSimulationStatePlugin:
// .add_systems(Update, sync_gpu_simulation_state)