/*!
 * gpu_simulation_state.wgsl
 *
 * Shared include for all GPU simulation driven shaders.
 * Contains the main GpuSimulationState uniform and useful utility functions.
 *
 * AG-SML v1.0
 */

struct GpuSimulationState {
    global_mercy_resonance: f32,
    council_valence: f32,
    rbe_flow_rate: f32,
    total_rbe_circulating: f32,
    player_rbe_balance: f32,
    player_mercy_attunement: f32,
    player_thrivability: f32,
    global_confidence: f32,
    active_council_action: u32,
    council_participants: u32,
    player_position: array<f32, 3>,
    player_velocity: array<f32, 3>,
    node_confidences: array<f32, 8>,
    time: f32,
    delta_time: f32,
};

// ============================================================================
// Utility Functions (usable by all shaders)
// ============================================================================

/// Simple 1D hash-based noise
fn hash(n: f32) -> f32 {
    return fract(sin(n) * 43758.5453);
}

/// Simple 2D noise
fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let a = hash(i.x + hash(i.y));
    let b = hash(i.x + 1.0 + hash(i.y));
    let c = hash(i.x + hash(i.y + 1.0));
    let d = hash(i.x + 1.0 + hash(i.y + 1.0));
    let u = f * f * (3.0 - 2.0 * f);
    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

/// Smooth pulse between 0 and 1
fn pulse(t: f32, frequency: f32, duty: f32) -> f32 {
    return smoothstep(0.0, duty, sin(t * frequency) * 0.5 + 0.5);
}

/// Exponential falloff (useful for glows and auras)
fn exp_falloff(dist: f32, sharpness: f32) -> f32 {
    return exp(-dist * sharpness);
}

/// Remap value from one range to another
fn remap(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    return out_min + (value - in_min) * (out_max - in_min) / (in_max - in_min);
}
