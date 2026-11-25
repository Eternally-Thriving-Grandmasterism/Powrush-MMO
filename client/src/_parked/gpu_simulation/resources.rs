/*!
 * gpu_simulation::resources
 *
 * Bridge resources for feeding real game state into GpuSimulationState.
 *
 * === RBE Integration Guidance ===
 * Real systems that should write to RbeGlobalState:
 *   - rbe_simulation / rbe_client_sync
 *   - server::rbe_harvest_handler and economy systems
 *   - Any system that computes flow_rate, total_circulating, or player_balance
 *
 * Recommended update pattern:
 *   rbe_state.flow_rate = current_economic_flow;
 *   rbe_state.total_circulating = total_resources_in_economy;
 *   rbe_state.player_balance = local_player_rbe_balance;
 *
 * AG-SML v1.0
 */

use bevy::prelude::*;

#[derive(Resource, Default, Clone, Debug)]
pub struct RbeGlobalState {
    pub flow_rate: f32,
    pub total_circulating: f32,
    pub player_balance: f32,
}

#[derive(Resource, Default, Clone, Debug)]
pub struct CouncilValence {
    pub value: f32,
    pub active_action: u32,
    pub participants: u32,
}

#[derive(Component, Default, Clone, Debug)]
pub struct MercyAttunement {
    pub value: f32,
    pub thrivability: f32,
}

#[derive(Resource, Default, Clone, Debug)]
pub struct GlobalConfidence {
    pub value: f32,
}
