/*!
 * mycelial_web_glow.wgsl
 *
 * Thematic web/glow effect for resource networks, mycelial connections,
 * energy webs, and organic infrastructure.
 *
 * Reacts to RBE flow, council valence, mercy resonance, time, and player velocity.
 *
 * AG-SML v1.0
 */

#import "shaders/include/gpu_simulation_state.wgsl"

@group(0) @binding(0)
var<uniform> sim: GpuSimulationState;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vertex_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(input.position, 1.0);
    output.uv = input.uv;
    return output;
}

@fragment
fn fragment_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let rbe = sim.rbe_flow_rate;
    let valence = sim.council_valence;
    let mercy = sim.global_mercy_resonance;
    let t = sim.time;
    let speed = length(vec3<f32>(
        sim.player_velocity[0],
        sim.player_velocity[1],
        sim.player_velocity[2]
    ));

    let uv = input.uv;

    // === Mycelial Web Pattern ===
    let web_scale = 12.0 + rbe * 6.0;
    let web = sin(uv.x * web_scale) * sin(uv.y * web_scale * 0.7);
    let web_lines = smoothstep(0.48, 0.52, abs(fract(web * 0.5 + t * 0.3) - 0.5));

    // === RBE Energy Flow along web ===
    let flow = sin(uv.x * 9.0 + t * 2.5 + rbe * 3.0) * 0.5 + 0.5;
    let energy = rbe * flow * 0.6;

    // === Council Valence Influence ===
    let council_glow = valence * 0.35;

    // === Mercy Warmth ===
    let mercy_warm = mercy * 0.25;

    // Base color (cool organic web)
    var color = vec3<f32>(0.3, 0.45, 0.35);

    // Add web lines
    color += vec3<f32>(0.15, 0.35, 0.25) * web_lines * 0.8;

    // RBE energy along the web
    color += vec3<f32>(0.4, 0.7, 0.5) * energy;

    // Council glow overlay
    color += vec3<f32>(0.2, 0.4, 0.9) * council_glow;

    // Mercy warmth
    color += vec3<f32>(0.5, 0.3, 0.15) * mercy_warm;

    // Player velocity energy pulse
    let vel_pulse = speed * 0.1 * sin(t * 4.0);
    color += vec3<f32>(vel_pulse * 0.3, vel_pulse * 0.5, vel_pulse * 0.2);

    // Subtle breathing
    let breathe = sin(t * 0.9) * 0.04;
    color *= (1.0 + breathe);

    // Alpha
    let alpha = 0.7 + web_lines * 0.25 + energy * 0.15;

    return vec4<f32>(color, alpha);
}
