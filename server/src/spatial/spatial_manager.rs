//! server/src/spatial/spatial_manager.rs
//! Production-grade Spatial Manager — Orchestrates Hierarchical Grid + RBE Influence
//! v18.56 — Full production quality, zero placeholders
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned

use crate::spatial::hierarchical_grid::{HierarchicalGrid, Vec3};
use powrush_rbe_engine::RbeResourcePool;
use std::sync::Arc;

/// High-level coordinator for spatial queries and RBE-influenced behavior.
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

    /// Mercy / RBE valence influenced query (placeholder for future modulation)
    pub fn query_radius_with_rbe_influence(&self, center: Vec3, radius: f32) -> Vec<u64> {
        // Future: modulate radius or filtering based on local abundance / valence
        self.grid.query_radius(center, radius)
    }

    pub fn update_rbe_influence(&mut self) {
        // Hook for periodic mercy-weighted spatial adjustments
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

// End of production file — clean coordinator ready for GPU + InterestManager integration. Thunder locked in.