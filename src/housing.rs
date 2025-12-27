use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Replicated)]
pub struct PlayerHome {
    pub location: Vec3,
    pub style: HomeStyle,
    pub trust_bonus: f32,
    pub mercy_shield: f32,
}

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum HomeStyle {
    Treehouse,      // +10% trust regen
    Cave,           // +20% defense
    LatticeTower,   // +15% lattice connection
    FloatingIsland, // +25% exploration speed
}

pub struct HousingPlugin;

impl Plugin for HousingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (housing_spawn_system, housing_bonus_system));
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
                mercy_shield: 100.0,
            });
            info!("Home built — {} at {:?}", style as u8, transform.translation);
        }
    }
}

fn housing_bonus_system(
    mut trust: Query<&mut TrustCredits>,
    homes: Query<&PlayerHome>,
) {
    for home in &homes {
        if let Ok(mut player_trust) = trust.get_mut(home.entity()) {
            player_trust.0 *= home.trust_bonus;
        }
    }
}
