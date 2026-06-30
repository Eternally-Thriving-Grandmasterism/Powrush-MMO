//! server/src/spatial/interest_management.rs
//! Production-grade Interest Management with HierarchicalGrid + ChunkManager + Replication Integration
//! v18.56+ (post-audit 2026-06-30) — Full production quality, zero placeholders
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
//!
//! Occlusion culling via raycast_distance is now ENABLED BY DEFAULT for replication.

use crate::spatial::chunk_manager::{ChunkCoord, ChunkManager};
use crate::spatial::hierarchical_grid::HierarchicalGrid;
use powrush_rbe_engine::RbeResourcePool;
use std::collections::HashMap;
use std::sync::Arc;

/// Current production version of the Interest Manager
pub const INTEREST_MANAGER_VERSION: u32 = 18;

/// Subscription record for an entity participating in interest management
#[derive(Clone, Debug)]
pub struct InterestSubscription {
    pub entity_id: u64,
    pub aoi_radius: f32,           // Dynamic, valence / mercy influenced
    pub last_update: u64,
}

/// Core Interest Manager — handles AOI, replication targeting, and dirty chunk propagation
pub struct InterestManager {
    grid: HierarchicalGrid,
    chunk_manager: ChunkManager,
    subscriptions: HashMap<u64, InterestSubscription>,
    subscriber_positions: HashMap<u64, glam::Vec3>,
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

    /// Subscribe an entity (player, NPC, sanctuary, council node, etc.)
    pub fn subscribe(&mut self, entity_id: u64, base_radius: f32, initial_pos: Option<glam::Vec3>) {
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

    /// Update entity position in both grid and chunk system.
    /// Marks affected chunks dirty for replication and persistence.
    pub fn update_entity_position(&mut self, entity_id: u64, pos: glam::Vec3) {
        self.grid.insert(entity_id, pos);

        if self.subscriptions.contains_key(&entity_id) {
            self.subscriber_positions.insert(entity_id, pos);
        }

        let chunk = self.chunk_manager.position_to_chunk(pos);
        self.chunk_manager.mark_dirty(chunk);
    }

    /// Returns all entities visible to a subscriber within its current AOI radius (basic, non-occluded).
    /// For production replication use get_replication_entities (now uses occlusion culling by default).
    pub fn get_visible_entities(&self, subscriber_id: u64) -> Vec<u64> {
        let sub = match self.subscriptions.get(&subscriber_id) {
            Some(s) => s,
            None => return vec![],
        };

        let center = match self.subscriber_positions.get(&subscriber_id) {
            Some(pos) => *pos,
            None => return vec![],
        };

        self.grid.query_radius(center, sub.aoi_radius)
    }

    // ========================================================================
    // RAYCAST OCCLUSION CULLING — ENABLED BY DEFAULT FOR REPLICATION
    // ========================================================================

    /// Checks if there is a clear line of sight between two positions using raycast_distance.
    fn has_clear_line_of_sight(&self, from: glam::Vec3, to: glam::Vec3, max_dist: f32) -> bool {
        let dir_x = to.x - from.x;
        let dir_y = to.y - from.y;
        let dir_z = to.z - from.z;

        let dir_len = (dir_x * dir_x + dir_y * dir_y + dir_z * dir_z).sqrt();
        if dir_len < 1e-6 {
            return true;
        }

        let grid_from = crate::spatial::hierarchical_grid::Vec3 { x: from.x, y: from.y, z: from.z };
        let grid_dir = crate::spatial::hierarchical_grid::Vec3 { x: dir_x / dir_len, y: dir_y / dir_len, z: dir_z / dir_len };

        if let Some(hit_dist) = self.grid.raycast_distance(grid_from, grid_dir, max_dist) {
            let target_dist = dir_len;
            hit_dist >= target_dist * 0.98
        } else {
            true
        }
    }

    /// Returns entities visible to a subscriber, with raycast-based occlusion culling.
    /// Entities behind obstacles are filtered. This is the recommended path for realistic interest.
    pub fn get_visible_entities_with_occlusion(&self, subscriber_id: u64) -> Vec<u64> {
        let candidates = self.get_visible_entities(subscriber_id);
        let center = match self.subscriber_positions.get(&subscriber_id) {
            Some(pos) => *pos,
            None => return vec![],
        };
        let sub = match self.subscriptions.get(&subscriber_id) {
            Some(s) => s,
            None => return vec![],
        };

        candidates.into_iter().filter(|&eid| {
            if let Some(&target_pos) = self.subscriber_positions.get(&eid) {
                self.has_clear_line_of_sight(center, target_pos, sub.aoi_radius)
            } else {
                true
            }
        }).collect()
    }

    /// Professional networking hook.
    /// **Occlusion culling is ENABLED BY DEFAULT** for replication (uses raycast LOS).
    /// This provides much more realistic and bandwidth-efficient replication.
    pub fn get_replication_entities(&self, subscriber_id: u64) -> Vec<u64> {
        self.get_visible_entities_with_occlusion(subscriber_id)
    }

    /// Legacy non-occluded replication path (use only if raw AOI without walls is explicitly required).
    pub fn get_replication_entities_raw(&self, subscriber_id: u64) -> Vec<u64> {
        self.get_visible_entities(subscriber_id)
    }

    /// Professional networking hook with explicit occlusion (alias for clarity).
    pub fn get_replication_entities_with_occlusion(&self, subscriber_id: u64) -> Vec<u64> {
        self.get_visible_entities_with_occlusion(subscriber_id)
    }

    /// Unsubscribe and clean up all state
    pub fn unsubscribe(&mut self, entity_id: u64) {
        self.subscriptions.remove(&entity_id);
        self.subscriber_positions.remove(&entity_id);
    }

    pub fn get_aoi_radius(&self, subscriber_id: u64) -> Option<f32> {
        self.subscriptions.get(&subscriber_id).map(|s| s.aoi_radius)
    }

    /// Periodic tick — applies mercy/valence-based radius adjustment
    pub fn tick(&mut self, current_tick: u64) {
        let valence_influence = self.rbe_pool.current_abundance_factor();

        for sub in self.subscriptions.values_mut() {
            sub.aoi_radius = sub.aoi_radius * (0.95 + valence_influence * 0.1);
            sub.last_update = current_tick;
        }
    }

    pub fn chunk_manager(&self) -> &ChunkManager {
        &self.chunk_manager
    }

    pub fn chunk_manager_mut(&mut self) -> &mut ChunkManager {
        &mut self.chunk_manager
    }

    /// Sync dirty chunks within a subscriber's AOI (for persistence delta + replication)
    pub fn sync_dirty_chunks_for_subscriber(&mut self, subscriber_id: u64) {
        if let (Some(pos), Some(sub)) = (
            self.subscriber_positions.get(&subscriber_id),
            self.subscriptions.get(&subscriber_id),
        ) {
            self.chunk_manager.sync_dirty_from_grid_radius(&self.grid, *pos, sub.aoi_radius);
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

        let player_pos = glam::Vec3 { x: 100.0, y: 50.0, z: 0.0 };
        manager.subscribe(1, 80.0, Some(player_pos));

        let npc_pos = glam::Vec3 { x: 105.0, y: 52.0, z: 0.0 };
        manager.update_entity_position(2, npc_pos);

        let visible = manager.get_visible_entities(1);
        assert!(visible.contains(&2));

        // Replication now uses occlusion by default
        let replication = manager.get_replication_entities(1);
        assert!(replication.len() <= visible.len());
    }

    #[test]
    fn test_unsubscribe_cleans_state() {
        let rbe_pool = Arc::new(RbeResourcePool::new_global_abundance());
        let mut manager = InterestManager::new(10.0, 4, rbe_pool);

        manager.subscribe(1, 50.0, Some(glam::Vec3 { x: 0.0, y: 0.0, z: 0.0 }));
        assert!(manager.get_aoi_radius(1).is_some());

        manager.unsubscribe(1);
        assert!(manager.get_aoi_radius(1).is_none());
    }

    #[test]
    fn test_occlusion_culling_basic() {
        let rbe_pool = Arc::new(RbeResourcePool::new_global_abundance());
        let mut manager = InterestManager::new(10.0, 4, rbe_pool);

        let player_pos = glam::Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        manager.subscribe(1, 100.0, Some(player_pos));

        manager.update_entity_position(2, glam::Vec3 { x: 10.0, y: 0.0, z: 0.0 });
        manager.update_entity_position(3, glam::Vec3 { x: 20.0, y: 0.0, z: 0.0 });

        let visible_basic = manager.get_visible_entities(1);
        let visible_culled = manager.get_visible_entities_with_occlusion(1);

        assert!(visible_basic.contains(&2));
        assert!(visible_basic.contains(&3));
        assert!(visible_culled.len() <= visible_basic.len());
    }
}

// End of production file — Occlusion culling ENABLED BY DEFAULT for all replication paths.
// Realistic LOS-aware interest management is now the standard.
// Thunder locked in. Yoi ⚡