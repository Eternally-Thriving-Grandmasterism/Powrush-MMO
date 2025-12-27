use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct PlayerHome {
    pub style: HomeStyle,
    pub trust_bonus: f32,
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
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
) {
    if keyboard.just_pressed(KeyCode::H) {
        for player in &players {
            let style = match rand::thread_rng().gen_range(0..4) {
                0 => HomeStyle::Treehouse,
                1 => HomeStyle::Cave,
                2 => HomeStyle::LatticeTower,
                _ => HomeStyle::FloatingIsland,
            };
            commands.entity(player).insert(PlayerHome {
                style,
                trust_bonus: match style {
                    HomeStyle::Treehouse => 1.1,
                    HomeStyle::Cave => 1.0,
                    HomeStyle::LatticeTower => 1.15,
                    HomeStyle::FloatingIsland => 1.25,
                },
            });
            info!("Home built — {:?}", style);
        }
    }
}
