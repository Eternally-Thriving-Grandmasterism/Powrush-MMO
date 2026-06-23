/*!
 * game/ambisonic/encoder.rs
 *
 * Basic Ambisonic Encoder (starting with 1st order)
 * Long-term foundation for Powrush-MMO spatial audio
 *
 * AG-SML v1.0
 */

use glam::Vec3;
use super::{AmbisonicCoefficients, AmbisonicOrder};

/// Encode a mono signal into 1st-order Ambisonic coefficients
/// based on source position relative to listener.
///
/// This is a simplified 1st-order encoder.
pub fn encode_1st_order(
    source_pos: Vec3,
    listener_pos: Vec3,
    signal: f32,
) -> AmbisonicCoefficients {
    let direction = (source_pos - listener_pos).normalize_or_zero();

    // W = omnidirectional (scaled)
    let w = signal * 0.7071; // 1/sqrt(2) normalization for 1st order

    // X, Y, Z directional components
    let x = signal * direction.x;
    let y = signal * direction.y;
    let z = signal * direction.z;

    AmbisonicCoefficients::new(w, x, y, z)
}

/// Higher-order stub (for future expansion)
pub fn encode(order: AmbisonicOrder, source_pos: Vec3, listener_pos: Vec3, signal: f32) -> AmbisonicCoefficients {
    match order {
        AmbisonicOrder::First => encode_1st_order(source_pos, listener_pos, signal),
        _ => {
            // TODO: Implement 2nd/3rd order encoding
            encode_1st_order(source_pos, listener_pos, signal)
        }
    }
}

// Thunder locked in. Yoi ⚡
