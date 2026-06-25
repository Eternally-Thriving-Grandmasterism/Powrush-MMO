/*!
 * server/src/persistence/faction_persistence.rs
 *
 * Phase 1: Faction Persistence - Player Lifecycle Integration
 * v1.5 | Added systems to hook into player join/leave.
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

// ... (previous structs and events remain the same) ...

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

// New events for player lifecycle integration
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

// ... (resources remain the same) ...

// ============================================================================
// Player Lifecycle Integration Systems
// ============================================================================

/// Automatically loads faction data when a player joins.
pub fn load_faction_data_on_player_join_system(
    mut join_events: EventReader<PlayerJoined>,
    mut load_events: EventWriter<LoadPlayerFactionData>,
) {
    for event in join_events.read() {
        load_events.send(LoadPlayerFactionData {
            player_entity: event.entity,
            player_id: event.player_id,
        });
        info!("Triggered faction data load for player {}", event.player_id);
    }
}

/// Automatically saves faction data when a player leaves.
pub fn save_faction_data_on_player_leave_system(
    mut leave_events: EventReader<PlayerLeft>,
    mut save_events: EventWriter<SavePlayerFactionData>,
) {
    for event in leave_events.read() {
        save_events.send(SavePlayerFactionData {
            player_entity: event.entity,
            player_id: event.player_id,
        });
        info!("Triggered faction data save for player {} on disconnect", event.player_id);
    }
}

// ... (rest of the systems remain the same) ...

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
            .add_systems(Update, (
                threshold_based_auto_save_system,
                periodic_faction_autosave_system,
                save_faction_data_system,
                load_faction_data_system,
                load_faction_data_on_player_join_system,
                save_faction_data_on_player_leave_system,
            ));
    }
}

// ============================================================================
// Usage Instructions
// ============================================================================
/*
HOW TO INTEGRATE WITH YOUR PLAYER LIFECYCLE:

1. When a player successfully connects and their entity is spawned:
   commands.send_event(PlayerJoined {
       entity: player_entity,
       player_id: player_account_id,
   });

2. When a player disconnects or their entity is about to be despawned:
   commands.send_event(PlayerLeft {
       entity: player_entity,
       player_id: player_account_id,
   });

This will automatically trigger load on join and save on leave.
*/
