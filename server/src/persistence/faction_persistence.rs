/*!
 * server/src/persistence/faction_persistence.rs
 *
 * Phase 1: Basic file-based persistence for FactionStanding + FactionMembership using RON.
 * v1.0 | Simple per-player RON files for development and early testing.
 *
 * AG-SML v1.0 | TOLC 8
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::rbe::components::{FactionMembership, FactionStanding};

/// Serializable representation of a player's faction data.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PlayerFactionData {
    pub factions: Vec<FactionStandingEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FactionStandingEntry {
    pub faction_id: u64,
    pub standing: f32,
}

/// Returns the path for a player's faction save file.
pub fn get_faction_save_path(player_id: u64) -> PathBuf {
    PathBuf::from(format!("saves/players/{}/faction_data.ron", player_id))
}

/// Saves a player's faction membership and standing to RON.
pub fn save_player_faction_data(
    world: &World,
    player_entity: Entity,
    player_id: u64,
) -> Result<(), String> {
    let path = get_faction_save_path(player_id);

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut data = PlayerFactionData { factions: Vec::new() };

    // Collect all FactionMembership + FactionStanding pairs
    if let Ok((membership, standing)) = world
        .query::<(&FactionMembership, &FactionStanding)>()
        .get(world, player_entity)
    {
        data.factions.push(FactionStandingEntry {
            faction_id: membership.faction_id,
            standing: standing.standing,
        });
    } else {
        // Fallback: save whatever standing components exist (even without membership)
        for (membership, standing) in world
            .query::<(&FactionMembership, &FactionStanding)>()
            .iter(world)
        {
            if membership.faction_id == standing.faction_id {
                data.factions.push(FactionStandingEntry {
                    faction_id: membership.faction_id,
                    standing: standing.standing,
                });
            }
        }
    }

    let serialized = ron::to_string(&data)
        .map_err(|e| format!("Failed to serialize faction data: {}", e))?;

    fs::write(&path, serialized)
        .map_err(|e| format!("Failed to write faction save file: {}", e))?;

    Ok(())
}

/// Loads a player's faction data from RON and returns it.
pub fn load_player_faction_data(player_id: u64) -> Option<PlayerFactionData> {
    let path = get_faction_save_path(player_id);

    if !path.exists() {
        return None;
    }

    let content = fs::read_to_string(&path).ok()?;
    ron::from_str(&content).ok()
}

/// Applies loaded faction data to a player entity.
pub fn apply_loaded_faction_data(
    commands: &mut Commands,
    entity: Entity,
    data: &PlayerFactionData,
) {
    for entry in &data.factions {
        commands.entity(entity).insert(FactionMembership {
            faction_id: entry.faction_id,
        });
        commands.entity(entity).insert(FactionStanding {
            faction_id: entry.faction_id,
            standing: entry.standing,
        });
    }
}
