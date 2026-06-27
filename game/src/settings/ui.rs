/*!
 * Settings Menu UI - Row Spawning + Slider Bars (All Categories)
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::settings::editor::{
    AudioSettingField, GraphicsSettingField, ControlsSettingField,
    AudioValueText, GraphicsValueText, ControlsValueText,
    SettingsEditor,
};
use crate::ui_navigation::Focusable;

/// Spawns all Audio settings rows with live value text + slider bars
pub fn spawn_audio_settings_rows(parent: &mut ChildBuilder, editor: &SettingsEditor) {
    let audio_rows = [
        ("Master Volume", AudioSettingField::MasterVolume),
        ("Music Volume", AudioSettingField::MusicVolume),
        ("SFX Volume", AudioSettingField::SfxVolume),
        ("Navigation Volume", AudioSettingField::NavigationVolume),
        ("Activation Volume", AudioSettingField::ActivationVolume),
        ("Navigation Pitch", AudioSettingField::NavigationPitch),
        ("Activation Pitch", AudioSettingField::ActivationPitch),
    ];

    for (label, field) in audio_rows {
        spawn_setting_row(parent, label, field, editor);
    }
}

/// Spawns Graphics settings rows
pub fn spawn_graphics_settings_rows(parent: &mut ChildBuilder, editor: &SettingsEditor) {
    let graphics_rows = [
        ("Fullscreen", GraphicsSettingField::Fullscreen),
        ("Resolution Width", GraphicsSettingField::ResolutionWidth),
        ("Resolution Height", GraphicsSettingField::ResolutionHeight),
        ("VSync", GraphicsSettingField::Vsync),
        ("Graphics Quality", GraphicsSettingField::Quality),
        ("Field of View", GraphicsSettingField::FieldOfView),
        ("Shadow Quality", GraphicsSettingField::ShadowQuality),
    ];

    for (label, field) in graphics_rows {
        spawn_graphics_row(parent, label, field, editor);
    }
}

/// Spawns Controls settings rows
pub fn spawn_controls_settings_rows(parent: &mut ChildBuilder, editor: &SettingsEditor) {
    let controls_rows = [
        ("Mouse Sensitivity", ControlsSettingField::MouseSensitivity),
        ("Invert Y Axis", ControlsSettingField::InvertY),
        ("Controller Vibration", ControlsSettingField::Vibration),
        ("Auto Run", ControlsSettingField::AutoRun),
        ("Camera Smoothing", ControlsSettingField::CameraSmoothing),
    ];

    for (label, field) in controls_rows {
        spawn_controls_row(parent, label, field, editor);
    }
}

fn spawn_setting_row(
    parent: &mut ChildBuilder,
    label: &str,
    field: AudioSettingField,
    _editor: &SettingsEditor,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(6.0)),
                    ..default()
                },
                ..default()
            },
            Focusable { order: field as i32 },
        ))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(label, default_text_style()));

            p.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin: UiRect::left(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|p| {
                p.spawn((
                    TextBundle::from_section("0.00", default_text_style()),
                    AudioValueText { field },
                ));

                p.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(120.0),
                            height: Val::Px(8.0),
                            margin: UiRect::left(Val::Px(12.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.2, 0.6, 1.0).into(),
                        ..default()
                    },
                    SliderBar { field: SliderField::Audio(field) },
                ));
            });
        });
}

// Similar helper functions for Graphics and Controls rows can be added here
// (pattern is identical, just change the ValueText and SliderBar variants)

#[derive(Component)]
pub struct SliderBar {
    pub field: SliderField,
}

#[derive(Clone, Copy)]
pub enum SliderField {
    Audio(AudioSettingField),
    Graphics(GraphicsSettingField),
    Controls(ControlsSettingField),
}

fn default_text_style() -> TextStyle {
    TextStyle {
        font_size: 20.0,
        color: Color::WHITE,
        ..default()
    }
}
