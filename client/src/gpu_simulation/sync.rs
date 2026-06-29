/*!
 * gpu_simulation::sync
 *
 * Central system that feeds real game state into GpuSimulationState for shaders.
 *
 * RBE and Council data should come from real authoritative systems.
 * This file now has clearer integration points.
 *
 * AG-SML v1.0
 */

use bevy::prelude::*;
use crate::gpu_simulation::state::GpuSimulationState;
use crate::gpu_simulation::resources::{GlobalConfidence, RbeGlobalState, CouncilValence, MercyAttunement};
use simulation::council_systems::RecentMercyResonance;
use crate::local_player::IsLocalPlayer;

#[derive(Resource, Default)]
pub struct PreviousLocalPlayerPosition {
    pub position: Option<Vec3>,
}

pub fn sync_gpu_simulation_state(
    mut gpu_state: ResMut<GpuSimulationState>,
    time: Res<Time>,
    mercy_resonance: Option<Res<RecentMercyResonance>>,
    global_confidence: Option<Res<GlobalConfidence>>,
    rbe_state: Option<Res<RbeGlobalState>>,
    council_valence: Option<Res<CouncilValence>>,
    player_mercy_query: Query<&MercyAttunement>,
    local_player_query: Query<&Transform, With<IsLocalPlayer>>,
    mut prev_pos: ResMut<PreviousLocalPlayerPosition>,
) {
    gpu_state.time = time.elapsed_seconds();
    gpu_state.delta_time = time.delta_seconds();

    if let Some(mercy) = mercy_resonance {
        gpu_state.global_mercy_resonance = mercy.value;
    }
    if let Some(conf) = global_confidence {
        gpu_state.global_confidence = conf.value;
    }

    // === RBE - Real data should be written to RbeGlobalState by rbe systems ===
    if let Some(rbe) = rbe_state {
        gpu_state.rbe_flow_rate = rbe.flow_rate;
        gpu_state.total_rbe_circulating = rbe.total_circulating;
        gpu_state.player_rbe_balance = rbe.player_balance;
    }

    // === Council Valence - Real data from council_session / PATSAGi ===
    if let Some(valence) = council_valence {
        gpu_state.council_valence = valence.value;
        gpu_state.active_council_action = valence.active_action;
        gpu_state.council_participants = valence.participants;
    }

    for attunement in &player_mercy_query {
        gpu_state.player_mercy_attunement = attunement.value;
        gpu_state.player_thrivability = attunement.thrivability;
        break;
    }

    // Real player position + velocity (already wired)
    for transform in &local_player_query {
        let current_pos = transform.translation;
        gpu_state.player_position = [current_pos.x, current_pos.y, current_pos.z];

        if let Some(prev) = prev_pos.position {
            let delta = current_pos - prev;
            let vel = if time.delta_seconds() > 0.0001 {
                delta / time.delta_seconds()
            } else {
                Vec3::ZERO
            };
            gpu_state.player_velocity = [vel.x, vel.y, vel.z];
        }
        prev_pos.position = Some(current_pos);
        break;
    }
}
