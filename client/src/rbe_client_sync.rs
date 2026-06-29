/*!
 * Placeholder resources for GpuSimulationState sync
 * TODO: Move these to the simulation crate when ready
 */

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

// Note: These are minimal placeholders so sync_gpu_simulation_state
// can compile and run. Replace them with the real authoritative versions
// from the simulation crate when available.