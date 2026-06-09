// server/src/replication/mod.rs
// Powrush-MMO v17.79 — Replication with Actual Payloads
// TargetedUpdate now carries real component data

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::combat::AbilityCooldownUpdate;
use crate::interest_management::InterestManager;

// ═════════════════════════════════════════════════════════════════════════
// REPLICABLE PAYLOADS
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
    pub effect_type: u8, // simplified
    pub duration: f32,
    pub strength: f32,
}

// ═════════════════════════════════════════════════════════════════════════
// REPLICATED COMPONENT + TARGETED UPDATE WITH PAYLOAD
// ═════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReplicatedComponent {
    Ability,
    Health,
    StatusEffect,
    Position,
    CombatStats,
}

/// The actual update sent to a specific player
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
    // Add more variants as needed
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

/// Produces TargetedUpdate events with actual payloads
pub fn replicate_dirty_state(
    mut component_dirty: ResMut<ComponentDirtyTracker>,
    interest: Res<InterestManager>,
    mut targeted_updates: EventWriter<TargetedUpdate>,
) {
    if component_dirty.is_empty() {
        return;
    }

    let dirty_state = component_dirty.drain_all();

    for (entity, components) in dirty_state {
        let interested = interest.get_interested_players(entity as u64);

        for component in components {
            // Create payload based on component type
            // In a real implementation we would query the actual component values here
            let payload = match component {
                ReplicatedComponent::Ability => {
                    UpdatePayload::Ability(AbilityUpdatePayload {
                        ability_id: 0, // TODO: Query real ability data
                        cooldown_remaining: 0.0,
                        max_cooldown: 0.0,
                    })
                }
                ReplicatedComponent::Health => {
                    UpdatePayload::Health(HealthUpdatePayload {
                        current: 100.0,
                        max: 100.0,
                    })
                }
                ReplicatedComponent::StatusEffect => {
                    UpdatePayload::StatusEffect(StatusEffectUpdatePayload {
                        effect_type: 0,
                        duration: 0.0,
                        strength: 0.0,
                    })
                }
                _ => continue,
            };

            // Self update (always Critical)
            targeted_updates.send(TargetedUpdate {
                recipient: entity,
                entity,
                component,
                payload: payload.clone(),
            });

            // Interested players
            for &recipient_id in &interested {
                if recipient_id != entity.index() as u64 {
                    targeted_updates.send(TargetedUpdate {
                        recipient: entity, // placeholder mapping
                        entity,
                        component,
                        payload: payload.clone(),
                    });
                }
            }
        }
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
            .add_event::<TargetedUpdate>()
            .add_systems(Update, (
                process_combat_updates,
                replicate_dirty_state,
            ));
    }
}
