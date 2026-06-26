/*!
 * Simple Settings Menu for UI Audio Configuration
 *
 * Allows players to adjust navigation and activation sound settings.
 * Fully compatible with the gamepad navigation system.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::ui_navigation::{Focusable, UiAudioSettings};

/// Marker for settings menu root entity
#[derive(Component)]
pub struct SettingsMenu;

/// Types of settings that can be adjusted
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SettingType {
    NavigationVolume,
    ActivationVolume,
    NavigationPitch,
    ActivationPitch,
}

#[derive(Component)]
pub struct SettingRow {
    pub setting: SettingType,
}

/// System to spawn a basic settings menu
pub fn spawn_settings_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            SettingsMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(30.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.15, 0.15, 0.2).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Audio Settings",
                            TextStyle {
                                font_size: 32.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        ..default()
                    });

                    // Setting rows
                    spawn_setting_row(parent, "Navigation Volume", SettingType::NavigationVolume);
                    spawn_setting_row(parent, "Activation Volume", SettingType::ActivationVolume);
                    spawn_setting_row(parent, "Navigation Pitch Variation", SettingType::NavigationPitch);
                    spawn_setting_row(parent, "Activation Pitch Variation", SettingType::ActivationPitch);

                    // Close instruction
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Press B / Circle to close",
                            TextStyle {
                                font_size: 18.0,
                                color: Color::srgb(0.7, 0.7, 0.7),
                                ..default()
                            },
                        ),
                        style: Style {
                            margin: UiRect::top(Val::Px(20.0)),
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

fn spawn_setting_row(parent: &mut ChildBuilder, label: &str, setting: SettingType) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                },
                ..default()
            },
            Focusable { order: setting as i32 },
            SettingRow { setting },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    label,
                    TextStyle {
                        font_size: 22.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                style: Style {
                    width: Val::Px(280.0),
                    ..default()
                },
                ..default()
            });

            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "0.00",
                        TextStyle {
                            font_size: 22.0,
                            color: Color::srgb(0.4, 0.8, 1.0),
                            ..default()
                        },
                    ),
                    ..default()
                },
                SettingValueText { setting },
            ));
        });
}

#[derive(Component)]
pub struct SettingValueText {
    pub setting: SettingType,
}

/// Updates the displayed values in the settings menu
pub fn update_setting_values(
    settings: Res<UiAudioSettings>,
    mut query: Query<(&SettingValueText, &mut Text)>,
) {
    for (value_text, mut text) in query.iter_mut() {
        let value = match value_text.setting {
            SettingType::NavigationVolume => settings.navigation_volume,
            SettingType::ActivationVolume => settings.activation_volume,
            SettingType::NavigationPitch => settings.navigation_pitch_variation,
            SettingType::ActivationPitch => settings.activation_pitch_variation,
        };

        text.sections[0].value = format!("{:.2}", value);
    }
}

/// Allows changing values with left/right input on focused setting
pub fn adjust_settings_with_input(
    buttons: Res<bevy::input::gamepad::GamepadButton>,
    axes: Res<bevy::input::gamepad::GamepadAxis>,
    focus: Res<crate::ui_navigation::UiFocus>,
    mut settings: ResMut<UiAudioSettings>,
    query: Query<&SettingRow>,
) {
    let Some(focused) = focus.current else { return; };

    let Ok(row) = query.get(focused) else { return; };

    let left = buttons.just_pressed(bevy::input::gamepad::GamepadButton::DPadLeft)
        || axes.get(bevy::input::gamepad::GamepadAxis::LeftStickX).unwrap_or(0.0) < -0.7;
    let right = buttons.just_pressed(bevy::input::gamepad::GamepadButton::DPadRight)
        || axes.get(bevy::input::gamepad::GamepadAxis::LeftStickX).unwrap_or(0.0) > 0.7;

    if !left && !right {
        return;
    }

    let change = if right { 0.05 } else { -0.05 };

    match row.setting {
        SettingType::NavigationVolume => {
            settings.navigation_volume = (settings.navigation_volume + change).clamp(0.0, 1.0);
        }
        SettingType::ActivationVolume => {
            settings.activation_volume = (settings.activation_volume + change).clamp(0.0, 1.0);
        }
        SettingType::NavigationPitch => {
            settings.navigation_pitch_variation =
                (settings.navigation_pitch_variation + change * 0.5).clamp(0.0, 0.15);
        }
        SettingType::ActivationPitch => {
            settings.activation_pitch_variation =
                (settings.activation_pitch_variation + change * 0.5).clamp(0.0, 0.15);
        }
    }
}

/// Plugin for the settings menu
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_setting_values)
            .add_systems(Update, adjust_settings_with_input);
    }
}
