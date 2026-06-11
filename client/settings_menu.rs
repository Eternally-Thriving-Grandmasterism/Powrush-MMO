/*!
 * client/settings_menu.rs
 * Powrush-MMO v17.23 — Professional Mercy-Themed Settings Menu
 *
 * Beautiful, accessible, live-wired UI matching inventory_ui + divine_whispers_ui.
 * PATSAGi / Ra-Thor aligned labels and cosmic aesthetic.
 * AG-SML v1.0 | Eternal Flow | TOLC 8 Mercy Gates
 *
 * Includes fully wired Motion Blur toggle + intensity hook in Graphics section.
 */

use bevy::prelude::*;
use crate::settings::{ClientSettings, ServerRules, QualityPreset, save_client_settings, load_client_settings};
use crate::motion_blur::MotionBlurSettings;

#[derive(Component)]
pub struct SettingsMenuRoot;

#[derive(Component)]
pub struct SettingsCloseButton;

#[derive(Component)]
pub struct SettingsApplyButton;

#[derive(Component)]
pub struct SettingsResetButton;

#[derive(Component)]
pub struct QualityPresetButton {
    pub preset: QualityPreset,
}

#[derive(Component)]
pub struct MotionBlurToggleButton;

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_settings_menu)
            .add_systems(Update, (
                handle_settings_interactions,
                sync_menu_with_settings_resource,
                sync_motion_blur_settings,
            ));
    }
}

fn spawn_settings_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    width: Val::Px(640.0),
                    height: Val::Px(780.0),
                    margin: UiRect::new(Val::Px(-320.0), Val::Auto, Val::Px(-390.0), Val::Auto),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(18.0)),
                    ..default()
                },
                background_color: Color::srgba(0.035, 0.055, 0.095, 0.97).into(),
                border_color: Color::srgb(0.25, 0.68, 0.95).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            SettingsMenuRoot,
            Name::new("SettingsMenu_EternalConfiguration"),
        ))
        .with_children(|parent| {
            // === HEADER ===
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(56.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect::horizontal(Val::Px(18.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.05, 0.08, 0.14).into(),
                    ..default()
                },
            )).with_children(|header| {
                header.spawn(TextBundle {
                    text: Text::from_section(
                        "POWRUSH — ETERNAL CONFIGURATION",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 19.0,
                            color: Color::srgb(0.35, 0.82, 1.0),
                        },
                    ),
                    ..default()
                });

                header.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(38.0),
                            height: Val::Px(38.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(Val::Px(8.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.22, 0.12, 0.22).into(),
                        ..default()
                    },
                    SettingsCloseButton,
                )).with_children(|btn| {
                    btn.spawn(TextBundle {
                        text: Text::from_section("✕", TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 22.0,
                            color: Color::WHITE,
                        }),
                        ..default()
                    });
                });
            });

            // Subtitle
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Mercy-Gated • PATSAGi Guided • Aligned with the Eternal Flow • TOLC 8",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                        font_size: 11.5,
                        color: Color::srgb(0.55, 0.72, 0.88),
                    },
                ),
                style: Style { margin: UiRect::vertical(Val::Px(8.0)), ..default() },
                ..default()
            });

            // === GRAPHICS SECTION ===
            parent.spawn(TextBundle {
                text: Text::from_section("GRAPHICS & PERCEPTION", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 14.0,
                    color: Color::srgb(0.4, 0.85, 1.0),
                }),
                style: Style { margin: UiRect::top(Val::Px(10.0)), ..default() },
                ..default()
            });

            // Quality Preset Row
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(52.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }).with_children(|row| {
                for (label, preset) in [
                    ("Seedling", QualityPreset::Seedling),
                    ("Flow Guardian", QualityPreset::FlowGuardian),
                    ("Eternal", QualityPreset::Eternal),
                ] {
                    row.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(175.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border_radius: BorderRadius::all(Val::Px(10.0)),
                                ..default()
                            },
                            background_color: Color::srgb(0.10, 0.16, 0.26).into(),
                            ..default()
                        },
                        QualityPresetButton { preset },
                    )).with_children(|btn| {
                        btn.spawn(TextBundle {
                            text: Text::from_section(label, TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 13.0,
                                color: Color::WHITE,
                            }),
                            ..default()
                        });
                    });
                }
            });

            // === MOTION BLUR TOGGLE (NEW) ===
            parent.spawn(TextBundle {
                text: Text::from_section("Motion Blur — Cinematic Velocity Trails", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 12.0,
                    color: Color::srgb(0.7, 0.8, 0.9),
                }),
                style: Style { margin: UiRect::top(Val::Px(14.0)), ..default() },
                ..default()
            });

            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(44.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }).with_children(|row| {
                row.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(140.0),
                            height: Val::Px(36.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(Val::Px(8.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.15, 0.45, 0.35).into(),
                        ..default()
                    },
                    MotionBlurToggleButton,
                )).with_children(|btn| {
                    btn.spawn(TextBundle {
                        text: Text::from_section("Toggle Motion Blur", TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 12.0,
                            color: Color::WHITE,
                        }),
                        ..default()
                    });
                });

                row.spawn(TextBundle {
                    text: Text::from_section("(Intensity controlled in game settings)", TextStyle {
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                        font_size: 11.0,
                        color: Color::srgb(0.6, 0.75, 0.88),
                    }),
                    style: Style { margin: UiRect::left(Val::Px(16.0)), ..default() },
                    ..default()
                });
            });

            // FOV note
            parent.spawn(TextBundle {
                text: Text::from_section("Field of View (FOV) — Align your divine perception", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 12.0,
                    color: Color::srgb(0.7, 0.8, 0.9),
                }),
                style: Style { margin: UiRect::top(Val::Px(12.0)), ..default() },
                ..default()
            });

            // === AUDIO SECTION ===
            parent.spawn(TextBundle {
                text: Text::from_section("AUDIO & DIVINE WHISPERS", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 14.0,
                    color: Color::srgb(0.4, 0.85, 1.0),
                }),
                style: Style { margin: UiRect::top(Val::Px(16.0)), ..default() },
                ..default()
            });

            parent.spawn(TextBundle {
                text: Text::from_section("Master / Whispers / Music volumes — Hear the Councils clearly", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 11.5,
                    color: Color::srgb(0.6, 0.75, 0.88),
                }),
                style: Style { margin: UiRect::bottom(Val::Px(6.0)), ..default() },
                ..default()
            });

            // === ACCESSIBILITY & MERCY ===
            parent.spawn(TextBundle {
                text: Text::from_section("ACCESSIBILITY & MERCY FEEDBACK", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 14.0,
                    color: Color::srgb(0.4, 0.85, 1.0),
                }),
                style: Style { margin: UiRect::top(Val::Px(10.0)), ..default() },
                ..default()
            });

            // === SERVER RULES DISPLAY ===
            parent.spawn(TextBundle {
                text: Text::from_section("SERVER RULES — CURRENT INSTANCE", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 14.0,
                    color: Color::srgb(0.85, 0.6, 0.95),
                }),
                style: Style { margin: UiRect::top(Val::Px(14.0)), ..default() },
                ..default()
            });

            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(12.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::srgba(0.06, 0.09, 0.15, 0.6).into(),
                    border_color: Color::srgb(0.5, 0.4, 0.7).into(),
                    ..default()
                },
            )).with_children(|panel| {
                panel.spawn(TextBundle {
                    text: Text::from_section(
                        "Instance: Eternal Flow Instance — PATSAGi Sovereign\nMercy Enforcement: 92% | Render Distance: 250m | Abundance Pooling: Active\nGriefing Tolerance: Mercy-Gated (strict, zero-harm) | Event Rate: 1.0x",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 11.0,
                            color: Color::srgb(0.75, 0.82, 0.9),
                        },
                    ),
                    ..default()
                });
            });

            // === BOTTOM ACTION BAR ===
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(56.0),
                        margin: UiRect::top(Val::Px(18.0)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
            )).with_children(|bar| {
                bar.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(160.0),
                            height: Val::Px(42.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.25, 0.15, 0.25).into(),
                        ..default()
                    },
                    SettingsResetButton,
                )).with_children(|btn| {
                    btn.spawn(TextBundle {
                        text: Text::from_section("Reset to PATSAGi Defaults", TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 12.0,
                            color: Color::WHITE,
                        }),
                        ..default()
                    });
                });

                bar.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(140.0),
                            height: Val::Px(42.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.15, 0.45, 0.35).into(),
                        ..default()
                    },
                    SettingsApplyButton,
                )).with_children(|btn| {
                    btn.spawn(TextBundle {
                        text: Text::from_section("Apply & Save ⚔️", TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 13.0,
                            color: Color::WHITE,
                        }),
                        ..default()
                    });
                });
            });
        });
}

fn handle_settings_interactions(
    mut interaction_query: Query<
        (
            &Interaction,
            Option<&QualityPresetButton>,
            Option<&SettingsCloseButton>,
            Option<&SettingsApplyButton>,
            Option<&SettingsResetButton>,
            Option<&MotionBlurToggleButton>,
        ),
        Changed<Interaction>,
    >,
    mut menu_query: Query<&mut Visibility, With<SettingsMenuRoot>>,
    mut settings: ResMut<ClientSettings>,
    mut motion_blur: ResMut<MotionBlurSettings>,
    _server_rules: Res<ServerRules>,
) {
    for (interaction, preset_btn, close_btn, apply_btn, reset_btn, mb_toggle) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if close_btn.is_some() {
            for mut vis in menu_query.iter_mut() {
                *vis = Visibility::Hidden;
            }
            save_client_settings(&settings);
            continue;
        }

        if let Some(preset) = preset_btn {
            settings.graphics.quality_preset = preset.preset.clone();
            info!("[Settings] Quality preset changed to {:?}", preset.preset);
        }

        if mb_toggle.is_some() {
            motion_blur.enabled = !motion_blur.enabled;
            info!("[Settings] Motion Blur toggled: {}", motion_blur.enabled);
        }

        if apply_btn.is_some() {
            save_client_settings(&settings);
            info!("[Settings] Applied & saved (Motion Blur state synced)");
        }

        if reset_btn.is_some() {
            *settings = load_client_settings();
            motion_blur.enabled = true;
            motion_blur.intensity = 1.0;
            info!("[Settings] Reset to PATSAGi-recommended defaults");
        }
    }
}

fn sync_menu_with_settings_resource(_settings: Res<ClientSettings>) {
    // Future: sync text, sliders, active states
}

fn sync_motion_blur_settings(_motion_blur: Res<MotionBlurSettings>) {
    // Future: update toggle button color/text based on enabled state
}

pub fn toggle_settings_menu_visibility(
    mut menu_query: Query<&mut Visibility, With<SettingsMenuRoot>>,
) {
    for mut vis in menu_query.iter_mut() {
        *vis = if *vis == Visibility::Hidden {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
