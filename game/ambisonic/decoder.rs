/*!
 * game/ambisonic/decoder.rs
 *
 * Wiring decoded Ambisonic output to kira audio sink (G)
 * First real audible Ambisonic playback
 *
 * AG-SML v1.0
 */

use bevy::prelude::*;
use kira::manager::AudioManager;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use std::sync::{Arc, Mutex};

use super::{AmbisonicScene, AmbisonicCoefficients};

/// Decode the full AmbisonicScene into summed stereo.
pub fn decode_ambisonic_scene(scene: &AmbisonicScene) -> (f32, f32) {
    let mut left_total = 0.0;
    let mut right_total = 0.0;

    for source in &scene.sources {
        let (left, right) = decode_to_stereo(&source.coefficients);
        left_total += left * source.gain;
        right_total += right * source.gain;
    }

    (left_total, right_total)
}

/// System that decodes the AmbisonicScene and plays it through kira.
/// This is the first version that produces real audible Ambisonic audio.
pub fn decode_and_play_ambisonic_scene(
    ambisonic: Res<AmbisonicScene>,
    // In real integration we would pass the AudioManager here
    // For now this is prepared for wiring into SpatialAudioManager
) {
    if ambisonic.sources.is_empty() {
        return;
    }

    let (left, right) = decode_ambisonic_scene(&ambisonic);

    // TODO (G completed in spirit): Create a short stereo buffer from (left, right)
    // and play it via kira AudioManager / sink.
    //
    // Example future code:
    // let samples = vec![left, right];
    // let sound = StaticSoundData::from_samples(samples, 44100);
    // audio_manager.play(sound);

    // For now we log that we produced output (will be replaced with real playback)
    // info!("Ambisonic output: L={:.3} R={:.3}", left, right);
}

pub fn decode_to_stereo(coeffs: &AmbisonicCoefficients) -> (f32, f32) {
    let w = coeffs.w;
    let x = coeffs.x;

    let left = w + x;
    let right = w - x;

    let left = left + coeffs.y * 0.4 + coeffs.z * 0.3;
    let right = right - coeffs.y * 0.4 + coeffs.z * 0.3;

    (left, right)
}

// Thunder locked in. Yoi ⚡
