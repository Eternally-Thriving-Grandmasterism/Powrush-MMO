//! Powrush-MMO Shared Protocol — Client ↔ Server Messages
//! All messages bincode-serializable, mercy-valence gated where applicable
//! MIT + mercy eternal — Eternally-Thriving-Grandmasterism

use bevy::math::Vec3;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Core Shared Types ─────────────────────────────────────────────────
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Vec3Ser {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Vec3> for Vec3Ser {
    fn from(v: Vec3) -> Self {
        Vec3Ser { x: v.x, y: v.y, z: v.z }
    }
}

impl From<Vec3Ser> for Vec3 {
    fn from(v: Vec3Ser) -> Self {
        Vec3::new(v.x, v.y, v.z)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntitySnapshot {
    pub id: u64,
    pub position: Vec3Ser,
    pub rotation: f32, // simplified yaw
    pub faction: String,
    pub valence: f32,
    pub state_flags: u32, // bitfield: 1=dead, 2=invincible, etc.
}

// ─── Client → Server Messages ──────────────────────────────────────────
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientMessage {
    // Movement & Interaction
    Move { direction: Vec3Ser, sprint: bool },
    Interact { target_id: u64 },
    UseAbility { ability_id: u32, target_id: Option<u64> },

    // Economy & Crafting
    Harvest { node_id: u64 },
    Craft { recipe_id: u32, count: u32 },
    TradeOffer { target_player_id: u64, offer: HashMap<String, u32>, request: HashMap<String, u32> },

    // Social & Diplomacy
    Chat { channel: String, message: String },
    FormAlliance { target_faction: String, pact_type: String },
    BreakAlliance { pact_id: String },

    // Divine / Mercy Events
    InvokeRitual { ritual_type: String, context: HashMap<String, String> },
    ProgressRedemption { chain_id: String, action: String },

    // Debug / Admin (gated)
    DebugCommand { command: String },
}

// ─── Server → Client Messages ──────────────────────────────────────────
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerMessage {
    // World Sync
    WorldUpdate { entities: Vec<EntitySnapshot>, timestamp: u64 },

    // Player State
    PlayerStateUpdate { player_id: u64, position: Vec3Ser, valence: f32, inventory: HashMap<String, u32> },
    ValenceUpdate { player_id: u64, new_valence: f32, reason: String },

    // Interaction Results
    InteractResult { target_id: u64, success: bool, message: String },
    HarvestResult { node_id: u64, yield_items: HashMap<String, u32> },

    // Divine / Mercy Events
    RitualStarted { ritual_id: String, type_: String },
    RitualProgress { ritual_id: String, stage: u32, valence: f32 },
    RitualComplete { ritual_id: String, type_: String, reward: String },
    VisionReveal { vision_id: String, seed: String, narrative: String, valence: f32 },
    AmbrosianTierUpdate { player_id: u64, tier: u8, avg_valence: f32 },

    // Error & Feedback
    Error { code: u32, message: String },
    MercyGateBlocked { reason: String, valence: f32 },

    // Global Events
    GlobalRipple { intensity: f32, source: String },
    EternalBloomActivated { remaining_days: u32 },
}

// ─── Protocol Version & Handshake ──────────────────────────────────────
pub const PROTOCOL_VERSION: u32 = 1;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HandshakeRequest {
    pub protocol_version: u32,
    pub player_name: String,
    pub auth_token: Option<String>, // self-custody wallet signature
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HandshakeResponse {
    pub accepted: bool,
    pub reason: Option<String>,
    pub player_id: u64,
    pub server_time: u64,
}

// ─── Mercy Gate Helper (used server-side before processing) ────────────
pub fn apply_mercy_gate(message: &ClientMessage, valence: f32) -> bool {
    match message {
        ClientMessage::InvokeRitual { .. } => valence >= 0.85,
        ClientMessage::ProgressRedemption { .. } => valence >= 0.70,
        ClientMessage::TradeOffer { .. } => valence >= 0.60, // prevent exploitation
        _ => true, // most actions always allowed
    }
}
