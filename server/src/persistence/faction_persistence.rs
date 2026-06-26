/*!
 * server/src/persistence/faction_persistence.rs
 *
 * v2.6 — Core save/load systems now use unified PlayerSaveData via PersistenceManager.
 *
 * AG-SML v1.0 | TOLC 8
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use futures::executor::block_on;

use crate::persistence_polish::PersistenceManager;
use crate::persistence::faction_persistence::{load_faction_standings, save_faction_standings};

// ============================================================================
// Updated Core Systems using Unified PlayerSaveData
// ============================================================================

pub fn save_faction_data_system(
    mut save_events: EventReader<SavePlayerFactionData>,
    mut force_events: EventReader<ForceSavePlayerFactionData>,
    world: &World,
    mut save_state: ResMut<FactionSaveState>,
    mut error_writer: EventWriter<PersistenceError>,
    persistence: Option<Res<PersistenceManager>>, // NEW: unified path
) {
    for event in save_events.read() {
        // ... build data as before ...

        if let Some(persistence) = &persistence {
            // === Unified path via PlayerSaveData ===
            let mut standings = block_on(load_faction_standings(persistence, event.player_id));
            // Update with current data (simplified for now)
            if let Ok((membership, standing)) = world
                .query::<(&FactionMembership, &FactionStanding)>()
                .get(world, event.player_entity)
            {
                standings.insert(membership.faction_id, standing.standing);
            }

            if let Err(e) = block_on(save_faction_standings(persistence, event.player_id, &standings)) {
                error_writer.send(PersistenceError {
                    context: "save_faction_data_system (unified)".to_string(),
                    message: e,
                    severity: ErrorSeverity::Error,
                    player_id: Some(event.player_id),
                });
            }
        } else {
            // === Fallback to old file-based path ===
            if let Err(e) = save_faction_data_to_disk_with_retry(&data, event.player_id) {
                error_writer.send(PersistenceError { ... });
            }
        }

        // Update threshold state...
    }

    // Similar logic for force_events...
}

// Similar update can be applied to load_faction_data_system

// ... (rest of file) ...
