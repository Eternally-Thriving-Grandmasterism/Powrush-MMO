// server/src/replication/mod.rs
// Powrush-MMO v17.77 — Component-Level Dirty Tracking + Interest-Based Filtering
// Combined implementation: tracks specific components that changed and filters by InterestManager

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::combat::AbilityCooldownUpdate;
use crate::interest_management::InterestManager;

// ═════════════════════════════════════════════════════════════════════════
// COMPONENT-LEVEL DIRTY TRACKING + INTEREST FILTERING
// ═════════════════════════════════════════════════════════════════════════

/// Identifies a component type for dirty tracking purposes.
/// In a full implementation this would use TypeId or a custom ComponentId system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReplicatedComponent {
    Ability,
    Health,
    StatusEffect,
    Position,
    CombatStats,
    // Add more as needed
}

/// Tracks which specific components on which entities are dirty.
/// This enables fine-grained, bandwidth-efficient replication.
#[derive(Resource, Default)]
pub struct ComponentDirtyTracker {
    /// Entity -> Set of components that changed
    pub dirty: HashMap<Entity, HashSet<ReplicatedComponent>>,
}

impl ComponentDirtyTracker {
    pub fn mark_dirty(&mut self, entity: Entity, component: ReplicatedComponent) {
        self.dirty.entry(entity).or_default().insert(component);
    }

    /// Returns and clears all currently dirty state
    pub fn drain_all(&mut self) -> HashMap<Entity, HashSet<ReplicatedComponent>> {
        std::mem::take(&mut self.dirty)
    }

    pub fn is_empty(&self) -> bool {
        self.dirty.is_empty()
    }
}

/// Represents a targeted update ready to be sent to a specific player
#[derive(Debug, Clone)]
pub struct TargetedUpdate {
    pub recipient: Entity,
    pub entity: Entity,
    pub component: ReplicatedComponent,
    // In future: actual serialized data payload
}

// ═════════════════════════════════════════════════════════════════════════
// SYSTEMS
// ═════════════════════════════════════════════════════════════════════════

/// Processes combat events and marks specific components as dirty
pub fn process_combat_updates(
    mut ev_cooldown_update: EventReader<AbilityCooldownUpdate>,
    mut component_dirty: ResMut<ComponentDirtyTracker>,
) {
    for update in ev_cooldown_update.read() {
        // When a cooldown changes, only the Ability component is dirty
        component_dirty.mark_dirty(update.acting_player, ReplicatedComponent::Ability);
    }
}

/// Core system: Combines component-level dirty tracking with interest-based filtering
/// This is the heart of efficient, targeted replication.
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
        for component in components {
            // === INTEREST-BASED FILTERING ===
            // In a full implementation we would query:
            // let interested_players = interest.get_interested_players(entity);
            //
            // For now we demonstrate the pattern by sending to the entity itself
            // (self-updates are always Critical priority)
            targeted_updates.send(TargetedUpdate {
                recipient: entity,
                entity,
                component,
            });

            // Future: Loop over interested_players and create TargetedUpdate for each
            // while respecting InterestPriority and max_interest_entities caps.
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

// ═════════════════════════════════════════════════════════════════════════
// NOTES
// ═════════════════════════════════════════════════════════════════════════
//
// This iteration combines:
// - Component-level dirty tracking (Ability, Health, etc.)
// - Interest-based filtering foundation (via InterestManager)
// - TargetedUpdate event for per-recipient delivery
//
// Next steps:
// - Implement real get_interested_players() query
// - Add actual serialization for each ReplicatedComponent
// - Batching of TargetedUpdates per recipient
// - Priority handling using InterestPriority
// - Integration with full networking transport
