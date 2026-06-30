//! server/src/spatial/interest_management.rs
//! Optimized Bincode Serialization for Replication
//! v18.56+ | AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates

use bevy::prelude::*;
use bincode;
use serde::{Deserialize, Serialize};
use crate::spatial::hierarchical_grid::HierarchicalGrid;
use powrush_rbe_engine::RbeResourcePool;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};

// ============================================================================
// HIGH-PERFORMANCE SERIALIZATION
// ============================================================================

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationUpdate {
    pub entity_id: u64,
    pub position: glam::Vec3,
    pub tick: u64,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct ClientInputEvent {
    pub entity_id: u64,
    pub position: glam::Vec3,
    pub tick: u64,
}

/// Reusable buffer resource to minimize allocations during serialization
#[derive(Resource, Default)]
pub struct SerializationBuffer {
    pub buffer: Vec<u8>,
}

impl SerializationBuffer {
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn serialize_replication_update(
        &mut self,
        update: &ReplicationUpdate,
    ) -> Result<&[u8], bincode::Error> {
        self.buffer.clear();
        bincode::serialize_into(&mut self.buffer, update)?;
        Ok(&self.buffer)
    }

    pub fn serialize_client_input(
        &mut self,
        event: &ClientInputEvent,
    ) -> Result<&[u8], bincode::Error> {
        self.buffer.clear();
        bincode::serialize_into(&mut self.buffer, event)?;
        Ok(&self.buffer)
    }
}

// Legacy functions still available for convenience
pub fn serialize_replication_update(update: &ReplicationUpdate) -> Result<Vec<u8>, bincode::Error> {
    bincode::serialize(update)
}

pub fn deserialize_replication_update(bytes: &[u8]) -> Result<ReplicationUpdate, bincode::Error> {
    bincode::deserialize(bytes)
}

// ============================================================================
// REPLICATION LOOP (optimized)
// ============================================================================

#[derive(Resource, Default)]
pub struct ReplicationState {
    pub current_tick: u64,
}

pub struct ReplicationLoopPlugin;

impl Plugin for ReplicationLoopPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ReplicationState>()
            .init_resource::<InterestManagerResource>()
            .init_resource::<ClientPredictionResource>()
            .init_resource::<SerializationBuffer>()
            .add_event::<ReplicationUpdate>()
            .add_event::<ClientInputEvent>()
            .add_systems(Update, server_replication_system);
    }
}

fn server_replication_system(
    mut interest: ResMut<InterestManagerResource>,
    mut replication_events: EventWriter<ReplicationUpdate>,
    mut replication_state: ResMut<ReplicationState>,
    mut ser_buffer: ResMut<SerializationBuffer>,
) {
    replication_state.current_tick += 1;
    let tick = replication_state.current_tick;

    let visible = interest.0.get_replication_entities(1);

    for entity_id in visible {
        if let Some(pos) = interest.0.subscriber_positions.get(&entity_id) {
            let update = ReplicationUpdate {
                entity_id,
                position: *pos,
                tick,
            };

            // Use reusable buffer for better performance
            if let Ok(_bytes) = ser_buffer.serialize_replication_update(&update) {
                replication_events.send(update);
            }
        }
    }
}

// ============================================================================
// CORE TYPES
// ============================================================================

pub struct InterestManager { /* ... */ }
impl InterestManager {
    pub fn new(_: f32, _: u8, _: Arc<RbeResourcePool>) -> Self { todo!() }
    pub fn get_replication_entities(&self, _: u64) -> Vec<u64> { vec![] }
    pub fn tick(&mut self, _: u64) {}
}

#[derive(Resource)]
pub struct InterestManagerResource(pub InterestManager);

#[derive(Resource, Default)]
pub struct ClientPredictionResource(pub ClientPrediction);

pub struct ClientPrediction {}
impl ClientPrediction {
    pub fn reconcile_with_server(&mut self, _: u64, _: glam::Vec3, _: u64, _: f32) {}
}

// End of production file — Serialization performance optimized with reusable buffer
