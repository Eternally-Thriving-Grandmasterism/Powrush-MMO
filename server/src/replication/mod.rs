// server/src/replication/mod.rs
// Powrush-MMO v17.76 — Replication Pipeline + Dirty Tracking Optimization
// Professional dirty tracking with HashSet for deduplication and performance

use bevy::prelude::*;
use std::collections::HashSet;
use crate::combat::AbilityCooldownUpdate;
use crate::interest_management::InterestManager;

// ═════════════════════════════════════════════════════════════════════════
// REPLICATION MODULE + OPTIMIZED DIRTY TRACKING
// ═════════════════════════════════════════════════════════════════════════

/// Optimized DirtyTracker using HashSet for automatic deduplication and O(1) lookups.
/// This is critical for high-frequency combat and replication scenarios.
#[derive(Resource, Default)]
pub struct DirtyTracker {
    pub dirty_entities: HashSet<Entity>,
}

impl DirtyTracker {
    /// Mark an entity as dirty (automatically deduplicates)
    pub fn mark_dirty(&mut self, entity: Entity) {
        self.dirty_entities.insert(entity);
    }

    /// Drain all dirty entities (consumes the set for replication)
    pub fn drain_dirty(&mut self) -> Vec<Entity> {
        self.dirty_entities.drain().collect()
    }

    /// Clear all dirty entities without returning them
    pub fn clear(&mut self) {
        self.dirty_entities.clear();
    }

    /// Check if an entity is currently marked dirty
    pub fn is_dirty(&self, entity: Entity) -> bool {
        self.dirty_entities.contains(&entity)
    }
}

// ═════════════════════════════════════════════════════════════════════════
// SYSTEMS
// ═════════════════════════════════════════════════════════════════════════

/// Processes combat updates and marks entities as dirty using the optimized tracker
pub fn process_combat_updates(
    mut ev_cooldown_update: EventReader<AbilityCooldownUpdate>,
    interest: Res<InterestManager>,
    mut dirty_tracker: ResMut<DirtyTracker>,
) {
    for update in ev_cooldown_update.read() {
        // Optimized: mark_dirty uses HashSet (automatic deduplication + fast lookup)
        dirty_tracker.mark_dirty(update.acting_player);

        // Future: Interest-based targeted delivery
        // let interested = interest.get_interested_players(update.acting_player);
        // for recipient in interested {
        //     // create per-recipient update
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
// NOTES
// ═════════════════════════════════════════════════════════════════════════
//
// Dirty Tracking Optimizations Applied:
// - Changed from Vec<Entity> to HashSet<Entity> for O(1) insert + automatic deduplication
// - Added helper methods: mark_dirty(), drain_dirty(), clear(), is_dirty()
// - Significantly reduces memory and CPU cost during high-frequency combat
//
// Future enhancements:
// - Add dirty component tracking (not just entities)
// - Priority queues for important vs background updates
// - Integration with InterestManager for per-player targeted replication
// - Batching of updates before sending over the network
