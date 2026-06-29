/*!
 * mycelial_web_glow.wgsl
 *
 * Refined organic web/glow effect for resource networks and mycelial visuals.
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
    let dist_from_center = length(uv - 0.5);

    // === Organic Web Pattern ===
    let web_scale = 11.0 + rbe * 7.0;
    let web = sin(uv.x * web_scale) * sin(uv.y * web_scale * 0.65);
    let web_lines = smoothstep(0.47, 0.53, abs(fract(web * 0.5 + t * 0.25) - 0.5));

    // === RBE Energy Pulses along web ===
    let pulse = sin(uv.x * 8.0 + t * 2.8 + rbe * 4.5) * 0.5 + 0.5;
    let energy = rbe * pulse * 0.7;

    // === Council Influence ===
    let council_tint = valence * 0.4;

    // === Mercy Warmth ===
    let mercy_tint = mercy * 0.3;

    // Base organic color
    var color = vec3<f32>(0.28, 0.42, 0.32);

    // Web lines
    color += vec3<f32>(0.18, 0.38, 0.28) * web_lines * 0.9;

    // RBE energy
    color += vec3<f32>(0.35, 0.75, 0.5) * energy;

    // Council cool overlay
    color += vec3<f32>(0.15, 0.35, 0.85) * council_tint;

    // Mercy warmth
    color += vec3<f32>(0.55, 0.35, 0.2) * mercy_tint;

    // Player velocity pulse
    let vel_pulse = speed * 0.12 * sin(t * 5.0);
    color += vec3<f32>(vel_pulse * 0.25, vel_pulse * 0.55, vel_pulse * 0.15);

    // Distance falloff for nicer edges
    let falloff = 1.0 - smoothstep(0.3, 0.85, dist_from_center);
    color *= falloff;

    // Gentle breathing
    let breathe = sin(t * 0.85) * 0.035;
    color *= (1.0 + breathe);

    // Alpha
    let alpha = 0.65 + web_lines * 0.3 + energy * 0.2;

    return vec4<f32>(color, alpha);
}
