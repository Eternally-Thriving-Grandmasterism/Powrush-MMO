/*!
 * Expanded sync system with Council, RBE, and Player integration
 */

pub fn sync_gpu_simulation_state(
    mut gpu_state: ResMut<GpuSimulationState>,
    time: Res<Time>,
    mercy_resonance: Option<Res<RecentMercyResonance>>,
    global_confidence: Option<Res<GlobalConfidence>>,
    // TODO: Add these resources when available in your game
    // council_valence: Option<Res<CouncilValence>>,
    // rbe_state: Option<Res<RbeGlobalState>>,
    // player_query: Query<(&Transform, &Velocity, &PlayerMercyAttunement), With<Player>>,
) {
    gpu_state.time = time.elapsed_seconds();
    gpu_state.delta_time = time.delta_seconds();

    if let Some(mercy) = mercy_resonance {
        gpu_state.global_mercy_resonance = mercy.value;
    }

    if let Some(conf) = global_confidence {
        gpu_state.global_confidence = conf.value;
    }

    // Example extensions (uncomment and adapt when resources exist):
    //
    // if let Some(valence) = council_valence {
    //     gpu_state.council_valence = valence.value;
    //     gpu_state.active_council_action = valence.active_action as u32;
    //     gpu_state.council_participants = valence.participants;
    // }
    //
    // if let Some(rbe) = rbe_state {
    //     gpu_state.rbe_flow_rate = rbe.flow_rate;
    //     gpu_state.total_rbe_circulating = rbe.total_circulating;
    //     gpu_state.player_rbe_balance = rbe.player_balance;
    // }
    //
    // if let Ok((transform, velocity, attunement)) = player_query.get_single() {
    //     gpu_state.player_position = transform.translation.to_array();
    //     gpu_state.player_velocity = velocity.0.to_array();
    //     gpu_state.player_mercy_attunement = attunement.value;
    // }
}