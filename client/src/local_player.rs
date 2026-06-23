/*!
 * LocalPlayer Resource + Initialization
 *
 * v19.2.9: Proper initialization of LocalPlayer with real player ID.
 */

use bevy::prelude::*;

/// Marker component for the local player entity
#[derive(Component)]
pub struct IsLocalPlayer;

/// Resource holding the local player's ID
#[derive(Resource, Default, Clone, Debug)]
pub struct LocalPlayer {
    pub id: u64,
}

/// System that initializes LocalPlayer resource when the local player spawns
pub fn initialize_local_player(
    mut commands: Commands,
    query: Query<(Entity, &PlayerId), Added<IsLocalPlayer>>,
    mut local_player: ResMut<LocalPlayer>,
) {
    for (entity, player_id) in query.iter() {
        local_player.id = player_id.0;
        info!("LocalPlayer initialized with ID: {}", player_id.0);

        // Optional: store entity reference if needed later
        commands.entity(entity).insert(LocalPlayerEntity(entity));
    }
}

/// Component to store the local player's entity (optional but useful)
#[derive(Component, Clone, Copy, Debug)]
pub struct LocalPlayerEntity(pub Entity);

/// Component holding the player's network / persistent ID
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct PlayerId(pub u64);

/// Plugin for LocalPlayer initialization
pub struct LocalPlayerPlugin;

impl Plugin for LocalPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LocalPlayer>()
            .add_systems(Update, initialize_local_player);
    }
}

// Usage:
// - When spawning the local player, insert IsLocalPlayer + PlayerId(real_id)
// - The system will automatically set LocalPlayer.id
// Thunder locked in. Yoi ⚡
