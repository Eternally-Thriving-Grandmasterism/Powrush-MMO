/*!
 * gpu_simulation::sync
 *
 * Deeper real RBE and Resource data integration.
 *
 * This version improves node_confidences population and provides clear
 * integration points for real harvest and economy systems.
 *
 * AG-SML v1.0
 */

use bevy::prelude::*;
use crate::gpu_simulation::state::GpuSimulationState;
use crate::gpu_simulation::resources::{GlobalConfidence, RbeGlobalState, CouncilValence, MercyAttunement};
use simulation::council_systems::RecentMercyResonance;
use crate::local_player::IsLocalPlayer;
use simulation::council::session as council_session;

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
    council_session_state: Option<Res<council_session::CouncilSessionState>>,
    resource_nodes: Query<&crate::game::resource_nodes::ResourceNode>,
) {
    gpu_state.time = time.elapsed_seconds();
    gpu_state.delta_time = time.delta_seconds();

    if let Some(mercy) = mercy_resonance {
        gpu_state.global_mercy_resonance = mercy.value;
    }
    if let Some(conf) = global_confidence {
        gpu_state.global_confidence = conf.value;
    }

    // === RBE via Bridge (preferred path for real systems) ===
    if let Some(rbe) = rbe_state {
        gpu_state.rbe_flow_rate = rbe.flow_rate;
        gpu_state.total_rbe_circulating = rbe.total_circulating;
        gpu_state.player_rbe_balance = rbe.player_balance;
    }

    // === Council Session ===
    if let Some(session) = council_session_state {
        gpu_state.council_valence = session.valence;
        gpu_state.active_council_action = session.active_action;
        gpu_state.council_participants = session.participant_count;
    } else if let Some(valence) = council_valence {
        gpu_state.council_valence = valence.value;
        gpu_state.active_council_action = valence.active_action;
        gpu_state.council_participants = valence.participants;
    }

    // === Resource Node Confidences (Improved) ===
    // Collect real node confidences. In a full implementation this could be
    // averaged, filtered by distance to player, or use a fixed set of important nodes.
    let mut i = 0;
    for node in &resource_nodes {
        if i < 8 {
            gpu_state.node_confidences[i] = node.confidence;
            i += 1;
        }
    }
    // Fill remaining slots with last known value or 0.0 if not enough nodes
    while i < 8 {
        gpu_state.node_confidences[i] = if i > 0 { gpu_state.node_confidences[i-1] } else { 0.0 };
        i += 1;
    }

    for attunement in &player_mercy_query {
        gpu_state.player_mercy_attunement = attunement.value;
        gpu_state.player_thrivability = attunement.thrivability;
        break;
    }

    // Real player position + velocity
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
