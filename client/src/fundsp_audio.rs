/*!
 * fundsp Procedural Audio Prototype
 *
 * Basic resonance layer for Epiphanies using functional DSP.
 * This is an experimental layer that can run alongside Kira (and later FMOD).
 */

use bevy::prelude::*;
use fundsp::hacker::*;

/// Builds a simple reactive resonance graph for Epiphanies.
/// Controlled by intensity (0.0 - 1.0).
pub fn build_epiphany_resonance(intensity: f32) -> Box<dyn AudioUnit64> {
    // Base sine oscillator whose frequency rises with intensity
    let base_freq = 80.0 + intensity * 120.0;

    // Main resonant tone
    let tone = sine_hz(base_freq)
        * (0.3 + intensity * 0.5);

    // Noise-based texture that increases with intensity
    let noise_texture = (noise() * 0.6 + sine_hz(base_freq * 1.5) * 0.4)
        * (0.1 + intensity * 0.6);

    // Low-pass filter that opens up with intensity
    let filtered = noise_texture >> lowpass_hz(400.0 + intensity * 800.0, 1.0);

    // Combine tone + texture
    let combined = (tone + filtered) * 0.7;

    // Add a soft envelope follower effect based on intensity
    let with_envelope = combined >> (pass() | envelope(move |t| 0.8 + intensity * 0.4));

    Box::new(with_envelope)
}

/// Resource to hold active procedural graphs (future expansion)
#[derive(Resource, Default)]
pub struct FundspAudio {
    // Placeholder for future graph management
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
    info!("[fundsp] Procedural audio prototype initialized");
}
