/*!
 * Real game system integration into GpuSimulationState
 */

use simulation::council_systems::RecentMercyResonance;
use simulation::game_state::GlobalConfidence; // adjust path if needed

/// Production-ready sync system that pulls from actual game resources.
pub fn sync_gpu_simulation_state(
    mut gpu_state: ResMut<GpuSimulationState>,
    time: Res<Time>,
    mercy_resonance: Option<Res<RecentMercyResonance>>,
    global_confidence: Option<Res<GlobalConfidence>>,
    // TODO: Add player query when you have a Player component
    // player_q: Query<&Transform, With<Player>>,
) {
    gpu_state.time = time.elapsed_seconds();
    gpu_state.delta_time = time.delta_seconds();

    if let Some(mercy) = mercy_resonance {
        gpu_state.global_mercy_resonance = mercy.value;
    }

    if let Some(conf) = global_confidence {
        gpu_state.global_confidence = conf.value;
    }

    // Example player position sync (uncomment when ready):
    // if let Ok(transform) = player_q.get_single() {
    //     gpu_state.player_position = transform.translation.to_array();
    // }

    // Future: Add council state, RBE flow values, etc.
}