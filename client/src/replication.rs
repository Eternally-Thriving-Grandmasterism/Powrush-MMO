// client/src/replication.rs
// Powrush-MMO Client Replication
// Applies server-sent TargetedUpdate batches to the local Bevy ECS

use bevy::prelude::*;
use crate::replication::{TargetedUpdate, ReplicatedComponent, UpdatePayload};
use crate::combat::{Ability, Health, StatusEffect}; // Assuming shared combat components

/// Applies a single TargetedUpdate received from the server.
/// Uses changed_fields mask for efficient partial updates.
pub fn apply_replication_update(
    commands: &mut Commands,
    update: TargetedUpdate,
    // Queries for existing components
    mut ability_query: Query<&mut Ability>,
    mut health_query: Query<&mut Health>,
    mut status_effect_query: Query<&mut StatusEffect>,
) {
    let entity = update.entity;

    match update.payload {
        UpdatePayload::Ability(payload) => {
            if let Ok(mut ability) = ability_query.get_mut(entity) {
                // Only update fields that changed according to the server
                if payload.changed_fields & crate::replication::ability_delta::ABILITY_ID != 0 {
                    ability.id = payload.ability_id;
                }
                if payload.changed_fields & crate::replication::ability_delta::COOLDOWN_REMAINING != 0 {
                    ability.last_used = payload.cooldown_remaining;
                }
                if payload.changed_fields & crate::replication::ability_delta::MAX_COOLDOWN != 0 {
                    ability.cooldown = payload.max_cooldown;
                }
            } else {
                // Entity doesn't have Ability yet - spawn/insert it
                commands.entity(entity).insert(Ability {
                    id: payload.ability_id,
                    cooldown: payload.max_cooldown,
                    last_used: payload.cooldown_remaining,
                    range: 0.0,           // default or from server if added later
                    ability_type: crate::combat::AbilityType::DirectDamage, // default
                    triggers_gcd: true,
                });
            }
        }

        UpdatePayload::Health(payload) => {
            if let Ok(mut health) = health_query.get_mut(entity) {
                if payload.changed_fields & 0b01 != 0 {
                    health.current = payload.current;
                }
                if payload.changed_fields & 0b10 != 0 {
                    health.max = payload.max;
                }
            } else {
                commands.entity(entity).insert(Health {
                    current: payload.current,
                    max: payload.max,
                });
            }
        }

        UpdatePayload::StatusEffect(payload) => {
            if let Ok(mut effect) = status_effect_query.get_mut(entity) {
                effect.duration = payload.duration;
                effect.strength = payload.strength;
                // effect_type can be updated if needed
            } else {
                commands.entity(entity).insert(StatusEffect {
                    effect_type: crate::combat::StatusEffectType::from_u8(payload.effect_type),
                    duration: payload.duration,
                    strength: payload.strength,
                });
            }
        }
    }
}

/// Helper to apply a whole batch of updates (recommended entry point from networking)
pub fn apply_replication_batch(
    commands: &mut Commands,
    updates: Vec<TargetedUpdate>,
    ability_query: &mut Query<&mut Ability>,
    health_query: &mut Query<&mut Health>,
    status_effect_query: &mut Query<&mut StatusEffect>,
) {
    for update in updates {
        apply_replication_update(commands, update, ability_query, health_query, status_effect_query);
    }
}
