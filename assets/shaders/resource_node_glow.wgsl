/*!
 * resource_node_glow.wgsl
 *
 * Dedicated glow effect for resource nodes.
 * Reacts to node confidence, RBE flow, mercy resonance, and council valence.
 *
 * Perfect for highlighting harvestable resources and economy nodes.
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
    let confidence = sim.node_confidences[0]; // Primary node confidence
    let rbe = sim.rbe_flow_rate;
    let mercy = sim.global_mercy_resonance;
    let valence = sim.council_valence;
    let t = sim.time;

    let uv = input.uv;
    let dist = length(uv - 0.5);

    // === Core Glow based on Confidence + RBE ===
    let core = exp(-dist * 4.5) * (0.5 + confidence * 0.8 + rbe * 0.3);

    // === Mercy Warm Pulse ===
    let mercy_pulse = sin(t * 1.8) * 0.5 + 0.5;
    let mercy_glow = mercy * mercy_pulse * 0.6;

    // === Council Valence Rim ===
    let valence_rim = smoothstep(0.65, 0.95, dist) * valence * 0.5;

    // Base node color (warm resource feel)
    var color = vec3<f32>(0.6, 0.45, 0.25);

    // Core glow
    color += vec3<f32>(0.8, 0.6, 0.3) * core;

    // Mercy warmth
    color += vec3<f32>(0.9, 0.5, 0.25) * mercy_glow;

    // Council rim
    color += vec3<f32>(0.3, 0.5, 0.9) * valence_rim;

    // Gentle breathing
    let breathe = sin(t * 1.2) * 0.04;
    color *= (1.0 + breathe);

    // Alpha with confidence influence
    let alpha = (0.6 + core * 0.5) * (0.7 + confidence * 0.3);

    return vec4<f32>(color, alpha);
}
