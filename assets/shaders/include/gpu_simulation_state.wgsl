/*!
 * gpu_simulation_state.wgsl
 * 
 * WGSL struct definitions matching Rust GpuSimulationState.
 * Include this in your shaders with:
 *   #import "shaders/include/gpu_simulation_state.wgsl"
 * 
 * AG-SML v1.0
 */

struct HotbarSlot {
    count: u32,
    cooldown_remaining: f32,
};

struct GpuSimulationState {
    hotbar: array<HotbarSlot, 8>,
    node_confidences: array<f32, 8>,
};

// Usage in shader:
// @group(0) @binding(0)
// var<uniform> sim_state: GpuSimulationState;