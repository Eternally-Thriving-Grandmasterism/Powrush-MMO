/*!
 * gpu_state_material.wgsl
 * 
 * Vertex + Fragment shader for GpuStateMaterial.
 * Binds full GpuSimulationState (recovered & complete) at group 0.
 * Demonstrates multiple live fields for visible validation of the GPU simulation pipeline.
 * 
 * Visual effects:
 * - Mercy resonance: base color intensity + warm glow
 * - Council valence: pulsing emissive highlight
 * - RBE flow: subtle hue shift + energy animation
 * - Time + delta: breathing / wave motion
 * - Global confidence: overall vibrancy
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
    let rbe_flow = sim.rbe_flow_rate;
    let confidence = sim.global_confidence;
    let t = sim.time;
    let dt = sim.delta_time;

    // Base from material
    let base = material.base_color.rgb;
    let alpha = material.base_color.a;

    // Mercy resonance: warm intensity boost + soft glow
    let mercy_glow = 0.6 + resonance * 0.5;
    var color = base * mercy_glow;

    // Council valence: pulsing emissive rim (breathing effect)
    let pulse = sin(t * 2.0 + valence * 3.0) * 0.5 + 0.5;
    let valence_boost = valence * pulse * 0.35;
    color = color + vec3<f32>(valence_boost * 0.3, valence_boost * 0.6, valence_boost * 0.9);

    // RBE flow: subtle hue shift + flowing energy
    let hue_shift = rbe_flow * 0.08;
    color = vec3<f32>(
        color.r * (1.0 + hue_shift),
        color.g * (1.0 - hue_shift * 0.5),
        color.b * (1.0 + hue_shift * 0.3)
    );

    // Time-based subtle animation (gentle breathing / wave on UV)
    let wave = sin(input.uv.x * 6.28318 + t * 1.5) * 0.03 * dt * 10.0;
    color = color * (1.0 + wave);

    // Global confidence: overall vibrancy / saturation boost
    let vibrancy = 0.85 + confidence * 0.25;
    color = mix(vec3<f32>(dot(color, vec3<f32>(0.299, 0.587, 0.114))), color, vibrancy);

    // Soft clamp
    color = clamp(color, vec3<f32>(0.0), vec3<f32>(2.0));

    return vec4<f32>(color, alpha);
}

struct GpuStateMaterialUniform {
    base_color: vec4<f32>,
};
