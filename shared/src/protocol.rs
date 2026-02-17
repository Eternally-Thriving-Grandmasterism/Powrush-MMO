//! Powrush-MMO Shared Protocol — Client ↔ Server Messages + Heartbeat
//! All messages bincode-serializable, mercy-valence gated where applicable
//! MIT + mercy eternal — Eternally-Thriving-Grandmasterism

use bevy::math::Vec3;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# pub struct Vec3Ser {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Vec3> for Vec3Ser {
    fn from(v: Vec3) -> Self {
        Self { x: v.x, y: v.y, z: v.z }
    }
}

impl From<Vec3Ser> for Vec3 {
    fn from(v: Vec3Ser) -> Self {
        Vec3::new(v.x, v.y, v.z)
    }
}

# pub struct EntitySnapshot {
    pub id: u64,
    pub position: Vec3Ser,
    pub rotation: f32,
    pub scale: f32,
    pub state: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientMessage {
    // Handshake & auth
    HandshakeRequest { version: u32, player_name: String, auth_token: Option<String> },
    // Movement & actions
    Move { delta: Vec3Ser },
    Jump,
    Interact { target_id: u64 },
    // Heartbeat (client → server)
    Ping { client_time_ms: u64 },
}

# pub enum ServerMessage {
    // Handshake & auth
    HandshakeResponse { accepted: bool, reason: Option<String>, player_id: u64, server_time: u64 },
    // World state
    WorldUpdate { entities: Vec<EntitySnapshot>, timestamp: u64 },
    // Player state
    ValenceUpdate { player_id: u64, new_valence: f32, reason: String },
    MercyGateBlocked { reason: String, valence: f32 },
    Error { message: String },
    // Heartbeat response (server → client)
    Pong { server_time_ms: u64, client_time_ms: u64 },
}

pub const PROTOCOL_VERSION: u32 = 2;

pub fn apply_mercy_gate(message: &ClientMessage, valence: f32) -> bool {
    match message {
        ClientMessage::Ping { .. } => true,
        ClientMessage::InvokeRitual { .. } => valence >= 0.85,
        ClientMessage::ProgressRedemption { .. } => valence >= 0.70,
        ClientMessage::TradeOffer { .. } => valence >= 0.60,
        _ => true,
    }
}
