/*!
 * Placeholder resources for GpuSimulationState sync
 * TODO: Move to simulation crate when ready
 */

use bevy::prelude::*;

#[derive(Resource, Default, Clone, Debug)]
pub struct GlobalConfidence {
    pub value: f32,
}

#[derive(Resource, Default, Clone, Debug)]
pub struct RbeGlobalState {
    pub flow_rate: f32,
    pub total_circulating: f32,
    pub player_balance: f32,
}

#[derive(Component, Default, Clone, Debug)]
pub struct MercyAttunement {
    pub value: f32,
    pub thrivability: f32,
}

#[derive(Resource, Default, Clone, Debug)]
pub struct CouncilValence {
    pub value: f32,
    pub active_action: u32,
    pub participants: u32,
}

// These are minimal placeholders.
// Replace with real authoritative versions from simulation crate when available.