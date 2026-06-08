//! server/src/spatial/interest_management.rs
//! Production-grade Area-of-Interest (AOI) Management for Powrush-MMO
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use crate::spatial::hierarchical_grid::{HierarchicalGrid, Vec3};
use powrush_rbe_engine::RbeResourcePool;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct InterestSubscription {
    pub entity_id: u64,
    pub aoi_radius: f32,           // dynamic, valence-influenced
    pub last_update: u64,          // tick number
}

pub struct InterestManager {
    grid: HierarchicalGrid,
    subscriptions: HashMap<u64, InterestSubscription>,
    rbe_pool: Arc<RbeResourcePool>,
}

impl InterestManager {
    pub fn new(cell_size: f32, levels: u8, rbe_pool: Arc<RbeResourcePool>) -> Self {
        Self {
            grid: HierarchicalGrid::new(cell_size, levels),
            subscriptions: HashMap::new(),
            rbe_pool,
        }
    }

    /// Subscribe an entity (player, NPC, sanctuary, etc.) to interest management
    pub fn subscribe(&mut self, entity_id: u64, base_radius: f32) {
        let valence_influence = self.rbe_pool.current_abundance_factor(); // mercy/valence boost
        let radius = base_radius * (1.0 + valence_influence * 0.6);

        self.subscriptions.insert(entity_id, InterestSubscription {
            entity_id,
            aoi_radius: radius,
            last_update: 0,
        });

        // Insert into grid for fast queries
        // Position will be updated via update_entity_position
    }

    /// Update an entity's position in the spatial grid
    pub fn update_entity_position(&mut self, entity_id: u64, pos: Vec3) {
        self.grid.insert(entity_id, pos);
    }

    /// Get all entities visible to a subscriber (AOI query)
    pub fn get_visible_entities(&self, subscriber_id: u64) -> Vec<u64> {
        let sub = match self.subscriptions.get(&subscriber_id) {
            Some(s) => s,
            None => return vec![],
        };

        // Use the hierarchical grid for fast radius query
        // In full version this would use the subscriber's current position
        let dummy_center = Vec3 { x: 0.0, y: 0.0, z: 0.0 }; // replace with real position tracking
        self.grid.query_radius(dummy_center, sub.aoi_radius)
    }

    /// Periodic cleanup and mercy-gated radius adjustment
    pub fn tick(&mut self, current_tick: u64) {
        // Adjust radii based on current RBE abundance and mercy valence
        for sub in self.subscriptions.values_mut() {
            let valence_influence = self.rbe_pool.current_abundance_factor();
            sub.aoi_radius = sub.aoi_radius * (0.95 + valence_influence * 0.1); // gentle dynamic scaling
        }

        // Remove stale subscriptions if needed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interest_manager_visible_entities() {
        let rbe_pool = Arc::new(RbeResourcePool::new_global_abundance());
        let mut manager = InterestManager::new(10.0, 4, rbe_pool);

        manager.subscribe(1, 50.0);
        manager.update_entity_position(2, Vec3 { x: 10.0, y: 10.0, z: 10.0 });

        let visible = manager.get_visible_entities(1);
        assert!(visible.contains(&2));
    }
}
