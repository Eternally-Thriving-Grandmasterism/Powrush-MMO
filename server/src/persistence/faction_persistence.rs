/*!
 * server/src/persistence/faction_persistence.rs
 *
 * Phase 1: Faction Persistence - Exponential Backoff + Safer Entity Mapping
 * v2.2 | Improved retry strategy and player_id/entity handling.
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

// ============================================================================
// Configuration
// ============================================================================

const MAX_SAVE_RETRIES: u32 = 3;
const BASE_RETRY_DELAY_MS: u64 = 50;

// ============================================================================
// Exponential Backoff Retry
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
                    debug!("Save succeeded on retry attempt {} for player {}", attempt, player_id);
                }
                return Ok(());
            }
            Err(e) => {
                last_error = e;
                let delay = BASE_RETRY_DELAY_MS * (1 << (attempt - 1)); // Exponential: 50, 100, 200ms

                warn!(
                    "Save attempt {} failed for player {}. Retrying in {}ms... (Error: {})",
                    attempt, player_id, delay, last_error
                );

                if attempt < MAX_SAVE_RETRIES {
                    thread::sleep(Duration::from_millis(delay));
                }
            }
        }
    }

    Err(format!(
        "Failed to save after {} attempts for player {}. Last error: {}",
        MAX_SAVE_RETRIES, player_id, last_error
    ))
}

// ============================================================================
// Improved Error Handler with Safer Recovery
// ============================================================================

pub fn persistence_error_handler_system(
    mut errors: EventReader<PersistenceError>,
    mut force_save_events: EventWriter<ForceSavePlayerFactionData>,
    world: &World,
) {
    for error in errors.read() {
        match error.severity {
            ErrorSeverity::Warning => {
                warn!("[Persistence] {}: {}", error.context, error.message);
            }
            ErrorSeverity::Error => {
                error!("[Persistence] {}: {}", error.context, error.message);
            }
            ErrorSeverity::Critical => {
                error!("[Persistence][CRITICAL] {}: {}", error.context, error.message);

                if let Some(player_id) = error.player_id {
                    // Try to find a valid entity for this player_id.
                    // For now we use a simple heuristic. In production you'd have a PlayerIdMapping resource.
                    let entity = Entity::from_raw(player_id);

                    if world.get_entity(entity).is_some() {
                        warn!("[RECOVERY] Automatically force-saving player {} due to critical error", player_id);
                        force_save_events.send(ForceSavePlayerFactionData {
                            player_entity: entity,
                            player_id,
                        });
                    } else {
                        error!(
                            "[Persistence][CRITICAL] Cannot auto-recover player {} — entity no longer exists.",
                            player_id
                        );
                    }
                } else {
                    error!("[Persistence][CRITICAL] No player context for recovery.");
                }
            }
        }
    }
}

// ... (rest of the systems and plugin remain the same) ...
