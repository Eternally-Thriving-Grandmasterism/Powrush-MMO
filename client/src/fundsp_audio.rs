/*!
 * fundsp Procedural Audio Prototype
 *
 * Lightweight granular-style layer using multiple overlapping voices
 * with randomization and detuning for a cloud-like Epiphany texture.
 */

use bevy::prelude::*;
use fundsp::hacker::*;

/// Builds a rich Epiphany resonance with a lightweight granular-style texture layer.
pub fn build_epiphany_resonance(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let intensity_var = var(intensity as f64);
    let i = intensity_var;

    // === Main Tonal Body (with light vibrato) ===
    let base_freq = 62.0 + i * 155.0;
    let vibrato = sine_hz(0.55) * (1.1 + i * 1.4);
    let tone_a = sine_hz(base_freq + vibrato);
    let tone_b = sine_hz(base_freq * 1.009);
    let main_body = (tone_a + tone_b) * (0.21 + i * 0.33);

    let harmonic = sine_hz(base_freq * 2.01) * (0.11 + i * 0.23);

    // === Lightweight Granular-Style Texture Layer ===
    // Multiple overlapping voices with slight detuning and filtering
    // Simulates granular cloud behavior using parallel voices + modulation

    // Voice 1 - Lower register, slower movement
    let g1_freq = base_freq * 0.5 + sine_hz(0.4) * (2.0 + i * 3.0);
    let g1 = sine_hz(g1_freq) * (0.09 + i * 0.18);
    let g1_filtered = g1 >> lowpass_hz(420.0 + i * 380.0, 1.6);

    // Voice 2 - Mid register, medium movement
    let g2_freq = base_freq * 1.5 + sine_hz(0.9) * (3.5 + i * 4.5);
    let g2 = sine_hz(g2_freq) * (0.08 + i * 0.17);
    let g2_filtered = g2 >> lowpass_hz(680.0 + i * 520.0, 1.5);

    // Voice 3 - Higher register, faster jitter
    let g3_freq = base_freq * 3.0 + sine_hz(2.2) * (4.0 + i * 6.0);
    let g3 = sine_hz(g3_freq) * (0.07 + i * 0.15);
    let g3_filtered = g3 >> lowpass_hz(950.0 + i * 650.0, 1.4);

    // Combine granular-style voices
    let granular_layer = (g1_filtered + g2_filtered + g3_filtered) * (0.65 + i * 0.35);

    // === Combine everything ===
    let combined = main_body + harmonic + granular_layer;

    // === Multi-layer Amplitude Modulation ===
    let breath_slow = sine_hz(0.07) * 0.24 + 0.76;
    let breath_mid = sine_hz(0.16) * 0.16 + 0.84;
    let modulated = combined * (0.79 + breath_slow * breath_mid * i * 0.28);

    // Final shaping
    let final = modulated >> lowpass_hz(1400.0 + i * 550.0, 1.0);

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
    info!("[fundsp] Lightweight granular-style layer active");
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
