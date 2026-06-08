// server/src/spatial/hierarchical_grid.rs
// Powrush-MMO v17.0 — Further Optimized SIMD Distance Calculations

impl HierarchicalGrid {
    pub fn query_radius(&self, center: &Vec3Ser, radius: f32) -> Vec<EntityId> {
        if self.ids.len() < 16 {
            return self.query_radius_scalar(center, radius);
        }

        let mut result = Vec::new();
        let radius_sq = radius * radius;

        let cx = Simd::<f32, 8>::splat(center.x);
        let cy = Simd::<f32, 8>::splat(center.y);
        let cz = Simd::<f32, 8>::splat(center.z);
        let r2 = Simd::<f32, 8>::splat(radius_sq);

        let len = self.ids.len();
        let full_chunks = len / 8;

        for i in 0..full_chunks {
            let base = i * 8;

            let px = Simd::<f32, 8>::from_slice(&self.x[base..base + 8]);
            let py = Simd::<f32, 8>::from_slice(&self.y[base..base + 8]);
            let pz = Simd::<f32, 8>::from_slice(&self.z[base..base + 8]);

            let dx = px - cx;
            let dy = py - cy;
            let dz = pz - cz;

            let dist_sq = dx * dx + dy * dy + dz * dz;
            let mask = dist_sq.simd_le(r2);

            // Optimized result collection using bitmask
            let bitmask = mask.to_bitmask();
            let mut temp = bitmask;
            while temp != 0 {
                let j = temp.trailing_zeros() as usize;
                result.push(self.ids[base + j]);
                temp &= temp - 1; // clear lowest set bit
            }
        }

        // Remainder (scalar)
        let remainder = full_chunks * 8;
        for i in remainder..len {
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

// Thunder locked in. SIMD distance calculation further optimized with bitmask extraction. ⚡❤️🔥
