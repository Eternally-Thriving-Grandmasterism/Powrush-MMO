/*!
 * gpu_simulation::resources
 *
 * Client-side bridge resources for GpuSimulationState.
 *
 * Real authoritative sources should update these every frame or on change:
 *
 * RBE:
 *   - simulation::rbe* or client rbe_client_sync / rbe_simulation
 *   - server::rbe_harvest_handler, rbe_integration
 *
 * Council:
 *   - simulation::council_systems, server::council_session
 *
 * Mercy:
 *   - simulation::council_systems::RecentMercyResonance + MercyAttunement
 *
 * AG-SML v1.0
 */

use bevy::prelude::*;

/// RBE state visible to GPU shaders/materials.
/// 
/// Real systems should write here:
///   flow_rate         <- current economic flow / throughput
///   total_circulating <- total resources in the economy
///   player_balance    <- local player's current balance
#[derive(Resource, Default, Clone, Debug)]
pub struct RbeGlobalState {
    pub flow_rate: f32,
    pub total_circulating: f32,
    pub player_balance: f32,
}

/// Council valence/activity for visual resonance.
/// Real systems (council_session, PATSAGi) should update this.
#[derive(Resource, Default, Clone, Debug)]
pub struct CouncilValence {
    pub value: f32,
    pub active_action: u32,
    pub participants: u32,
}

/// Per-entity mercy/thrivability.
#[derive(Component, Default, Clone, Debug)]
pub struct MercyAttunement {
    pub value: f32,
    pub thrivability: f32,
}

/// Global world confidence (safety, harmony, prediction certainty).
#[derive(Resource, Default, Clone, Debug)]
pub struct GlobalConfidence {
    pub value: f32,
}
