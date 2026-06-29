/*!
 * gpu_state_material.wgsl
 * 
 * Vertex + Fragment shader for GpuStateMaterial.
 * Binds GpuSimulationState at group 0 and material data at group 1.
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
    // Example: Modulate color by global mercy resonance
    let resonance = sim.global_mercy_resonance;
    let base = material.base_color.rgb;
    let final_color = base * (0.6 + resonance * 0.4);

    return vec4<f32>(final_color, material.base_color.a);
}

struct GpuStateMaterialUniform {
    base_color: vec4<f32>,
};