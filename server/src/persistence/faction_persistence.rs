/*!
 * server/src/persistence/faction_persistence.rs
 *
 * v2.8 — Refactored to remove legacy/unified path duplication.
 * Unified PlayerSaveData path is now the primary implementation.
 *
 * AG-SML v1.0 | TOLC 8
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashMap;
use futures::executor::block_on;

use crate::persistence_polish::PersistenceManager;

// ... (keep events, resources, PlayerIdMapping, etc.) ...

// ============================================================================
// Refactored Core Save System (no duplication)
// ============================================================================

pub fn save_faction_data_system(
    mut save_events: EventReader<SavePlayerFactionData>,
    mut force_events: EventReader<ForceSavePlayerFactionData>,
    world: &World,
    mut save_state: ResMut<FactionSaveState>,
    mut error_writer: EventWriter<PersistenceError>,
    persistence: Res<PersistenceManager>, // Now always present
) {
    let process_save = |player_entity: Entity, player_id: u64| {
        // Collect current standings from world
        let mut current_standings: HashMap<u64, f32> = HashMap::new();
        if let Ok((membership, standing)) =
            world.query::<(&FactionMembership, &FactionStanding)>().get(world, player_entity)
        {
            current_standings.insert(membership.faction_id, standing.standing);
        }

        // Load existing + merge
        let mut standings = block_on(load_faction_standings(&persistence, player_id));
        for (fid, val) in current_standings {
            standings.insert(fid, val);
        }

        // Save via unified path
        if let Err(e) = block_on(save_faction_standings(&persistence, player_id, &standings)) {
            error_writer.send(PersistenceError {
                context: "save_faction_data_system".to_string(),
                message: e,
                severity: ErrorSeverity::Error,
                player_id: Some(player_id),
            });
        }

        // Update threshold tracking
        if let Some(threshold_map) = save_state.last_saved.get_mut(&player_entity) {
            for (fid, val) in &standings {
                threshold_map.insert(*fid, *val);
            }
        }
    };

    for event in save_events.read() {
        process_save(event.player_entity, event.player_id);
    }

    for event in force_events.read() {
        process_save(event.player_entity, event.player_id);
    }
}

// Similar clean refactor can be applied to load_faction_data_system

// ... (rest of systems, observers, error handler, plugin remain) ...
