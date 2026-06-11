/*!
 * fundsp Procedural Audio Prototype
 *
 * Deep modulation overhaul - Epiphany resonance to the nth degree.
 * Multiple layers of filter, amplitude, frequency, and cross-modulation.
 */

use bevy::prelude::*;
use fundsp::hacker::*;

/// Builds a deeply modulated, rich resonance graph for Epiphanies.
/// Features extensive modulation at multiple rates and depths.
pub fn build_epiphany_resonance(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let intensity_var = var(intensity as f64);
    let i = intensity_var; // shorthand

    // === Base Frequency ===
    let base_freq = 62.0 + i * 155.0;

    // === Tonal Body with Multi-Layer Frequency Modulation ===
    // Slow vibrato + medium FM + light fast jitter
    let slow_vibrato = sine_hz(0.6) * (1.2 + i * 1.5);
    let medium_fm = sine_hz(4.5) * (0.8 + i * 1.8);
    let fast_jitter = sine_hz(11.0) * (0.3 + i * 0.6);

    let tone_a = sine_hz(base_freq + slow_vibrato + medium_fm + fast_jitter);
    let tone_b = sine_hz(base_freq * 1.009);

    let main_body = (tone_a + tone_b) * (0.20 + i * 0.34);

    // === Harmonic Layer with Cross-Modulation ===
    let harm_fm = sine_hz(2.8) * (1.5 + i * 2.2);
    let harmonic = sine_hz(base_freq * 2.015 + harm_fm) * (0.11 + i * 0.24);

    // === Multi-Layer Noise Texture with Cross-Modulation ===
    let noise_core = noise() * (0.11 + i * 0.40);

    // Filter 1: Slow majestic movement
    let filter1_mod = sine_hz(0.07) * (320.0 + i * 380.0) + 420.0;
    let layer1 = noise_core >> lowpass_hz(240.0 + i * 780.0 + filter1_mod, 1.8);

    // Filter 2: Medium movement modulated by noise (cross-modulation)
    let filter2_mod = sine_hz(0.19) * (280.0 + i * 320.0) + 380.0;
    let cross_mod = noise() * (0.4 + i * 0.6);
    let layer2 = noise_core * 0.7
        >> lowpass_hz(180.0 + i * 650.0 + filter2_mod + cross_mod * 300.0, 1.5);

    let noise_layers = (layer1 + layer2) * 0.85;

    // === Combine All Layers ===
    let combined = main_body + harmonic + noise_layers;

    // === Multi-Rate Amplitude Modulation (Breathing) ===
    let breath_slow = sine_hz(0.06) * 0.28 + 0.72;
    let breath_mid = sine_hz(0.17) * 0.18 + 0.82;
    let breath_fast = sine_hz(0.9) * 0.09 + 0.91;

    let amplitude_mod = breath_slow * breath_mid * breath_fast;
    let modulated = combined * (0.78 + amplitude_mod * i * 0.32);

    // === Final Shaping ===
    let final = modulated >> lowpass_hz(1450.0 + i * 580.0, 1.0);

    (Box::new(final * 0.70), intensity_var)
}

/// Represents an active rolling procedural Epiphany resonance.
pub struct ActiveEpiphanyResonance {
    pub graph: Box<dyn AudioUnit64>,
    pub intensity_var: Shared<f64>,
    pub remaining_duration: f32,
    pub total_duration: f32,
    pub chunk_duration: f32,
    pub position: Vec3,
}

#[derive(Resource, Default)]
pub struct ActiveProceduralEpiphanies {
    pub instances: Vec<ActiveEpiphanyResonance>,
}

/// Renders the next chunk from an active instance.
pub fn render_next_chunk(instance: &mut ActiveEpiphanyResonance) -> Vec<f32> {
    let sample_rate = 44100.0;
    let num_samples = (instance.chunk_duration * sample_rate) as usize;
    let mut buffer = vec![0.0; num_samples];
    instance.graph.render(sample_rate, &mut buffer);
    buffer
}

/// Update intensity of a running Epiphany resonance.
pub fn update_epiphany_intensity(instance: &ActiveEpiphanyResonance, new_intensity: f32) {
    let clamped = new_intensity.clamp(0.0, 1.0) as f64;
    instance.intensity_var.set(clamped);
}

pub struct FundspAudioPlugin;

impl Plugin for FundspAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ActiveProceduralEpiphanies>()
            .add_systems(Startup, setup_fundsp)
            .add_systems(Update, update_rolling_chunks);
    }
}

fn setup_fundsp(mut commands: Commands) {
    info!("[fundsp] Deep modulation (nth degree) active");
}

/// System that renders chunks and evolves intensity automatically.
fn update_rolling_chunks(
    mut active: ResMut<ActiveProceduralEpiphanies>,
    spatial_manager: Res<crate::spatial_audio::SpatialAudioManager>,
) {
    let mut i = 0;
    while i < active.instances.len() {
        let instance = &mut active.instances[i];

        if instance.remaining_duration > 0.0 {
            let progress = 1.0 - (instance.remaining_duration / instance.total_duration);

            let evolved = if progress < 0.55 {
                0.65 + (progress / 0.55) * 0.55
            } else {
                1.2 - ((progress - 0.55) / 0.45) * 0.5
            };

            let base_intensity = instance.intensity_var.get() as f32;
            let final_intensity = (base_intensity * evolved).clamp(0.35, 1.2);

            instance.intensity_var.set(final_intensity as f64);

            let samples = render_next_chunk(instance);

            if !samples.is_empty() {
                let volume = (0.37 + final_intensity * 0.33).clamp(0.30, 0.74);

                spatial_manager.play_generated_spatial(
                    samples,
                    instance.position,
                    Vec3::ZERO,
                    volume,
                );
            }

            instance.remaining_duration -= instance.chunk_duration;
        }

        if instance.remaining_duration <= 0.0 {
            active.instances.remove(i);
        } else {
            i += 1;
        }
    }
}
