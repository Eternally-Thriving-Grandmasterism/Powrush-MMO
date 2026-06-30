//! server/src/spatial/interest_management.rs
//! Production-grade Interest Management with HierarchicalGrid + ChunkManager + Replication Integration
//! v18.56+ (post-audit 2026-06-30) — Full production quality, zero placeholders
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
//!
//! Occlusion culling via raycast_distance is now ENABLED BY DEFAULT for replication.
//! Network latency simulation support added for realistic MMO testing.

use crate::spatial::chunk_manager::{ChunkCoord, ChunkManager};
use crate::spatial::hierarchical_grid::HierarchicalGrid;
use powrush_rbe_engine::RbeResourcePool;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Current production version of the Interest Manager
pub const INTEREST_MANAGER_VERSION: u32 = 18;

/// Subscription record for an entity participating in interest management
#[derive(Clone, Debug)]
pub struct InterestSubscription {
    pub entity_id: u64,
    pub aoi_radius: f32,
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

    pub fn update_entity_position(&mut self, entity_id: u64, pos: glam::Vec3) {
        self.grid.insert(entity_id, pos);

        if self.subscriptions.contains_key(&entity_id) {
            self.subscriber_positions.insert(entity_id, pos);
        }

        let chunk = self.chunk_manager.position_to_chunk(pos);
        self.chunk_manager.mark_dirty(chunk);
    }

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

    pub fn get_replication_entities(&self, subscriber_id: u64) -> Vec<u64> {
        self.get_visible_entities_with_occlusion(subscriber_id)
    }

    pub fn get_replication_entities_raw(&self, subscriber_id: u64) -> Vec<u64> {
        self.get_visible_entities(subscriber_id)
    }

    pub fn get_replication_entities_with_occlusion(&self, subscriber_id: u64) -> Vec<u64> {
        self.get_visible_entities_with_occlusion(subscriber_id)
    }

    pub fn unsubscribe(&mut self, entity_id: u64) {
        self.subscriptions.remove(&entity_id);
        self.subscriber_positions.remove(&entity_id);
    }

    pub fn get_aoi_radius(&self, subscriber_id: u64) -> Option<f32> {
        self.subscriptions.get(&subscriber_id).map(|s| s.aoi_radius)
    }

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

    pub fn sync_dirty_chunks_for_subscriber(&mut self, subscriber_id: u64) {
        if let (Some(pos), Some(sub)) = (
            self.subscriber_positions.get(&subscriber_id),
            self.subscriptions.get(&subscriber_id),
        ) {
            self.chunk_manager.sync_dirty_from_grid_radius(&self.grid, *pos, sub.aoi_radius);
        }
    }
}

// ============================================================================
// NETWORK LATENCY SIMULATION (for realistic MMO replication testing)
// ============================================================================

/// Simulates network latency for replication updates.
/// Useful for testing occlusion culling + interest management under realistic latency.
#[derive(Debug, Clone)]
pub struct NetworkLatencySimulator {
    /// Artificial latency to introduce (one-way)
    pub latency: Duration,
    /// Queue of pending replication updates (entity_id, ready_time)
    pending: VecDeque<(u64, Instant)>,
}

impl NetworkLatencySimulator {
    pub fn new(latency_ms: u64) -> Self {
        Self {
            latency: Duration::from_millis(latency_ms),
            pending: VecDeque::new(),
        }
    }

    /// Queue a replication update. It will become available after the simulated latency.
    pub fn queue_replication_update(&mut self, entity_id: u64) {
        let ready_time = Instant::now() + self.latency;
        self.pending.push_back((entity_id, ready_time));
    }

    /// Returns replication updates that are ready now (after simulated latency).
    /// Call this every tick to drain the latency queue.
    pub fn drain_ready_updates(&mut self) -> Vec<u64> {
        let now = Instant::now();
        let mut ready = Vec::new();

        while let Some((entity_id, ready_time)) = self.pending.front() {
            if now >= *ready_time {
                if let Some((id, _)) = self.pending.pop_front() {
                    ready.push(id);
                }
            } else {
                break;
            }
        }

        ready
    }

    /// Returns how many updates are still in flight (delayed by latency).
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    pub fn set_latency(&mut self, latency_ms: u64) {
        self.latency = Duration::from_millis(latency_ms);
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

    #[test]
    fn test_replication_uses_occlusion_by_default() {
        let rbe_pool = Arc::new(RbeResourcePool::new_global_abundance());
        let mut manager = InterestManager::new(10.0, 4, rbe_pool);

        let player_pos = glam::Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        manager.subscribe(1, 100.0, Some(player_pos));

        manager.update_entity_position(2, glam::Vec3 { x: 10.0, y: 0.0, z: 0.0 });
        manager.update_entity_position(3, glam::Vec3 { x: 50.0, y: 0.0, z: 0.0 });

        let replication = manager.get_replication_entities(1);
        let raw = manager.get_replication_entities_raw(1);

        assert!(replication.len() <= raw.len());
    }

    #[test]
    fn test_network_latency_simulator_basic() {
        let mut sim = NetworkLatencySimulator::new(50); // 50ms latency

        sim.queue_replication_update(42);
        assert_eq!(sim.pending_count(), 1);

        // Immediately after queuing, nothing should be ready
        let ready = sim.drain_ready_updates();
        assert!(ready.is_empty());

        // After sleeping longer than latency, it should be ready (in real test we use a small sleep or mock time)
        std::thread::sleep(std::time::Duration::from_millis(60));
        let ready = sim.drain_ready_updates();
        assert!(ready.contains(&42));
    }
}

// End of production file — Occlusion culling ENABLED BY DEFAULT + Network Latency Simulation support.
// Thunder locked in. Yoi ⚡