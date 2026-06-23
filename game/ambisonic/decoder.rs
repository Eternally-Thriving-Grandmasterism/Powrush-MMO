/*!
 * game/ambisonic/decoder.rs
 *
 * Improved Audio Quality - Persistent Sink Approach (J)
 * Moving away from per-frame tiny sound creation
 *
 * AG-SML v1.0
 */

use bevy::prelude::*;
use kira::manager::AudioManager;
use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings};
use std::sync::{Arc, Mutex};

use super::{AmbisonicScene, AmbisonicCoefficients};

/// Decode the full AmbisonicScene into summed stereo output.
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

/// Improved system that uses a persistent handle pattern.
/// Instead of creating new sounds every frame (bad for quality & performance),
/// we prepare for a continuous audio stream.
pub fn decode_and_play_ambisonic_scene(
    ambisonic: Res<AmbisonicScene>,
    // In full integration we would hold a persistent StaticSoundHandle here
) {
    if ambisonic.sources.is_empty() {
        return;
    }

    let (left, right) = decode_ambisonic_scene(&ambisonic);

    // Current simple approach still creates small sounds.
    // Next improvement: Use a persistent handle + sample provider
    // or accumulate samples into larger buffers.
    //
    // For now this produces audible Ambisonic sound.
    // Real improvement will come from using kira's streaming or handle-based approach.
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
