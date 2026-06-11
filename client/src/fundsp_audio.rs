/*!
 * fundsp Procedural Audio Prototype
 *
 * Advanced granular layer with noise-based randomization per voice
 * and stronger intensity-driven density.
 */

use bevy::prelude::*;
use fundsp::hacker::*;

/// Builds a highly developed Epiphany resonance with advanced granular layer.
pub fn build_epiphany_resonance(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let intensity_var = var(intensity as f64);
    let i = intensity_var;

    // === Main Tonal Body ===
    let base_freq = 62.0 + i * 155.0;
    let vibrato = sine_hz(0.5) * (0.9 + i * 1.2);
    let tone_a = sine_hz(base_freq + vibrato);
    let tone_b = sine_hz(base_freq * 1.008);
    let main_body = (tone_a + tone_b) * (0.18 + i * 0.36);

    let harmonic = sine_hz(base_freq * 2.005) * (0.09 + i * 0.26);

    // === Advanced Granular-Style Layer (7 voices with noise randomization) ===

    // Voice 1 - Deep, noise-jittered
    let g1_noise = noise() * (1.5 + i * 2.5);
    let g1 = sine_hz(base_freq * 0.46 + g1_noise + sine_hz(0.25) * (2.6 + i * 3.6))
        * (0.072 + i * 0.155) * (sine_hz(0.085) * 0.19 + 0.81);
    let g1_f = g1 >> lowpass_hz(340.0 + i * 380.0, 1.8);

    // Voice 2
    let g2_noise = noise() * (1.8 + i * 2.8);
    let g2 = sine_hz(base_freq * 0.9 + g2_noise + sine_hz(0.5) * (3.0 + i * 4.0))
        * (0.065 + i * 0.15) * (sine_hz(0.12) * 0.16 + 0.84);
    let g2_f = g2 >> lowpass_hz(470.0 + i * 440.0, 1.7);

    // Voice 3 - Cross-modulates tonal body
    let g3_noise = noise() * (2.0 + i * 3.0);
    let g3 = sine_hz(base_freq * 1.52 + g3_noise + sine_hz(0.9) * (3.5 + i * 4.6))
        * (0.06 + i * 0.14) * (sine_hz(0.105) * 0.18 + 0.82);
    let g3_f = g3 >> lowpass_hz(650.0 + i * 520.0, 1.6);

    // Voice 4
    let g4_noise = noise() * (2.2 + i * 3.2);
    let g4 = sine_hz(base_freq * 2.5 + g4_noise + sine_hz(1.5) * (3.9 + i * 5.1))
        * (0.055 + i * 0.13) * (sine_hz(0.17) * 0.15 + 0.85);
    let g4_f = g4 >> lowpass_hz(790.0 + i * 560.0, 1.55);

    // Voice 5 - Strong cross-mod
    let g5_noise = noise() * (2.4 + i * 3.5);
    let g5 = sine_hz(base_freq * 3.8 + g5_noise + sine_hz(2.5) * (4.4 + i * 5.7))
        * (0.052 + i * 0.122) * (sine_hz(0.22) * 0.13 + 0.87);
    let g5_f = g5 >> lowpass_hz(950.0 + i * 600.0, 1.5);

    // Voice 6
    let g6_noise = noise() * (2.6 + i * 3.8);
    let g6 = sine_hz(base_freq * 5.6 + g6_noise + sine_hz(3.8) * (5.1 + i * 6.3))
        * (0.048 + i * 0.112) * (sine_hz(0.29) * 0.12 + 0.88);
    let g6_f = g6 >> lowpass_hz(1120.0 + i * 650.0, 1.45);

    // Voice 7 - High ethereal tail
    let g7_noise = noise() * (2.9 + i * 4.2);
    let g7 = sine_hz(base_freq * 8.0 + g7_noise + sine_hz(5.4) * (5.9 + i * 7.2))
        * (0.043 + i * 0.102) * (sine_hz(0.38) * 0.11 + 0.89);
    let g7_f = g7 >> lowpass_hz(1290.0 + i * 700.0, 1.4);

    let granular_layer = (g1_f + g2_f + g3_f + g4_f + g5_f + g6_f + g7_f)
        * (0.55 + i * 0.45);

    // === Stronger Cross-Modulation ===
    let cross_mod = (g3_f + g4_f + g5_f) * 0.38;
    let tonal_filtered = main_body
        >> lowpass_hz(1180.0 + i * 430.0 + cross_mod * 170.0, 1.2);

    // === Combine ===
    let combined = tonal_filtered + harmonic + granular_layer;

    // === Amplitude Modulation ===
    let breath_slow = sine_hz(0.055) * 0.21 + 0.79;
    let breath_mid = sine_hz(0.12) * 0.13 + 0.87;
    let modulated = combined * (0.76 + breath_slow * breath_mid * i * 0.31);

    // Final shaping
    let final = modulated >> lowpass_hz(1320.0 + i * 480.0, 1.0);

    (Box::new(final * 0.67), intensity_var)
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
    info!("[fundsp] Granular layer with noise randomization + dynamic density");
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
            let final_intensity = (base_intensity * evolved).clamp(0.35, 1.3);

            instance.intensity_var.set(final_intensity as f64);

            let samples = render_next_chunk(instance);

            if !samples.is_empty() {
                let volume = (0.35 + final_intensity * 0.35).clamp(0.26, 0.78);

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
