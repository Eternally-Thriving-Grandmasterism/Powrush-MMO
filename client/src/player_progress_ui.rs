/*!
 * Player Progress UI v18.10
 *
 * Reactive panel showing epiphany progress, muscle memory,
 * and active temporary multipliers from epiphanies.
 */

use bevy::prelude::*;
use simulation::player_persistence::PlayerSaveData;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Component)]
struct PlayerProgressPanel;

#[derive(Component)]
struct EpiphanyCountText;

#[derive(Component)]
struct MuscleMemoryText;

#[derive(Component)]
struct ActiveMultiplierText;

pub struct PlayerProgressUIPlugin;

impl Plugin for PlayerProgressUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player_progress_ui)
            .add_systems(Update, (
                toggle_player_progress_ui,
                update_player_progress_ui,
            ));
    }
}

fn spawn_player_progress_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(8.0),
                    right: Val::Percent(3.0),
                    width: Val::Px(300.0),
                    padding: UiRect::all(Val::Px(18.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(14.0)),
                    flex_direction: FlexDirection::Column,
                    visibility: Visibility::Hidden,
                    ..default()
                },
                background_color: Color::srgba(0.06, 0.08, 0.12, 0.96).into(),
                border_color: Color::srgb(0.4, 0.75, 1.0).into(),
                ..default()
            },
            PlayerProgressPanel,
            Name::new("PlayerProgressPanel"),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "PROGRESS",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 18.0,
                        color: Color::srgb(0.6, 0.85, 1.0),
                    },
                ),
                style: Style {
                    margin: UiRect::bottom(Val::Px(14.0)),
                    ..default()
                },
                ..default()
            });

            // Epiphany Count
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Epiphanies: 0",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                },
                EpiphanyCountText,
            ));

            // Muscle Memory
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Muscle Memory: 1.00x",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    ),
                    style: Style {
                        margin: UiRect::top(Val::Px(8.0)),
                        ..default()
                    },
                    ..default()
                },
                MuscleMemoryText,
            ));

            // Active Temporary Multiplier
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Multiplier: 1.00x (inactive)",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 15.0,
                            color: Color::srgb(1.0, 0.9, 0.5),
                        },
                    ),
                    style: Style {
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                },
                ActiveMultiplierText,
            ));
        });
}

fn toggle_player_progress_ui(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<PlayerProgressPanel>>,
) {
    if keyboard.just_pressed(KeyCode::F2) {
        for mut visibility in query.iter_mut() {
            *visibility = if *visibility == Visibility::Hidden {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

fn update_player_progress_ui(
    save_data: Res<PlayerSaveData>,
    mut epiphany_text: Query<&mut Text, With<EpiphanyCountText>>,
    mut muscle_text: Query<&mut Text, With<MuscleMemoryText>>,
    mut multiplier_text: Query<&mut Text, With<ActiveMultiplierText>>,
) {
    // Update Epiphany Count
    for mut text in epiphany_text.iter_mut() {
        text.sections[0].value = format!("Epiphanies: {}", save_data.epiphanies.len());
    }

    // Update Muscle Memory
    for mut text in muscle_text.iter_mut() {
        text.sections[0].value = format!("Muscle Memory: {:.2}x", save_data.muscle_memory_level);
    }

    // Update Active Temporary Multiplier
    for mut text in multiplier_text.iter_mut() {
        if save_data.has_active_multiplier() {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let remaining = if save_data.temporary_multiplier_expires_at > now {
                save_data.temporary_multiplier_expires_at - now
            } else {
                0
            };

            let minutes = remaining / 60;
            let seconds = remaining % 60;

            text.sections[0].value = format!(
                "Multiplier: {:.2}x ({}m {}s)",
                save_data.temporary_harvest_multiplier,
                minutes,
                seconds
            );
            text.sections[0].style.color = Color::srgb(1.0, 0.85, 0.4);
        } else {
            text.sections[0].value = "Multiplier: 1.00x (inactive)".to_string();
            text.sections[0].style.color = Color::srgb(0.7, 0.7, 0.7);
        }
    }
}
