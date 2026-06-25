/*!
 * server/src/persistence/faction_persistence.rs
 *
 * Phase 1: ECS-native Faction Persistence with Automatic Triggers.
 * v1.2 | Added reactive + periodic autosave triggers.
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
// Data Structures & Events
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
// Resource for Autosave Timing
// ============================================================================

#[derive(Resource, Default)]
pub struct FactionAutosaveTimer {
    pub timer: Timer,
}

// ============================================================================
// Helper I/O Functions
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
// Automatic Save Triggers
// ============================================================================

/// Reactively saves when a meaningful standing change occurs.
pub fn auto_save_on_standing_change_system(
    mut standing_events: EventReader<FactionStandingChangedEvent>,
    mut save_events: EventWriter<SavePlayerFactionData>,
) {
    for event in standing_events.read() {
        // Only auto-save on meaningful increases (e.g. +0.05 or more)
        if event.delta >= 0.05 {
            save_events.send(SavePlayerFactionData {
                player_entity: Entity::from_raw(event.player_entity_id),
                player_id: event.player_entity_id, // Using entity id as player_id for now
            });
        }
    }
}

/// Periodic autosave for all players with faction data (every 5 minutes by default).
pub fn periodic_faction_autosave_system(
    time: Res<Time>,
    mut timer: ResMut<FactionAutosaveTimer>,
    faction_query: Query<(Entity, &FactionMembership, &FactionStanding)>,
    mut save_events: EventWriter<SavePlayerFactionData>,
) {
    timer.timer.tick(time.delta());

    if timer.timer.just_finished() {
        for (entity, membership, _standing) in faction_query.iter() {
            save_events.send(SavePlayerFactionData {
                player_entity: entity,
                player_id: entity.index() as u64, // placeholder until real player IDs exist
            });
        }
        debug!("Triggered periodic faction autosave for all players with data");
    }
}

// ============================================================================
// Core Save/Load Systems
// ============================================================================

pub fn save_faction_data_system(
    mut events: EventReader<SavePlayerFactionData>,
    world: &World,
) {
    for event in events.read() {
        let mut data = PlayerFactionData { factions: Vec::new() };

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
            warn!("Failed to save faction data: {}", e);
        }
    }
}

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
            .init_resource::<FactionAutosaveTimer>(FactionAutosaveTimer {
                timer: Timer::new(Duration::from_secs(300), TimerMode::Repeating),
            })
            .add_event::<SavePlayerFactionData>()
            .add_event::<LoadPlayerFactionData>()
            .add_systems(Update, (
                auto_save_on_standing_change_system,
                periodic_faction_autosave_system,
                save_faction_data_system,
                load_faction_data_system,
            ));
    }
}
