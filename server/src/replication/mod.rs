// server/src/replication/mod.rs
// Powrush-MMO v17.90 — Hybrid Domain-Specific Encoder
//
// Implements a lightweight, structure-aware encoding layer that runs
// before general compression. This dramatically reduces payload size
// by exploiting the shape of replication data (VarInt, delta encoding,
// bit packing).
//
// Design Philosophy: Respect the data. Encode with intention.

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::combat::{Ability, AbilityCooldownUpdate, Health, StatusEffect};
use crate::interest_management::InterestManager;

// ═════════════════════════════════════════════════════════════════════════
// HYBRID DOMAIN-SPECIFIC ENCODER
// ═════════════════════════════════════════════════════════════════════════

/// Lightweight domain-specific encoding result
#[derive(Debug, Clone)]
pub struct EncodedBatch {
    pub data: Vec<u8>,
    pub original_size: usize,
    pub encoded_size: usize,
}

/// Encodes a batch of TargetedUpdates using domain-specific techniques
/// before general compression. This is the hybrid layer.
pub fn encode_domain_specific(updates: &[TargetedUpdate]) -> EncodedBatch {
    let mut buffer = Vec::with_capacity(updates.len() * 32);
    let original_size = bincode::serialize(updates).map(|b| b.len()).unwrap_or(0);

    for update in updates {
        // Write component type (1 byte)
        buffer.push(update.component as u8);

        // Write entity ID as VarInt (compact for small IDs)
        write_varint(&mut buffer, update.entity.index());

        match &update.payload {
            UpdatePayload::Ability(p) => {
                buffer.push(0); // payload type
                write_varint(&mut buffer, p.ability_id as u64);
                write_varint(&mut buffer, (p.cooldown_remaining * 1000.0) as u64); // ms precision
                write_varint(&mut buffer, (p.max_cooldown * 1000.0) as u64);
                buffer.push(p.changed_fields);
            }
            UpdatePayload::Health(p) => {
                buffer.push(1);
                write_varint(&mut buffer, (p.current * 10.0) as u64);  // 0.1 precision
                write_varint(&mut buffer, (p.max * 10.0) as u64);
                buffer.push(p.changed_fields);
            }
            UpdatePayload::StatusEffect(p) => {
                buffer.push(2);
                buffer.push(p.effect_type);
                write_varint(&mut buffer, (p.duration * 100.0) as u64);
                write_varint(&mut buffer, (p.strength * 100.0) as u64);
                buffer.push(p.changed_fields);
            }
        }
    }

    let encoded_size = buffer.len();

    EncodedBatch {
        data: buffer,
        original_size,
        encoded_size,
    }
}

/// Simple VarInt writer (LEB128 style)
fn write_varint(buffer: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        buffer.push((value as u8) | 0x80);
        value >>= 7;
    }
    buffer.push(value as u8);
}

// ═════════════════════════════════════════════════════════════════════════
// (Rest of the file continues with existing delta detection, payloads, etc.)
// The hybrid encoder is integrated in compress_batch below.
// ═════════════════════════════════════════════════════════════════════════

// ... existing code for LastReplicatedState, payloads, TargetedUpdate, etc. ...

// Updated compress_batch to use hybrid domain-specific encoding first
pub fn compress_batch(
    updates: &[TargetedUpdate],
    config: &CompressionConfig,
) -> (Vec<u8>, usize, usize) {
    if !config.enabled || updates.is_empty() {
        let serialized = bincode::serialize(updates).unwrap_or_default();
        return (serialized.clone(), serialized.len(), serialized.len());
    }

    // === HYBRID DOMAIN-SPECIFIC ENCODING LAYER ===
    let encoded = encode_domain_specific(updates);

    // Skip general compression for very small results
    if encoded.encoded_size < config.min_size_to_compress {
        return (encoded.data.clone(), encoded.original_size, encoded.encoded_size);
    }

    // Apply zstd on the already domain-optimized data
    let compressed = zstd::encode_all(&encoded.data[..], config.level).unwrap_or(encoded.data.clone());
    let final_size = compressed.len();

    (compressed, encoded.original_size, final_size)
}

// ... rest of the systems and plugin remain the same ...

pub fn send_replication_batches(
    mut batcher: ResMut<ReplicationBatcher>,
    compression_config: Res<CompressionConfig>,
) {
    let batches = batcher.drain_batches();
    if batches.is_empty() {
        return;
    }

    for (recipient, updates) in batches {
        let (compressed, original_size, final_size) =
            compress_batch(&updates, &compression_config);

        let ratio = if original_size > 0 {
            (final_size as f32 / original_size as f32) * 100.0
        } else {
            100.0
        };

        println!(
            "[Replication] Hybrid encoded + zstd: {:?} | {} updates | {} -> {} bytes ({:.1}%)",
            recipient, updates.len(), original_size, final_size, ratio
        );

        // TRANSPORT HOOK
    }
}
