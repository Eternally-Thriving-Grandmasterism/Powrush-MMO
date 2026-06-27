/*!
 * Settings Editor - Input Handling for Value Adjustment
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::settings::editor::{SettingsEditor, AudioSettingField, GraphicsSettingField, ControlsSettingField};
use crate::ui_navigation::UiFocus;

/// Handles Left/Right input on focused settings rows to adjust values
pub fn handle_settings_input(
    mut editor: Option<ResMut<SettingsEditor>>,
    focus: Res<UiFocus>,
    buttons: Res<bevy::input::gamepad::GamepadButton>,
    axes: Res<bevy::input::gamepad::GamepadAxis>,
    keyboard: Res<Input<KeyCode>>,
) {
    let Some(editor) = editor.as_mut() else { return; };
    let Some(focused_entity) = focus.current else { return; };

    // Check for Left/Right input
    let left = buttons.just_pressed(bevy::input::gamepad::GamepadButton::DPadLeft)
        || axes.get(bevy::input::gamepad::GamepadAxis::LeftStickX).unwrap_or(0.0) < -0.6
        || keyboard.just_pressed(KeyCode::ArrowLeft);

    let right = buttons.just_pressed(bevy::input::gamepad::GamepadButton::DPadRight)
        || axes.get(bevy::input::gamepad::GamepadAxis::LeftStickX).unwrap_or(0.0) > 0.6
        || keyboard.just_pressed(KeyCode::ArrowRight);

    if !left && !right {
        return;
    }

    let step = if right { 0.05 } else { -0.05 };

    // Try Audio fields first
    if let Ok(value_text) = /* query for AudioValueText on focused_entity */ {
        match value_text.field {
            AudioSettingField::MasterVolume => editor.audio.master_volume = (editor.audio.master_volume + step).clamp(0.0, 1.0),
            AudioSettingField::MusicVolume => editor.audio.music_volume = (editor.audio.music_volume + step).clamp(0.0, 1.0),
            AudioSettingField::SfxVolume => editor.audio.sfx_volume = (editor.audio.sfx_volume + step).clamp(0.0, 1.0),
            AudioSettingField::NavigationVolume => editor.audio.navigation_volume = (editor.audio.navigation_volume + step).clamp(0.0, 1.0),
            AudioSettingField::ActivationVolume => editor.audio.activation_volume = (editor.audio.activation_volume + step).clamp(0.0, 1.0),
            AudioSettingField::NavigationPitch => editor.audio.navigation_pitch_variation = (editor.audio.navigation_pitch_variation + step * 0.5).clamp(0.0, 0.2),
            AudioSettingField::ActivationPitch => editor.audio.activation_pitch_variation = (editor.audio.activation_pitch_variation + step * 0.5).clamp(0.0, 0.2),
        }
        editor.dirty = true;
        return;
    }

    // Similar blocks for Graphics and Controls can be added
    // (pattern is identical)
}

// Note: Full implementation would query the specific ValueText component on the focused entity.
// For production, we recommend using a more unified component or entity association.
