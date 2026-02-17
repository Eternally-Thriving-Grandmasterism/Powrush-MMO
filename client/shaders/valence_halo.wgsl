// valence_halo.wgsl — Ra-Thor Mercy-Gated Halo Shader
// Soft emissive + subtle distortion based on valence score
// MIT + mercy eternal

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct ValenceUniform {
    valence: f32,          // 0.0–1.0
    pulse_time: f32,
    base_color: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> valence_uniform: ValenceUniform;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.uv = model.uv;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv * 2.0 - 1.0; // centered -1..1
    let dist = length(uv);

    // Soft halo falloff
    let halo = smoothstep(0.4, 0.1, dist);

    // Valence-scaled intensity & color
    let intensity = valence_uniform.valence * (0.6 + 0.4 * sin(valence_uniform.pulse_time * 2.0));
    let color = valence_uniform.base_color * intensity * halo;

    // Subtle radial distortion for higher valence
    let distortion = valence_uniform.valence * 0.08 * sin(dist * 20.0 + valence_uniform.pulse_time * 3.0);
    let distorted_uv = uv * (1.0 + distortion);
    let distorted_dist = length(distorted_uv);
    let extra_glow = smoothstep(0.35, 0.05, distorted_dist) * 0.3 * valence_uniform.valence;

    return vec4<f32>(color.rgb + vec3<f32>(extra_glow), color.a);
}
