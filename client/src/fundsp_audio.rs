/*!
 * fundsp Procedural Audio Prototype
 *
 * Supports live parameter updates (e.g. intensity) on active Epiphany resonance.
 */

use bevy::prelude::*;
use fundsp::hacker::*;

/// Builds a refined, evolving resonance graph for Epiphanies.
/// Intensity is now a live variable that can be updated while the graph is running.
pub fn build_epiphany_resonance(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let intensity_var = var( intensity as f64 );

    let base_freq = 65.0 + intensity_var * 160.0;

    let tone_a = sine_hz(base_freq);
    let tone_b = sine_hz(base_freq * 1.008);
    let main_body = (tone_a + tone_b) * (0.22 + intensity_var * 0.32);

    let harmonic = sine_hz(base_freq * 2.02) * (0.12 + intensity_var * 0.22);

    let noise_base = noise() * (0.12 + intensity_var * 0.38);
    let filtered_noise = noise_base >> lowpass_hz(280.0 + intensity_var * 1400.0, 1.8);

    let slow_mod = sine_hz(0.18) * 0.4 + 0.6;
    let modulated = (main_body + harmonic + filtered_noise)
        * (0.85 + slow_mod * intensity_var * 0.35);

    let final = modulated >> lowpass_hz(1600.0 + intensity_var * 700.0, 1.0);

    (Box::new(final * 0.72), intensity_var)
}

/// Represents an active rolling procedural Epiphany resonance with live parameters.
pub struct ActiveEpiphanyResonance {
    pub graph: Box<dyn AudioUnit64>,
    pub intensity_var: Shared<f64>,   // Live intensity handle
    pub remaining_duration: f32,
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

/// Update the intensity of an active Epiphany resonance (can be called anytime).
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
    info!("[fundsp] Live parameter updates supported");
}

/// System that renders and plays overlapping chunks from active procedural Epiphanies.
fn update_rolling_chunks(
    mut active: ResMut<ActiveProceduralEpiphanies>,
    spatial_manager: Res<crate::spatial_audio::SpatialAudioManager>,
) {
    let mut i = 0;
    while i < active.instances.len() {
        let instance = &mut active.instances[i];

        if instance.remaining_duration > 0.0 {
            let samples = render_next_chunk(instance);

            if !samples.is_empty() {
                let volume = (0.38 + (instance.intensity_var.get() as f32) * 0.32).clamp(0.32, 0.68);

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
