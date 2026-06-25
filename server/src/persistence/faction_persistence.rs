/*!
 * server/src/persistence/faction_persistence.rs
 *
 * Phase 1: Faction Persistence - Retry Logic for Failed Saves
 * v2.1 | Added automatic retry with backoff for disk write failures.
 *
 * AG-SML v1.0 | TOLC 8
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use crate::rbe::components::{FactionMembership, FactionStanding};
use crate::rbe::rbe_plugin::FactionStandingChangedEvent;

// ... (previous structs and events) ...

// ============================================================================
// Retry Configuration
// ============================================================================

const MAX_SAVE_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 50;

// ============================================================================
// Retry-Enabled Save Function
// ============================================================================

pub fn save_faction_data_to_disk_with_retry(
    data: &PlayerFactionData,
    player_id: u64,
) -> Result<(), String> {
    let mut last_error = String::new();

    for attempt in 1..=MAX_SAVE_RETRIES {
        match save_faction_data_to_disk(data, player_id) {
            Ok(()) => {
                if attempt > 1 {
                    debug!("Save succeeded on attempt {} for player {}", attempt, player_id);
                }
                return Ok(());
            }
            Err(e) => {
                last_error = e;
                warn!(
                    "Save attempt {} failed for player {}: {}. Retrying...",
                    attempt, player_id, last_error
                );

                if attempt < MAX_SAVE_RETRIES {
                    thread::sleep(Duration::from_millis(RETRY_DELAY_MS));
                }
            }
        }
    }

    Err(format!(
        "Failed to save faction data for player {} after {} attempts. Last error: {}",
        player_id, MAX_SAVE_RETRIES, last_error
    ))
}

// ============================================================================
// Updated Save System with Retry + Error Bus Integration
// ============================================================================

pub fn save_faction_data_system(
    mut save_events: EventReader<SavePlayerFactionData>,
    mut force_events: EventReader<ForceSavePlayerFactionData>,
    world: &World,
    mut save_state: ResMut<FactionSaveState>,
    mut error_writer: EventWriter<PersistenceError>,
) {
    // Handle normal + threshold-based saves
    for event in save_events.read() {
        let mut data = PlayerFactionData { factions: Vec::new() };

        if let Ok((membership, standing)) = world
            .query::<(&FactionMembership, &FactionStanding)>()
            .get(world, event.player_entity)
        {
            data.factions.push(FactionStandingEntry {
                faction_id: membership.faction_id,
                standing: standing.standing,
            });

            save_state
                .last_saved
                .entry(event.player_entity)
                .or_default()
                .insert(membership.faction_id, standing.standing);
        }

        if let Err(e) = save_faction_data_to_disk_with_retry(&data, event.player_id) {
            error_writer.send(PersistenceError {
                context: "save_faction_data_system".to_string(),
                message: e,
                severity: ErrorSeverity::Error,
                player_id: Some(event.player_id),
            });
        }
    }

    // Handle forced saves (also with retry)
    for event in force_events.read() {
        let mut data = PlayerFactionData { factions: Vec::new() };

        if let Ok((membership, standing)) = world
            .query::<(&FactionMembership, &FactionStanding)>()
            .get(world, event.player_entity)
        {
            data.factions.push(FactionStandingEntry {
                faction_id: membership.faction_id,
                standing: standing.standing,
            });

            save_state
                .last_saved
                .entry(event.player_entity)
                .or_default()
                .insert(membership.faction_id, standing.standing);
        }

        if let Err(e) = save_faction_data_to_disk_with_retry(&data, event.player_id) {
            error_writer.send(PersistenceError {
                context: "save_faction_data_system (forced)".to_string(),
                message: format!("Force save failed after retries: {}", e),
                severity: ErrorSeverity::Critical,
                player_id: Some(event.player_id),
            });
        } else {
            info!("Force save completed successfully for player {}", event.player_id);
        }
    }
}

// ... (rest of the file including observers, error handler, and plugin) ...
