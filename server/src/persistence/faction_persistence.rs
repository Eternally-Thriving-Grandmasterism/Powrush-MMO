/*!
 * server/src/persistence/faction_persistence.rs
 *
 * Phase 1: Faction Persistence - Automatic Force Save on Critical Errors
 * v2.0 | Critical errors now automatically trigger ForceSavePlayerFactionData.
 *
 * AG-SML v1.0 | TOLC 8
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use crate::rbe::components::{FactionMembership, FactionStanding};
use crate::rbe::rbe_plugin::FactionStandingChangedEvent;

// ... (previous code) ...

#[derive(Event, Clone, Debug)]
pub struct PersistenceError {
    pub context: String,
    pub message: String,
    pub severity: ErrorSeverity,
    pub player_id: Option<u64>,
}

// ... (other events including ForceSavePlayerFactionData) ...

// ============================================================================
// Central Error Handler with Automatic Force Save Recovery
// ============================================================================

pub fn persistence_error_handler_system(
    mut errors: EventReader<PersistenceError>,
    mut force_save_events: EventWriter<ForceSavePlayerFactionData>,
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
                    warn!("[Persistence][RECOVERY] Automatically triggering force save for player {} due to critical error", player_id);

                    // Automatic recovery: Force an immediate save, bypassing normal thresholds
                    force_save_events.send(ForceSavePlayerFactionData {
                        player_entity: Entity::from_raw(player_id), // Note: This assumes player_id == entity index for now
                        player_id,
                    });
                } else {
                    error!("[Persistence][CRITICAL] No player_id provided. Cannot auto-recover. Manual intervention required.");
                }
            }
        }
    }
}

// ... (rest of systems and plugin) ...
