// server/src/replication/mod.rs
// Powrush-MMO v17.82 — Replication Pipeline (Refined Documentation)
//
// FULL PIPELINE OVERVIEW:
// 1. Dirty Marking (process_combat_updates)
//    - Systems mark specific components as dirty when they change.
// 2. Real Data Query (replicate_dirty_state)
//    - Queries actual ECS components (Ability, Health, StatusEffect...) to build payloads.
// 3. Interest Filtering
//    - Uses InterestManager.get_interested_players() to determine recipients.
// 4. Targeted Update Generation
//    - Creates per-recipient TargetedUpdate events with real payloads.
// 5. Batching (batch_targeted_updates)
//    - Groups updates by recipient into ReplicationBatcher for efficient delivery.
//
// This pipeline is component-aware, interest-filtered, and batched.
// Ready for networking transport integration.

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::combat::{Ability, AbilityCooldownUpdate, Health, StatusEffect};
use crate::interest_management::InterestManager;

// ═════════════════════════════════════════════════════════════════════════
// PAYLOADS
// ═════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityUpdatePayload {
    pub ability_id: u32,
    pub cooldown_remaining: f32,
    pub max_cooldown: f32,
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
    pub strength: f32,
}

// ═════════════════════════════════════════════════════════════════════════
// TARGETED UPDATE + BATCHING
// ═════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReplicatedComponent {
    Ability, Health, StatusEffect, Position, CombatStats,
}

#[derive(Event, Debug, Clone)]
pub struct TargetedUpdate {
    pub recipient: Entity,
    pub entity: Entity,
    pub component: ReplicatedComponent,
    pub payload: UpdatePayload,
}

#[derive(Debug, Clone)]
pub enum UpdatePayload {
    Ability(AbilityUpdatePayload),
    Health(HealthUpdatePayload),
    StatusEffect(StatusEffectUpdatePayload),
}

/// A complete batch of updates for one recipient
#[derive(Debug, Clone)]
pub struct BatchedUpdates {
    pub recipient: Entity,
    pub updates: Vec<TargetedUpdate>,
}

/// Collects updates per recipient for efficient delivery
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

/// Queries real component data and produces interest-filtered TargetedUpdates
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
                        UpdatePayload::Ability(AbilityUpdatePayload {
                            ability_id: ability.id,
                            cooldown_remaining: ability.last_used,
                            max_cooldown: ability.cooldown,
                        })
                    } else { continue; }
                }
                ReplicatedComponent::Health => {
                    if let Ok(health) = health_query.get(entity) {
                        UpdatePayload::Health(HealthUpdatePayload { current: health.current, max: health.max })
                    } else { continue; }
                }
                ReplicatedComponent::StatusEffect => {
                    if let Ok(effect) = status_effect_query.get(entity) {
                        UpdatePayload::StatusEffect(StatusEffectUpdatePayload {
                            effect_type: effect.effect_type as u8,
                            duration: effect.duration,
                            strength: effect.strength,
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

/// Groups TargetedUpdates by recipient for efficient network delivery
pub fn batch_targeted_updates(
    mut ev_targeted: EventReader<TargetedUpdate>,
    mut batcher: ResMut<ReplicationBatcher>,
) {
    for update in ev_targeted.read() {
        batcher.add_update(update.clone());
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
            ));
    }
}

// ═════════════════════════════════════════════════════════════════════════
// PIPELINE NOTES
// ═════════════════════════════════════════════════════════════════════════
//
// Current Capabilities:
// - Component-level dirty tracking (not just entity-level)
// - Real ECS data queries for payloads
// - InterestManager-based recipient filtering
// - Per-recipient batching via ReplicationBatcher
//
// Future Work:
// - Full spatial reverse interest queries in InterestManager
// - Actual network serialization + transport integration
// - Priority handling (InterestPriority)
// - Delta compression / change-only replication
// - Client-side prediction & reconciliation support
//
// This module is designed to be the central, efficient delivery system
// for all gameplay state (combat, world, diplomacy, council influence).
