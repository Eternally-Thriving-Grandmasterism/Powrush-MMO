// valence_thread.wgsl — Ra-Thor Lattice Thread Shader
// Golden-silver threads that weave & pulse with valence
// MIT + mercy eternal

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct ThreadUniform {
    valence: f32,
    time: f32,
    thread_color: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> thread_uniform: ThreadUniform;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.uv = model.uv;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    // Simple thread-like wave pattern
    let wave = sin(uv.y * 20.0 + thread_uniform.time * 2.0 + uv.x * 5.0) * 0.5 + 0.5;
    let thread = smoothstep(0.45, 0.55, wave);

    // Valence intensity & color shift
    let intensity = thread_uniform.valence * (0.7 + 0.3 * sin(thread_uniform.time * 1.5));
    var color = thread_uniform.thread_color * intensity * thread;

    // Subtle noise for organic feel
    let noise = fract(sin(dot(uv, vec2<f32>(12.9898, 78.233))) * 43758.5453);
    color += vec4<f32>(noise * 0.05 * thread_uniform.valence);

    // Fade edges
    let edge_fade = smoothstep(0.0, 0.1, uv.x) * smoothstep(1.0, 0.9, uv.x);
    color *= edge_fade;

    return vec4<f32>(color.rgb, color.a * thread_uniform.valence);
}
