/*!
 * gpu_state_material.wgsl
 *
 * Advanced visual material that reacts to live GpuSimulationState.
 * Now with rich PATSAGi / Council / RBE / Mercy effects.
 *
 * This shader (and the include) can be reused or imported by:
 *   - valence_halo.wgsl
 *   - mycelial_web_glow.wgsl
 *   - particle systems
 *   - custom post-FX
 *
 * Fields available via sim:
 *   mercy, council_valence, rbe_flow, time, confidence, player_*, etc.
 *
 * AG-SML v1.0
 */

#import "shaders/include/gpu_simulation_state.wgsl"

@group(0) @binding(0)
var<uniform> sim: GpuSimulationState;

@group(1) @binding(1)
var<uniform> material: GpuStateMaterialUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_pos: vec3<f32>,
};

@vertex
fn vertex_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(input.position, 1.0);
    output.uv = input.uv;
    output.world_pos = input.position;
    return output;
}

@fragment
fn fragment_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let resonance = sim.global_mercy_resonance;
    let valence = sim.council_valence;
    let rbe = sim.rbe_flow_rate;
    let confidence = sim.global_confidence;
    let t = sim.time;
    let dt = sim.delta_time;

    let base = material.base_color.rgb;
    let alpha = material.base_color.a;

    // === Mercy Resonance: warm core glow + soft outer bloom ===
    let mercy_core = 0.55 + resonance * 0.6;
    var color = base * mercy_core;

    // === Council Valence: pulsing aura rings (PATSAGi signature) ===
    let ring_phase = t * 1.8 + valence * 4.0;
    let ring = abs(sin(ring_phase) * 0.5 + 0.5);
    let valence_aura = valence * ring * 0.45;
    color += vec3<f32>(valence_aura * 0.2, valence_aura * 0.55, valence_aura * 0.95);

    // === RBE Flow: flowing energy veins / tendrils ===
    let flow_wave = sin(input.uv.x * 12.0 + t * 2.5 + rbe * 3.0) * 0.5 + 0.5;
    let rbe_energy = rbe * flow_wave * 0.35;
    color = mix(color, color * vec3<f32>(1.1, 0.95, 0.85), rbe_energy);

    // === Time + Delta: organic breathing / life pulse ===
    let breathe = sin(t * 1.2) * 0.04 * (1.0 + dt * 8.0);
    color *= (1.0 + breathe);

    // === Global Confidence: vibrancy + subtle chromatic shift ===
    let vib = 0.88 + confidence * 0.22;
    let lum = dot(color, vec3<f32>(0.299, 0.587, 0.114));
    color = mix(vec3<f32>(lum), color, vib);

    // Soft highlight on high resonance + valence ("mercy crown" feel)
    let crown = pow(max(resonance, valence * 0.6), 1.8) * 0.25;
    color += vec3<f32>(crown * 0.6, crown * 0.9, crown);

    color = clamp(color, vec3<f32>(0.0), vec3<f32>(2.2));

    return vec4<f32>(color, alpha);
}

struct GpuStateMaterialUniform {
    base_color: vec4<f32>,
};
