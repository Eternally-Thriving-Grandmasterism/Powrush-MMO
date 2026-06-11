/*!
 * fundsp Procedural Audio Prototype
 *
 * Further developed granular layer with 7 voices and per-voice
 * amplitude variation for more heterogeneous, living texture.
 */

use bevy::prelude::*;
use fundsp::hacker::*;

/// Builds a rich Epiphany resonance with a highly developed granular-style layer.
pub fn build_epiphany_resonance(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let intensity_var = var(intensity as f64);
    let i = intensity_var;

    // === Main Tonal Body ===
    let base_freq = 62.0 + i * 155.0;
    let vibrato = sine_hz(0.52) * (1.0 + i * 1.3);
    let tone_a = sine_hz(base_freq + vibrato);
    let tone_b = sine_hz(base_freq * 1.0085);
    let main_body = (tone_a + tone_b) * (0.19 + i * 0.35);

    let harmonic = sine_hz(base_freq * 2.008) * (0.095 + i * 0.25);

    // === Highly Developed Granular-Style Layer (7 voices) ===
    // Each voice has unique modulation and amplitude behavior

    // Voice 1 - Deep, slow pulsing
    let g1 = sine_hz(base_freq * 0.47 + sine_hz(0.28) * (2.8 + i * 3.8))
        * (0.075 + i * 0.16) * (sine_hz(0.09) * 0.18 + 0.82);
    let g1_f = g1 >> lowpass_hz(360.0 + i * 400.0, 1.75);

    // Voice 2 - Low-mid, medium movement
    let g2 = sine_hz(base_freq * 0.92 + sine_hz(0.55) * (3.2 + i * 4.2))
        * (0.068 + i * 0.155) * (sine_hz(0.13) * 0.15 + 0.85);
    let g2_f = g2 >> lowpass_hz(490.0 + i * 460.0, 1.65);

    // Voice 3 - Mid, cross-modulates tonal body
    let g3 = sine_hz(base_freq * 1.55 + sine_hz(0.95) * (3.6 + i * 4.8))
        * (0.062 + i * 0.145) * (sine_hz(0.11) * 0.17 + 0.83);
    let g3_f = g3 >> lowpass_hz(680.0 + i * 540.0, 1.55);

    // Voice 4 - Upper-mid
    let g4 = sine_hz(base_freq * 2.55 + sine_hz(1.6) * (4.0 + i * 5.2))
        * (0.058 + i * 0.135) * (sine_hz(0.18) * 0.14 + 0.86);
    let g4_f = g4 >> lowpass_hz(820.0 + i * 580.0, 1.5);

    // Voice 5 - High, fast movement
    let g5 = sine_hz(base_freq * 3.9 + sine_hz(2.7) * (4.5 + i * 5.8))
        * (0.055 + i * 0.125) * (sine_hz(0.24) * 0.12 + 0.88);
    let g5_f = g5 >> lowpass_hz(980.0 + i * 620.0, 1.45);

    // Voice 6 - Very high, shimmering
    let g6 = sine_hz(base_freq * 5.8 + sine_hz(4.1) * (5.2 + i * 6.5))
        * (0.05 + i * 0.115) * (sine_hz(0.31) * 0.11 + 0.89);
    let g6_f = g6 >> lowpass_hz(1150.0 + i * 680.0, 1.4);

    // Voice 7 - Ethereal high tail
    let g7 = sine_hz(base_freq * 8.2 + sine_hz(5.8) * (6.0 + i * 7.5))
        * (0.045 + i * 0.105) * (sine_hz(0.42) * 0.10 + 0.90);
    let g7_f = g7 >> lowpass_hz(1320.0 + i * 720.0, 1.35);

    let granular_layer = (g1_f + g2_f + g3_f + g4_f + g5_f + g6_f + g7_f)
        * (0.58 + i * 0.42);

    // === Cross-Modulation from granular into tonal body ===
    let cross_mod = (g3_f + g4_f + g5_f) * 0.35;
    let tonal_filtered = main_body
        >> lowpass_hz(1200.0 + i * 450.0 + cross_mod * 160.0, 1.25);

    // === Combine ===
    let combined = tonal_filtered + harmonic + granular_layer;

    // === Amplitude Modulation ===
    let breath_slow = sine_hz(0.06) * 0.22 + 0.78;
    let breath_mid = sine_hz(0.13) * 0.14 + 0.86;
    let modulated = combined * (0.77 + breath_slow * breath_mid * i * 0.30);

    // Final shaping
    let final = modulated >> lowpass_hz(1350.0 + i * 500.0, 1.0);

    (Box::new(final * 0.68), intensity_var)
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
    info!("[fundsp] Granular layer pushed further (7 voices + per-voice amp)");
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
            let final_intensity = (base_intensity * evolved).clamp(0.35, 1.25);

            instance.intensity_var.set(final_intensity as f64);

            let samples = render_next_chunk(instance);

            if !samples.is_empty() {
                let volume = (0.36 + final_intensity * 0.34).clamp(0.28, 0.76);

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
