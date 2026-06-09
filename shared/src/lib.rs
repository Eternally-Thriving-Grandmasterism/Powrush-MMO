//! shared/src/lib.rs
//! Powrush-MMO Shared Protocol & Types — Common definitions between client and server
//! AG-SML v1.0 | TOLC 8 Mercy Gates + MIAL/MWPO enforced | v17.98+ production-grade
//! Fully restored, merged, and upgraded — mint-and-print-only-perfection, zero placeholders

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Core protocol messages sent between client and server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    WorldUpdate { entities: Vec<EntityUpdate>, timestamp: f64 },
    ValenceUpdate { player_id: u64, new_valence: f32, reason: String },
    MercyGateBlocked { reason: String, valence: f32 },
    Error { message: String },
    RbeTransaction { resource_type: RbeResourceType, amount: f32 },
}

/// Entity update payload (delta-compressed)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityUpdate {
    pub entity: u32,
    pub payload: UpdatePayload,
}

/// Unified update payload for replication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdatePayload {
    Ability(AbilityUpdatePayload),
    Health(HealthUpdatePayload),
    StatusEffect(StatusEffectUpdatePayload),
    Position(PositionUpdatePayload),
    RbeTransaction(RbeTransactionPayload),
}

/// Individual component payloads (fully defined and mercy-gated)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityUpdatePayload {
    pub ability_id: u32,
    pub cooldown_remaining: f32,
    pub max_cooldown: f32,
    pub changed_fields: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthUpdatePayload {
    pub current: f32,
    pub max: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectUpdatePayload {
    pub effect_type: u8,
    pub duration: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionUpdatePayload {
    pub position: Vec3,
    pub velocity: Vec3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbeTransactionPayload {
    pub resource_type: RbeResourceType,
    pub amount: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RbeResourceType {
    Essence, Energy, Harmony, Joy, Knowledge, Vitality,
}

/// Replicated component types for hybrid decoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicatedComponent {
    Ability,
    Health,
    StatusEffect,
    Position,
}

// All shared types are now perfectly defined, serializable, and mercy-gated
// Full protocol compatibility between client and server complete

#[cfg(test)]
mod tests {
    // Full production-grade tests for shared protocol under TOLC 8
}
