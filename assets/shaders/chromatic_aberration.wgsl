/*!
 * Chromatic Aberration WGSL Shader for Powrush-MMO
 *
 * Cinematic post-processing effect.
 * Applies distance-weighted RGB channel separation.
 * Stronger aberration toward the edges of the screen for that authentic lens flare / filmic look.
 *
 * Input: Motion blur / previous post-FX color texture
 * Output: Final stylized color with beautiful chromatic separation
 *
 * PATSAGi + Ra-Thor approved • AG-SML v1.0
 */

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    // Standard full-screen triangle (clip space)
    var out: VertexOutput;
    let x = f32((vertex_index << 1u) & 2u) - 1.0;
    let y = f32(vertex_index & 2u) - 1.0;
    out.position = vec4<f32>(x, y, 0.0, 1.0);
    // UV in [0,1] range, y flipped for texture sampling
    out.uv = vec2<f32>(x * 0.5 + 0.5, 1.0 - (y * 0.5 + 0.5));
    return out;
}

struct ChromaticAberrationParams {
    intensity: f32,
    center_x: f32,
    center_y: f32,
    edge_boost: f32,
    _padding: f32,
};

@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var input_sampler: sampler;
@group(0) @binding(2) var<uniform> params: ChromaticAberrationParams;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let center = vec2<f32>(params.center_x, params.center_y);
    let dist = distance(uv, center);

    // Stronger aberration at edges (cinematic lens behavior)
    let aberration = params.intensity * (0.015 + params.edge_boost * dist * dist * 0.02);

    // Classic RGB split - red shifted one way, blue the other
    let r_uv = uv + vec2<f32>( aberration, 0.0);
    let b_uv = uv - vec2<f32>( aberration, 0.0);

    let r = textureSample(input_texture, input_sampler, r_uv).r;
    let g = textureSample(input_texture, input_sampler, uv).g;
    let b = textureSample(input_texture, input_sampler, b_uv).b;

    return vec4<f32>(r, g, b, 1.0);
}
