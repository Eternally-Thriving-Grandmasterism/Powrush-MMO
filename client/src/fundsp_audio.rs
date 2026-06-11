/*!
 * fundsp Procedural Audio Prototype
 *
 * Granular layer pushed further with additional resonance
 * and more internal cross-modulation between voices.
 */

use bevy::prelude::*;
use fundsp::hacker::*;

/// Builds a highly advanced Epiphany resonance with a very rich granular layer.
pub fn build_epiphany_resonance(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let intensity_var = var(intensity as f64);
    let i = intensity_var;

    // === Main Tonal Body ===
    let base_freq = 62.0 + i * 155.0;
    let vibrato = sine_hz(0.43) * (0.76 + i * 0.95);
    let tone_a = sine_hz(base_freq + vibrato);
    let tone_b = sine_hz(base_freq * 1.0055);
    let main_body = (tone_a + tone_b) * (0.155 + i * 0.385);

    let harmonic = sine_hz(base_freq * 1.996) * (0.076 + i * 0.31);

    // === Highly Advanced Granular Layer (8 voices) ===
    // More resonance + additional internal cross-modulation

    // Voice 1 - Deep, resonant
    let res1 = 2.2 + i * 2.0;
    let g1 = sine_hz(base_freq * 0.42 + noise() * (1.9 + i * 2.9) + sine_hz(0.17) * (2.1 + i * 3.1))
        * (0.062 + i * 0.13) * (sine_hz(0.068) * 0.24 + 0.76);
    let g1_f = g1 >> resonator_hz(280.0 + i * 320.0, res1);

    // Voice 2 - Band-pass
    let g2 = sine_hz(base_freq * 0.8 + noise() * (2.2 + i * 3.2) + sine_hz(0.4) * (2.6 + i * 3.6))
        * (0.055 + i * 0.125) * (sine_hz(0.095) * 0.21 + 0.79);
    let g2_f = g2 >> bandpass_hz(410.0 + i * 380.0, 1.95);

    // Voice 3 - Cross-modulates tonal body + internal cross to Voice 5
    let res3 = 2.6 + i * 2.4;
    let g3 = sine_hz(base_freq * 1.38 + noise() * (2.5 + i * 3.5) + sine_hz(0.76) * (3.1 + i * 4.2))
        * (0.05 + i * 0.115) * (sine_hz(0.082) * 0.22 + 0.78);
    let g3_f = g3 >> resonator_hz(540.0 + i * 420.0, res3);

    // Voice 4 - Band-pass + noise amp + internal cross to Voice 6
    let g4_amp = noise() * (0.105 + i * 0.165) + 0.895;
    let g4 = sine_hz(base_freq * 2.25 + noise() * (2.8 + i * 3.8) + sine_hz(1.25) * (3.5 + i * 4.7))
        * (0.046 + i * 0.105) * g4_amp;
    let g4_f = g4 >> bandpass_hz(680.0 + i * 490.0, 1.9);

    // Voice 5 - Receives internal cross from Voice 3, resonant
    let res5 = 2.1 + i * 1.7;
    let g5_amp = noise() * (0.095 + i * 0.155) + 0.905;
    let g5_freq_mod = g3 * 16.0;
    let g5 = sine_hz(base_freq * 3.45 + noise() * (3.1 + i * 4.1) + g5_freq_mod + sine_hz(2.05) * (4.0 + i * 5.3))
        * (0.042 + i * 0.1) * g5_amp;
    let g5_f = g5 >> resonator_hz(840.0 + i * 520.0, res5);

    // Voice 6 - Receives internal cross from Voice 4, band-pass
    let g6_freq_mod = g4 * 14.0;
    let g6 = sine_hz(base_freq * 5.05 + noise() * (3.4 + i * 4.5) + g6_freq_mod + sine_hz(3.2) * (4.65 + i * 5.85))
        * (0.038 + i * 0.09) * (sine_hz(0.24) * 0.17 + 0.83);
    let g6_f = g6 >> bandpass_hz(1000.0 + i * 570.0, 1.7);

    // Voice 7 - High ethereal, resonant
    let res7 = 2.3 + i * 2.2;
    let g7 = sine_hz(base_freq * 7.2 + noise() * (3.8 + i * 5.0) + sine_hz(4.55) * (5.45 + i * 6.65))
        * (0.034 + i * 0.08) * (sine_hz(0.32) * 0.16 + 0.84);
    let g7_f = g7 >> resonator_hz(1160.0 + i * 580.0, res7);

    // Voice 8 - Very high shimmer, noise amp
    let g8_amp = noise() * (0.085 + i * 0.145) + 0.915;
    let g8 = sine_hz(base_freq * 10.2 + noise() * (4.3 + i * 5.7) + sine_hz(6.4) * (6.1 + i * 7.6))
        * (0.03 + i * 0.07) * g8_amp;
    let g8_f = g8 >> lowpass_hz(1360.0 + i * 660.0, 1.65);

    let granular_mix = 0.44 + i * 0.56;
    let granular_layer = (g1_f + g2_f + g3_f + g4_f + g5_f + g6_f + g7_f + g8_f) * granular_mix;

    // === Cross-Modulation into tonal body ===
    let cross_mod = (g3_f + g4_f + g5_f) * 0.48;
    let tonal_filtered = main_body
        >> lowpass_hz(1060.0 + i * 370.0 + cross_mod * 220.0, 0.95);

    // === Combine ===
    let combined = tonal_filtered + harmonic + granular_layer;

    // === Amplitude Modulation ===
    let breath_slow = sine_hz(0.044) * 0.17 + 0.83;
    let breath_mid = sine_hz(0.095) * 0.1 + 0.9;
    let modulated = combined * (0.71 + breath_slow * breath_mid * i * 0.36);

    // Final shaping
    let final = modulated >> lowpass_hz(1180.0 + i * 420.0, 1.0);

    (Box::new(final * 0.62), intensity_var)
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
    info!("[fundsp] Granular layer with more resonance + internal cross-mod");
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
            let final_intensity = (base_intensity * evolved).clamp(0.35, 1.55);

            instance.intensity_var.set(final_intensity as f64);

            let samples = render_next_chunk(instance);

            if !samples.is_empty() {
                let volume = (0.3 + final_intensity * 0.4).clamp(0.18, 0.88);

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
