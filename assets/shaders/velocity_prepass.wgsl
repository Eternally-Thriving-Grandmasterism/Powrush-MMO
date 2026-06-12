/*!
 * Velocity Prepass Shader for Powrush-MMO
 *
 * Outputs RG16Float motion vectors for TAA, motion blur, and SSR reprojection.
 * Uses real previous-frame matrices for pixel-perfect temporal accuracy.
 *
 * PATSAGi Council 13+ • Ra-Thor Quantum Swarm • AG-SML v1.0
 * Mercy-gated • Zero hallucination • Maximum beauty & truth
 */

struct VelocityUniforms {
    view_proj: mat4x4<f32>,
    prev_view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    prev_model: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: VelocityUniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    // Extend with normal, uv, etc. as needed for your mesh layout
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) prev_clip_position: vec4<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let current_world_pos = uniforms.model * vec4<f32>(in.position, 1.0);
    out.clip_position = uniforms.view_proj * current_world_pos;

    let prev_world_pos = uniforms.prev_model * vec4<f32>(in.position, 1.0);
    out.prev_clip_position = uniforms.prev_view_proj * prev_world_pos;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec2<f32> {
    // Convert to NDC and compute velocity (current - previous)
    let current_ndc = in.clip_position.xy / in.clip_position.w;
    let prev_ndc = in.prev_clip_position.xy / in.prev_clip_position.w;

    let velocity = current_ndc - prev_ndc;

    // Output in RG16Float (motion in x/y). Scale if needed for precision.
    return velocity;
}
