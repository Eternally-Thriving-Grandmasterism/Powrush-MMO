/*!
 * Expanded GpuSimulationState with Council, RBE, and Player data
 */

#[repr(C)]
#[derive(Resource, Clone, Debug)]
pub struct GpuSimulationState {
    pub hotbar: [HotbarSlot; 8],
    pub node_confidences: [f32; 8],

    // Existing
    pub global_mercy_resonance: f32,
    pub global_confidence: f32,
    pub player_position: [f32; 3],
    pub time: f32,
    pub delta_time: f32,

    // === Council State ===
    pub council_valence: f32,
    pub active_council_action: u32,        // enum index
    pub council_participants: u32,

    // === RBE / Economy ===
    pub rbe_flow_rate: f32,
    pub total_rbe_circulating: f32,
    pub player_rbe_balance: f32,

    // === Player State ===
    pub player_velocity: [f32; 3],
    pub player_mercy_attunement: f32,
    pub player_thrivability: f32,

    pub _padding: [u32; 1], // maintain alignment
}