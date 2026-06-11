/*!
 * fundsp Procedural Audio Prototype
 *
 * Granular layer pushed even further with band-pass voices
 * and intensity-dependent resonance depth.
 */

use bevy::prelude::*;
use fundsp::hacker::*;

/// Builds a highly advanced Epiphany resonance with a very rich granular layer.
pub fn build_epiphany_resonance(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let intensity_var = var(intensity as f64);
    let i = intensity_var;

    // === Main Tonal Body ===
    let base_freq = 62.0 + i * 155.0;
    let vibrato = sine_hz(0.45) * (0.8 + i * 1.05);
    let tone_a = sine_hz(base_freq + vibrato);
    let tone_b = sine_hz(base_freq * 1.0065);
    let main_body = (tone_a + tone_b) * (0.165 + i * 0.375);

    let harmonic = sine_hz(base_freq * 2.0) * (0.08 + i * 0.29);

    // === Highly Advanced Granular Layer (8 voices) ===
    // Band-pass voices + intensity-dependent resonance

    // Voice 1 - Deep, resonant (resonance scales with intensity)
    let res1 = 2.4 + i * 1.8;
    let g1 = sine_hz(base_freq * 0.43 + noise() * (2.0 + i * 3.0) + sine_hz(0.19) * (2.2 + i * 3.2))
        * (0.066 + i * 0.14) * (sine_hz(0.072) * 0.22 + 0.78);
    let g1_f = g1 >> resonator_hz(300.0 + i * 340.0, res1);

    // Voice 2
    let g2 = sine_hz(base_freq * 0.84 + noise() * (2.3 + i * 3.3) + sine_hz(0.44) * (2.7 + i * 3.7))
        * (0.059 + i * 0.135) * (sine_hz(0.105) * 0.19 + 0.81);
    let g2_f = g2 >> lowpass_hz(430.0 + i * 400.0, 1.85);

    // Voice 3 - Cross-modulates tonal body, resonant
    let res3 = 2.8 + i * 2.2;
    let g3 = sine_hz(base_freq * 1.42 + noise() * (2.6 + i * 3.6) + sine_hz(0.8) * (3.2 + i * 4.3))
        * (0.054 + i * 0.125) * (sine_hz(0.09) * 0.2 + 0.8);
    let g3_f = g3 >> resonator_hz(580.0 + i * 460.0, res3);

    // Voice 4 - Band-pass character + noise amp
    let g4_amp = noise() * (0.115 + i * 0.175) + 0.885;
    let g4 = sine_hz(base_freq * 2.35 + noise() * (2.9 + i * 3.9) + sine_hz(1.35) * (3.6 + i * 4.8))
        * (0.05 + i * 0.115) * g4_amp;
    let g4_f = g4 >> bandpass_hz(720.0 + i * 510.0, 1.8);

    // Voice 5 - Strong cross-mod, noise amp
    let g5_amp = noise() * (0.105 + i * 0.165) + 0.895;
    let g5 = sine_hz(base_freq * 3.55 + noise() * (3.2 + i * 4.2) + sine_hz(2.15) * (4.1 + i * 5.4))
        * (0.046 + i * 0.11) * g5_amp;
    let g5_f = g5 >> lowpass_hz(880.0 + i * 550.0, 1.65);

    // Voice 6 - Band-pass character
    let g6 = sine_hz(base_freq * 5.25 + noise() * (3.5 + i * 4.6) + sine_hz(3.4) * (4.8 + i * 6.0))
        * (0.042 + i * 0.1) * (sine_hz(0.26) * 0.15 + 0.85);
    let g6_f = g6 >> bandpass_hz(1040.0 + i * 590.0, 1.6);

    // Voice 7 - High ethereal, resonant
    let res7 = 2.5 + i * 2.0;
    let g7 = sine_hz(base_freq * 7.5 + noise() * (3.9 + i * 5.1) + sine_hz(4.85) * (5.6 + i * 6.8))
        * (0.038 + i * 0.09) * (sine_hz(0.34) * 0.14 + 0.86);
    let g7_f = g7 >> resonator_hz(1200.0 + i * 620.0, res7);

    // Voice 8 - Very high shimmer, noise amp
    let g8_amp = noise() * (0.095 + i * 0.155) + 0.905;
    let g8 = sine_hz(base_freq * 10.6 + noise() * (4.4 + i * 5.8) + sine_hz(6.8) * (6.3 + i * 7.8))
        * (0.034 + i * 0.08) * g8_amp;
    let g8_f = g8 >> lowpass_hz(1400.0 + i * 700.0, 1.55);

    let granular_mix = 0.48 + i * 0.52;
    let granular_layer = (g1_f + g2_f + g3_f + g4_f + g5_f + g6_f + g7_f + g8_f) * granular_mix;

    // === Cross-Modulation ===
    let cross_mod = (g3_f + g4_f + g5_f) * 0.44;
    let tonal_filtered = main_body
        >> lowpass_hz(1100.0 + i * 390.0 + cross_mod * 200.0, 1.05);

    // === Combine ===
    let combined = tonal_filtered + harmonic + granular_layer;

    // === Amplitude Modulation ===
    let breath_slow = sine_hz(0.048) * 0.18 + 0.82;
    let breath_mid = sine_hz(0.105) * 0.11 + 0.89;
    let modulated = combined * (0.73 + breath_slow * breath_mid * i * 0.34);

    // Final shaping
    let final = modulated >> lowpass_hz(1220.0 + i * 440.0, 1.0);

    (Box::new(final * 0.64), intensity_var)
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
    info!("[fundsp] Granular layer with band-pass + intensity-dependent resonance");
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
            let final_intensity = (base_intensity * evolved).clamp(0.35, 1.45);

            instance.intensity_var.set(final_intensity as f64);

            let samples = render_next_chunk(instance);

            if !samples.is_empty() {
                let volume = (0.32 + final_intensity * 0.38).clamp(0.22, 0.84);

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
