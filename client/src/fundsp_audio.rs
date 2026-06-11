/*!
 * fundsp Procedural Audio Prototype
 *
 * Advanced granular layer pushed further: 8 voices with varied
 * filter behaviors and stronger dynamic density based on intensity.
 */

use bevy::prelude::*;
use fundsp::hacker::*;

/// Builds a highly advanced Epiphany resonance with a very developed granular layer.
pub fn build_epiphany_resonance(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let intensity_var = var(intensity as f64);
    let i = intensity_var;

    // === Main Tonal Body ===
    let base_freq = 62.0 + i * 155.0;
    let vibrato = sine_hz(0.48) * (0.85 + i * 1.15);
    let tone_a = sine_hz(base_freq + vibrato);
    let tone_b = sine_hz(base_freq * 1.0075);
    let main_body = (tone_a + tone_b) * (0.17 + i * 0.37);

    let harmonic = sine_hz(base_freq * 2.003) * (0.085 + i * 0.27);

    // === Highly Advanced Granular Layer (8 voices) ===
    // Varied filter behaviors + strong intensity-based density

    // Voice 1 - Deep, slow, smooth filter
    let g1 = sine_hz(base_freq * 0.45 + noise() * (2.2 + i * 3.2) + sine_hz(0.22) * (2.4 + i * 3.4))
        * (0.07 + i * 0.15) * (sine_hz(0.08) * 0.2 + 0.8);
    let g1_f = g1 >> lowpass_hz(320.0 + i * 360.0, 1.85);

    // Voice 2
    let g2 = sine_hz(base_freq * 0.88 + noise() * (2.5 + i * 3.5) + sine_hz(0.48) * (2.9 + i * 3.9))
        * (0.063 + i * 0.145) * (sine_hz(0.115) * 0.17 + 0.83);
    let g2_f = g2 >> lowpass_hz(450.0 + i * 420.0, 1.75);

    // Voice 3 - Cross-modulates tonal body
    let g3 = sine_hz(base_freq * 1.48 + noise() * (2.8 + i * 3.8) + sine_hz(0.85) * (3.4 + i * 4.5))
        * (0.058 + i * 0.135) * (sine_hz(0.1) * 0.18 + 0.82);
    let g3_f = g3 >> lowpass_hz(620.0 + i * 500.0, 1.65);

    // Voice 4
    let g4 = sine_hz(base_freq * 2.45 + noise() * (3.1 + i * 4.1) + sine_hz(1.45) * (3.8 + i * 5.0))
        * (0.053 + i * 0.125) * (sine_hz(0.16) * 0.16 + 0.84);
    let g4_f = g4 >> lowpass_hz(760.0 + i * 540.0, 1.6);

    // Voice 5 - Strong cross-mod
    let g5 = sine_hz(base_freq * 3.7 + noise() * (3.4 + i * 4.4) + sine_hz(2.35) * (4.3 + i * 5.6))
        * (0.05 + i * 0.118) * (sine_hz(0.21) * 0.14 + 0.86);
    let g5_f = g5 >> lowpass_hz(920.0 + i * 580.0, 1.55);

    // Voice 6
    let g6 = sine_hz(base_freq * 5.45 + noise() * (3.7 + i * 4.8) + sine_hz(3.6) * (5.0 + i * 6.2))
        * (0.046 + i * 0.108) * (sine_hz(0.28) * 0.13 + 0.87);
    let g6_f = g6 >> lowpass_hz(1080.0 + i * 620.0, 1.5);

    // Voice 7 - High ethereal
    let g7 = sine_hz(base_freq * 7.8 + noise() * (4.1 + i * 5.3) + sine_hz(5.1) * (5.8 + i * 7.0))
        * (0.042 + i * 0.098) * (sine_hz(0.36) * 0.12 + 0.88);
    let g7_f = g7 >> lowpass_hz(1250.0 + i * 660.0, 1.45);

    // Voice 8 - Very high shimmer (appears more at high intensity)
    let g8 = sine_hz(base_freq * 11.0 + noise() * (4.6 + i * 6.0) + sine_hz(7.2) * (6.5 + i * 8.0))
        * (0.038 + i * 0.09) * (sine_hz(0.48) * 0.11 + 0.89);
    let g8_f = g8 >> lowpass_hz(1450.0 + i * 750.0, 1.4);

    // Dynamic density: higher voices contribute more at higher intensity
    let granular_mix = 0.52 + i * 0.48;
    let granular_layer = (g1_f + g2_f + g3_f + g4_f + g5_f + g6_f + g7_f + g8_f) * granular_mix;

    // === Cross-Modulation ===
    let cross_mod = (g3_f + g4_f + g5_f) * 0.4;
    let tonal_filtered = main_body
        >> lowpass_hz(1150.0 + i * 410.0 + cross_mod * 180.0, 1.15);

    // === Combine ===
    let combined = tonal_filtered + harmonic + granular_layer;

    // === Amplitude Modulation ===
    let breath_slow = sine_hz(0.052) * 0.20 + 0.80;
    let breath_mid = sine_hz(0.115) * 0.12 + 0.88;
    let modulated = combined * (0.75 + breath_slow * breath_mid * i * 0.32);

    // Final shaping
    let final = modulated >> lowpass_hz(1280.0 + i * 460.0, 1.0);

    (Box::new(final * 0.66), intensity_var)
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
    info!("[fundsp] Granular layer with 8 voices + dynamic density + noise");
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
            let final_intensity = (base_intensity * evolved).clamp(0.35, 1.35);

            instance.intensity_var.set(final_intensity as f64);

            let samples = render_next_chunk(instance);

            if !samples.is_empty() {
                let volume = (0.34 + final_intensity * 0.36).clamp(0.25, 0.8);

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
