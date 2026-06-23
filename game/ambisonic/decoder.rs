/*!
 * game/ambisonic/decoder.rs
 *
 * Ambisonic Decoder with Audio Output Hook (E)
 * Long-term Hybrid Architecture
 *
 * AG-SML v1.0
 */

use super::AmbisonicCoefficients;

/// Decode Ambisonic coefficients to stereo samples.
/// Returns (left, right) ready to be sent to an audio sink.
pub fn decode_to_stereo(coeffs: &AmbisonicCoefficients) -> (f32, f32) {
    let w = coeffs.w;
    let x = coeffs.x;

    let left = w + x;
    let right = w - x;

    // Gentle contribution from Y/Z
    let left = left + coeffs.y * 0.4 + coeffs.z * 0.3;
    let right = right - coeffs.y * 0.4 + coeffs.z * 0.3;

    (left, right)
}

/// Future integration point:
/// This function will be called per-frame to decode the entire AmbisonicScene
/// and feed the resulting stereo buffer into kira / rodio / Bevy audio.
pub fn decode_scene_to_audio_output() {
    // TODO (E): Sum all decoded sources and send to audio sink
    // For now this is a placeholder for the output pipeline.
}

// Thunder locked in. Yoi ⚡
