/*!
 * server/src/persistence/faction_persistence.rs
 *
 * Phase 1: Faction Persistence - Central Error Event Bus Exploration
 * v1.8 | Added PersistenceError event bus for centralized error handling.
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

// ============================================================================
// Central Error Event Bus
// ============================================================================

#[derive(Event, Clone, Debug)]
pub struct PersistenceError {
    pub context: String,
    pub message: String,
    pub severity: ErrorSeverity,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorSeverity {
    Warning,
    Error,
    Critical,
}

// ============================================================================
// Data Structures & Events (abbreviated for clarity)
// ============================================================================

#[derive(Event, Clone, Debug)]
pub struct PlayerJoined {
    pub entity: Entity,
    pub player_id: u64,
}

#[derive(Event, Clone, Debug)]
pub struct PlayerLeft {
    pub entity: Entity,
    pub player_id: u64,
}

// ... (other events like SavePlayerFactionData, LoadPlayerFactionData remain) ...

// ============================================================================
// Observers using Central Error Bus
// ============================================================================

fn on_player_joined(
    trigger: Trigger<PlayerJoined>,
    mut commands: Commands,
    world: &World,
    mut error_writer: EventWriter<PersistenceError>,
) {
    let event = trigger.event();

    if world.get_entity(event.entity).is_none() {
        error_writer.send(PersistenceError {
            context: "on_player_joined".to_string(),
            message: format!("Entity {:?} no longer exists for player {}", event.entity, event.player_id),
            severity: ErrorSeverity::Warning,
        });
        return;
    }

    if world.get::<FactionMembership>(event.entity).is_some() {
        // Not really an error, but we can log it via the bus if desired
        return;
    }

    commands.send_event(LoadPlayerFactionData {
        player_entity: event.entity,
        player_id: event.player_id,
    });
}

fn on_player_left(
    trigger: Trigger<PlayerLeft>,
    mut commands: Commands,
    world: &World,
    mut error_writer: EventWriter<PersistenceError>,
) {
    let event = trigger.event();

    if world.get_entity(event.entity).is_none() {
        error_writer.send(PersistenceError {
            context: "on_player_left".to_string(),
            message: format!("Entity {:?} no longer exists for player {}", event.entity, event.player_id),
            severity: ErrorSeverity::Warning,
        });
        return;
    }

    commands.send_event(SavePlayerFactionData {
        player_entity: event.entity,
        player_id: event.player_id,
    });
}

// ============================================================================
// Central Error Handler System
// ============================================================================

/// Centralized system that processes all PersistenceError events.
pub fn persistence_error_handler_system(
    mut errors: EventReader<PersistenceError>,
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
                // Future: could trigger emergency save, alerts, etc.
            }
        }
    }
}

// ============================================================================
// Plugin
// ============================================================================

pub struct FactionPersistencePlugin;

impl Plugin for FactionPersistencePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PersistenceError>()
            .add_observer(on_player_joined)
            .add_observer(on_player_left)
            .add_systems(Update, persistence_error_handler_system)
            // ... other systems ...
    }
}
