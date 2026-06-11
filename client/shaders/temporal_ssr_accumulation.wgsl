/*!
 * Temporal SSR Accumulation Shader
 * Reduces noise in Screen Space Reflections using temporal reprojection.
 *
 * Run this as a second pass after the main SSR ray marching pass.
 */

struct TemporalUniforms {
    camera_position: vec3<f32>,
    prev_camera_position: vec3<f32>,
    view: mat4x4<f32>,
    inv_view: mat4x4<f32>,
    projection: mat4x4<f32>,
    inv_projection: mat4x4<f32>,
    prev_view: mat4x4<f32>,
    prev_projection: mat4x4<f32>,
    blend_factor: f32,        // 0.9 = strong accumulation, lower = more responsive
};

@group(0) @binding(0)
var<uniform> uniforms: TemporalUniforms;

@group(0) @binding(1)
var current_ssr: texture_2d<f32>;
@group(0) @binding(2)
var history_ssr: texture_2d<f32>;
@group(0) @binding(3)
var depth_texture: texture_depth_2d;
@group(0) @binding(4)
var linear_sampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@location(0) position: vec2<f32>) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(position, 0.0, 1.0);
    out.uv = position * 0.5 + 0.5;
    out.uv.y = 1.0 - out.uv.y;
    return out;
}

// Reproject a screen UV from current frame to previous frame
fn reproject_uv(uv: vec2<f32>, depth: f32) -> vec2<f32> {
    let ndc = vec4<f32>(uv * 2.0 - 1.0, depth, 1.0);
    let view_pos = uniforms.inv_projection * ndc;
    let world_pos = uniforms.inv_view * view_pos;

    let prev_clip = uniforms.prev_projection * uniforms.prev_view * world_pos;
    let prev_ndc = prev_clip.xyz / prev_clip.w;
    return prev_ndc.xy * 0.5 + 0.5;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let depth = textureSample(depth_texture, linear_sampler, uv);

    if (depth >= 1.0) {
        return vec4<f32>(0.0);
    }

    let current = textureSample(current_ssr, linear_sampler, uv);

    // Reproject to previous frame
    let prev_uv = reproject_uv(uv, depth);

    // Sample history if reprojected UV is valid
    var history = vec4<f32>(0.0);
    if (prev_uv.x >= 0.0 && prev_uv.x <= 1.0 && prev_uv.y >= 0.0 && prev_uv.y <= 1.0) {
        history = textureSample(history_ssr, linear_sampler, prev_uv);
    }

    // Temporal blend (simple exponential moving average)
    let blended = mix(current, history, uniforms.blend_factor);

    return blended;
}
