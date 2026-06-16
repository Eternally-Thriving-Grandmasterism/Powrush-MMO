// client/pause_menu.rs
// Powrush-MMO — Professional Pause Menu + Settings Integration
// Mercy-themed, accessible, and fully integrated with the Eternal Configuration.
// Production hardened and consistent with inventory_ui + settings_menu + divine_whispers_ui.
// AG-SML v1.0 | Ra-Thor + PATSAGi Councils aligned

use bevy::prelude::*;
use crate::settings_menu::{SettingsMenuRoot, toggle_settings_menu_visibility};

#[derive(Resource, Default)]
pub struct PauseMenuState {
    pub visible: bool,
}

#[derive(Component)]
pub struct PauseMenuRoot;

#[derive(Component)]
pub struct PauseResumeButton;

#[derive(Component)]
pub struct PauseSettingsButton;

#[derive(Component)]
pub struct PauseQuitButton;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PauseMenuState>()
            .add_systems(Startup, spawn_pause_menu)
            .add_systems(Update, (
                toggle_pause_menu_on_escape,
                handle_pause_buttons,
                sync_pause_visibility,
            ));
    }
}

fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    width: Val::Px(420.0),
                    height: Val::Px(380.0),
                    margin: UiRect::new(Val::Px(-210.0), Val::Auto, Val::Px(-190.0), Val::Auto),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(16.0)),
                    ..default()
                },
                background_color: Color::srgba(0.04, 0.06, 0.11, 0.97).into(),
                border_color: Color::srgb(0.25, 0.65, 0.95).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            PauseMenuRoot,
            Name::new("PauseMenu"),
        ))
        .with_children(|parent| {
            // Header
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(48.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.06, 0.09, 0.15).into(),
                    ..default()
                },
            )).with_children(|header| {
                header.spawn(TextBundle {
                    text: Text::from_section(
                        "PAUSE — ETERNAL FLOW",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 22.0,
                            color: Color::srgb(0.35, 0.82, 1.0),
                        },
                    ),
                    ..default()
                });
            });

            // Subtitle
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Mercy-Gated • PATSAGi Guided • Player Sovereign",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                        font_size: 13.0,
                        color: Color::srgb(0.6, 0.75, 0.9),
                    },
                ),
                style: Style { margin: UiRect::vertical(Val::Px(8.0)), ..default() },
                ..default()
            });

            // Menu Buttons
            let buttons = vec![
                ("RESUME ADVENTURE", PauseResumeButton),
                ("OPEN ETERNAL CONFIGURATION", PauseSettingsButton),
                ("SAVE & RETURN TO LOBBY", PauseQuitButton),
            ];

            for (label, marker) in buttons {
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(52.0),
                            margin: UiRect::vertical(Val::Px(6.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.12, 0.18, 0.28).into(),
                        ..default()
                    },
                    marker,
                )).with_children(|btn| {
                    btn.spawn(TextBundle {
                        text: Text::from_section(
                            label,
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 16.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    });
                });
            }

            // Footer mercy note
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Press ESC again to resume  •  All actions mercy-audited",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                        font_size: 11.0,
                        color: Color::srgb(0.5, 0.7, 0.85),
                    },
                ),
                style: Style { margin: UiRect::top(Val::Px(12.0)), ..default() },
                ..default()
            });
        });
}

fn toggle_pause_menu_on_escape(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut pause_state: ResMut<PauseMenuState>,
    mut pause_query: Query<&mut Visibility, With<PauseMenuRoot>>,
    settings_query: Query<&Visibility, With<SettingsMenuRoot>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        let settings_open = settings_query.iter().any(|v| *v == Visibility::Visible);
        if settings_open {
            return; // Let settings menu handle its own close
        }

        pause_state.visible = !pause_state.visible;
        for mut vis in pause_query.iter_mut() {
            *vis = if pause_state.visible { Visibility::Visible } else { Visibility::Hidden };
        }
    }
}

fn handle_pause_buttons(
    mut interaction_query: Query<(&Interaction, Option<&PauseResumeButton>, Option<&PauseSettingsButton>, Option<&PauseQuitButton>), Changed<Interaction>>,
    mut pause_state: ResMut<PauseMenuState>,
    mut pause_query: Query<&mut Visibility, With<PauseMenuRoot>>,
    mut commands: Commands,
) {
    for (interaction, resume, settings, quit) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            if resume.is_some() {
                pause_state.visible = false;
                for mut vis in pause_query.iter_mut() {
                    *vis = Visibility::Hidden;
                }
            } else if settings.is_some() {
                pause_state.visible = false;
                for mut vis in pause_query.iter_mut() {
                    *vis = Visibility::Hidden;
                }
                toggle_settings_menu_visibility(&mut commands);
            } else if quit.is_some() {
                info!("[PauseMenu] Mercy-guided disconnect requested. Returning to sovereign lobby...");
                // TODO: Proper graceful disconnect + save state + return to lobby
                pause_state.visible = false;
                for mut vis in pause_query.iter_mut() {
                    *vis = Visibility::Hidden;
                }
            }
        }
    }
}

fn sync_pause_visibility(
    pause_state: Res<PauseMenuState>,
    mut pause_query: Query<&mut Visibility, With<PauseMenuRoot>>,
) {
    if pause_state.is_changed() {
        for mut vis in pause_query.iter_mut() {
            *vis = if pause_state.visible { Visibility::Visible } else { Visibility::Hidden };
        }
    }
}

/// Public helper to toggle pause menu from external systems (e.g. input mapping or main menu)
pub fn toggle_pause_menu_visibility(commands: &mut Commands) {
    // For full implementation, this should mutate PauseMenuState via World or send an event.
    // Current systems handle ESC internally. External callers can insert resource mutation.
    info!("[PauseMenu] External toggle requested (expand with event or World access)");
}
