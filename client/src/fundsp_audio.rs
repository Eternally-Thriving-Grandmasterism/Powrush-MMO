/*!
 * fundsp Procedural Audio Prototype
 *
 * Expanded reactive resonance graph for Epiphanies.
 * Generates evolving harmonic + textural layers based on intensity.
 */

use bevy::prelude::*;
use fundsp::hacker::*;

/// Builds an expanded, intensity-reactive resonance graph for Epiphanies.
pub fn build_epiphany_resonance(intensity: f32) -> Box<dyn AudioUnit64> {
    let intensity = intensity.clamp(0.0, 1.0);

    // Base frequency rises with intensity
    let base_freq = 70.0 + intensity * 180.0;

    // Main resonant tone (slightly detuned pair for richness)
    let tone1 = sine_hz(base_freq);
    let tone2 = sine_hz(base_freq * 1.005);
    let main_tone = (tone1 + tone2) * (0.25 + intensity * 0.35);

    // Harmonic layer (octave above)
    let harmonic = sine_hz(base_freq * 2.0) * (0.15 + intensity * 0.25);

    // Noise-based texture that becomes more prominent with intensity
    let noise_layer = noise() * (0.15 + intensity * 0.45);

    // Resonant low-pass filter that opens with intensity
    let filtered_noise = noise_layer >> lowpass_hz(300.0 + intensity * 1200.0, 1.5);

    // Combine all layers
    let combined = main_tone + harmonic + filtered_noise;

    // Soft envelope / amplitude modulation based on intensity
    let with_mod = combined >> (pass() | envelope(move |_| 0.7 + intensity * 0.6));

    // Final gentle low-pass to tame highs
    let final = with_mod >> lowpass_hz(1800.0 + intensity * 600.0, 0.8);

    Box::new(final * 0.8)
}

/// Renders a short audio buffer (in seconds) from the resonance graph.
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
    // Future: active graph management, real-time playback, etc.
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
    info!("[fundsp] Expanded Epiphany resonance prototype initialized");
}
