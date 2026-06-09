// server/src/replication/mod.rs
// Powrush-MMO v17.73 — Replication Pipeline (Foundation)
// Professional skeleton for dirty tracking + interest-aware state delivery
// This module will become the central nervous system for all client-bound state.

use bevy::prelude::*;
use crate::combat::AbilityCooldownUpdate;
use crate::interest_management::InterestManager;

// ═════════════════════════════════════════════════════════════════════════
// REPLICATION MODULE OVERVIEW
// ═════════════════════════════════════════════════════════════════════════
//
// Responsibilities:
// - Track dirty/changed state that needs to be sent to clients
// - Filter updates using InterestManager (per-player targeting)
// - Batch and prioritize outgoing updates
// - Eventually integrate with the networking transport layer
//
// Current Phase: Foundation + Interest-aware broadcasting of combat events

// ═════════════════════════════════════════════════════════════════════════
// RESOURCES
// ═════════════════════════════════════════════════════════════════════════

/// Tracks entities and components that have changed and need replication
#[derive(Resource, Default)]
pub struct DirtyTracker {
    // Future: Use generational indices or sparse sets for efficiency
    pub dirty_entities: Vec<Entity>,
}

// ═════════════════════════════════════════════════════════════════════════
// SYSTEMS
// ═════════════════════════════════════════════════════════════════════════

/// Consumes AbilityCooldownUpdate events and prepares them for interest-aware delivery
pub fn process_combat_updates(
    mut ev_cooldown_update: EventReader<AbilityCooldownUpdate>,
    interest: Res<InterestManager>,
    mut dirty_tracker: ResMut<DirtyTracker>,
) {
    for update in ev_cooldown_update.read() {
        // For now we simply mark the acting player as dirty.
        // In later iterations we will:
        // - Query interested players using InterestManager
        // - Create per-recipient update packets
        // - Batch and prioritize

        if !dirty_tracker.dirty_entities.contains(&update.acting_player) {
            dirty_tracker.dirty_entities.push(update.acting_player);
        }

        // Placeholder for future interest-based filtering:
        // let interested = interest.get_interested_players(update.acting_player);
        // for recipient in interested {
        //     // send targeted update to recipient
        // }
    }
}

// ═════════════════════════════════════════════════════════════════════════
// PLUGIN
// ═════════════════════════════════════════════════════════════════════════

pub struct ReplicationPlugin;

impl Plugin for ReplicationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DirtyTracker>()
            .add_systems(Update, process_combat_updates);
    }
}

// ═════════════════════════════════════════════════════════════════════════
// NOTES & FUTURE WORK
// ═════════════════════════════════════════════════════════════════════════
//
// Next steps for this module:
// - Implement proper interest filtering using InterestManager::get_interested_players()
// - Create per-recipient update packets
// - Add batching and prioritization logic
// - Integrate with the networking transport (TokioTransport)
// - Support more event types (world events, diplomacy, faction changes, etc.)
// - Add support for carrying lightweight "meaning/context" from Ra-Thor/PATSAGi Councils
//
// This module is intentionally kept minimal in v17.73 to establish the foundation cleanly.
