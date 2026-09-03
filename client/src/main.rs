//! Lived first hour — the player door.
//! WASD walk · Space jump · Shift sprint · E harvest/tend · I satchel · H hide guidance · R allocate.
//! Contact: info@Rathor.ai

use bevy::prelude::*;
use powrush_client::PowrushClientBundle;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Powrush-MMO — first hour".into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins(PowrushClientBundle)
        .add_systems(Startup, spawn_sun_and_camera)
        .run();
}

fn spawn_sun_and_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 8.0, 14.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 12_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 18.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
