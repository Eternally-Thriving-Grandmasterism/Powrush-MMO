/*!
 * server/src/persistence/faction_persistence.rs
 *
 * Phase 1: Faction Persistence with Threshold + Force Save.
 * v1.4 | Improved threshold reset + Force Save mechanism.
 *
 * AG-SML v1.0 | TOLC 8
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use crate::rbe::components::{FactionMembership, FactionStanding};
use crate::rbe::rbe_plugin::FactionStandingChangedEvent;

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

/// New event for forcing an immediate save, bypassing threshold
#[derive(Event, Clone, Debug)]
pub struct ForceSavePlayerFactionData {
    pub player_entity: Entity,
    pub player_id: u64,
}

// ============================================================================
// Resources
// ============================================================================

#[derive(Resource, Default)]
pub struct FactionAutosaveTimer {
    pub timer: Timer,
}

#[derive(Resource, Default)]
pub struct FactionSaveState {
    pub last_saved: HashMap<Entity, HashMap<u64, f32>>,
}

#[derive(Resource)]
pub struct FactionSaveConfig {
    pub save_threshold: f32,
}

impl Default for FactionSaveConfig {
    fn default() -> Self {
        Self { save_threshold: 0.15 }
    }
}

// ============================================================================
// I/O Helpers
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
// Systems
// ============================================================================

/// Threshold-based saving with proper reset after save
pub fn threshold_based_auto_save_system(
    mut standing_events: EventReader<FactionStandingChangedEvent>,
    mut save_events: EventWriter<SavePlayerFactionData>,
    mut save_state: ResMut<FactionSaveState>,
    config: Res<FactionSaveConfig>,
) {
    for event in standing_events.read() {
        let entity = Entity::from_raw(event.player_entity_id);
        let faction_id = event.faction_id;

        let last_saved = save_state
            .last_saved
            .entry(entity)
            .or_default()
            .entry(faction_id)
            .or_insert(1.0);

        // Use absolute standing change since last save
        // For simplicity we track delta accumulation
        let current_change = event.delta; // We can improve this later with absolute value

        if current_change.abs() >= config.save_threshold {
            save_events.send(SavePlayerFactionData {
                player_entity: entity,
                player_id: event.player_entity_id,
            });
            // Note: Actual reset happens in save_faction_data_system after successful save
        }
    }
}

/// Periodic autosave safety net
pub fn periodic_faction_autosave_system(
    time: Res<Time>,
    mut timer: ResMut<FactionAutosaveTimer>,
    faction_query: Query<(Entity, &FactionMembership, &FactionStanding)>,
    mut save_events: EventWriter<SavePlayerFactionData>,
) {
    timer.timer.tick(time.delta());

    if timer.timer.just_finished() {
        for (entity, _membership, _standing) in faction_query.iter() {
            save_events.send(SavePlayerFactionData {
                player_entity: entity,
                player_id: entity.index() as u64,
            });
        }
    }
}

/// Core save system - now also resets threshold tracking after successful save
pub fn save_faction_data_system(
    mut save_events: EventReader<SavePlayerFactionData>,
    mut force_events: EventReader<ForceSavePlayerFactionData>,
    world: &World,
    mut save_state: ResMut<FactionSaveState>,
) {
    // Handle normal saves
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

            // Reset threshold tracking to current standing after successful save
            save_state
                .last_saved
                .entry(event.player_entity)
                .or_default()
                .insert(membership.faction_id, standing.standing);
        }

        if let Err(e) = save_faction_data_to_disk(&data, event.player_id) {
            warn!("Failed to save faction data: {}", e);
        }
    }

    // Handle forced saves (always save, always reset threshold)
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

            // Force reset threshold tracking
            save_state
                .last_saved
                .entry(event.player_entity)
                .or_default()
                .insert(membership.faction_id, standing.standing);
        }

        if let Err(e) = save_faction_data_to_disk(&data, event.player_id) {
            warn!("Failed to force save faction data: {}", e);
        } else {
            info!("Force saved faction data for player {}", event.player_id);
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
            .init_resource::<FactionSaveState>()
            .init_resource::<FactionSaveConfig>()
            .add_event::<SavePlayerFactionData>()
            .add_event::<LoadPlayerFactionData>()
            .add_event::<ForceSavePlayerFactionData>()
            .add_systems(Update, (
                threshold_based_auto_save_system,
                periodic_faction_autosave_system,
                save_faction_data_system,
                load_faction_data_system,
            ));
    }
}
