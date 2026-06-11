/*!
 * Temporal SSR Accumulation Shader v18.15+
 * Supports both camera-only reprojection and per-pixel velocity from velocity prepass.
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
    blend_factor: f32,
    use_velocity: u32,           // 0 = camera only, 1 = use velocity texture
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
var velocity_texture: texture_2d<f32>;  // From velocity prepass (Rg16Float)
@group(0) @binding(5)
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

fn reproject_with_camera(uv: vec2<f32>, depth: f32) -> vec2<f32> {
    let ndc = vec4<f32>(uv * 2.0 - 1.0, depth, 1.0);
    let view_pos = uniforms.inv_projection * ndc;
    let world_pos = uniforms.inv_view * view_pos;

    let prev_clip = uniforms.prev_projection * uniforms.prev_view * world_pos;
    let prev_ndc = prev_clip.xyz / prev_clip.w;
    return prev_ndc.xy * 0.5 + 0.5;
}

fn reproject_with_velocity(uv: vec2<f32>, depth: f32) -> vec2<f32> {
    let velocity = textureSample(velocity_texture, linear_sampler, uv).xy;
    // Simple approximation: prev_uv = current_uv - velocity (in screen space)
    // For better results, velocity should be in view space or world space
    return uv - velocity * 0.5; // Scale factor may need tuning
}

fn clamp_to_neighborhood(current: vec4<f32>, history: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {
    let texel_size = 1.0 / vec2<f32>(textureDimensions(current_ssr));
    var min_c = current;
    var max_c = current;

    for (var y = -1; y <= 1; y = y + 1) {
        for (var x = -1; x <= 1; x = x + 1) {
            let offset = vec2<f32>(f32(x), f32(y)) * texel_size;
            let s = textureSample(current_ssr, linear_sampler, uv + offset);
            min_c = min(min_c, s);
            max_c = max(max_c, s);
        }
    }
    return clamp(history, min_c, max_c);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let depth = textureSample(depth_texture, linear_sampler, uv);

    if (depth >= 1.0) {
        return vec4<f32>(0.0);
    }

    let current = textureSample(current_ssr, linear_sampler, uv);

    var prev_uv: vec2<f32>;
    if (uniforms.use_velocity != 0u) {
        prev_uv = reproject_with_velocity(uv, depth);
    } else {
        prev_uv = reproject_with_camera(uv, depth);
    }

    var history = vec4<f32>(0.0);
    var weight = 0.0;

    if (prev_uv.x >= 0.0 && prev_uv.x <= 1.0 && prev_uv.y >= 0.0 && prev_uv.y <= 1.0) {
        let history_depth = textureSample(depth_texture, linear_sampler, prev_uv);
        if (abs(depth - history_depth) < 0.04) {
            history = textureSample(history_ssr, linear_sampler, prev_uv);
            history = clamp_to_neighborhood(current, history, uv);
            weight = uniforms.blend_factor;
        }
    }

    let blended = mix(current, history, weight);
    return blended;
}
