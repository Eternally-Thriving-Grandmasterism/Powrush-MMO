use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct PlayerLevel {
    pub level: u32,
    pub experience: f32,
    pub exp_to_next: f32,
}

pub struct LevelingPlugin;

impl Plugin for LevelingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, leveling_system);
    }
}

fn leveling_system(
    mut query: Query<&mut PlayerLevel>,
    time: Res<Time>,
) {
    for mut level in &mut query {
        level.experience += time.delta_seconds() * 10.0;  // Mercy exp gain
        if level.experience >= level.exp_to_next {
            level.level += 1;
            level.experience = 0.0;
            level.exp_to_next *= 1.5;  // Exponential
            info!("Level up — now level {}", level.level);
        }
    }
}
