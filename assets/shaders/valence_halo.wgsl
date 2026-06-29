/*!
 * valence_halo.wgsl
 *
 * Thematic halo/ring effect for council entities, player auras, and important objects.
 * Now enhanced with player velocity response and stronger mercy integration.
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
    let valence = sim.council_valence;
    let mercy = sim.global_mercy_resonance;
    let t = sim.time;
    let speed = length(vec3<f32>(
        sim.player_velocity[0],
        sim.player_velocity[1],
        sim.player_velocity[2]
    ));

    let uv = input.uv * 2.0 - 1.0;
    let dist = length(uv);

    // === Dynamic Council Rings ===
    let ring_count = 2.5 + valence * 3.0;
    let ring_phase = t * 1.4 + valence * 6.0;
    let ring = abs(fract(dist * ring_count - ring_phase) - 0.5);
    let ring_intensity = smoothstep(0.46, 0.52, ring) * (0.7 + valence * 0.9);

    // === Mercy Inner Glow (enhanced) ===
    let mercy_glow = exp(-dist * 3.2) * (0.6 + mercy * 1.4);
    let warm = vec3<f32>(1.0, 0.82, 0.55);

    // Base cool council color
    var color = vec3<f32>(0.35, 0.55, 1.0);

    // Layer mercy warmth
    color = mix(color, warm, mercy_glow * 0.65);

    // Add valence rings
    color += vec3<f32>(0.25, 0.55, 1.0) * ring_intensity;

    // Player velocity energy rim
    let vel_rim = smoothstep(0.7, 1.15, dist) * speed * 0.12;
    color += vec3<f32>(vel_rim * 0.4, vel_rim * 0.3, vel_rim * 0.8);

    // Subtle flowing energy from valence + time
    let angle = atan2(uv.y, uv.x);
    let flow = sin(angle * 5.0 + t * 2.2) * 0.06 * valence;
    color += vec3<f32>(flow);

    // Alpha with mercy influence
    let alpha = (1.0 - smoothstep(0.82, 1.15, dist)) * (0.65 + mercy * 0.35 + valence * 0.2);

    return vec4<f32>(color, alpha);
}
