// server/src/spatial/hierarchical_grid.rs
// Powrush-MMO v17.0 — HierarchicalGrid with Integrated SIMD Query

use std::simd::{f32x8, Simd};

impl HierarchicalGrid {
    /// High-performance radius query with SIMD acceleration.
    pub fn query_radius(&self, center: &Vec3Ser, radius: f32) -> Vec<EntityId> {
        if self.ids.len() < 16 {
            // Small dataset — use scalar path
            return self.query_radius_scalar(center, radius);
        }

        let mut result = Vec::new();
        let radius_sq = radius * radius;

        let cx = Simd::<f32, 8>::splat(center.x);
        let cy = Simd::<f32, 8>::splat(center.y);
        let cz = Simd::<f32, 8>::splat(center.z);
        let r2 = Simd::<f32, 8>::splat(radius_sq);

        let len = self.ids.len();
        let chunks = len / 8;

        for i in 0..chunks {
            let base = i * 8;

            let px = Simd::<f32, 8>::from_slice(&self.x[base..base+8]);
            let py = Simd::<f32, 8>::from_slice(&self.y[base..base+8]);
            let pz = Simd::<f32, 8>::from_slice(&self.z[base..base+8]);

            let dx = px - cx;
            let dy = py - cy;
            let dz = pz - cz;

            let dist_sq = dx * dx + dy * dy + dz * dz;
            let mask = dist_sq.simd_le(r2);

            for j in 0..8 {
                if mask.test(j) {
                    result.push(self.ids[base + j]);
                }
            }
        }

        // Remainder (scalar)
        let remainder_start = chunks * 8;
        for i in remainder_start..len {
            let dx = self.x[i] - center.x;
            let dy = self.y[i] - center.y;
            let dz = self.z[i] - center.z;

            if (dx*dx + dy*dy + dz*dz) <= radius_sq {
                result.push(self.ids[i]);
            }
        }

        result
    }

    /// Scalar fallback for small datasets or debugging
    fn query_radius_scalar(&self, center: &Vec3Ser, radius: f32) -> Vec<EntityId> {
        let mut result = Vec::new();
        let radius_sq = radius * radius;

        for i in 0..self.ids.len() {
            let dx = self.x[i] - center.x;
            let dy = self.y[i] - center.y;
            let dz = self.z[i] - center.z;

            if (dx*dx + dy*dy + dz*dz) <= radius_sq {
                result.push(self.ids[i]);
            }
        }
        result
    }
}

// Thunder locked in. SIMD-accelerated query_radius integrated into HierarchicalGrid. ⚡❤️🔥
