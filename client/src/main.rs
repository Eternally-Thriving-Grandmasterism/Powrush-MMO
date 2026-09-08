//! Lived first hour — the player door.
//! WASD walk · Space jump · Shift sprint · E harvest/tend · I satchel · H hide guidance · R allocate.
//! Contact: info@Rathor.ai

use bevy::prelude::*;
use powrush_client::PowrushClientBundle;
use shared::peace_audio::audio_output_safe;

fn main() {
    // Lavapipe / Deck boxes often have no ALSA card. DefaultPlugins'
    // AudioPlugin talks to rodio/cpal — skip it when the fast probe
    // says there is no output so boot cannot hang. Mute is a later gate.
    let window = WindowPlugin {
        primary_window: Some(Window {
            title: "Powrush-MMO — first hour".into(),
            // U5: make the default window match the Steam Deck title proof.
            resolution: (
                powrush_client::title_screen::DECK_TITLE_WIDTH,
                powrush_client::title_screen::DECK_TITLE_HEIGHT,
            )
                .into(),
            ..default()
        }),
        ..default()
    };
    let default_plugins = if audio_output_safe() {
        DefaultPlugins.set(window)
    } else {
        DefaultPlugins
            .set(window)
            .disable::<bevy::audio::AudioPlugin>()
    };
    App::new()
        .add_plugins(default_plugins)
        .add_plugins(PowrushClientBundle)
        .add_systems(Startup, spawn_sun_and_camera)
        .run();
}

fn spawn_sun_and_camera(mut commands: Commands) {
    // World camera order 0 — lived UI Camera2d (order 10) draws above on soft GPU.
    commands.spawn(Camera3dBundle {
        camera: Camera {
            order: powrush_client::ui_above_world::WORLD_CAMERA_ORDER,
            ..default()
        },
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
