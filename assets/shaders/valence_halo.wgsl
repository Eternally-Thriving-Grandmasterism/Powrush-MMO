/*!
 * valence_halo.wgsl
 *
 * Beautiful, thematic halo/ring effect driven by live GpuSimulationState.
 * Designed for council entities, important structures, player auras, or any object
 * that should visually express PATSAGi Council valence and Mercy resonance.
 *
 * Usage:
 *   - Can be used as a custom material or as a post-process / overlay pass.
 *   - Reuses the same gpu_simulation_state.wgsl include as gpu_state_material.
 *
 * Key driven fields:
 *   - council_valence   → ring count, pulse intensity, rotation speed
 *   - global_mercy_resonance → inner glow warmth and bloom
 *   - time + delta_time   → smooth animation
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
    let valence = sim.council_valence;
    let mercy = sim.global_mercy_resonance;
    let t = sim.time;
    let dt = sim.delta_time;

    let uv = input.uv * 2.0 - 1.0;           // centered UV
    let dist = length(uv);

    // === Council Valence Rings ===
    let ring_count = 3.0 + valence * 2.0;
    let ring = abs(fract(dist * ring_count - t * (1.5 + valence * 1.2)) - 0.5);
    let ring_intensity = smoothstep(0.48, 0.5, ring) * (0.6 + valence * 0.8);

    // === Mercy Inner Glow ===
    let mercy_glow = exp(-dist * 3.5) * (0.5 + mercy * 1.2);
    let warm = vec3<f32>(1.0, 0.85, 0.6);

    // === Base Color + Layering ===
    var color = vec3<f32>(0.4, 0.6, 1.0);     // cool council blue base
    color = mix(color, warm, mercy_glow * 0.6);
    color += vec3<f32>(0.3, 0.6, 1.0) * ring_intensity;

    // Subtle rotation / flowing energy from valence
    let angle = atan2(uv.y, uv.x);
    let flow = sin(angle * 4.0 + t * 2.0) * 0.08 * valence;
    color += vec3<f32>(flow);

    // Soft vignette + alpha
    let alpha = (1.0 - smoothstep(0.85, 1.1, dist)) * (0.7 + mercy * 0.3 + valence * 0.2);

    return vec4<f32>(color, alpha);
}
