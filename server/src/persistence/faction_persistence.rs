/*!
 * server/src/persistence/faction_persistence.rs
 *
 * Phase 1: Faction Persistence - Using Observers instead of event listener systems.
 * v1.6 | Replaced event listener systems with modern Bevy Observers.
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

// ... (Data structs remain the same) ...

#[derive(Event, Clone, Debug)]
pub struct SavePlayerFactionData {
    pub player_entity: Entity,
    pub player_id: u64,
}

#[derive(Event, Clone, Debug)]
pub struct LoadPlayerFactionData {
    pub player_entity: Entity,
    pub player_id: u64,
}

#[derive(Event, Clone, Debug)]
pub struct ForceSavePlayerFactionData {
    pub player_entity: Entity,
    pub player_id: u64,
}

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

// ... (Resources remain the same) ...

// ============================================================================
// Observers (replacing event listener systems)
// ============================================================================

/// Observer that loads faction data when a player joins.
fn on_player_joined(
    trigger: Trigger<PlayerJoined>,
    mut commands: Commands,
) {
    let event = trigger.event();
    commands.send_event(LoadPlayerFactionData {
        player_entity: event.entity,
        player_id: event.player_id,
    });
    info!("Observer: Loading faction data for player {}", event.player_id);
}

/// Observer that saves faction data when a player leaves.
fn on_player_left(
    trigger: Trigger<PlayerLeft>,
    mut commands: Commands,
) {
    let event = trigger.event();
    commands.send_event(SavePlayerFactionData {
        player_entity: event.entity,
        player_id: event.player_id,
    });
    info!("Observer: Saving faction data for player {} on leave", event.player_id);
}

// ... (Other systems like threshold_based_auto_save_system remain unchanged) ...

// ============================================================================
// Plugin
// ============================================================================

pub struct FactionPersistencePlugin;

impl Plugin for FactionPersistencePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<FactionAutosaveTimer>(FactionAutosaveTimer {
                timer: Timer::new(Duration::from_secs(300), TimerMode::Repeating),
            })
            .init_resource::<FactionSaveState>()
            .init_resource::<FactionSaveConfig>()
            .add_event::<SavePlayerFactionData>()
            .add_event::<LoadPlayerFactionData>()
            .add_event::<ForceSavePlayerFactionData>()
            .add_event::<PlayerJoined>()
            .add_event::<PlayerLeft>()
            // Register Observers
            .add_observer(on_player_joined)
            .add_observer(on_player_left)
            .add_systems(Update, (
                threshold_based_auto_save_system,
                periodic_faction_autosave_system,
                save_faction_data_system,
                load_faction_data_system,
            ));
    }
}

// ============================================================================
// Usage (same as before)
// ============================================================================
/*
Send these events from your player connection code:

commands.send_event(PlayerJoined { entity, player_id });
commands.send_event(PlayerLeft { entity, player_id });
*/
