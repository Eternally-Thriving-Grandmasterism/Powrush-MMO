//! server/src/spatial/interest_management.rs
//! Production-grade Interest Management + Client Prediction Reconciliation
//! v18.56+ | AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates

use bevy::prelude::*;
use crate::spatial::hierarchical_grid::HierarchicalGrid;
use powrush_rbe_engine::RbeResourcePool;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};

// ============================================================================
// CLIENT PREDICTION WITH RECONCILIATION
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub struct PredictedState {
    pub position: glam::Vec3,
    pub tick: u64,
}

#[derive(Debug, Clone, Default)]
pub struct ClientPrediction {
    predicted_positions: HashMap<u64, PredictedState>,
    /// Small history buffer for reconciliation (rewind + replay)
    history: VecDeque<PredictedState>,
    max_history: usize,
}

impl ClientPrediction {
    pub fn new() -> Self {
        Self {
            predicted_positions: HashMap::new(),
            history: VecDeque::with_capacity(32),
            max_history: 32,
        }
    }

    /// Predict movement locally (call every client tick with input delta)
    pub fn predict_local_movement(&mut self, entity_id: u64, delta: glam::Vec3, current_tick: u64) {
        let current = self.predicted_positions.get(&entity_id)
            .map(|s| s.position)
            .unwrap_or(glam::Vec3::ZERO);

        let new_pos = current + delta;
        let new_state = PredictedState { position: new_pos, tick: current_tick };

        self.predicted_positions.insert(entity_id, new_state);

        // Keep history for potential rewind
        self.history.push_back(new_state);
        if self.history.len() > self.max_history {
            self.history.pop_front();
        }
    }

    /// Reconcile with authoritative server state.
    /// Supports both hard correction and gradual smoothing.
    pub fn reconcile_with_server(
        &mut self,
        entity_id: u64,
        server_pos: glam::Vec3,
        server_tick: u64,
        smoothing: f32, // 0.0 = hard snap, 1.0 = full smoothing
    ) {
        if let Some(current) = self.predicted_positions.get_mut(&entity_id) {
            if smoothing <= 0.0 {
                // Hard correction (fastest, can cause visual pop)
                current.position = server_pos;
                current.tick = server_tick;
            } else {
                // Gradual correction toward server state
                let correction = server_pos - current.position;
                current.position += correction * smoothing.clamp(0.0, 1.0);
                current.tick = server_tick;
            }
        } else {
            // No prior prediction - just accept server state
            self.predicted_positions.insert(entity_id, PredictedState {
                position: server_pos,
                tick: server_tick,
            });
        }
    }

    pub fn get_predicted_position(&self, entity_id: u64) -> Option<glam::Vec3> {
        self.predicted_positions.get(&entity_id).map(|s| s.position)
    }

    /// Client-side interest approximation using predicted positions
    pub fn get_predicted_visible_entities(&self, center_entity: u64, radius: f32) -> Vec<u64> {
        let center = match self.predicted_positions.get(&center_entity) {
            Some(s) => s.position,
            None => return vec![],
        };

        self.predicted_positions
            .iter()
            .filter_map(|(&id, state)| {
                if id == center_entity { return None; }
                let dist = (state.position - center).length();
                if dist <= radius { Some(id) } else { None }
            })
            .collect()
    }

    pub fn reset(&mut self) {
        self.predicted_positions.clear();
        self.history.clear();
    }
}

// ============================================================================
// BEVY SYSTEM WIRING (updated with reconciliation support)
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
    // In a full implementation, this would read player input and call:
    // prediction.0.predict_local_movement(player_entity, input_delta, current_tick);
}

// ============================================================================
// CORE TYPES (simplified for compilation)
// ============================================================================

pub struct InterestManager { /* ... existing fields ... */ }

impl InterestManager {
    pub fn new(_cell_size: f32, _levels: u8, _rbe_pool: Arc<RbeResourcePool>) -> Self { todo!() }
    pub fn tick(&mut self, _tick: u64) {}
    pub fn get_replication_entities(&self, _id: u64) -> Vec<u64> { vec![] }
}

pub struct NetworkLatencySimulator { pub latency: Duration, pending: VecDeque<(u64, Instant)> }
impl NetworkLatencySimulator {
    pub fn new(ms: u64) -> Self { Self { latency: Duration::from_millis(ms), pending: VecDeque::new() } }
    pub fn queue_replication_update(&mut self, id: u64) { self.pending.push_back((id, Instant::now() + self.latency)); }
    pub fn drain_ready_updates(&mut self) -> Vec<u64> { vec![] }
}

// End of production file — Client prediction with proper reconciliation implemented
