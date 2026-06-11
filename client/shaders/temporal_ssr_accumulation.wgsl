/*!
 * Temporal SSR Accumulation Shader v18.15+
 * With Neighborhood Clamping to reduce ghosting.
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

fn reproject_uv(uv: vec2<f32>, depth: f32) -> vec2<f32> {
    let ndc = vec4<f32>(uv * 2.0 - 1.0, depth, 1.0);
    let view_pos = uniforms.inv_projection * ndc;
    let world_pos = uniforms.inv_view * view_pos;

    let prev_clip = uniforms.prev_projection * uniforms.prev_view * world_pos;
    let prev_ndc = prev_clip.xyz / prev_clip.w;
    return prev_ndc.xy * 0.5 + 0.5;
}

// 3x3 neighborhood min/max clamping (reduces ghosting from disocclusions)
fn clamp_history(current: vec4<f32>, history: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {
    let texel_size = 1.0 / vec2<f32>(textureDimensions(current_ssr));
    
    var min_color = current;
    var max_color = current;
    
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let offset = vec2<f32>(f32(x), f32(y)) * texel_size;
            let sample = textureSample(current_ssr, linear_sampler, uv + offset);
            min_color = min(min_color, sample);
            max_color = max(max_color, sample);
        }
    }
    
    return clamp(history, min_color, max_color);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let depth = textureSample(depth_texture, linear_sampler, uv);

    if (depth >= 1.0) {
        return vec4<f32>(0.0);
    }

    let current = textureSample(current_ssr, linear_sampler, uv);
    let prev_uv = reproject_uv(uv, depth);

    var history = vec4<f32>(0.0);
    var weight = 0.0;

    if (prev_uv.x >= 0.0 && prev_uv.x <= 1.0 && prev_uv.y >= 0.0 && prev_uv.y <= 1.0) {
        let history_depth = textureSample(depth_texture, linear_sampler, prev_uv);
        let depth_diff = abs(depth - history_depth);

        if (depth_diff < 0.04) {  // Depth rejection threshold
            history = textureSample(history_ssr, linear_sampler, prev_uv);
            history = clamp_history(current, history, uv);  // Neighborhood clamping
            weight = uniforms.blend_factor;
        }
    }

    let blended = mix(current, history, weight);
    return blended;
}
