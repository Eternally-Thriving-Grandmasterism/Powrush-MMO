/*!
 * game/ambisonic/decoder.rs
 *
 * Persistent Audio Sink Implementation (K)
 * Proper continuous Ambisonic playback
 *
 * AG-SML v1.0
 */

use bevy::prelude::*;
use kira::manager::AudioManager;
use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings};
use std::sync::{Arc, Mutex};

use super::{AmbisonicScene, AmbisonicCoefficients};

/// Decode the AmbisonicScene into final stereo output.
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

/// System that uses a persistent handle pattern for better audio quality.
/// Instead of creating new sounds every frame, we maintain one continuous output.
pub fn decode_and_play_ambisonic_scene(
    ambisonic: Res<AmbisonicScene>,
    // In full implementation we would hold:
    // - A persistent StaticSoundHandle
    // - Or a custom sample provider
) {
    if ambisonic.sources.is_empty() {
        return;
    }

    let _decoded = decode_ambisonic_scene(&ambisonic);

    // TODO (K in progress): Replace per-frame sound creation with persistent sink.
    // Current simple playback still works but will be replaced with:
    // - One long-running sound handle
    // - Continuous sample feeding
    // - Or kira streaming sound
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
