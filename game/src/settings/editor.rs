/*!
 * Settings Editor - Complete Input Handling
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::settings::editor::{SettingsEditor, AudioValueText, GraphicsValueText, ControlsValueText};
use crate::ui_navigation::UiFocus;

/// Handles Left/Right input to adjust the focused setting's value
pub fn handle_settings_input(
    mut editor: Option<ResMut<SettingsEditor>>,
    focus: Res<UiFocus>,
    buttons: Res<bevy::input::gamepad::GamepadButton>,
    axes: Res<bevy::input::gamepad::GamepadAxis>,
    keyboard: Res<Input<KeyCode>>,
    audio_query: Query<&AudioValueText>,
    graphics_query: Query<&GraphicsValueText>,
    controls_query: Query<&ControlsValueText>,
) {
    let Some(editor) = editor.as_mut() else { return; };
    let Some(focused) = focus.current else { return; };

    let left = buttons.just_pressed(bevy::input::gamepad::GamepadButton::DPadLeft)
        || axes.get(bevy::input::gamepad::GamepadAxis::LeftStickX).unwrap_or(0.0) < -0.6
        || keyboard.just_pressed(KeyCode::ArrowLeft);

    let right = buttons.just_pressed(bevy::input::gamepad::GamepadButton::DPadRight)
        || axes.get(bevy::input::gamepad::GamepadAxis::LeftStickX).unwrap_or(0.0) > 0.6
        || keyboard.just_pressed(KeyCode::ArrowRight);

    if !left && !right { return; }

    let step = if right { 0.05 } else { -0.05 };

    // Audio
    if let Ok(value_text) = audio_query.get(focused) {
        match value_text.field {
            crate::settings::editor::AudioSettingField::MasterVolume => editor.audio.master_volume = (editor.audio.master_volume + step).clamp(0.0, 1.0),
            crate::settings::editor::AudioSettingField::MusicVolume => editor.audio.music_volume = (editor.audio.music_volume + step).clamp(0.0, 1.0),
            crate::settings::editor::AudioSettingField::SfxVolume => editor.audio.sfx_volume = (editor.audio.sfx_volume + step).clamp(0.0, 1.0),
            crate::settings::editor::AudioSettingField::NavigationVolume => editor.audio.navigation_volume = (editor.audio.navigation_volume + step).clamp(0.0, 1.0),
            crate::settings::editor::AudioSettingField::ActivationVolume => editor.audio.activation_volume = (editor.audio.activation_volume + step).clamp(0.0, 1.0),
            crate::settings::editor::AudioSettingField::NavigationPitch => editor.audio.navigation_pitch_variation = (editor.audio.navigation_pitch_variation + step * 0.5).clamp(0.0, 0.2),
            crate::settings::editor::AudioSettingField::ActivationPitch => editor.audio.activation_pitch_variation = (editor.audio.activation_pitch_variation + step * 0.5).clamp(0.0, 0.2),
        }
        editor.dirty = true;
        return;
    }

    // Graphics
    if let Ok(value_text) = graphics_query.get(focused) {
        match value_text.field {
            crate::settings::editor::GraphicsSettingField::Fullscreen => editor.graphics.fullscreen = !editor.graphics.fullscreen,
            crate::settings::editor::GraphicsSettingField::ResolutionWidth => editor.graphics.resolution_width = (editor.graphics.resolution_width as i32 + (step * 100.0) as i32).clamp(640, 3840) as u32,
            crate::settings::editor::GraphicsSettingField::ResolutionHeight => editor.graphics.resolution_height = (editor.graphics.resolution_height as i32 + (step * 100.0) as i32).clamp(480, 2160) as u32,
            crate::settings::editor::GraphicsSettingField::Vsync => editor.graphics.vsync = !editor.graphics.vsync,
            crate::settings::editor::GraphicsSettingField::Quality => {
                let new_q = (editor.graphics.graphics_quality as i32 + if right { 1 } else { -1 }).clamp(0, 3) as u8;
                editor.graphics.graphics_quality = unsafe { std::mem::transmute(new_q) };
            }
            crate::settings::editor::GraphicsSettingField::FieldOfView => editor.graphics.field_of_view = (editor.graphics.field_of_view + step * 10.0).clamp(60.0, 120.0),
            crate::settings::editor::GraphicsSettingField::ShadowQuality => editor.graphics.shadow_quality = ((editor.graphics.shadow_quality as i32 + if right { 1 } else { -1 }).clamp(0, 3)) as u8,
        }
        editor.dirty = true;
        return;
    }

    // Controls
    if let Ok(value_text) = controls_query.get(focused) {
        match value_text.field {
            crate::settings::editor::ControlsSettingField::MouseSensitivity => editor.controls.mouse_sensitivity = (editor.controls.mouse_sensitivity + step).clamp(0.1, 4.0),
            crate::settings::editor::ControlsSettingField::InvertY => editor.controls.invert_y_axis = !editor.controls.invert_y_axis,
            crate::settings::editor::ControlsSettingField::Vibration => editor.controls.controller_vibration = !editor.controls.controller_vibration,
            crate::settings::editor::ControlsSettingField::AutoRun => editor.controls.auto_run = !editor.controls.auto_run,
            crate::settings::editor::ControlsSettingField::CameraSmoothing => editor.controls.camera_smoothing = (editor.controls.camera_smoothing + step).clamp(0.0, 1.0),
        }
        editor.dirty = true;
    }
}
