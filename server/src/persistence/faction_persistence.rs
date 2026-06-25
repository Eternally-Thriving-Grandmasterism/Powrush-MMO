/*!
 * server/src/persistence/faction_persistence.rs
 *
 * Phase 1: ECS-native Faction Persistence (RON file-based).
 * v1.1 | Refactored to proper Bevy systems + events.
 *
 * AG-SML v1.0 | TOLC 8
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::rbe::components::{FactionMembership, FactionStanding};

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PlayerFactionData {
    pub factions: Vec<FactionStandingEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FactionStandingEntry {
    pub faction_id: u64,
    pub standing: f32,
}

// ============================================================================
// Events
// ============================================================================

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

// ============================================================================
// Helper Functions (pure I/O)
// ============================================================================

pub fn get_faction_save_path(player_id: u64) -> PathBuf {
    PathBuf::from(format!("saves/players/{}/faction_data.ron", player_id))
}

pub fn save_faction_data_to_disk(data: &PlayerFactionData, player_id: u64) -> Result<(), String> {
    let path = get_faction_save_path(player_id);

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let serialized = ron::to_string(data)
        .map_err(|e| format!("RON serialization failed: {}", e))?;

    fs::write(&path, serialized)
        .map_err(|e| format!("Failed to write file: {}", e))
}

pub fn load_faction_data_from_disk(player_id: u64) -> Option<PlayerFactionData> {
    let path = get_faction_save_path(player_id);
    if !path.exists() { return None; }

    let content = fs::read_to_string(path).ok()?;
    ron::from_str(&content).ok()
}

// ============================================================================
// ECS Systems
// ============================================================================

/// System that saves faction data when a SavePlayerFactionData event is received.
pub fn save_faction_data_system(
    mut events: EventReader<SavePlayerFactionData>,
    world: &World,
    mut commands: Commands,
) {
    for event in events.read() {
        let mut data = PlayerFactionData { factions: Vec::new() };

        // Query the specific player
        if let Ok((membership, standing)) = world
            .query::<(&FactionMembership, &FactionStanding)>()
            .get(world, event.player_entity)
        {
            data.factions.push(FactionStandingEntry {
                faction_id: membership.faction_id,
                standing: standing.standing,
            });
        }

        if let Err(e) = save_faction_data_to_disk(&data, event.player_id) {
            warn!("Failed to save faction data for player {}: {}", event.player_id, e);
        } else {
            debug!("Saved faction data for player {}", event.player_id);
        }
    }
}

/// System that loads faction data when a LoadPlayerFactionData event is received.
pub fn load_faction_data_system(
    mut events: EventReader<LoadPlayerFactionData>,
    mut commands: Commands,
) {
    for event in events.read() {
        if let Some(data) = load_faction_data_from_disk(event.player_id) {
            for entry in &data.factions {
                commands.entity(event.player_entity).insert(FactionMembership {
                    faction_id: entry.faction_id,
                });
                commands.entity(event.player_entity).insert(FactionStanding {
                    faction_id: entry.faction_id,
                    standing: entry.standing,
                });
            }
            info!("Loaded faction data for player {}", event.player_id);
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
            .add_event::<SavePlayerFactionData>()
            .add_event::<LoadPlayerFactionData>()
            .add_systems(Update, (
                save_faction_data_system,
                load_faction_data_system,
            ));
    }
}
