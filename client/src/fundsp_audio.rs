/*!
 * fundsp Procedural Audio Prototype
 *
 * Granular layer pushed further with resonance on select voices
 * and noise-based amplitude modulation for extra organic variation.
 */

use bevy::prelude::*;
use fundsp::hacker::*;

/// Builds a highly advanced Epiphany resonance with a very rich granular layer.
pub fn build_epiphany_resonance(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let intensity_var = var(intensity as f64);
    let i = intensity_var;

    // === Main Tonal Body ===
    let base_freq = 62.0 + i * 155.0;
    let vibrato = sine_hz(0.46) * (0.8 + i * 1.1);
    let tone_a = sine_hz(base_freq + vibrato);
    let tone_b = sine_hz(base_freq * 1.007);
    let main_body = (tone_a + tone_b) * (0.17 + i * 0.37);

    let harmonic = sine_hz(base_freq * 2.002) * (0.085 + i * 0.28);

    // === Advanced Granular Layer (8 voices) ===
    // Some voices have resonance, others have noise amplitude modulation

    // Voice 1 - Deep, resonant
    let g1 = sine_hz(base_freq * 0.44 + noise() * (2.1 + i * 3.1) + sine_hz(0.2) * (2.3 + i * 3.3))
        * (0.068 + i * 0.145) * (sine_hz(0.075) * 0.21 + 0.79);
    let g1_f = g1 >> resonator_hz(310.0 + i * 350.0, 2.8);

    // Voice 2
    let g2 = sine_hz(base_freq * 0.86 + noise() * (2.4 + i * 3.4) + sine_hz(0.46) * (2.8 + i * 3.8))
        * (0.061 + i * 0.14) * (sine_hz(0.11) * 0.18 + 0.82);
    let g2_f = g2 >> lowpass_hz(440.0 + i * 410.0, 1.8);

    // Voice 3 - Cross-modulates tonal body, resonant
    let g3 = sine_hz(base_freq * 1.45 + noise() * (2.7 + i * 3.7) + sine_hz(0.82) * (3.3 + i * 4.4))
        * (0.056 + i * 0.13) * (sine_hz(0.095) * 0.19 + 0.81);
    let g3_f = g3 >> resonator_hz(600.0 + i * 480.0, 3.2);

    // Voice 4 - Noise amplitude modulation
    let g4_amp = noise() * (0.12 + i * 0.18) + 0.88;
    let g4 = sine_hz(base_freq * 2.4 + noise() * (3.0 + i * 4.0) + sine_hz(1.4) * (3.7 + i * 4.9))
        * (0.051 + i * 0.12) * g4_amp;
    let g4_f = g4 >> lowpass_hz(740.0 + i * 530.0, 1.65);

    // Voice 5 - Strong cross-mod, noise amp
    let g5_amp = noise() * (0.11 + i * 0.17) + 0.89;
    let g5 = sine_hz(base_freq * 3.6 + noise() * (3.3 + i * 4.3) + sine_hz(2.25) * (4.2 + i * 5.5))
        * (0.048 + i * 0.115) * g5_amp;
    let g5_f = g5 >> lowpass_hz(900.0 + i * 570.0, 1.6);

    // Voice 6
    let g6 = sine_hz(base_freq * 5.35 + noise() * (3.6 + i * 4.7) + sine_hz(3.5) * (4.9 + i * 6.1))
        * (0.044 + i * 0.105) * (sine_hz(0.27) * 0.14 + 0.86);
    let g6_f = g6 >> lowpass_hz(1060.0 + i * 610.0, 1.55);

    // Voice 7 - High ethereal, resonant
    let g7 = sine_hz(base_freq * 7.65 + noise() * (4.0 + i * 5.2) + sine_hz(5.0) * (5.7 + i * 6.9))
        * (0.04 + i * 0.095) * (sine_hz(0.35) * 0.13 + 0.87);
    let g7_f = g7 >> resonator_hz(1220.0 + i * 640.0, 2.6);

    // Voice 8 - Very high shimmer, noise amp
    let g8_amp = noise() * (0.10 + i * 0.16) + 0.90;
    let g8 = sine_hz(base_freq * 10.8 + noise() * (4.5 + i * 5.9) + sine_hz(7.0) * (6.4 + i * 7.9))
        * (0.036 + i * 0.085) * g8_amp;
    let g8_f = g8 >> lowpass_hz(1420.0 + i * 720.0, 1.5);

    let granular_mix = 0.50 + i * 0.50;
    let granular_layer = (g1_f + g2_f + g3_f + g4_f + g5_f + g6_f + g7_f + g8_f) * granular_mix;

    // === Cross-Modulation ===
    let cross_mod = (g3_f + g4_f + g5_f) * 0.42;
    let tonal_filtered = main_body
        >> lowpass_hz(1120.0 + i * 400.0 + cross_mod * 190.0, 1.1);

    // === Combine ===
    let combined = tonal_filtered + harmonic + granular_layer;

    // === Amplitude Modulation ===
    let breath_slow = sine_hz(0.05) * 0.19 + 0.81;
    let breath_mid = sine_hz(0.11) * 0.115 + 0.885;
    let modulated = combined * (0.74 + breath_slow * breath_mid * i * 0.33);

    // Final shaping
    let final = modulated >> lowpass_hz(1250.0 + i * 450.0, 1.0);

    (Box::new(final * 0.65), intensity_var)
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
    info!("[fundsp] Granular layer with resonance + noise amplitude modulation");
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
            let final_intensity = (base_intensity * evolved).clamp(0.35, 1.4);

            instance.intensity_var.set(final_intensity as f64);

            let samples = render_next_chunk(instance);

            if !samples.is_empty() {
                let volume = (0.33 + final_intensity * 0.37).clamp(0.24, 0.82);

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
