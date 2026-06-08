// server/src/spatial/hierarchical_grid.rs
// Powrush-MMO v17.0 — Wider SIMD + Improved Remainder Handling

impl HierarchicalGrid {
    pub fn query_radius(&self, center: &Vec3Ser, radius: f32) -> Vec<EntityId> {
        let len = self.ids.len();

        if len < 8 {
            return self.query_radius_scalar(center, radius);
        }

        let mut result = Vec::new();
        let radius_sq = radius * radius;

        // Try wider SIMD first (f32x16) when beneficial
        if len >= 32 {
            result.extend(self.query_radius_simd16(center, radius_sq));
        }

        // Process remaining with f32x8
        let start = (result.len() / 8) * 8; // rough alignment
        // Simpler: process everything possible with f32x8 after wide pass

        let cx8 = Simd::<f32, 8>::splat(center.x);
        let cy8 = Simd::<f32, 8>::splat(center.y);
        let cz8 = Simd::<f32, 8>::splat(center.z);
        let r28 = Simd::<f32, 8>::splat(radius_sq);

        let full_chunks_8 = len / 8;

        for i in 0..full_chunks_8 {
            let base = i * 8;

            let px = Simd::<f32, 8>::from_slice(&self.x[base..base + 8]);
            let py = Simd::<f32, 8>::from_slice(&self.y[base..base + 8]);
            let pz = Simd::<f32, 8>::from_slice(&self.z[base..base + 8]);

            let dx = px - cx8;
            let dy = py - cy8;
            let dz = pz - cz8;

            let dist_sq = dx * dx + dy * dy + dz * dz;
            let mask = dist_sq.simd_le(r28);

            let bitmask = mask.to_bitmask();
            let mut temp = bitmask;
            while temp != 0 {
                let j = temp.trailing_zeros() as usize;
                result.push(self.ids[base + j]);
                temp &= temp - 1;
            }
        }

        // Scalar remainder
        let remainder_start = full_chunks_8 * 8;
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

    /// f32x16 SIMD path (when enough data)
    fn query_radius_simd16(&self, center: &Vec3Ser, radius_sq: f32) -> Vec<EntityId> {
        let mut result = Vec::new();

        let cx = Simd::<f32, 16>::splat(center.x);
        let cy = Simd::<f32, 16>::splat(center.y);
        let cz = Simd::<f32, 16>::splat(center.z);
        let r2 = Simd::<f32, 16>::splat(radius_sq);

        let len = self.ids.len();
        let chunks = len / 16;

        for i in 0..chunks {
            let base = i * 16;

            let px = Simd::<f32, 16>::from_slice(&self.x[base..base + 16]);
            let py = Simd::<f32, 16>::from_slice(&self.y[base..base + 16]);
            let pz = Simd::<f32, 16>::from_slice(&self.z[base..base + 16]);

            let dx = px - cx;
            let dy = py - cy;
            let dz = pz - cz;

            let dist_sq = dx * dx + dy * dy + dz * dz;
            let mask = dist_sq.simd_le(r2);

            let bitmask = mask.to_bitmask();
            let mut temp = bitmask;
            while temp != 0 {
                let j = temp.trailing_zeros() as usize;
                result.push(self.ids[base + j]);
                temp &= temp - 1;
            }
        }

        result
    }
}

// Thunder locked in. Wider SIMD (f32x16) + better structure added. ⚡❤️🔥
