// game/network/transport.rs
// Powrush-MMO — Network Transport Abstraction
// AG-SML v1.0 License

use crate::game::networking::{ClientMessage, ServerMessage};
use std::net::SocketAddr;

/// Unique identifier for a connected client
pub type ConnectionId = u64;

/// Delivery guarantee for messages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendReliability {
    /// Must arrive, in order (TCP-like)
    Reliable,
    /// May be lost, no ordering guarantee (UDP-like)
    Unreliable,
}

/// Events emitted by the transport layer
#[derive(Debug)]
pub enum NetworkEvent {
    ClientConnected {
        connection_id: ConnectionId,
        address: SocketAddr,
    },
    ClientDisconnected {
        connection_id: ConnectionId,
    },
    MessageReceived {
        connection_id: ConnectionId,
        message: ClientMessage,
    },
}

/// Trait that any networking backend must implement.
/// This allows us to swap transports later (e.g. SpacetimeDB, custom UDP, etc.)
pub trait NetworkTransport: Send + Sync {
    /// Start listening for connections (if applicable)
    fn start(&mut self) -> Result<(), String>;

    /// Send a message to a specific client
    fn send(
        &mut self,
        connection_id: ConnectionId,
        message: ServerMessage,
        reliability: SendReliability,
    ) -> Result<(), String>;

    /// Broadcast a message to all connected clients
    fn broadcast(&mut self, message: ServerMessage, reliability: SendReliability);

    /// Poll for new network events (connections, disconnections, messages)
    fn poll_events(&mut self) -> Vec<NetworkEvent>;

    /// Disconnect a specific client
    fn disconnect(&mut self, connection_id: ConnectionId);

    /// Check if a client is still connected
    fn is_connected(&self, connection_id: ConnectionId) -> bool;

    /// Get current number of connected clients
    fn connected_clients(&self) -> usize;
}