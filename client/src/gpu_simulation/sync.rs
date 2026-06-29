/*!
 * Sync system for GpuSimulationState
 */

use bevy::prelude::*;
use crate::gpu_simulation::state::GpuSimulationState;
use crate::gpu_simulation::resources::{GlobalConfidence, RbeGlobalState, CouncilValence};
use simulation::council_systems::RecentMercyResonance;

pub fn sync_gpu_simulation_state(
    mut gpu_state: ResMut<GpuSimulationState>,
    time: Res<Time>,
    mercy_resonance: Option<Res<RecentMercyResonance>>,
    global_confidence: Option<Res<GlobalConfidence>>,
    rbe_state: Option<Res<RbeGlobalState>>,
    council_valence: Option<Res<CouncilValence>>,
) {
    gpu_state.time = time.elapsed_seconds();
    gpu_state.delta_time = time.delta_seconds();

    if let Some(mercy) = mercy_resonance {
        gpu_state.global_mercy_resonance = mercy.value;
    }

    if let Some(conf) = global_confidence {
        gpu_state.global_confidence = conf.value;
    }

    if let Some(rbe) = rbe_state {
        gpu_state.rbe_flow_rate = rbe.flow_rate;
        gpu_state.total_rbe_circulating = rbe.total_circulating;
        gpu_state.player_rbe_balance = rbe.player_balance;
    }

    if let Some(valence) = council_valence {
        gpu_state.council_valence = valence.value;
        gpu_state.active_council_action = valence.active_action;
        gpu_state.council_participants = valence.participants;
    }
}