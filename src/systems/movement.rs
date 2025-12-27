use bevy::prelude::*;

pub fn player_movement_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::W) { direction.z -= 1.0; }
    if keyboard.pressed(KeyCode::S) { direction.z += 1.0; }
    if keyboard.pressed(KeyCode::A) { direction.x -= 1.0; }
    if keyboard.pressed(KeyCode::D) { direction.x += 1.0; }

    let speed = 5.0;
    for mut transform in &mut query {
        transform.translation += direction.normalize_or_zero() * speed * time.delta_seconds();
    }
}
