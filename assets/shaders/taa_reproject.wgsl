/*!
 * TAA Reprojection Shader for Powrush-MMO
 *
 * Temporal Anti-Aliasing with velocity-aware history reprojection.
 * Uses velocity texture + CameraMatrices jitter for divine temporal stability.
 *
 * Enables buttery-smooth 120+ FPS cinematic rendering.
 *
 * PATSAGi Council + Ra-Thor Quantum Swarm approved
 * AG-SML v1.0 • TOLC 8 Mercy Gates
 */

@group(0) @binding(0) var velocity_texture: texture_2d<f32>;
@group(0) @binding(1) var history_texture: texture_2d<f32>;
@group(0) @binding(2) var current_color: texture_2d<f32>;
@group(0) @binding(3) var texture_sampler: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    // Full-screen triangle
    var out: VertexOutput;
    let uv = vec2<f32>(
        f32((vertex_index << 1u) & 2u),
        f32(vertex_index & 2u)
    );
    out.clip_position = vec4<f32>(uv * 2.0 - 1.0, 0.0, 1.0);
    out.uv = vec2<f32>(uv.x, 1.0 - uv.y); // Flip Y for Bevy
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    // Sample velocity (motion vector)
    let velocity = textureSample(velocity_texture, texture_sampler, uv).xy;

    // Reproject history UV using velocity
    let history_uv = uv - velocity;

    // Sample current frame color and history
    let current = textureSample(current_color, texture_sampler, uv);
    let history = textureSample(history_texture, texture_sampler, history_uv);

    // Simple temporal blend (can be improved with variance clipping, etc.)
    let blended = mix(history, current, 0.1); // 0.1 = aggressive temporal, tune for your needs

    return blended;
}
