// shared/src/protocol.rs
// Powrush-MMO Protocol v15.6 — Full Combat + Health Sync + PATSAGi Validation ready
// Full mercy-gated message set with Ra-Thor / PATSAGi integration + combat state

use bevy::math::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vec3Ser {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySnapshot {
    pub id: u64,
    pub position: Vec3Ser,
    pub rotation: f32,
    pub scale: f32,
    pub state: u8,
    pub health: Option<HealthComponent>,  // v15.6: Live combat health sync for all entities (players, NPCs, bosses)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    HandshakeRequest { version: u32, player_name: String, auth_token: Option<String> },
    Move { delta: Vec3Ser },
    Jump,
    Interact { target_id: u64 },
    Ping { client_time_ms: u64 },

    // === Divine / PATSAGi / RBE Live Ra-Thor Integration ===
    DivineCouncilQuery { query: String, context: Option<String> },
    RbeAbundanceQuery { query: String },
    InvokeRitual { ritual_type: String, intensity: f32 },
    ProgressRedemption { target: Option<u64>, mercy_offering: f32 },
    TradeOffer { target_id: u64, offer: String },

    // === Combat v15.6 (mercy-gated + PATSAGi validated) ===
    AbilityCast { ability_id: u32, target_id: Option<u64>, position: Option<Vec3Ser> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    HandshakeResponse { accepted: bool, reason: Option<String>, player_id: u64, server_time: u64 },
    WorldUpdate { entities: Vec<EntitySnapshot>, timestamp: u64 },
    ValenceUpdate { player_id: u64, new_valence: f32, reason: String },
    MercyGateBlocked { reason: String, valence: f32 },
    Error { message: String },
    Pong { server_time_ms: u64, client_time_ms: u64 },

    // === Responses for live PATSAGi Councils + RBE ===
    DivineCouncilResponse { content: String, source: String },
    RbeGuidanceResponse { content: String },

    // === Combat v15.6 ===
    DamageApplied { target_id: u64, amount: f32, source_id: u64, is_critical: bool },
    CombatEvent { event_type: String, data: String },
}

pub const PROTOCOL_VERSION: u32 = 2;

/// Applies mercy gate validation for high-valence messages.
/// Returns true if the message is allowed at the current player valence level.
/// Combat actions (AbilityCast) require moderate valence + optional PATSAGi council validation for divine abilities.
pub fn apply_mercy_gate(message: &ClientMessage, valence: f32) -> bool {
    match message {
        ClientMessage::Ping { .. } => true,
        ClientMessage::DivineCouncilQuery { .. } => valence >= 0.75,
        ClientMessage::RbeAbundanceQuery { .. } => valence >= 0.65,
        ClientMessage::InvokeRitual { .. } => valence >= 0.85,
        ClientMessage::ProgressRedemption { .. } => valence >= 0.70,
        ClientMessage::TradeOffer { .. } => valence >= 0.60,
        ClientMessage::AbilityCast { .. } => valence >= 0.55, // Moderate for combat; PATSAGi hook adds extra sovereign validation layer
        _ => true,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthComponent {
    pub current: f32,
    pub max: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub id: u32,
    pub name: String,
    pub damage: f32,
    pub range: f32,
    pub cooldown_ms: u64,
    pub mercy_cost: f32, // Future: mercy-gated divine abilities
    // TODO v15.7: Add projectile_speed, effect_type (melee vs projectile distinction)
}
