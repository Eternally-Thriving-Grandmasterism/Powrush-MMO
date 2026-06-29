/*!
 * gpu_simulation_state.wgsl
 * 
 * Matching WGSL definition for Rust GpuSimulationState.
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
};

// ==================== USAGE EXAMPLE ====================

// In your shader:
//
// @group(0) @binding(0)
// var<uniform> sim: GpuSimulationState;
//
// Then access:
// let resonance = sim.global_mercy_resonance;
// let node3_conf = sim.node_confidences[2];
// let player_x = sim.player_position.x;