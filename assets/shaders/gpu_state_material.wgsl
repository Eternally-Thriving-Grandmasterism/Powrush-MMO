/*!
 * gpu_state_material.wgsl
 *
 * Advanced visual material driven by live GpuSimulationState.
 * Now using shared utility functions for more organic effects.
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
    let alpha = material.base_color.a;

    // === Mercy Resonance Core ===
    let mercy_core = 0.5 + resonance * 0.7;
    var color = base * mercy_core;

    // === Council Valence Aura Rings (using pulse utility) ===
    let ring_pulse = pulse(t * 1.6 + valence * 5.0, 1.0, 0.6);
    let valence_aura = valence * ring_pulse * 0.55;
    color += vec3<f32>(valence_aura * 0.15, valence_aura * 0.6, valence_aura * 1.0);

    // === RBE Flow Tendrils with noise variation ===
    let noise_val = noise(input.uv * 3.0 + t * 0.5);
    let flow_wave = sin(input.uv.x * 14.0 + t * 3.0 + rbe * 4.0 + noise_val * 2.0) * 0.5 + 0.5;
    let rbe_energy = rbe * flow_wave * 0.42;
    color = mix(color, color * vec3<f32>(1.15, 0.92, 0.8), rbe_energy);

    // === Player Velocity Response ===
    let vel_influence = speed * 0.08;
    color += vec3<f32>(vel_influence * 0.3, vel_influence * 0.2, vel_influence * 0.5);

    // === Time Breathing ===
    let breathe = sin(t * 1.1) * 0.035 * (1.0 + dt * 10.0);
    color *= (1.0 + breathe);

    // === Confidence Vibrancy ===
    let vib = 0.85 + confidence * 0.25;
    let lum = dot(color, vec3<f32>(0.299, 0.587, 0.114));
    color = mix(vec3<f32>(lum), color, vib);

    // Mercy + Council crown highlight
    let crown = pow(max(resonance, valence * 0.65), 2.0) * 0.28;
    color += vec3<f32>(crown * 0.5, crown * 0.95, crown * 1.1);

    color = clamp(color, vec3<f32>(0.0), vec3<f32>(2.5));

    return vec4<f32>(color, alpha);
}

struct GpuStateMaterialUniform {
    base_color: vec4<f32>,
};
