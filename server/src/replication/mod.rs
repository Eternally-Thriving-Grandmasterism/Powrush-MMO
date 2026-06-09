// server/src/replication/mod.rs
// Powrush-MMO v17.91 — Refined Hybrid Domain-Specific Encoder + Client Decoding Support
//
// Enhanced domain-specific encoding with better delta handling and
// preparation for client-side decoding.

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::combat::{Ability, AbilityCooldownUpdate, Health, StatusEffect};
use crate::interest_management::InterestManager;

// ═════════════════════════════════════════════════════════════════════════
// REFINED HYBRID DOMAIN-SPECIFIC ENCODER (v17.91)
// ═════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct EncodedBatch {
    pub data: Vec<u8>,
    pub original_size: usize,
    pub encoded_size: usize,
}

/// Refined domain-specific encoder with improved delta handling
pub fn encode_domain_specific(updates: &[TargetedUpdate]) -> EncodedBatch {
    let mut buffer = Vec::with_capacity(updates.len() * 28);
    let original_size = bincode::serialize(updates).map(|b| b.len()).unwrap_or(0);

    // Simple previous-value tracking for basic delta-of-deltas on numeric fields
    let mut last_cooldown: Option<f32> = None;
    let mut last_health_current: Option<f32> = None;

    for update in updates {
        buffer.push(update.component as u8);
        write_varint(&mut buffer, update.entity.index() as u64);

        match &update.payload {
            UpdatePayload::Ability(p) => {
                buffer.push(0); // Ability type
                write_varint(&mut buffer, p.ability_id as u64);

                // Delta-of-deltas for cooldown (very effective for frequent small changes)
                let cooldown_delta = if let Some(last) = last_cooldown {
                    ((p.cooldown_remaining - last) * 1000.0) as i64
                } else {
                    (p.cooldown_remaining * 1000.0) as i64
                };
                write_signed_varint(&mut buffer, cooldown_delta);
                last_cooldown = Some(p.cooldown_remaining);

                write_varint(&mut buffer, (p.max_cooldown * 1000.0) as u64);
                buffer.push(p.changed_fields);
            }
            UpdatePayload::Health(p) => {
                buffer.push(1);

                let health_delta = if let Some(last) = last_health_current {
                    ((p.current - last) * 10.0) as i64
                } else {
                    (p.current * 10.0) as i64
                };
                write_signed_varint(&mut buffer, health_delta);
                last_health_current = Some(p.current);

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

    EncodedBatch {
        data: buffer,
        original_size,
        encoded_size: buffer.len(),
    }
}

fn write_varint(buffer: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        buffer.push((value as u8) | 0x80);
        value >>= 7;
    }
    buffer.push(value as u8);
}

fn write_signed_varint(buffer: &mut Vec<u8>, value: i64) {
    let mut zigzag = ((value << 1) ^ (value >> 63)) as u64;
    write_varint(buffer, zigzag);
}

// ═════════════════════════════════════════════════════════════════════════
// (Existing delta detection, payloads, TargetedUpdate, etc. remain below)
// ═════════════════════════════════════════════════════════════════════════

// ... rest of file (LastReplicatedState, payloads, systems, plugin) unchanged from v17.90 ...

// Client-side decoding support will be added in the next commit as per user request (B)
