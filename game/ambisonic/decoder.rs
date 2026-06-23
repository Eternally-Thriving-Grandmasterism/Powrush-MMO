/*!
 * game/ambisonic/decoder.rs
 *
 * Basic Ambisonic Decoder to Binaural
 * Long-term foundation for Powrush-MMO
 *
 * Phase 1: Simple virtual speaker + existing HRTF path
 *
 * AG-SML v1.0
 */

use glam::Vec3;
use super::AmbisonicCoefficients;

/// Decode 1st-order Ambisonic coefficients to stereo (binaural approximation).
///
/// Current implementation: Simple virtual speaker decoding.
/// Future: Integrate with HRTF or proper binaural decoder.
pub fn decode_1st_order_to_stereo(coeffs: &AmbisonicCoefficients) -> (f32, f32) {
    // Very basic virtual speaker decoding for left/right
    // W contributes to both, X contributes to left/right balance
    let left = coeffs.w + coeffs.x * 0.5;
    let right = coeffs.w - coeffs.x * 0.5;

    // Simple Y/Z contribution (can be improved later)
    let left = left + coeffs.y * 0.3 + coeffs.z * 0.2;
    let right = right - coeffs.y * 0.3 + coeffs.z * 0.2;

    (left, right)
}

/// General decode entry point
pub fn decode_to_stereo(coeffs: &AmbisonicCoefficients) -> (f32, f32) {
    decode_1st_order_to_stereo(coeffs)
}

// Thunder locked in. Yoi ⚡
