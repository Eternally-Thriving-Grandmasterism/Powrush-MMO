use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement_system);
    }
}

fn player_movement_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::W) { direction.y += 1.0; }
    if keyboard.pressed(KeyCode::S) { direction.y -= 1.0; }
    if keyboard.pressed(KeyCode::A) { direction.x -= 1.0; }
    if keyboard.pressed(KeyCode::D) { direction.x += 1.0; }

    for mut transform in &mut query {
        transform.translation += direction.normalize_or_zero() * 5.0 * time.delta_seconds();
    }
}
