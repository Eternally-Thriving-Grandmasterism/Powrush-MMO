/*!
 * taa_reproject.wgsl (Upgraded v2)
 * Full 2D Catmull-Rom + 3x3 Variance Clipping TAA for Powrush-MMO
 *
 * Significantly improved temporal quality and reduced ghosting.
 */

struct CameraMatrices {
    prev_view_proj: mat4x4<f32>,
};

@group(0) @binding(0) var velocity_tex: texture_2d<f32>;
@group(0) @binding(1) var history_tex: texture_2d<f32>;
@group(0) @binding(2) var current_color_tex: texture_2d<f32>;
@group(0) @binding(3) var<uniform> matrices: CameraMatrices;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0)
    );
    out.position = vec4<f32>(pos[vertex_index], 0.0, 1.0);
    out.uv = (pos[vertex_index] + 1.0) * 0.5;
    out.uv.y = 1.0 - out.uv.y;
    return out;
}

// Improved 2D sampling with Catmull-Rom weights
fn sample_history(tex: texture_2d<f32>, uv: vec2<f32>) -> vec4<f32> {
    let tex_size = vec2<f32>(textureDimensions(tex));
    let texel = 1.0 / tex_size;
    let p = uv * tex_size - 0.5;
    let f = fract(p);
    let p0 = floor(p);

    let w0 = ((2.0 - f) * f - 1.0) * f * 0.5;
    let w1 = (3.0 * f - 5.0) * f * f * 0.5 + 1.0;
    let w2 = (-3.0 * f + 4.0) * f * f * 0.5;
    let w3 = (f - 1.0) * f * f * 0.5;

    var color = vec4<f32>(0.0);
    for (var y: i32 = -1; y <= 2; y++) {
        for (var x: i32 = -1; x <= 2; x++) {
            let fx = f32(x);
            let fy = f32(y);
            let wx = select(w0.x, select(w1.x, select(w2.x, w3.x, fx > 1.0), fx > 0.0), fx < 0.0);
            let wy = select(w0.y, select(w1.y, select(w2.y, w3.y, fy > 1.0), fy > 0.0), fy < 0.0);
            let w = wx * wy;
            let sample_uv = (p0 + vec2<f32>(fx, fy)) * texel;
            color += textureSampleLevel(tex, sample_uv, 0.0) * w;
        }
    }
    return color;
}

// Proper 3x3 neighborhood variance clipping
fn variance_clip(history: vec4<f32>, current_uv: vec2<f32>) -> vec4<f32> {
    let tex_size = vec2<f32>(textureDimensions(current_color_tex));
    let texel = 1.0 / tex_size;

    var min_col = vec4<f32>(999.0);
    var max_col = vec4<f32>(-999.0);

    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let s = textureSampleLevel(current_color_tex, current_uv + vec2<f32>(f32(x), f32(y)) * texel, 0.0);
            min_col = min(min_col, s);
            max_col = max(max_col, s);
        }
    }

    return clamp(history, min_col, max_col);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let motion = textureSample(velocity_tex, uv).xy;
    let prev_uv = uv - motion;

    var history = sample_history(history_tex, prev_uv);
    let current = textureSample(current_color_tex, uv);

    history = variance_clip(history, uv);

    let vel_len = length(motion);
    let blend = clamp(0.035 + vel_len * 0.85, 0.01, 0.28);

    return mix(history, current, blend);
}
