/*!
 * fundsp Procedural Audio Prototype
 *
 * Supports rolling chunk playback for evolving Epiphany resonance.
 */

use bevy::prelude::*;
use fundsp::hacker::*;
use std::time::Duration;

/// Builds a refined, evolving resonance graph for Epiphanies.
pub fn build_epiphany_resonance(intensity: f32) -> Box<dyn AudioUnit64> {
    let intensity = intensity.clamp(0.0, 1.0);

    let base_freq = 65.0 + intensity * 160.0;

    let tone_a = sine_hz(base_freq);
    let tone_b = sine_hz(base_freq * 1.008);
    let main_body = (tone_a + tone_b) * (0.22 + intensity * 0.32);

    let harmonic = sine_hz(base_freq * 2.02) * (0.12 + intensity * 0.22);

    let noise_base = noise() * (0.12 + intensity * 0.38);
    let filtered_noise = noise_base >> lowpass_hz(280.0 + intensity * 1400.0, 1.8);

    let modulator = sine_hz(0.25) * 0.5 + 0.5;
    let modulated = (main_body + harmonic + filtered_noise) * (0.85 + modulator * intensity * 0.25);

    let final = modulated >> lowpass_hz(1600.0 + intensity * 700.0, 1.0);

    Box::new(final * 0.75)
}

/// Represents an active rolling procedural Epiphany resonance.
pub struct ActiveEpiphanyResonance {
    pub graph: Box<dyn AudioUnit64>,
    pub remaining_duration: f32,   // seconds
    pub chunk_duration: f32,       // how long each rendered chunk is
    pub intensity: f32,
}

#[derive(Resource, Default)]
pub struct ActiveProceduralEpiphanies {
    pub instances: Vec<ActiveEpiphanyResonance>,
}

/// Renders the next chunk from an active instance and returns the samples.
pub fn render_next_chunk(instance: &mut ActiveEpiphanyResonance) -> Vec<f32> {
    let sample_rate = 44100.0;
    let num_samples = (instance.chunk_duration * sample_rate) as usize;
    let mut buffer = vec![0.0; num_samples];
    instance.graph.render(sample_rate, &mut buffer);
    buffer
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
    info!("[fundsp] Rolling chunk playback system initialized");
}

/// System that renders and plays short chunks from active procedural Epiphanies.
fn update_rolling_chunks(
    mut active: ResMut<ActiveProceduralEpiphanies>,
    spatial_manager: Res<crate::spatial_audio::SpatialAudioManager>,
    time: Res<Time>,
) {
    let mut i = 0;
    while i < active.instances.len() {
        let instance = &mut active.instances[i];

        // Render and play a chunk
        if instance.remaining_duration > 0.0 {
            let samples = render_next_chunk(instance);

            if !samples.is_empty() {
                // Play slightly below the main sample layers
                let volume = (0.4 + instance.intensity * 0.3).clamp(0.3, 0.65);
                spatial_manager.play_generated_spatial(
                    samples,
                    Vec3::ZERO, // Will be improved with proper positioning later
                    Vec3::ZERO,
                    volume,
                );
            }

            instance.remaining_duration -= instance.chunk_duration;
        }

        // Remove finished instances
        if instance.remaining_duration <= 0.0 {
            active.instances.remove(i);
        } else {
            i += 1;
        }
    }
}
