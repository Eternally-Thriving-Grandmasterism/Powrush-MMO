/*!
 * fundsp Procedural Audio Prototype
 *
 * Granular layer pushed further with more band-pass voices
 * and internal cross-modulation between granular voices.
 */

use bevy::prelude::*;
use fundsp::hacker::*;

/// Builds a highly advanced Epiphany resonance with a very rich granular layer.
pub fn build_epiphany_resonance(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let intensity_var = var(intensity as f64);
    let i = intensity_var;

    // === Main Tonal Body ===
    let base_freq = 62.0 + i * 155.0;
    let vibrato = sine_hz(0.44) * (0.78 + i * 1.0);
    let tone_a = sine_hz(base_freq + vibrato);
    let tone_b = sine_hz(base_freq * 1.006);
    let main_body = (tone_a + tone_b) * (0.16 + i * 0.38);

    let harmonic = sine_hz(base_freq * 1.998) * (0.078 + i * 0.30);

    // === Highly Advanced Granular Layer (8 voices) ===
    // More band-pass + internal cross-modulation

    // Voice 1 - Deep, resonant
    let res1 = 2.3 + i * 1.9;
    let g1 = sine_hz(base_freq * 0.425 + noise() * (1.95 + i * 2.95) + sine_hz(0.18) * (2.15 + i * 3.15))
        * (0.064 + i * 0.135) * (sine_hz(0.07) * 0.23 + 0.77);
    let g1_f = g1 >> resonator_hz(290.0 + i * 330.0, res1);

    // Voice 2 - Band-pass
    let g2 = sine_hz(base_freq * 0.82 + noise() * (2.25 + i * 3.25) + sine_hz(0.42) * (2.65 + i * 3.65))
        * (0.057 + i * 0.13) * (sine_hz(0.1) * 0.2 + 0.8);
    let g2_f = g2 >> bandpass_hz(420.0 + i * 390.0, 1.9);

    // Voice 3 - Cross-modulates tonal body + internal cross to Voice 5
    let res3 = 2.7 + i * 2.3;
    let g3 = sine_hz(base_freq * 1.4 + noise() * (2.55 + i * 3.55) + sine_hz(0.78) * (3.15 + i * 4.25))
        * (0.052 + i * 0.12) * (sine_hz(0.085) * 0.21 + 0.79);
    let g3_f = g3 >> resonator_hz(560.0 + i * 440.0, res3);

    // Voice 4 - Band-pass + noise amp
    let g4_amp = noise() * (0.11 + i * 0.17) + 0.89;
    let g4 = sine_hz(base_freq * 2.3 + noise() * (2.85 + i * 3.85) + sine_hz(1.3) * (3.55 + i * 4.75))
        * (0.048 + i * 0.11) * g4_amp;
    let g4_f = g4 >> bandpass_hz(700.0 + i * 500.0, 1.85);

    // Voice 5 - Receives internal cross-mod from Voice 3, noise amp
    let g5_amp = noise() * (0.1 + i * 0.16) + 0.9;
    let g5_freq_mod = g3 * 18.0; // internal cross-modulation from Voice 3
    let g5 = sine_hz(base_freq * 3.5 + noise() * (3.15 + i * 4.15) + g5_freq_mod + sine_hz(2.1) * (4.05 + i * 5.35))
        * (0.044 + i * 0.105) * g5_amp;
    let g5_f = g5 >> lowpass_hz(860.0 + i * 540.0, 1.7);

    // Voice 6 - Band-pass
    let g6 = sine_hz(base_freq * 5.15 + noise() * (3.45 + i * 4.55) + sine_hz(3.3) * (4.7 + i * 5.9))
        * (0.04 + i * 0.095) * (sine_hz(0.25) * 0.16 + 0.84);
    let g6_f = g6 >> bandpass_hz(1020.0 + i * 580.0, 1.65);

    // Voice 7 - High ethereal, resonant
    let res7 = 2.4 + i * 2.1;
    let g7 = sine_hz(base_freq * 7.35 + noise() * (3.85 + i * 5.05) + sine_hz(4.7) * (5.5 + i * 6.7))
        * (0.036 + i * 0.085) * (sine_hz(0.33) * 0.15 + 0.85);
    let g7_f = g7 >> resonator_hz(1180.0 + i * 600.0, res7);

    // Voice 8 - Very high shimmer, noise amp
    let g8_amp = noise() * (0.09 + i * 0.15) + 0.91;
    let g8 = sine_hz(base_freq * 10.4 + noise() * (4.35 + i * 5.75) + sine_hz(6.6) * (6.2 + i * 7.7))
        * (0.032 + i * 0.075) * g8_amp;
    let g8_f = g8 >> lowpass_hz(1380.0 + i * 680.0, 1.6);

    let granular_mix = 0.46 + i * 0.54;
    let granular_layer = (g1_f + g2_f + g3_f + g4_f + g5_f + g6_f + g7_f + g8_f) * granular_mix;

    // === Cross-Modulation into tonal body ===
    let cross_mod = (g3_f + g4_f + g5_f) * 0.46;
    let tonal_filtered = main_body
        >> lowpass_hz(1080.0 + i * 380.0 + cross_mod * 210.0, 1.0);

    // === Combine ===
    let combined = tonal_filtered + harmonic + granular_layer;

    // === Amplitude Modulation ===
    let breath_slow = sine_hz(0.046) * 0.175 + 0.825;
    let breath_mid = sine_hz(0.1) * 0.105 + 0.895;
    let modulated = combined * (0.72 + breath_slow * breath_mid * i * 0.35);

    // Final shaping
    let final = modulated >> lowpass_hz(1200.0 + i * 430.0, 1.0);

    (Box::new(final * 0.63), intensity_var)
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
    info!("[fundsp] Granular layer with internal cross-mod + more band-pass");
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
            let final_intensity = (base_intensity * evolved).clamp(0.35, 1.5);

            instance.intensity_var.set(final_intensity as f64);

            let samples = render_next_chunk(instance);

            if !samples.is_empty() {
                let volume = (0.31 + final_intensity * 0.39).clamp(0.2, 0.86);

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
