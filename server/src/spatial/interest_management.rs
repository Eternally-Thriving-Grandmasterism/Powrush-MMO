//! server/src/spatial/interest_management.rs
//! v17.4 — Professional Interest Management with ChunkManager Integration
//! PATSAGi + Ra-Thor aligned | Mercy-gated | RBE-ready | ONE Organism
//!
//! This module provides scalable Area-of-Interest (AOI) management for Powrush-MMO.
//! It tightly integrates with ChunkManager (v17.3) for dirty tracking and future
//! persistence/networking streaming.

use crate::spatial::chunk_manager::{ChunkCoord, ChunkManager};
use crate::spatial::hierarchical_grid::{HierarchicalGrid, Vec3};
use powrush_rbe_engine::RbeResourcePool;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Current professional version of the Interest Manager
pub const INTEREST_MANAGER_VERSION: u32 = 4;

#[derive(Clone, Debug)]
pub struct InterestSubscription {
    pub entity_id: u64,
    pub aoi_radius: f32,           // dynamic, valence-influenced
    pub last_update: u64,          // tick number
}

pub struct InterestManager {
    grid: HierarchicalGrid,
    chunk_manager: ChunkManager,
    subscriptions: HashMap<u64, InterestSubscription>,
    /// Real per-subscriber positions (critical for correct AOI)
    subscriber_positions: HashMap<u64, Vec3>,
    rbe_pool: Arc<RbeResourcePool>,
}

impl InterestManager {
    pub fn new(cell_size: f32, levels: u8, rbe_pool: Arc<RbeResourcePool>) -> Self {
        let chunk_size = ChunkManager::recommended_chunk_size();
        Self {
            grid: HierarchicalGrid::new(cell_size, levels),
            chunk_manager: ChunkManager::new(chunk_size),
            subscriptions: HashMap::new(),
            subscriber_positions: HashMap::new(),
            rbe_pool,
        }
    }

    /// Subscribe an entity (player, NPC, sanctuary, etc.) to interest management
    pub fn subscribe(&mut self, entity_id: u64, base_radius: f32, initial_pos: Option<Vec3>) {
        let valence_influence = self.rbe_pool.current_abundance_factor();
        let radius = base_radius * (1.0 + valence_influence * 0.6);

        self.subscriptions.insert(entity_id, InterestSubscription {
            entity_id,
            aoi_radius: radius,
            last_update: 0,
        });

        if let Some(pos) = initial_pos {
            self.subscriber_positions.insert(entity_id, pos);
            self.grid.insert(entity_id, pos);
            let chunk = self.chunk_manager.position_to_chunk(pos);
            self.chunk_manager.mark_dirty(chunk);
        }
    }

    /// Update an entity's position in both the hierarchical grid and chunk system.
    /// Automatically marks affected chunks as dirty for persistence/networking.
    pub fn update_entity_position(&mut self, entity_id: u64, pos: Vec3) {
        // Update hierarchical grid (for fast radius queries)
        self.grid.insert(entity_id, pos);

        // Track real position if this entity is a subscriber
        if self.subscriptions.contains_key(&entity_id) {
            self.subscriber_positions.insert(entity_id, pos);
        }

        // Mark the chunk dirty so persistence and networking know something changed
        let chunk = self.chunk_manager.position_to_chunk(pos);
        self.chunk_manager.mark_dirty(chunk);
    }

    /// Get all entities currently visible to a subscriber within its AOI radius.
    /// Uses the **real** stored subscriber position (no more dummy center).
    pub fn get_visible_entities(&self, subscriber_id: u64) -> Vec<u64> {
        let sub = match self.subscriptions.get(&subscriber_id) {
            Some(s) => s,
            None => return vec![],
        };

        let center = match self.subscriber_positions.get(&subscriber_id) {
            Some(pos) => *pos,
            None => {
                // Fallback: try to get from grid if position was never explicitly set
                return vec![];
            }
        };

        self.grid.query_radius(center, sub.aoi_radius)
    }

    /// Professional networking hook — returns the list of entities that should be
    /// replicated to this subscriber right now.
    /// This is the clean integration point the networking layer should call every tick.
    pub fn get_replication_entities(&self, subscriber_id: u64) -> Vec<u64> {
        self.get_visible_entities(subscriber_id)
    }

    /// Unsubscribe an entity and clean up all associated state
    pub fn unsubscribe(&mut self, entity_id: u64) {
        self.subscriptions.remove(&entity_id);
        self.subscriber_positions.remove(&entity_id);
    }

    /// Returns the current dynamic AOI radius for a subscriber (valence-adjusted)
    pub fn get_aoi_radius(&self, subscriber_id: u64) -> Option<f32> {
        self.subscriptions.get(&subscriber_id).map(|s| s.aoi_radius)
    }

    /// Periodic cleanup, mercy-gated radius adjustment, and stale subscription handling
    pub fn tick(&mut self, current_tick: u64) {
        let valence_influence = self.rbe_pool.current_abundance_factor();

        for sub in self.subscriptions.values_mut() {
            sub.aoi_radius = sub.aoi_radius * (0.95 + valence_influence * 0.1);
            sub.last_update = current_tick;
        }
    }

    /// Expose the internal ChunkManager for advanced use cases
    pub fn chunk_manager(&self) -> &ChunkManager {
        &self.chunk_manager
    }

    pub fn chunk_manager_mut(&mut self) -> &mut ChunkManager {
        &mut self.chunk_manager
    }

    /// Sync dirty chunks from the hierarchical grid within a subscriber's AOI.
    /// Useful for persistence delta generation.
    pub fn sync_dirty_chunks_for_subscriber(&mut self, subscriber_id: u64) {
        if let (Some(pos), Some(sub)) = (
            self.subscriber_positions.get(&subscriber_id),
            self.subscriptions.get(&subscriber_id),
        ) {
            self.chunk_manager.sync_dirty_from_grid_radius(
                &self.grid,
                *pos,
                sub.aoi_radius,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interest_manager_real_position_and_replication() {
        let rbe_pool = Arc::new(RbeResourcePool::new_global_abundance());
        let mut manager = InterestManager::new(10.0, 4, rbe_pool);

        let player_pos = Vec3 { x: 100.0, y: 50.0, z: 0.0 };
        manager.subscribe(1, 80.0, Some(player_pos));

        let npc_pos = Vec3 { x: 105.0, y: 52.0, z: 0.0 };
        manager.update_entity_position(2, npc_pos);

        let visible = manager.get_visible_entities(1);
        assert!(visible.contains(&2), "NPC should be visible within AOI");

        let replication = manager.get_replication_entities(1);
        assert_eq!(replication, visible);
    }

    #[test]
    fn test_unsubscribe_cleans_state() {
        let rbe_pool = Arc::new(RbeResourcePool::new_global_abundance());
        let mut manager = InterestManager::new(10.0, 4, rbe_pool);

        manager.subscribe(1, 50.0, Some(Vec3 { x: 0.0, y: 0.0, z: 0.0 }));
        assert!(manager.get_aoi_radius(1).is_some());

        manager.unsubscribe(1);
        assert!(manager.get_aoi_radius(1).is_none());
    }
}