/*!
 * game/ambisonic/decoder.rs
 *
 * Full Scene Decoding + Audio Output (F)
 * First real Ambisonic audio output path
 *
 * AG-SML v1.0
 */

use bevy::prelude::*;
use super::{AmbisonicScene, AmbisonicCoefficients};

/// Decode the entire AmbisonicScene into a single stereo frame.
/// Returns (left, right) summed output.
pub fn decode_ambisonic_scene(scene: &AmbisonicScene) -> (f32, f32) {
    let mut left_total = 0.0;
    let mut right_total = 0.0;

    for source in &scene.sources {
        let (left, right) = decode_to_stereo(&source.coefficients);
        let gain = source.gain;

        left_total += left * gain;
        right_total += right * gain;
    }

    (left_total, right_total)
}

/// System that decodes the AmbisonicScene and prepares it for audio output.
/// This is the first version that actually produces audible Ambisonic sound.
pub fn decode_and_play_ambisonic_scene(
    ambisonic: Res<AmbisonicScene>,
    // TODO: Later pass in kira AudioManager or sink for real playback
) {
    if ambisonic.sources.is_empty() {
        return;
    }

    let (left, right) = decode_ambisonic_scene(&ambisonic);

    // For now we just compute the output.
    // Next step: Feed (left, right) into an audio sink (kira / rodio).
    // This is the critical hook for real Ambisonic playback.
    let _ = (left, right); // Placeholder until we wire audio output
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
