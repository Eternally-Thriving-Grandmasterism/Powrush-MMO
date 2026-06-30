/*!
 * resonance_field.wgsl
 *
 * Area resonance field effect for visualizing global mercy and council state.
 * Useful for large-scale effects, zones, or environmental storytelling.
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
    let resonance = sim.global_mercy_resonance;
    let valence = sim.council_valence;
    let t = sim.time;

    let uv = input.uv * 2.0 - 1.0;
    let dist = length(uv);

    // === Large Scale Field ===
    let field = sin(dist * 6.0 - t * 0.8) * 0.5 + 0.5;
    let field_intensity = field * (0.4 + resonance * 0.6);

    // === Council Modulation ===
    let council_mod = valence * 0.35 * sin(t * 1.2 + dist * 4.0);

    // Base field color
    var color = vec3<f32>(0.3, 0.4, 0.6);

    // Mercy warmth in the field
    color += vec3<f32>(0.4, 0.3, 0.2) * field_intensity;

    // Council cool modulation
    color += vec3<f32>(0.2, 0.35, 0.6) * council_mod;

    // Gentle breathing
    let breathe = sin(t * 0.6) * 0.05;
    color *= (1.0 + breathe);

    // Soft alpha falloff
    let alpha = (0.5 + field_intensity * 0.4) * (1.0 - smoothstep(0.7, 1.3, dist));

    return vec4<f32>(color, alpha);
}
