/*!
 * gpu_simulation::resources
 *
 * Client-side bridge resources for synchronizing live game state
 * into GpuSimulationState (for shaders/materials).
 *
 * These are intentionally lightweight. Authoritative logic lives in:
 *   - simulation crate (council_systems, rbe, economy, player)
 *   - server crate (authoritative RBE, council sessions, harvesting)
 *   - client systems (local player, prediction, rbe_client_sync, council feedback)
 *
 * Real systems should update these resources each frame (or on change).
 * The sync_gpu_simulation_state system then copies them to the GPU uniform.
 *
 * AG-SML v1.0
 */

use bevy::prelude::*;

/// Global confidence / certainty in the current world state.
/// Updated by: safety_net, anomaly detection, or council harmony systems.
#[derive(Resource, Default, Clone, Debug)]
pub struct GlobalConfidence {
    pub value: f32,
}

/// Aggregated RBE (Resource-Based Economy) state visible to GPU.
/// Updated by: rbe_client_sync, rbe_simulation, or server replication.
#[derive(Resource, Default, Clone, Debug)]
pub struct RbeGlobalState {
    pub flow_rate: f32,
    pub total_circulating: f32,
    pub player_balance: f32,
}

/// Per-entity or player mercy/thrivability attunement.
/// Updated by: mercy systems, ascension, or player progression.
#[derive(Component, Default, Clone, Debug)]
pub struct MercyAttunement {
    pub value: f32,
    pub thrivability: f32,
}

/// Council valence / activity state for visual resonance.
/// Updated by: council_session, council_mercy_trial, or PATSAGi replication.
#[derive(Resource, Default, Clone, Debug)]
pub struct CouncilValence {
    pub value: f32,
    pub active_action: u32,
    pub participants: u32,
}
