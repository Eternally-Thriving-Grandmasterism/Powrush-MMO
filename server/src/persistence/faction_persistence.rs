/*!
 * server/src/persistence/faction_persistence.rs
 *
 * v2.4 — Refactored maintain_mapping_on_join for robustness and clarity.
 *
 * AG-SML v1.0 | TOLC 8
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashMap;

// ... (rest of file) ...

// ============================================================================
// Improved Mapping Maintenance
// ============================================================================

/// Maintains the PlayerIdMapping when a player joins.
/// - Idempotent: does nothing if player_id is already mapped.
/// - Defensive: skips if the entity no longer exists.
/// - Observable: logs important state changes.
fn maintain_mapping_on_join(
    mut events: EventReader<PlayerJoined>,
    mut mapping: ResMut<PlayerIdMapping>,
    world: &World,
) {
    for event in events.read() {
        // Skip if already registered (idempotent)
        if mapping.contains(event.player_id) {
            debug!(
                "PlayerIdMapping: player {} already mapped to {:?}. Skipping duplicate join.",
                event.player_id, mapping.get_entity(event.player_id)
            );
            continue;
        }

        // Defensive check: entity must still exist
        if world.get_entity(event.entity).is_none() {
            warn!(
                "PlayerIdMapping: received PlayerJoined for player {} but entity {:?} no longer exists. Skipping.",
                event.player_id, event.entity
            );
            continue;
        }

        mapping.insert(event.player_id, event.entity);
        debug!("PlayerIdMapping: registered {} → {:?}", event.player_id, event.entity);
    }
}

fn maintain_mapping_on_leave(
    mut events: EventReader<PlayerLeft>,
    mut mapping: ResMut<PlayerIdMapping>,
) {
    for event in events.read() {
        if mapping.remove_by_id(event.player_id).is_some() {
            debug!("PlayerIdMapping: removed player {}", event.player_id);
        }
    }
}

// ... (rest of the file unchanged) ...
