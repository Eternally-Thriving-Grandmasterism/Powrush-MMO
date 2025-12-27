use bevy::prelude::*;
use bevy_replicon::prelude::*;
use rand::Rng;

#[derive(Component, Replicated)]
pub struct PlayerHome {
    pub location: Vec3,
    pub style: HomeStyle,
    pub trust_bonus: f32,
    pub built: bool,
}

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum HomeStyle {
    Treehouse,
    Cave,
    LatticeTower,
    FloatingIsland,
}

pub struct HousingPlugin;

impl Plugin for HousingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, housing_spawn_system);
    }
}

fn housing_spawn_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    players: Query<(Entity, &Transform), With<Player>>,
) {
    if keyboard.just_pressed(KeyCode::H) {
        let mut rng = rand::thread_rng();
        for (entity, transform) in &players {
            let style = match rng.gen_range(0..4) {
                0 => HomeStyle::Treehouse,
                1 => HomeStyle::Cave,
                2 => HomeStyle::LatticeTower,
                _ => HomeStyle::FloatingIsland,
            };
            commands.entity(entity).insert(PlayerHome {
                location: transform.translation,
                style,
                trust_bonus: match style {
                    HomeStyle::Treehouse => 1.1,
                    HomeStyle::Cave => 1.0,
                    HomeStyle::LatticeTower => 1.15,
                    HomeStyle::FloatingIsland => 1.25,
                },
                built: true,
            });
            info!("Home built — {:?}", style);
        }
    }
}
