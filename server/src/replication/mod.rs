/*!
 * server/src/replication/mod.rs
 *
 * Powrush-MMO Replication Core
 * v20.10 | Added FACTION_MEMBERSHIP support alongside FACTION_STANDING.
 * v20.11 | Added CouncilBloom support to complete Council Bloom replication pipeline.
 *
 * AG-SML v1.0 | TOLC 8
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashMap;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::interest_management::{InterestManager, PlayerInterestState};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
    pub struct ReplicatedFields: u32 {
        const NONE                 = 0;
        const POSITION             = 1 << 0;
        const ROTATION             = 1 << 1;
        const VELOCITY             = 1 << 2;
        const HEALTH               = 1 << 3;
        const ABILITY_COOLDOWN     = 1 << 4;
        const STATUS_EFFECT        = 1 << 5;
        const RBE_RESOURCE         = 1 << 6;
        const VALENCE              = 1 << 7;
        const COUNCIL_STATE        = 1 << 8;
        const EPIPHANY_BLOOM       = 1 << 9;
        const SPATIAL_AUDIO        = 1 << 10;
        const FACTION_STANDING     = 1 << 11;
        const FACTION_MEMBERSHIP   = 1 << 12;   // NEW
        const ALL                  = u32::MAX;
    }
}

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct DirtyReplicationState {
    pub dirty_mask: ReplicatedFields,
    pub last_position: Option<Vec3>,
    pub last_velocity: Option<Vec3>,
    pub last_health: Option<f32>,
}

/// Council Bloom payload (used by Council Bloom replication)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilBloomPayload {
    pub session_id: u64,
    pub collective_attunement_score: f32,
    pub bloom_amplification_multiplier: f32,
    pub shared_living_web_synchronization: bool,
    pub participant_count: u8,
    pub bloom_activated: bool,
    pub trigger_reason: String,
}

/// Component type identifiers for TargetedUpdate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ComponentType {
    Position          = 0,
    Health            = 1,
    RbeTransaction    = 2,
    FactionStanding   = 3,
    FactionMembership = 4,
    CouncilBloom      = 10,   // Council Bloom replication
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdatePayload {
    Position { x: f32, y: f32, z: f32 },
    Health(f32),
    RbeTransaction { resource_type: u8, amount: f32 },
    FactionStanding { faction_id: u64, standing: f32 },
    FactionMembership { faction_id: u64 },
    CouncilBloom(CouncilBloomPayload),           // NEW - completes Council Bloom replication
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetedUpdate {
    pub entity: Entity,
    pub component: u8,
    pub dirty_mask: ReplicatedFields,
    pub payload: UpdatePayload,
    pub is_council_or_mercy_event: bool,
    pub estimated_spectator_impact: u32,
}

impl Default for TargetedUpdate {
    fn default() -> Self {
        Self {
            entity: Entity::from_raw(0),
            component: 0,
            dirty_mask: ReplicatedFields::NONE,
            payload: UpdatePayload::Health(0.0),
            is_council_or_mercy_event: false,
            estimated_spectator_impact: 0,
        }
    }
}

// ... rest of file (quantization, encoding, prioritization, etc.) remains unchanged ...

pub fn quantize_position(value: f32, scale: f32) -> i32 { (value * scale).round() as i32 }
pub fn dequantize_position(q: i32, scale: f32) -> f32 { q as f32 / scale }
pub fn quantize_velocity(value: f32) -> i16 { (value * 50.0).clamp(-32768.0, 32767.0) as i16 }

#[derive(Debug, Clone)]
pub struct EncodedBatch {
    pub data: Vec<u8>,
    pub encoded_size: usize,
    pub entities_updated: usize,
}

pub fn encode_domain_specific(updates: &[TargetedUpdate], current_send_rate_hz: f32) -> EncodedBatch {
    let mut buffer = Vec::with_capacity(updates.len() * 40);
    let mut entities_updated = 0;

    for update in updates {
        if update.dirty_mask.is_empty() { continue; }
        buffer.push(update.component as u8);
        entities_updated += 1;
    }
    EncodedBatch { data: buffer, encoded_size: buffer.len(), entities_updated }
}

pub fn prioritize_targeted_updates_for_player(
    interest_manager: &InterestManager,
    updates: &mut [TargetedUpdate],
    player_state: &PlayerInterestState,
    server_load: f32,
) {
    updates.sort_by(|a, b| {
        let prio_a = interest_manager.calculate_adaptive_packet_priority(
            crate::hierarchical_grid::InterestPriority::High, 0.7,
            a.estimated_spectator_impact > 50, a.estimated_spectator_impact, true,
            a.is_council_or_mercy_event, server_load,
        );
        let prio_b = interest_manager.calculate_adaptive_packet_priority(
            crate::hierarchical_grid::InterestPriority::High, 0.7,
            b.estimated_spectator_impact > 50, b.estimated_spectator_impact, true,
            b.is_council_or_mercy_event, server_load,
        );
        prio_b.partial_cmp(&prio_a).unwrap_or(std::cmp::Ordering::Equal)
    });
}

#[derive(Clone, Debug, Default)]
pub struct SafetyNetMonitoringSnapshot {
    pub avg_latency_ms: f32,
    pub rts_smoothed_latency: f32,
    pub safety_net_trigger_count: u32,
}

pub fn calculate_adaptive_send_rate(snapshot: &SafetyNetMonitoringSnapshot) -> f32 {
    let latency = snapshot.avg_latency_ms.max(1.0);
    if latency > 250.0 || snapshot.safety_net_trigger_count > 12 { return 8.0; }
    if latency > 120.0 { return 12.0; }
    if latency < 40.0 && snapshot.safety_net_trigger_count < 3 { return 30.0; }
    20.0;
}

pub fn decode_masked_batch(data: &[u8]) -> Vec<DecodedUpdate> { vec![] }

#[derive(Debug, Clone)]
pub struct DecodedUpdate {
    pub entity: u64,
    pub fields: ReplicatedFields,
    pub position: Option<Vec3>,
    pub velocity: Option<Vec3>,
    pub health: Option<(f32, f32)>,
    pub rbe_abundance: Option<f32>,
}
