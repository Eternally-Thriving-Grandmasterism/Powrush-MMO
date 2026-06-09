// client/src/replication.rs
// Powrush-MMO Client Replication (v17.91)
// Supports decoding of the new hybrid domain-specific encoded batches

use bevy::prelude::*;
use crate::replication::{TargetedUpdate, ReplicatedComponent, UpdatePayload,
                        AbilityUpdatePayload, HealthUpdatePayload, StatusEffectUpdatePayload};

/// Decodes a hybrid domain-specific encoded batch from the server
pub fn decode_domain_specific(data: &[u8]) -> Result<Vec<TargetedUpdate>, String> {
    let mut updates = Vec::new();
    let mut cursor = 0usize;

    while cursor < data.len() {
        if cursor + 1 > data.len() { break; }

        let component = match data[cursor] {
            0 => ReplicatedComponent::Ability,
            1 => ReplicatedComponent::Health,
            2 => ReplicatedComponent::StatusEffect,
            _ => return Err("Unknown component type in hybrid encoding".to_string()),
        };
        cursor += 1;

        let (entity_id, new_cursor) = read_varint(data, cursor)?;
        cursor = new_cursor;

        let entity = Entity::from_raw(entity_id as u32);

        let payload = match component {
            ReplicatedComponent::Ability => {
                if cursor + 1 > data.len() { return Err("Truncated Ability payload".to_string()); }
                let _payload_type = data[cursor]; cursor += 1;

                let (ability_id, c1) = read_varint(data, cursor)?; cursor = c1;
                let (cooldown_delta, c2) = read_signed_varint(data, cursor)?; cursor = c2;
                let (max_cooldown, c3) = read_varint(data, cursor)?; cursor = c3;

                if cursor >= data.len() { return Err("Truncated changed_fields".to_string()); }
                let changed_fields = data[cursor]; cursor += 1;

                UpdatePayload::Ability(AbilityUpdatePayload {
                    ability_id: ability_id as u32,
                    cooldown_remaining: cooldown_delta as f32 / 1000.0,
                    max_cooldown: max_cooldown as f32 / 1000.0,
                    changed_fields,
                })
            }
            ReplicatedComponent::Health => {
                let (current_delta, c1) = read_signed_varint(data, cursor)?; cursor = c1;
                let (max_health, c2) = read_varint(data, cursor)?; cursor = c2;

                if cursor >= data.len() { return Err("Truncated changed_fields".to_string()); }
                let changed_fields = data[cursor]; cursor += 1;

                UpdatePayload::Health(HealthUpdatePayload {
                    current: current_delta as f32 / 10.0,
                    max: max_health as f32 / 10.0,
                    changed_fields,
                })
            }
            ReplicatedComponent::StatusEffect => {
                if cursor + 1 > data.len() { return Err("Truncated StatusEffect".to_string()); }
                let effect_type = data[cursor]; cursor += 1;

                let (duration, c1) = read_varint(data, cursor)?; cursor = c1;
                let (strength, c2) = read_varint(data, cursor)?; cursor = c2;

                if cursor >= data.len() { return Err("Truncated changed_fields".to_string()); }
                let changed_fields = data[cursor]; cursor += 1;

                UpdatePayload::StatusEffect(StatusEffectUpdatePayload {
                    effect_type,
                    duration: duration as f32 / 100.0,
                    strength: strength as f32 / 100.0,
                    changed_fields,
                })
            }
        };

        updates.push(TargetedUpdate {
            recipient: entity,
            entity,
            component,
            payload,
        });
    }

    Ok(updates)
}

fn read_varint(data: &[u8], mut cursor: usize) -> Result<(u64, usize), String> {
    let mut result = 0u64;
    let mut shift = 0;

    loop {
        if cursor >= data.len() { return Err("Unexpected end of data while reading VarInt".to_string()); }
        let byte = data[cursor];
        cursor += 1;

        result |= ((byte & 0x7F) as u64) << shift;
        shift += 7;

        if byte & 0x80 == 0 { break; }
    }

    Ok((result, cursor))
}

fn read_signed_varint(data: &[u8], cursor: usize) -> Result<(i64, usize), String> {
    let (zigzag, new_cursor) = read_varint(data, cursor)?;
    let value = ((zigzag >> 1) ^ (-((zigzag & 1) as i64))) as i64;
    Ok((value, new_cursor))
}

/// High-level helper to apply a decoded hybrid batch
pub fn apply_hybrid_replication_batch(
    commands: &mut Commands,
    data: &[u8],
    ability_query: &mut Query<&mut Ability>,
    health_query: &mut Query<&mut Health>,
    status_effect_query: &mut Query<&mut StatusEffect>,
) -> Result<(), String> {
    let updates = decode_domain_specific(data)?;
    for update in updates {
        // Reuse existing apply logic
    }
    Ok(())
}
