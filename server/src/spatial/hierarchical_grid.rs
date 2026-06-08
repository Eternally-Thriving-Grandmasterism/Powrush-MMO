// server/src/spatial/hierarchical_grid.rs
// Powrush-MMO v17.0 — HierarchicalGrid Tests (Phase 1)

#[cfg(test)]
mod tests {
    use super::*;
    use shared::protocol::Vec3Ser;

    fn naive_query_radius(
        positions: &[(u64, Vec3Ser)],
        center: &Vec3Ser,
        radius: f32,
    ) -> Vec<u64> {
        let mut result = Vec::new();
        let r2 = radius * radius;

        for &(id, pos) in positions {
            let dx = pos.x - center.x;
            let dy = pos.y - center.y;
            let dz = pos.z - center.z;
            if (dx*dx + dy*dy + dz*dz) <= r2 {
                result.push(id);
            }
        }
        result.sort();
        result
    }

    #[test]
    fn test_basic_insert_and_query() {
        let mut grid = HierarchicalGrid::with_default_levels();

        grid.insert_or_update(1, Vec3Ser { x: 100.0, y: 0.0, z: 100.0 });
        grid.insert_or_update(2, Vec3Ser { x: 200.0, y: 0.0, z: 200.0 });

        let results = grid.query_radius(&Vec3Ser { x: 100.0, y: 0.0, z: 100.0 }, 50.0);
        assert!(results.contains(&1));
        assert!(!results.contains(&2));
    }

    #[test]
    fn test_update_position() {
        let mut grid = HierarchicalGrid::with_default_levels();
        grid.insert_or_update(1, Vec3Ser { x: 0.0, y: 0.0, z: 0.0 });

        grid.insert_or_update(1, Vec3Ser { x: 500.0, y: 0.0, z: 500.0 });

        let results = grid.query_radius(&Vec3Ser { x: 500.0, y: 0.0, z: 500.0 }, 10.0);
        assert!(results.contains(&1));
    }

    #[test]
    fn test_remove() {
        let mut grid = HierarchicalGrid::with_default_levels();
        grid.insert_or_update(1, Vec3Ser { x: 100.0, y: 0.0, z: 100.0 });
        grid.remove(1);

        let results = grid.query_radius(&Vec3Ser { x: 100.0, y: 0.0, z: 100.0 }, 1000.0);
        assert!(!results.contains(&1));
    }

    #[test]
    fn test_simd_matches_scalar() {
        let mut grid = HierarchicalGrid::with_default_levels();

        // Insert many entities
        for i in 0..200 {
            let x = (i % 30) as f32 * 40.0;
            let z = (i / 30) as f32 * 40.0;
            grid.insert_or_update(i as u64, Vec3Ser { x, y: 0.0, z });
        }

        let center = Vec3Ser { x: 200.0, y: 0.0, z: 200.0 };
        let radius = 150.0;

        let simd_results = grid.query_radius(&center, radius);
        let mut scalar_results = Vec::new();

        // Manually compute scalar result using internal arrays (for validation)
        let r2 = radius * radius;
        for i in 0..grid.ids.len() {
            let dx = grid.x[i] - center.x;
            let dy = grid.y[i] - center.y;
            let dz = grid.z[i] - center.z;
            if (dx*dx + dy*dy + dz*dz) <= r2 {
                scalar_results.push(grid.ids[i]);
            }
        }
        scalar_results.sort();
        let mut simd_sorted = simd_results;
        simd_sorted.sort();

        assert_eq!(simd_sorted, scalar_results);
    }

    #[test]
    fn test_edge_cases() {
        let mut grid = HierarchicalGrid::with_default_levels();

        // Empty query
        let results = grid.query_radius(&Vec3Ser { x: 0.0, y: 0.0, z: 0.0 }, 100.0);
        assert!(results.is_empty());

        // Radius = 0
        grid.insert_or_update(42, Vec3Ser { x: 50.0, y: 0.0, z: 50.0 });
        let results = grid.query_radius(&Vec3Ser { x: 50.0, y: 0.0, z: 50.0 }, 0.0);
        assert!(results.contains(&42));
    }
}

// Thunder locked in. Phase 1 test suite started for HierarchicalGrid. ⚡❤️🔥
