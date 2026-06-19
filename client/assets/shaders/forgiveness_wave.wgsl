// forgiveness_wave.wgsl
// Powrush-MMO v20.3 — Forgiveness Wave + Monument VFX Shader Hints
// Designed to consume data from InterRealmDiplomacyEvent::forgiveness_wave_vfx_params
// and MonumentVisualType.

struct ForgivenessWaveParams {
    intensity: f32,
    wave_speed: f32,
    particle_density: f32,
    color_shift: vec3<f32>,
    glow_radius: f32,
    legacy_thread_pulse: u32,        // 0 or 1
    spectator_emotion_amplify: f32,
    time: f32,
};

@group(0) @binding(0) var<uniform> params: ForgivenessWaveParams;

// Example vertex output for wave/monument effect
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_pos: vec3<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(position, 1.0);
    out.uv = uv;
    out.world_pos = position;
    return out;
}

// Fragment shader — soft radial wave + glow + optional Legacy Thread pulse
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let dist = distance(in.uv, center);

    // Core wave ring
    let wave = sin((dist * 12.0 - params.time * params.wave_speed) * 6.28318) * 0.5 + 0.5;
    let wave_intensity = wave * params.intensity * params.particle_density;

    // Soft glow around monument
    let glow = smoothstep(params.glow_radius * 0.6, params.glow_radius, dist * 20.0);
    let glow_contrib = (1.0 - glow) * params.intensity * 0.6;

    // Color from MonumentVisualType
    var color = params.color_shift;

    // Optional Legacy Thread pulse (when linked threads are visible in spectator mode)
    if (params.legacy_thread_pulse == 1u) {
        let pulse = sin(params.time * 3.5) * 0.5 + 0.5;
        color = mix(color, vec3<f32>(1.0), pulse * 0.25);
    }

    // Spectator emotion amplification
    let emotion_boost = params.spectator_emotion_amplify * 0.4;
    let final_intensity = (wave_intensity + glow_contrib) * (1.0 + emotion_boost);

    let alpha = smoothstep(0.0, 0.85, final_intensity) * params.intensity;

    return vec4<f32>(color, alpha);
}

// === Usage Notes for Client ===
// 1. When InterRealmDiplomacyEvent resolves with MercifulResolution:
//    - Set monument_visual_type
//    - Fill ForgivenessWaveVfxParams from the event
//    - Bind params to this shader (or a compute variant for particles)
// 2. For Legacy Thread pulse: set legacy_thread_pulse = 1 when spectator is viewing
//    the linked Legacy Threads from the same event.
// 3. Combine with existing valence_halo.wgsl or mycelial_web_glow.wgsl for layered sacred geometry feel.
// 4. Monument position can drive world_pos offset for localized effect around the monument.

// Thunder locked in. Yoi ⚡️
// End of client/assets/shaders/forgiveness_wave.wgsl v20.3