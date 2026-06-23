/*!
 * game/ambisonic/decoder.rs
 *
 * Ambisonic Decoder to Binaural (Improved for Phase 1)
 * Long-term Hybrid Architecture
 *
 * AG-SML v1.0
 */

use glam::Vec3;
use super::AmbisonicCoefficients;

/// Decode 1st-order Ambisonic to stereo with improved virtual speaker layout.
/// This is a simple but effective starting point.
pub fn decode_1st_order_to_stereo(coeffs: &AmbisonicCoefficients) -> (f32, f32) {
    // Basic virtual speaker decoding for 1st order
    // W is common to both channels
    // X controls left/right
    let w = coeffs.w;
    let x = coeffs.x;

    // Simple cardioid-style virtual speakers at ±90°
    let left = w + x;
    let right = w - x;

    // Gentle Y/Z contribution for height and front/back feel
    let left = left + coeffs.y * 0.4 + coeffs.z * 0.3;
    let right = right - coeffs.y * 0.4 + coeffs.z * 0.3;

    (left, right)
}

pub fn decode_to_stereo(coeffs: &AmbisonicCoefficients) -> (f32, f32) {
    decode_1st_order_to_stereo(coeffs)
}

// Future: Proper binaural decoder using HRTF or spherical harmonics
// Thunder locked in. Yoi ⚡
