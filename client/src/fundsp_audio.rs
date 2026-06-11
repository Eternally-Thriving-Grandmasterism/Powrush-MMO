/*!
 * fundsp Procedural Audio Prototype
 *
 * Refined modulation + light frequency modulation for richer Epiphany resonance.
 */

use bevy::prelude::*;
use fundsp::hacker::*;

/// Builds a rich, modulated resonance graph for Epiphanies.
/// Features refined modulation depths/rates + gentle frequency modulation.
pub fn build_epiphany_resonance(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let intensity_var = var(intensity as f64);

    // === Tonal Body ===
    let base_freq = 65.0 + intensity_var * 160.0;

    // Light frequency modulation (gentle vibrato) on one oscillator
    let vibrato = sine_hz(0.7) * 1.8; // ~1.8 Hz vibrato, subtle depth
    let tone_a = sine_hz(base_freq + vibrato);
    let tone_b = sine_hz(base_freq * 1.008);

    let main_body = (tone_a + tone_b) * (0.22 + intensity_var * 0.32);

    let harmonic = sine_hz(base_freq * 2.02) * (0.12 + intensity_var * 0.22);

    // === Noise Texture with Dynamic Filtering ===
    let noise_base = noise() * (0.12 + intensity_var * 0.38);

    // Refined filter modulation (slower, more majestic)
    let filter_mod = sine_hz(0.09) * 380.0 + 480.0;
    let filtered_noise = noise_base
        >> lowpass_hz(260.0 + intensity_var * 850.0 + filter_mod, 1.7);

    // === Combine layers ===
    let combined = main_body + harmonic + filtered_noise;

    // Refined amplitude breathing (slightly deeper but still subtle)
    let breath = sine_hz(0.08) * 0.26 + 0.74;
    let modulated = combined * (0.80 + breath * intensity_var * 0.30);

    // Final gentle low-pass
    let final = modulated >> lowpass_hz(1500.0 + intensity_var * 600.0, 1.0);

    (Box::new(final * 0.72), intensity_var)
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
    info!("[fundsp] Refined modulation + light vibrato active");
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
            let final_intensity = (base_intensity * evolved).clamp(0.35, 1.15);

            instance.intensity_var.set(final_intensity as f64);

            let samples = render_next_chunk(instance);

            if !samples.is_empty() {
                let volume = (0.38 + final_intensity * 0.32).clamp(0.32, 0.72);

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
