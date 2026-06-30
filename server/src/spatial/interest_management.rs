//! server/src/spatial/interest_management.rs
//! bevy_renet with Reliable + Unreliable Channels
//! v18.56+ | AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates

use bevy::prelude::*;
use bincode;
use serde::{Deserialize, Serialize};

use renet::{RenetClient, RenetServer, ServerEvent};
use std::net::SocketAddr;
use std::time::SystemTime;

// ============================================================================
// CHANNEL DEFINITIONS (wise separation of concerns)
// ============================================================================

/// Reliable Ordered channel — for important state (spawns, ownership, critical replication)
pub const RELIABLE_REPLICATION_CHANNEL: u8 = 0;

/// Unreliable channel — for high-frequency updates (position, movement, transform)
pub const UNRELIABLE_POSITION_CHANNEL: u8 = 1;

// ============================================================================
// REPLICATION DATA
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
// bevy_renet INTEGRATION (with channel separation)
// ============================================================================

#[derive(Resource)]
pub struct RenetServerResource(pub RenetServer);

#[derive(Resource)]
pub struct RenetClientResource(pub RenetClient);

/// Send high-frequency position updates over **unreliable** channel
fn send_position_updates_unreliable(
    mut events: EventReader<ReplicationUpdate>,
    mut server: ResMut<RenetServerResource>,
    mut ser_buffer: ResMut<SerializationBuffer>,
) {
    for update in events.read() {
        if let Ok(bytes) = ser_buffer.serialize_replication_update(update) {
            for client_id in server.0.connected_clients() {
                // Send on unreliable channel for responsiveness
                server.0.send_message(client_id, UNRELIABLE_POSITION_CHANNEL, bytes.clone());
            }
        }
    }
}

/// Send important replication data over **reliable** channel (example)
fn send_important_replication_reliable(
    mut events: EventReader<ReplicationUpdate>,
    mut server: ResMut<RenetServerResource>,
    mut ser_buffer: ResMut<SerializationBuffer>,
) {
    for update in events.read() {
        if let Ok(bytes) = ser_buffer.serialize_replication_update(update) {
            for client_id in server.0.connected_clients() {
                // Send on reliable ordered channel
                server.0.send_message(client_id, RELIABLE_REPLICATION_CHANNEL, bytes.clone());
            }
        }
    }
}

/// Receive on unreliable channel (high-frequency position)
fn receive_unreliable_position(
    mut client: ResMut<RenetClientResource>,
    mut replication_events: EventWriter<ReplicationUpdate>,
) {
    while let Some(message) = client.0.receive_message(UNRELIABLE_POSITION_CHANNEL) {
        if let Ok(update) = deserialize_replication_update(&message) {
            replication_events.send(update);
        }
    }
}

/// Receive on reliable channel
fn receive_reliable_replication(
    mut client: ResMut<RenetClientResource>,
    mut replication_events: EventWriter<ReplicationUpdate>,
) {
    while let Some(message) = client.0.receive_message(RELIABLE_REPLICATION_CHANNEL) {
        if let Ok(update) = deserialize_replication_update(&message) {
            replication_events.send(update);
        }
    }
}

// ============================================================================
// SERIALIZATION BUFFER
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

// ============================================================================
// PLUGIN
// ============================================================================

pub struct ReplicationLoopPlugin;

impl Plugin for ReplicationLoopPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SerializationBuffer>()
            .add_event::<ReplicationUpdate>()
            .add_systems(Update, (
                send_position_updates_unreliable,
                receive_unreliable_position,
                // send_important_replication_reliable, // enable when needed
                // receive_reliable_replication,
            ));
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

// End of production file — Unreliable channels for high-frequency updates added wisely
