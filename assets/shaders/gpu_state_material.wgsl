/*!
 * gpu_state_material.wgsl
 *
 * Optimized version with reduced instruction count and better math efficiency.
 *
 * Optimization techniques applied:
 * - Cached repeated sin() calls
 * - Reduced pow() usage
 * - Simplified mix chains where possible
 * - Precomputed common terms
 *
 * AG-SML v1.0
 */

#import "shaders/include/gpu_simulation_state.wgsl"

@group(0) @binding(0)
var<uniform> sim: GpuSimulationState;

@group(1) @binding(1)
var<uniform> material: GpuStateMaterialUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_pos: vec3<f32>,
};

@vertex
fn vertex_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(input.position, 1.0);
    output.uv = input.uv;
    output.world_pos = input.position;
    return output;
}

@fragment
fn fragment_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let resonance = sim.global_mercy_resonance;
    let valence = sim.council_valence;
    let rbe = sim.rbe_flow_rate;
    let confidence = sim.global_confidence;
    let t = sim.time;
    let dt = sim.delta_time;

    let vel = vec3<f32>(sim.player_velocity[0], sim.player_velocity[1], sim.player_velocity[2]);
    let speed = length(vel);

    let base = material.base_color.rgb;

    // === Cached common terms ===
    let t_sin = sin(t);
    let t_cos = cos(t);

    // === Mercy Core ===
    let mercy_core = 0.5 + resonance * 0.7;
    var color = base * mercy_core;

    // === Council Rings (optimized) ===
    let ring_phase = t * 1.6 + valence * 5.0;
    let ring = abs(sin(ring_phase) * 0.5 + 0.5);
    let valence_aura = valence * ring * 0.55;
    color += vec3<f32>(valence_aura * 0.15, valence_aura * 0.6, valence_aura * 1.0);

    // === RBE Tendrils (reduced trig) ===
    let noise_val = noise(input.uv * 3.0 + t * 0.5);
    let flow_arg = input.uv.x * 14.0 + t * 3.0 + rbe * 4.0 + noise_val * 2.0;
    let flow_wave = sin(flow_arg) * 0.5 + 0.5;
    let rbe_energy = rbe * flow_wave * 0.42;
    color = mix(color, color * vec3<f32>(1.15, 0.92, 0.8), rbe_energy);

    // === Velocity ===
    let vel_influence = speed * 0.08;
    color += vec3<f32>(vel_influence * 0.3, vel_influence * 0.2, vel_influence * 0.5);

    // === Breathing (cached) ===
    let breathe = t_sin * 0.035 * (1.0 + dt * 10.0);
    color *= (1.0 + breathe);

    // === Confidence ===
    let vib = 0.85 + confidence * 0.25;
    let lum = dot(color, vec3<f32>(0.299, 0.587, 0.114));
    color = mix(vec3<f32>(lum), color, vib);

    // === Crown (avoided extra pow where possible) ===
    let crown_base = max(resonance, valence * 0.65);
    let crown = crown_base * crown_base * 0.28; // cheaper than pow(x, 2.0)
    color += vec3<f32>(crown * 0.5, crown * 0.95, crown * 1.1);

    color = clamp(color, vec3<f32>(0.0), vec3<f32>(2.5));

    return vec4<f32>(color, material.base_color.a);
}

struct GpuStateMaterialUniform {
    base_color: vec4<f32>,
};
