/*!
 * gpu_simulation::sync
 *
 * Central sync for feeding real game state into GpuSimulationState.
 *
 * This version has deeper integration attempts for RBE and improved Council wiring.
 *
 * Real authoritative sources:
 * - RBE:      simulation::rbe*, simulation::economy, client rbe_* modules
 * - Council:  simulation::council_systems, server::council_session
 * - Player:   IsLocalPlayer + Transform (already wired)
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

// Try to pull real RBE data if these resources exist in simulation
use simulation::rbe as sim_rbe; // adjust if module structure differs

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
    // Attempt to read real RBE state from simulation if available
    sim_rbe_state: Option<Res<sim_rbe::RbeEconomyState>>,
) {
    gpu_state.time = time.elapsed_seconds();
    gpu_state.delta_time = time.delta_seconds();

    if let Some(mercy) = mercy_resonance {
        gpu_state.global_mercy_resonance = mercy.value;
    }
    if let Some(conf) = global_confidence {
        gpu_state.global_confidence = conf.value;
    }

    // === RBE Data ===
    // Priority 1: Use bridge resource if real systems are already writing to it
    if let Some(rbe) = rbe_state {
        gpu_state.rbe_flow_rate = rbe.flow_rate;
        gpu_state.total_rbe_circulating = rbe.total_circulating;
        gpu_state.player_rbe_balance = rbe.player_balance;
    }
    // Priority 2: Try to pull directly from simulation RBE state if present
    else if let Some(sim_rbe) = sim_rbe_state {
        // Adjust field names to match actual simulation::rbe::RbeEconomyState
        gpu_state.rbe_flow_rate = sim_rbe.flow_rate;
        gpu_state.total_rbe_circulating = sim_rbe.total_resources;
        gpu_state.player_rbe_balance = sim_rbe.player_balance;
    }

    // === Council Valence ===
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

    // Real player position + velocity (wired)
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
