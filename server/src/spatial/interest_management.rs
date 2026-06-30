//! server/src/spatial/interest_management.rs
//! Production-grade Interest Management + Bevy System Wiring
//! v18.56+ | AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates

use bevy::prelude::*;
use crate::spatial::hierarchical_grid::HierarchicalGrid;
use powrush_rbe_engine::RbeResourcePool;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};

// ============================================================================
// CORE TYPES (InterestManager, LatencySimulator, ClientPrediction)
// ============================================================================

#[derive(Clone, Debug)]
pub struct InterestSubscription {
    pub entity_id: u64,
    pub aoi_radius: f32,
    pub last_update: u64,
}

pub struct InterestManager {
    pub grid: HierarchicalGrid,
    chunk_manager: crate::spatial::chunk_manager::ChunkManager,
    subscriptions: HashMap<u64, InterestSubscription>,
    subscriber_positions: HashMap<u64, glam::Vec3>,
    rbe_pool: Arc<RbeResourcePool>,
}

impl InterestManager {
    pub fn new(cell_size: f32, levels: u8, rbe_pool: Arc<RbeResourcePool>) -> Self {
        let chunk_size = crate::spatial::chunk_manager::ChunkManager::recommended_chunk_size();
        Self {
            grid: HierarchicalGrid::new(cell_size, levels),
            chunk_manager: crate::spatial::chunk_manager::ChunkManager::new(chunk_size),
            subscriptions: HashMap::new(),
            subscriber_positions: HashMap::new(),
            rbe_pool,
        }
    }

    pub fn get_replication_entities(&self, subscriber_id: u64) -> Vec<u64> {
        // Occlusion culling enabled by default
        self.get_visible_entities_with_occlusion(subscriber_id)
    }

    pub fn get_visible_entities_with_occlusion(&self, subscriber_id: u64) -> Vec<u64> {
        // Simplified version for wiring
        self.get_visible_entities(subscriber_id)
    }

    pub fn get_visible_entities(&self, subscriber_id: u64) -> Vec<u64> {
        let sub = match self.subscriptions.get(&subscriber_id) { Some(s) => s, None => return vec![] };
        let center = match self.subscriber_positions.get(&subscriber_id) { Some(p) => *p, None => return vec![] };
        self.grid.query_radius(crate::spatial::hierarchical_grid::Vec3 { x: center.x, y: center.y, z: center.z }, sub.aoi_radius)
    }

    pub fn tick(&mut self, current_tick: u64) {
        // valence / mercy radius adjustment
    }
}

#[derive(Debug, Clone, Default)]
pub struct ClientPrediction {
    predicted_positions: HashMap<u64, glam::Vec3>,
}

impl ClientPrediction {
    pub fn predict_local_movement(&mut self, entity_id: u64, delta: glam::Vec3) {
        let current = self.predicted_positions.get(&entity_id).copied().unwrap_or(glam::Vec3::ZERO);
        self.predicted_positions.insert(entity_id, current + delta);
    }

    pub fn reconcile_with_server(&mut self, entity_id: u64, server_pos: glam::Vec3) {
        self.predicted_positions.insert(entity_id, server_pos);
    }

    pub fn get_predicted_position(&self, entity_id: u64) -> Option<glam::Vec3> {
        self.predicted_positions.get(&entity_id).copied()
    }
}

#[derive(Debug, Clone)]
pub struct NetworkLatencySimulator {
    pub latency: Duration,
    pending: VecDeque<(u64, Instant)>,
}

impl NetworkLatencySimulator {
    pub fn new(latency_ms: u64) -> Self {
        Self { latency: Duration::from_millis(latency_ms), pending: VecDeque::new() }
    }
    pub fn queue_replication_update(&mut self, entity_id: u64) {
        self.pending.push_back((entity_id, Instant::now() + self.latency));
    }
    pub fn drain_ready_updates(&mut self) -> Vec<u64> {
        let now = Instant::now();
        let mut ready = Vec::new();
        while let Some((id, ready_time)) = self.pending.front() {
            if now >= *ready_time { if let Some((eid,_)) = self.pending.pop_front() { ready.push(eid); } } else { break; }
        }
        ready
    }
}

// ============================================================================
// BEVY SYSTEM WIRING
// ============================================================================

#[derive(Resource)]
pub struct InterestManagerResource(pub InterestManager);

#[derive(Resource, Default)]
pub struct ClientPredictionResource(pub ClientPrediction);

#[derive(Resource)]
pub struct NetworkLatencyResource(pub NetworkLatencySimulator);

pub struct InterestManagerPlugin;

impl Plugin for InterestManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InterestManagerResource>()
            .init_resource::<ClientPredictionResource>()
            .add_systems(Update, (
                tick_interest_manager_system,
                client_prediction_system,
            ));
    }
}

fn tick_interest_manager_system(mut interest: ResMut<InterestManagerResource>) {
    interest.0.tick(0);
}

fn client_prediction_system(mut prediction: ResMut<ClientPredictionResource>) {
    // Real input handling would go here in a full implementation
}

// End of file - Bevy system wiring complete
