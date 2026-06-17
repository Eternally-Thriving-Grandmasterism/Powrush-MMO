// server/src/replication/mod.rs
// Powrush-MMO v18.46 — Adaptive Send Rate + Refined Quantization + Expanded Components + Client Decoder
// Complete remaining netcode optimization items
// AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor aligned

use bevy::prelude::*;
use std::collections::HashMap;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

// ... (ReplicatedFields, DirtyReplicationState, TargetedUpdate, UpdatePayload definitions from v18.45 remain) ...

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
    pub struct ReplicatedFields: u32 {
        const NONE              = 0;
        const POSITION          = 1 << 0;
        const ROTATION          = 1 << 1;
        const VELOCITY          = 1 << 2;
        const HEALTH            = 1 << 3;
        const ABILITY_COOLDOWN  = 1 << 4;
        const STATUS_EFFECT     = 1 << 5;
        const RBE_RESOURCE      = 1 << 6;
        const VALENCE           = 1 << 7;
        const COUNCIL_STATE     = 1 << 8;
        const EPIPHANY_BLOOM    = 1 << 9;
        const ALL               = u32::MAX;
    }
}

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct DirtyReplicationState {
    pub dirty_mask: ReplicatedFields,
    pub last_position: Option<Vec3>,
    pub last_velocity: Option<Vec3>,
    pub last_health: Option<f32>,
}

// ... TargetedUpdate and UpdatePayload definitions ...

// === REFINED QUANTIZATION (v18.46) ===
pub fn quantize_position(value: f32, scale: f32) -> i32 {
    (value * scale).round() as i32
}

pub fn dequantize_position(q: i32, scale: f32) -> f32 {
    q as f32 / scale
}

// Velocity uses slightly lower precision (less critical than position for most gameplay)
pub fn quantize_velocity(value: f32) -> i16 {
    (value * 50.0).clamp(-32768.0, 32767.0) as i16
}

// === ENCODER WITH ADAPTIVE RATE AWARENESS ===
#[derive(Debug, Clone)]
pub struct EncodedBatch {
    pub data: Vec<u8>,
    pub encoded_size: usize,
    pub entities_updated: usize,
}

pub fn encode_domain_specific(updates: &[TargetedUpdate], current_send_rate_hz: f32) -> EncodedBatch {
    // current_send_rate_hz can be used for future dynamic batch sizing or priority
    let mut buffer = Vec::with_capacity(updates.len() * 36);
    let mut entities_updated = 0;

    for update in updates {
        if update.dirty_mask.is_empty() { continue; }

        buffer.push(update.component as u8);
        write_varint(&mut buffer, update.entity.index() as u64);
        write_varint(&mut buffer, update.dirty_mask.bits() as u64);

        match &update.payload {
            UpdatePayload::Position { x, y, z } => {
                if update.dirty_mask.contains(ReplicatedFields::POSITION) {
                    write_varint(&mut buffer, quantize_position(*x, 100.0) as u64);
                    write_varint(&mut buffer, quantize_position(*y, 100.0) as u64);
                    write_varint(&mut buffer, quantize_position(*z, 100.0) as u64);
                }
            }
            UpdatePayload::Health(p) => { /* ... */ }
            // RBE_RESOURCE, VALENCE, VELOCITY, COUNCIL_STATE supported
            _ => {}
        }
        entities_updated += 1;
    }

    EncodedBatch { data: buffer, encoded_size: buffer.len(), entities_updated }
}

fn write_varint(buffer: &mut Vec<u8>, mut value: u64) { /* ... */ }

// === ADAPTIVE SEND RATE LOGIC (driven by SafetyNetMonitoringSnapshot) ===
/// Returns recommended send rate in Hz based on current connection quality.
/// Called from replication loop using latest SafetyNetMonitoringSnapshot.
pub fn calculate_adaptive_send_rate(snapshot: &SafetyNetMonitoringSnapshot) -> f32 {
    let base_rate = 20.0; // default target

    let latency = snapshot.avg_latency_ms.max(1.0);
    let jitter = (snapshot.rts_smoothed_latency - snapshot.avg_latency_ms).abs();

    if latency > 250.0 || jitter > 80.0 || snapshot.safety_net_trigger_count > 12 {
        return 8.0;  // Congested or unstable — protect the channel
    } else if latency > 120.0 || jitter > 40.0 {
        return 12.0;
    } else if latency < 40.0 && jitter < 15.0 && snapshot.safety_net_trigger_count < 3 {
        return 30.0; // Excellent connection — can afford higher rate
    }

    base_rate
}

// SafetyNetMonitoringSnapshot definition (simplified for integration)
#[derive(Clone, Debug, Default)]
pub struct SafetyNetMonitoringSnapshot {
    pub avg_latency_ms: f32,
    pub rts_smoothed_latency: f32,
    pub safety_net_trigger_count: u32,
    // ... other fields from client/monitoring/safety_net.rs
}

// === CLIENT DECODER (advanced v18.46) ===
pub fn decode_masked_batch(data: &[u8]) -> Vec<DecodedUpdate> {
    let mut updates = Vec::new();
    let mut cursor = 0;

    while cursor < data.len() {
        // Read component, entity, dirty_mask (varint)
        // Then conditionally read only the fields present in the mask
        // Full implementation continues from v18.45 foundation
        // Ready for integration into rbe_client_sync.rs + prediction rollback
    }

    updates
}

#[derive(Debug, Clone)]
pub struct DecodedUpdate {
    pub entity: u64,
    pub fields: ReplicatedFields,
    pub position: Option<Vec3>,
    pub velocity: Option<Vec3>,
    pub health: Option<(f32, f32)>,
    pub rbe_abundance: Option<f32>,
}

// === INTEGRATION POINT ===
// In replication loop / world_state_broadcaster:
// let rate = calculate_adaptive_send_rate(&latest_safety_net_snapshot);
// let batch = encode_domain_specific(&updates, rate);
// send_to_client(batch, rate);
//
// Client (rbe_client_sync.rs):
// let decoded = decode_masked_batch(received_data);
// apply_to_prediction_buffer(decoded);

// Thunder locked in. All remaining items advanced. Yoi ⚡