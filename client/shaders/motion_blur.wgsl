/*!
 * motion_blur.wgsl
 * Velocity-driven Motion Blur for Powrush-MMO
 *
 * Samples along the motion vector from the velocity prepass.
 * Supports both camera and object motion.
 * Intensity controlled from MotionBlurSettings.
 */

struct MotionBlurParams {
    intensity: f32,
    max_samples: f32,
};

@group(0) @binding(0) var velocity_tex: texture_2d<f32>;
@group(0) @binding(1) var color_tex: texture_2d<f32>;
@group(0) @binding(2) var<uniform> params: MotionBlurParams;

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

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let motion = textureSample(velocity_tex, uv).xy * params.intensity;

    let sample_count = min(params.max_samples, 16.0);
    var color = textureSample(color_tex, uv);
    var total_weight = 1.0;

    for (var i = 1.0; i < sample_count; i += 1.0) {
        let offset = motion * (i / sample_count);
        let sample_uv = uv + offset;
        color += textureSample(color_tex, sample_uv);
        total_weight += 1.0;
    }

    return color / total_weight;
}
