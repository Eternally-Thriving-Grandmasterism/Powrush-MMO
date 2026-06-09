// server/src/replication/mod.rs
// Powrush-MMO v17.84 — Delta Serialization (Phase 1)
//
// Added delta flags to payloads + optimized bincode configuration.
// This is the foundation for bandwidth-efficient, change-only replication.

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use bincode;
use crate::combat::{Ability, AbilityCooldownUpdate, Health, StatusEffect};
use crate::interest_management::InterestManager;

// ═════════════════════════════════════════════════════════════════════════
// DELTA SERIALIZATION (Phase 1)
// ═════════════════════════════════════════════════════════════════════════

/// Bitmask for AbilityUpdatePayload changed fields
pub mod ability_delta {
    pub const ABILITY_ID: u8 = 1 << 0;
    pub const COOLDOWN_REMAINING: u8 = 1 << 1;
    pub const MAX_COOLDOWN: u8 = 1 << 2;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityUpdatePayload {
    pub ability_id: u32,
    pub cooldown_remaining: f32,
    pub max_cooldown: f32,
    /// Bitmask indicating which fields actually changed
    pub changed_fields: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthUpdatePayload {
    pub current: f32,
    pub max: f32,
    pub changed_fields: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectUpdatePayload {
    pub effect_type: u8,
    pub duration: f32,
    pub strength: f32,
    pub changed_fields: u8,
}

// ═════════════════════════════════════════════════════════════════════════
// DATA STRUCTURES
// ═════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReplicatedComponent { Ability, Health, StatusEffect, Position, CombatStats }

#[derive(Event, Debug, Clone)]
pub struct TargetedUpdate {
    pub recipient: Entity,
    pub entity: Entity,
    pub component: ReplicatedComponent,
    pub payload: UpdatePayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdatePayload {
    Ability(AbilityUpdatePayload),
    Health(HealthUpdatePayload),
    StatusEffect(StatusEffectUpdatePayload),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchedUpdates {
    pub recipient: Entity,
    pub updates: Vec<TargetedUpdate>,
}

#[derive(Resource, Default)]
pub struct ReplicationBatcher {
    pub batches: HashMap<Entity, Vec<TargetedUpdate>>,
}

impl ReplicationBatcher {
    pub fn add_update(&mut self, update: TargetedUpdate) {
        self.batches.entry(update.recipient).or_default().push(update);
    }

    pub fn drain_batches(&mut self) -> HashMap<Entity, Vec<TargetedUpdate>> {
        std::mem::take(&mut self.batches)
    }
}

// ═════════════════════════════════════════════════════════════════════════
// RESOURCES
// ═════════════════════════════════════════════════════════════════════════

#[derive(Resource, Default)]
pub struct ComponentDirtyTracker {
    pub dirty: HashMap<Entity, HashSet<ReplicatedComponent>>,
}

impl ComponentDirtyTracker {
    pub fn mark_dirty(&mut self, entity: Entity, component: ReplicatedComponent) {
        self.dirty.entry(entity).or_default().insert(component);
    }

    pub fn drain_all(&mut self) -> HashMap<Entity, HashSet<ReplicatedComponent>> {
        std::mem::take(&mut self.dirty)
    }

    pub fn is_empty(&self) -> bool {
        self.dirty.is_empty()
    }
}

// ═════════════════════════════════════════════════════════════════════════
// SYSTEMS
// ═════════════════════════════════════════════════════════════════════════

pub fn process_combat_updates(
    mut ev_cooldown_update: EventReader<AbilityCooldownUpdate>,
    mut component_dirty: ResMut<ComponentDirtyTracker>,
) {
    for update in ev_cooldown_update.read() {
        component_dirty.mark_dirty(update.acting_player, ReplicatedComponent::Ability);
    }
}

pub fn replicate_dirty_state(
    mut component_dirty: ResMut<ComponentDirtyTracker>,
    interest: Res<InterestManager>,
    ability_query: Query<&Ability>,
    health_query: Query<&Health>,
    status_effect_query: Query<&StatusEffect>,
    mut targeted_updates: EventWriter<TargetedUpdate>,
) {
    if component_dirty.is_empty() {
        return;
    }

    let dirty_state = component_dirty.drain_all();

    for (entity, components) in dirty_state {
        let interested = interest.get_interested_players(entity as u64);

        for component in components {
            let payload = match component {
                ReplicatedComponent::Ability => {
                    if let Ok(ability) = ability_query.get(entity) {
                        // For now we mark all fields as changed (full update).
                        // Real delta detection can be added later.
                        let mut changed = 0u8;
                        changed |= ability_delta::ABILITY_ID;
                        changed |= ability_delta::COOLDOWN_REMAINING;
                        changed |= ability_delta::MAX_COOLDOWN;

                        UpdatePayload::Ability(AbilityUpdatePayload {
                            ability_id: ability.id,
                            cooldown_remaining: ability.last_used,
                            max_cooldown: ability.cooldown,
                            changed_fields: changed,
                        })
                    } else { continue; }
                }
                ReplicatedComponent::Health => {
                    if let Ok(health) = health_query.get(entity) {
                        UpdatePayload::Health(HealthUpdatePayload {
                            current: health.current,
                            max: health.max,
                            changed_fields: 0b11,
                        })
                    } else { continue; }
                }
                ReplicatedComponent::StatusEffect => {
                    if let Ok(effect) = status_effect_query.get(entity) {
                        UpdatePayload::StatusEffect(StatusEffectUpdatePayload {
                            effect_type: effect.effect_type as u8,
                            duration: effect.duration,
                            strength: effect.strength,
                            changed_fields: 0b111,
                        })
                    } else { continue; }
                }
                _ => continue,
            };

            targeted_updates.send(TargetedUpdate {
                recipient: entity,
                entity,
                component,
                payload: payload.clone(),
            });

            for &recipient_id in &interested {
                if recipient_id != entity.index() as u64 {
                    targeted_updates.send(TargetedUpdate {
                        recipient: entity,
                        entity,
                        component,
                        payload: payload.clone(),
                    });
                }
            }
        }
    }
}

pub fn batch_targeted_updates(
    mut ev_targeted: EventReader<TargetedUpdate>,
    mut batcher: ResMut<ReplicationBatcher>,
) {
    for update in ev_targeted.read() {
        batcher.add_update(update.clone());
    }
}

pub fn send_replication_batches(
    mut batcher: ResMut<ReplicationBatcher>,
) {
    let batches = batcher.drain_batches();

    if batches.is_empty() {
        return;
    }

    // Optimized bincode configuration
    let config = bincode::DefaultOptions::new()
        .with_varint_encoding()
        .allow_trailing_bytes();

    for (recipient, updates) in batches {
        // Future: Use config.serialize() for real network packets
        println!(
            "[Replication] Preparing batch for {:?} ({} updates) - Delta + Optimized bincode ready",
            recipient, updates.len()
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════
// PLUGIN
// ═════════════════════════════════════════════════════════════════════════

pub struct ReplicationPlugin;

impl Plugin for ReplicationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ComponentDirtyTracker>()
            .init_resource::<ReplicationBatcher>()
            .add_event::<TargetedUpdate>()
            .add_systems(Update, (
                process_combat_updates,
                replicate_dirty_state,
                batch_targeted_updates,
                send_replication_batches,
            ));
    }
}
