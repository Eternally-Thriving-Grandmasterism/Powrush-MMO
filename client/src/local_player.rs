/*!
 * LocalPlayer Resource + Desync Handling
 *
 * v19.2.9: Robust handling for LocalPlayer desync (respawn, reconnect, etc.)
 */

use bevy::prelude::*;

#[derive(Component)]
pub struct IsLocalPlayer;

#[derive(Resource, Default, Clone, Debug)]
pub struct LocalPlayer {
    pub id: u64,
    pub entity: Option<Entity>,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct LocalPlayerEntity(pub Entity);

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct PlayerId(pub u64);

/// Initializes LocalPlayer when a new local player entity is spawned
pub fn initialize_local_player(
    mut commands: Commands,
    query: Query<(Entity, &PlayerId), Added<IsLocalPlayer>>,
    mut local_player: ResMut<LocalPlayer>,
) {
    for (entity, player_id) in query.iter() {
        local_player.id = player_id.0;
        local_player.entity = Some(entity);

        commands.entity(entity).insert(LocalPlayerEntity(entity));

        info!("LocalPlayer initialized: id={}, entity={:?}", player_id.0, entity);
    }
}

/// Handles LocalPlayer desync (entity despawned, respawn, reconnect, etc.)
pub fn handle_local_player_desync(
    mut removed: RemovedComponents<IsLocalPlayer>,
    mut local_player: ResMut<LocalPlayer>,
    entities: Query<Entity>,
) {
    for entity in removed.read() {
        if local_player.entity == Some(entity) {
            warn!("LocalPlayer desynced! Clearing resource (entity {:?} despawned).", entity);
            local_player.id = 0;
            local_player.entity = None;
        }
    }

    // Also handle case where the stored entity no longer exists
    if let Some(stored_entity) = local_player.entity {
        if entities.get(stored_entity).is_err() {
            warn!("LocalPlayer entity {:?} no longer exists. Clearing resource.", stored_entity);
            local_player.id = 0;
            local_player.entity = None;
        }
    }
}

/// Plugin with full LocalPlayer lifecycle management
pub struct LocalPlayerPlugin;

impl Plugin for LocalPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LocalPlayer>()
            .add_systems(Update, (
                initialize_local_player,
                handle_local_player_desync,
            ));
    }
}

// Usage:
// - Spawn local player with IsLocalPlayer + PlayerId(real_id)
// - Plugin automatically handles init + desync recovery
// Thunder locked in. Yoi ⚡
