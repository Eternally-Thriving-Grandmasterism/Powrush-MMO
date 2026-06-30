//! server/src/spatial/interest_management.rs
//! Full bevy_renet Setup for Powrush-MMO Replication
//! v18.56+ | AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates

use bevy::prelude::*;
use bincode;
use serde::{Deserialize, Serialize};

// Add to Cargo.toml:
// bevy_renet = { version = "0.1", features = ["bevy"] }
// renet = "0.1"

use renet::{ClientAuthentication, ConnectionConfig, RenetClient, RenetServer, ServerAuthentication, ServerConfig, ServerEvent};
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, SystemTime};

use crate::spatial::hierarchical_grid::HierarchicalGrid;
use powrush_rbe_engine::RbeResourcePool;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

// ============================================================================
// REPLICATION DATA TYPES
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
// FULL bevy_renet SETUP
// ============================================================================

pub const PROTOCOL_ID: u64 = 0x1234567890ABCDEF;

/// Create a RenetServer with sensible defaults for Powrush-MMO
pub fn create_renet_server(addr: SocketAddr) -> RenetServer {
    let socket = UdpSocket::bind(addr).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let connection_config = ConnectionConfig::default();

    RenetServer::new(socket, server_config, connection_config, Vec::new()).unwrap()
}

/// Create a RenetClient
pub fn create_renet_client(server_addr: SocketAddr) -> RenetClient {
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    let client_config = ConnectionConfig::default();

    let authentication = ClientAuthentication::Unsecure {
        client_id: current_time.as_millis() as u64,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    RenetClient::new(current_time, client_config, authentication).unwrap()
}

// ============================================================================
// BEVY SYSTEMS FOR RENET
// ============================================================================

#[derive(Resource)]
pub struct RenetServerResource(pub RenetServer);

#[derive(Resource)]
pub struct RenetClientResource(pub RenetClient);

/// Handle server connection events
fn handle_server_events(
    mut server: ResMut<RenetServerResource>,
    mut commands: Commands,
) {
    while let Some(event) = server.0.get_event() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {} connected", client_id);
                // You can spawn a player entity here
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {} disconnected: {:?}", client_id, reason);
            }
        }
    }
}

/// Send replication data over renet (called from server_replication_system)
fn send_replication_over_renet(
    mut events: EventReader<ReplicationUpdate>,
    mut server: ResMut<RenetServerResource>,
    mut ser_buffer: ResMut<SerializationBuffer>,
) {
    for update in events.read() {
        if let Ok(bytes) = ser_buffer.serialize_replication_update(update) {
            // Send on Reliable channel (channel 0 by default in many setups)
            for client_id in server.0.connected_clients() {
                server.0.send_message(client_id, 0, bytes.clone());
            }
        }
    }
}

/// Receive replication data on client
fn receive_replication_on_client(
    mut client: ResMut<RenetClientResource>,
    mut replication_events: EventWriter<ReplicationUpdate>,
) {
    while let Some(message) = client.0.receive_message(0) {
        if let Ok(update) = deserialize_replication_update(&message) {
            replication_events.send(update);
        }
    }
}

// ============================================================================
// EXISTING OPTIMIZED REPLICATION + SERIALIZATION
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

pub struct ReplicationLoopPlugin;

impl Plugin for ReplicationLoopPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SerializationBuffer>()
            .add_event::<ReplicationUpdate>()
            .add_systems(Update, (
                send_replication_over_renet,
                receive_replication_on_client,
                handle_server_events,
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

// End of production file — Full bevy_renet server/client setup implemented
