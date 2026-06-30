/*!
 * forgiveness_wave.wgsl
 *
 * Mercy-themed wave effect for forgiveness, redemption, healing, or global mercy events.
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
    let mercy = sim.global_mercy_resonance;
    let t = sim.time;

    let uv = input.uv * 2.0 - 1.0;
    let dist = length(uv);

    // === Mercy Wave ===
    let wave = sin(dist * 5.0 - t * 1.5) * 0.5 + 0.5;
    let wave_intensity = wave * mercy;

    // === Warm Mercy Color ===
    let warm = vec3<f32>(0.9, 0.6, 0.4);

    // Base cool tone
    var color = vec3<f32>(0.4, 0.5, 0.6);

    // Mercy wave overlay
    color = mix(color, warm, wave_intensity * 0.7);

    // Gentle breathing
    let breathe = sin(t * 0.7) * 0.04;
    color *= (1.0 + breathe);

    // Soft alpha
    let alpha = (0.5 + wave_intensity * 0.4) * (1.0 - smoothstep(0.6, 1.2, dist));

    return vec4<f32>(color, alpha);
}
