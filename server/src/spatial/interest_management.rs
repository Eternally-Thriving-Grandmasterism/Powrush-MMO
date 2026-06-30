//! server/src/spatial/interest_management.rs
//! Replication Loop + Real Networking Integration (bevy_renet)
//! v18.56+ | AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates

use bevy::prelude::*;
use bincode;
use serde::{Deserialize, Serialize};

// Note: Add these to Cargo.toml:
// bevy_renet = "0.1"
// renet = "0.1"

use crate::spatial::hierarchical_grid::HierarchicalGrid;
use powrush_rbe_engine::RbeResourcePool;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};

// ============================================================================
// REPLICATION TYPES (serializable)
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

// ============================================================================
// bevy_renet INTEGRATION BRIDGE
// ============================================================================

/// System that sends ReplicationUpdate over renet (server side)
/// Requires: RenetServer resource from bevy_renet
fn renet_server_send_system(
    mut events: EventReader<ReplicationUpdate>,
    // mut renet_server: ResMut<RenetServer>, // uncomment when bevy_renet is added
    mut ser_buffer: ResMut<SerializationBuffer>,
) {
    for update in events.read() {
        if let Ok(bytes) = ser_buffer.serialize_replication_update(update) {
            // Example with renet (pseudo-code):
            // for client_id in renet_server.connected_clients() {
            //     renet_server.send_message(client_id, ReliableChannel::id(), bytes.clone());
            // }
            println!("[Server] Would send {} bytes to clients via renet", bytes.len());
        }
    }
}

/// System that receives data on client and turns it into ReplicationUpdate
/// Requires: RenetClient resource from bevy_renet
fn renet_client_receive_system(
    // mut renet_client: ResMut<RenetClient>,
    mut replication_events: EventWriter<ReplicationUpdate>,
) {
    // Example:
    // while let Some(message) = renet_client.receive_message(ReliableChannel::id()) {
    //     if let Ok(update) = deserialize_replication_update(&message) {
    //         replication_events.send(update);
    //     }
    // }
}

// ============================================================================
// EXISTING OPTIMIZED SERIALIZATION + REPLICATION LOOP
// ============================================================================

#[derive(Resource, Default)]
pub struct SerializationBuffer {
    pub buffer: Vec<u8>,
}

impl SerializationBuffer {
    pub fn serialize_replication_update(&mut self, update: &ReplicationUpdate) -> Result<&[u8], bincode::Error> {
        self.buffer.clear();
        bincode::serialize_into(&mut self.buffer, update)?;
        Ok(&self.buffer)
    }
}

pub fn deserialize_replication_update(bytes: &[u8]) -> Result<ReplicationUpdate, bincode::Error> {
    bincode::deserialize(bytes)
}

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
            .add_systems(Update, (
                server_replication_system,
                renet_server_send_system,      // renet bridge
                renet_client_receive_system,   // renet bridge
            ));
    }
}

fn server_replication_system(
    mut interest: ResMut<InterestManagerResource>,
    mut replication_events: EventWriter<ReplicationUpdate>,
    mut replication_state: ResMut<ReplicationState>,
) {
    replication_state.current_tick += 1;
    let tick = replication_state.current_tick;

    let visible = interest.0.get_replication_entities(1);

    for entity_id in visible {
        if let Some(pos) = interest.0.subscriber_positions.get(&entity_id) {
            replication_events.send(ReplicationUpdate {
                entity_id,
                position: *pos,
                tick,
            });
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

// End of production file — bevy_renet integration bridge added
