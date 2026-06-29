/*!
 * gpu_simulation_state.wgsl - Updated with Council, RBE, and Player fields
 */

struct HotbarSlot {
    count: u32,
    cooldown_remaining: f32,
};

struct GpuSimulationState {
    hotbar: array<HotbarSlot, 8>,
    node_confidences: array<f32, 8>,

    global_mercy_resonance: f32,
    global_confidence: f32,
    player_position: vec3<f32>,
    time: f32,
    delta_time: f32,

    // Council
    council_valence: f32,
    active_council_action: u32,
    council_participants: u32,

    // RBE
    rbe_flow_rate: f32,
    total_rbe_circulating: f32,
    player_rbe_balance: f32,

    // Player
    player_velocity: vec3<f32>,
    player_mercy_attunement: f32,
    player_thrivability: f32,
};