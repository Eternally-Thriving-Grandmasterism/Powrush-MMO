/*!
 * fundsp Procedural Audio Prototype
 *
 * Refined reactive resonance graph for Epiphanies.
 * Features evolving harmonics, modulation, and richer texture.
 */

use bevy::prelude::*;
use fundsp::hacker::*;

/// Builds a refined, evolving resonance graph for Epiphanies.
pub fn build_epiphany_resonance(intensity: f32) -> Box<dyn AudioUnit64> {
    let intensity = intensity.clamp(0.0, 1.0);

    // Base frequency with gentle upward drift based on intensity
    let base_freq = 65.0 + intensity * 160.0;

    // Rich detuned oscillator pair (main body)
    let tone_a = sine_hz(base_freq);
    let tone_b = sine_hz(base_freq * 1.008);
    let main_body = (tone_a + tone_b) * (0.22 + intensity * 0.32);

    // Harmonic layer (octave + slight detune)
    let harmonic = sine_hz(base_freq * 2.02) * (0.12 + intensity * 0.22);

    // Evolving noise texture with dynamic filtering
    let noise_base = noise() * (0.12 + intensity * 0.38);
    let filtered_noise = noise_base >> lowpass_hz(280.0 + intensity * 1400.0, 1.8);

    // Slow modulation (gentle "breathing" or swelling)
    let modulator = sine_hz(0.25) * 0.5 + 0.5; // slow LFO
    let modulated = (main_body + harmonic + filtered_noise) * (0.85 + modulator * intensity * 0.25);

    // Final gentle high-shelf / low-pass to keep it ethereal
    let final = modulated >> lowpass_hz(1600.0 + intensity * 700.0, 1.0);

    Box::new(final * 0.75)
}

/// Renders a short audio buffer from the resonance graph.
pub fn render_epiphany_buffer(intensity: f32, duration_secs: f32) -> Vec<f32> {
    let mut graph = build_epiphany_resonance(intensity);
    let sample_rate = 44100.0;
    let num_samples = (duration_secs * sample_rate) as usize;
    let mut buffer = vec![0.0; num_samples];
    graph.render(sample_rate, &mut buffer);
    buffer
}

#[derive(Resource, Default)]
pub struct FundspAudio {
    // Future expansion: real-time graphs, multiple active instances, etc.
}

pub struct FundspAudioPlugin;

impl Plugin for FundspAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<FundspAudio>()
            .add_systems(Startup, setup_fundsp);
    }
}

fn setup_fundsp(mut commands: Commands) {
    info!("[fundsp] Refined Epiphany resonance prototype ready");
}
