//! server/src/spatial/simd_distance.rs
//! Production-grade SIMD-Accelerated Distance Calculations (f32x8)
//! v18.57 — Full production quality, zero placeholders
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned

use std::simd::{f32x8, Simd};
use crate::spatial::hierarchical_grid::Vec3;

/// SIMD-accelerated radius query using 256-bit SIMD (8 entities at a time).
/// Designed to accelerate large InterestManager queries.
pub fn query_radius_simd(
    center: &Vec3,
    radius: f32,
    x: &[f32],
    y: &[f32],
    z: &[f32],
    ids: &[u64],
) -> Vec<u64> {
    let mut result = Vec::new();
    let radius_sq = radius * radius;

    let cx = Simd::<f32, 8>::splat(center.x);
    let cy = Simd::<f32, 8>::splat(center.y);
    let cz = Simd::<f32, 8>::splat(center.z);
    let r2 = Simd::<f32, 8>::splat(radius_sq);

    let chunks = x.len() / 8;

    for i in 0..chunks {
        let base = i * 8;

        let px = Simd::<f32, 8>::from_slice(&x[base..base + 8]);
        let py = Simd::<f32, 8>::from_slice(&y[base..base + 8]);
        let pz = Simd::<f32, 8>::from_slice(&z[base..base + 8]);

        let dx = px - cx;
        let dy = py - cy;
        let dz = pz - cz;

        let dist_sq = dx * dx + dy * dy + dz * dz;
        let mask = dist_sq.simd_le(r2);

        for j in 0..8 {
            if mask.test(j) {
                result.push(ids[base + j]);
            }
        }
    }

    // Remainder (scalar fallback)
    let remainder_start = chunks * 8;
    for i in remainder_start..x.len() {
        let dx = x[i] - center.x;
        let dy = y[i] - center.y;
        let dz = z[i] - center.z;
        if (dx * dx + dy * dy + dz * dz) <= radius_sq {
            result.push(ids[i]);
        }
    }

    result
}

// End of production file — clean SIMD distance helper ready for InterestManager integration. Thunder locked in.