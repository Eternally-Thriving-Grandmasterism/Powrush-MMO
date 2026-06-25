/*!
 * server/src/persistence/faction_persistence.rs
 *
 * Phase 1: Faction Persistence - Observer Error Handling Exploration
 * v1.7 | Added defensive error handling to Observers.
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

// ... (previous code remains) ...

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

// ============================================================================
// Observers with Error Handling
// ============================================================================

/// Observer for player join with defensive checks.
fn on_player_joined(
    trigger: Trigger<PlayerJoined>,
    mut commands: Commands,
    world: &World,
) {
    let event = trigger.event();

    // Defensive check: Does the entity still exist?
    if world.get_entity(event.entity).is_none() {
        warn!(
            "PlayerJoined observer received event for non-existent entity {:?} (player_id: {}). Ignoring.",
            event.entity, event.player_id
        );
        return;
    }

    // Optional: Check if player already has faction data loaded (avoid duplicate loads)
    if world.get::<FactionMembership>(event.entity).is_some() {
        debug!("Player {} already has faction data loaded. Skipping reload.", event.player_id);
        return;
    }

    commands.send_event(LoadPlayerFactionData {
        player_entity: event.entity,
        player_id: event.player_id,
    });

    info!("Observer: Queued faction data load for player {}", event.player_id);
}

/// Observer for player leave with defensive checks.
fn on_player_left(
    trigger: Trigger<PlayerLeft>,
    mut commands: Commands,
    world: &World,
) {
    let event = trigger.event();

    if world.get_entity(event.entity).is_none() {
        warn!(
            "PlayerLeft observer received event for non-existent entity {:?} (player_id: {}). Ignoring.",
            event.entity, event.player_id
        );
        return;
    }

    commands.send_event(SavePlayerFactionData {
        player_entity: event.entity,
        player_id: event.player_id,
    });

    info!("Observer: Queued faction data save for player {} on leave", event.player_id);
}

// ... (rest of the file unchanged) ...

// ============================================================================
// Plugin (unchanged registration)
// ============================================================================

pub struct FactionPersistencePlugin;

impl Plugin for FactionPersistencePlugin {
    fn build(&self, app: &mut App) {
        // ... same as before ...
        .add_observer(on_player_joined)
        .add_observer(on_player_left)
        // ...
    }
}
