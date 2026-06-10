/*!
 * Player Progress UI v18.10
 *
 * Simple panel showing key progression stats from PlayerSaveData.
 */

use bevy::prelude::*;
use simulation::player_persistence::PlayerSaveData;

#[derive(Component)]
struct PlayerProgressPanel;

#[derive(Component)]
struct EpiphanyCountText;

#[derive(Component)]
struct MuscleMemoryText;

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
                    width: Val::Px(280.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    border: UiRect::all(Val::Px(1.5)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    flex_direction: FlexDirection::Column,
                    visibility: Visibility::Hidden,
                    ..default()
                },
                background_color: Color::srgba(0.06, 0.08, 0.12, 0.95).into(),
                border_color: Color::srgb(0.4, 0.7, 0.95).into(),
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
                        font_size: 16.0,
                        color: Color::srgb(0.6, 0.8, 1.0),
                    },
                ),
                style: Style {
                    margin: UiRect::bottom(Val::Px(12.0)),
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
                            font_size: 15.0,
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
                            font_size: 15.0,
                            color: Color::WHITE,
                        },
                    ),
                    style: Style {
                        margin: UiRect::top(Val::Px(6.0)),
                        ..default()
                    },
                    ..default()
                },
                MuscleMemoryText,
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
) {
    for mut text in epiphany_text.iter_mut() {
        text.sections[0].value = format!("Epiphanies: {}", save_data.epiphanies.len());
    }

    for mut text in muscle_text.iter_mut() {
        text.sections[0].value = format!("Muscle Memory: {:.2}x", save_data.muscle_memory_level);
    }
}
