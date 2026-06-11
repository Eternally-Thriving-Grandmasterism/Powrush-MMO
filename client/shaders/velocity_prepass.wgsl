/*!
 * Velocity Prepass Shader v18.15+
 * Outputs per-pixel motion vectors for temporal techniques (SSR, TAA, etc.).
 *
 * Outputs velocity in RG channels (Rg16Float target recommended).
 * For rigid objects: velocity = current_world_pos - previous_world_pos
 *
 * Note: For skinned meshes you would need previous bone matrices.
 */

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) normal: vec3<f32>,
};

struct InstanceInput {
    @location(3) model_matrix_0: vec4<f32>,
    @location(4) model_matrix_1: vec4<f32>,
    @location(5) model_matrix_2: vec4<f32>,
    @location(6) model_matrix_3: vec4<f32>,
    // Add previous model matrix here when using instancing
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) prev_world_pos: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> view_proj: mat4x4<f32>;

@group(0) @binding(1)
var<uniform> prev_view_proj: mat4x4<f32>;

// For per-object previous transform, this would come from instance data or push constants
@group(1) @binding(0)
var<uniform> model: mat4x4<f32>;

@group(1) @binding(1)
var<uniform> prev_model: mat4x4<f32>;

@vertex
fn vs_main(
    model: VertexInput,
    @builtin(instance_index) instance_index: u32,
) -> VertexOutput {
    var out: VertexOutput;

    let model_matrix = model; // Replace with instance data when using instancing
    let prev_model_matrix = prev_model;

    let current_world_pos = model_matrix * vec4<f32>(model.position, 1.0);
    let prev_world_pos = prev_model_matrix * vec4<f32>(model.position, 1.0);

    out.position = view_proj * current_world_pos;
    out.world_pos = current_world_pos.xyz;
    out.prev_world_pos = prev_world_pos.xyz;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec2<f32> {
    let velocity = in.world_pos - in.prev_world_pos;
    return velocity.xy; // Store XY velocity in RG channels
}
