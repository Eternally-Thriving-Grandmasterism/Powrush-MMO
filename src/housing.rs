use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerHome {
    pub location: Vec3,
    pub style: HomeStyle,
}

#[derive(Clone, Copy)]
pub enum HomeStyle {
    Treehouse,
    Cave,
    LatticeTower,
    FloatingIsland,
}

pub struct HousingPlugin;

impl Plugin for HousingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, housing_system);
    }
}

fn housing_system(
    mut commands: Commands,
    players: Query<(Entity, &Transform), With<Player>>,
) {
    for (entity, transform) in &players {
        // Simple: press H to spawn home
        if /* key press H */ false {
            commands.spawn((
                PlayerHome {
                    location: transform.translation,
                    style: HomeStyle::LatticeTower,
                },
                // Procedural home mesh + particles
            ));
            info!("Home built — mercy shelter");
        }
    }
}
