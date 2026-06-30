/*!
 * energy_burst.wgsl
 *
 * Lightweight, reusable energy burst effect.
 * Suitable for ability activations, harvesting bursts, impacts, or special effects.
 *
 * Reacts to RBE, mercy, council valence, and player velocity.
 *
 * AG-SML v1.0
 */

#import "shaders/include/gpu_simulation_state.wgsl"

@group(0) @binding(0)
var<uniform> sim: GpuSimulationState;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vertex_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(input.position, 1.0);
    output.uv = input.uv;
    return output;
}

@fragment
fn fragment_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let rbe = sim.rbe_flow_rate;
    let mercy = sim.global_mercy_resonance;
    let valence = sim.council_valence;
    let t = sim.time;
    let speed = length(vec3<f32>(
        sim.player_velocity[0],
        sim.player_velocity[1],
        sim.player_velocity[2]
    ));

    let uv = input.uv * 2.0 - 1.0;
    let dist = length(uv);

    // === Core Burst ===
    let core = exp_falloff(dist, 5.0) * (0.8 + rbe * 0.4);

    // === Mercy Ring ===
    let mercy_ring = pulse(t * 3.0, 1.0, 0.4) * mercy * smoothstep(0.4, 0.9, dist);

    // === Council Accent ===
    let council_accent = valence * 0.3 * smoothstep(0.6, 1.1, dist);

    // === Velocity Energy ===
    let vel_energy = speed * 0.15 * (1.0 - dist);

    var color = vec3<f32>(0.4, 0.6, 0.9);
    color += vec3<f32>(0.6, 0.8, 0.5) * core;
    color += vec3<f32>(0.9, 0.5, 0.3) * mercy_ring;
    color += vec3<f32>(0.3, 0.5, 0.9) * council_accent;
    color += vec3<f32>(0.5, 0.7, 0.4) * vel_energy;

    // Subtle breathing
    let breathe = sin(t * 2.5) * 0.06;
    color *= (1.0 + breathe);

    let alpha = core * 0.9 + mercy_ring * 0.5 + (1.0 - dist) * 0.3;

    return vec4<f32>(color, alpha);
}
