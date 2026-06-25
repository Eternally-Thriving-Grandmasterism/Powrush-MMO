/*!
 * server/src/persistence/faction_persistence.rs
 *
 * v2.3 — Added PlayerIdMapping resource for robust player_id ↔ Entity mapping.
 *
 * AG-SML v1.0 | TOLC 8
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashMap;

// ... (keep existing imports and structs) ...

// ============================================================================
// PlayerIdMapping Resource
// ============================================================================

/// Safe bidirectional mapping between player_id and Bevy Entity.
/// Replaces fragile Entity::from_raw usage throughout the persistence layer.
#[derive(Resource, Default)]
pub struct PlayerIdMapping {
    id_to_entity: HashMap<u64, Entity>,
    entity_to_id: HashMap<Entity, u64>,
}

impl PlayerIdMapping {
    pub fn insert(&mut self, player_id: u64, entity: Entity) {
        self.id_to_entity.insert(player_id, entity);
        self.entity_to_id.insert(entity, player_id);
    }

    pub fn get_entity(&self, player_id: u64) -> Option<Entity> {
        self.id_to_entity.get(&player_id).copied()
    }

    pub fn get_player_id(&self, entity: Entity) -> Option<u64> {
        self.entity_to_id.get(&entity).copied()
    }

    pub fn remove_by_id(&mut self, player_id: u64) -> Option<Entity> {
        if let Some(entity) = self.id_to_entity.remove(&player_id) {
            self.entity_to_id.remove(&entity);
            Some(entity)
        } else {
            None
        }
    }

    pub fn contains(&self, player_id: u64) -> bool {
        self.id_to_entity.contains_key(&player_id)
    }
}

// ============================================================================
// Mapping Maintenance Systems
// ============================================================================

fn maintain_mapping_on_join(
    mut events: EventReader<PlayerJoined>,
    mut mapping: ResMut<PlayerIdMapping>,
) {
    for event in events.read() {
        mapping.insert(event.player_id, event.entity);
    }
}

fn maintain_mapping_on_leave(
    mut events: EventReader<PlayerLeft>,
    mut mapping: ResMut<PlayerIdMapping>,
) {
    for event in events.read() {
        mapping.remove_by_id(event.player_id);
    }
}

// ============================================================================
// Updated Critical Error Recovery using Mapping
// ============================================================================

pub fn persistence_error_handler_system(
    mut errors: EventReader<PersistenceError>,
    mut force_save_events: EventWriter<ForceSavePlayerFactionData>,
    mapping: Res<PlayerIdMapping>,
) {
    for error in errors.read() {
        if error.severity != ErrorSeverity::Critical { continue; }

        error!("[Persistence][CRITICAL] {}: {}", error.context, error.message);

        if let Some(player_id) = error.player_id {
            if let Some(entity) = mapping.get_entity(player_id) {
                warn!("[RECOVERY] Auto force-saving player {} via mapping", player_id);
                force_save_events.send(ForceSavePlayerFactionData {
                    player_entity: entity,
                    player_id,
                });
            } else {
                error!("[Persistence][CRITICAL] Player {} has no valid entity in mapping.", player_id);
            }
        }
    }
}

// ... (rest of file unchanged) ...

// Update Plugin to include mapping resource and systems
impl Plugin for FactionPersistencePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerIdMapping>()
            .add_systems(Update, (maintain_mapping_on_join, maintain_mapping_on_leave))
            // ... existing observers and systems ...
    }
}
