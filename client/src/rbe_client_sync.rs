/*!
 * sync_gpu_simulation_state with recommended real resource wiring
 * (based on codebase patterns - adjust paths as needed)
 */

use simulation::council_systems::RecentMercyResonance;
// use simulation::game_state::GlobalConfidence;
// use simulation::rbe::RbeGlobalState;
// use simulation::player::{Player, MercyAttunement};

pub fn sync_gpu_simulation_state(
    mut gpu_state: ResMut<GpuSimulationState>,
    time: Res<Time>,
    mercy_resonance: Option<Res<RecentMercyResonance>>,
    // global_confidence: Option<Res<GlobalConfidence>>,
    // rbe_state: Option<Res<RbeGlobalState>>,
    // player_q: Query<(&Transform, &Velocity, &MercyAttunement), With<Player>>,
) {
    gpu_state.time = time.elapsed_seconds();
    gpu_state.delta_time = time.delta_seconds();

    if let Some(mercy) = mercy_resonance {
        gpu_state.global_mercy_resonance = mercy.value;
    }

    // Global Confidence
    // if let Some(conf) = global_confidence {
    //     gpu_state.global_confidence = conf.value;
    // }

    // RBE State
    // if let Some(rbe) = rbe_state {
    //     gpu_state.rbe_flow_rate = rbe.flow_rate;
    //     gpu_state.total_rbe_circulating = rbe.total_circulating;
    //     gpu_state.player_rbe_balance = rbe.player_balance;
    // }

    // Player State
    // if let Ok((transform, velocity, attunement)) = player_q.get_single() {
    //     gpu_state.player_position = transform.translation.to_array();
    //     gpu_state.player_velocity = velocity.0.to_array();
    //     gpu_state.player_mercy_attunement = attunement.value;
    // }
}