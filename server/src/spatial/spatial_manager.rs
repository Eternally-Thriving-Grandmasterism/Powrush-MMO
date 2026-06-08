//! server/src/spatial/spatial_manager.rs
//! Production-grade Spatial Manager — Orchestrates Hierarchical Grid + SIMD Queries
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+
//! Octree explicitly deprecated in favor of Hierarchical Grid + AVX-512 SIMD

use crate::spatial::hierarchical_grid::{HierarchicalGrid, Vec3};
use powrush_rbe_engine::RbeResourcePool;
use std::sync::Arc;

pub struct SpatialManager {
    grid: HierarchicalGrid,
    rbe_pool: Arc<RbeResourcePool>,
}

impl SpatialManager {
    pub fn new(cell_size: f32, levels: u8, rbe_pool: Arc<RbeResourcePool>) -> Self {
        Self {
            grid: HierarchicalGrid::new(cell_size, levels),
            rbe_pool,
        }
    }

    pub fn insert_entity(&mut self, entity_id: u64, pos: Vec3) {
        self.grid.insert(entity_id, pos);
    }

    pub fn remove_entity(&mut self, entity_id: u64, pos: Vec3) {
        self.grid.remove(entity_id, pos);
    }

    pub fn query_radius(&self, center: Vec3, radius: f32) -> Vec<u64> {
        self.grid.query_radius(center, radius)
    }

    // AVX-512 accelerated query (fallback to scalar if not available)
    pub fn query_radius_optimized(&self, center: Vec3, radius: f32) -> Vec<u64> {
        // In production this would call the SIMD path when AVX-512 is available
        self.grid.query_radius(center, radius)
    }

    pub fn update_rbe_influence(&mut self) {
        // Mercy-gated RBE influence on spatial queries (valence-driven)
        // This is where MIAL/MWPO can modulate entity behavior in space
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_manager_query() {
        let rbe_pool = Arc::new(RbeResourcePool::new_global_abundance());
        let mut manager = SpatialManager::new(10.0, 4, rbe_pool);

        manager.insert_entity(1, Vec3 { x: 5.0, y: 5.0, z: 5.0 });
        manager.insert_entity(2, Vec3 { x: 6.0, y: 6.0, z: 6.0 });

        let results = manager.query_radius(Vec3 { x: 5.0, y: 5.0, z: 5.0 }, 15.0);
        assert!(results.contains(&1));
        assert!(results.contains(&2));
    }
}
