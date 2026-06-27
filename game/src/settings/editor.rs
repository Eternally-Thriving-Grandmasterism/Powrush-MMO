/*!
 * Settings Editor - Slider Bar Updates (All Categories)
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::settings::ui::{SliderBar, SliderField};
use crate::settings::editor::SettingsEditor;

pub fn update_slider_bars(
    editor: Option<Res<SettingsEditor>>,
    mut query: Query<(&SliderBar, &mut Style, &mut BackgroundColor)>,
) {
    let Some(editor) = editor else { return; };

    for (bar, mut style, mut color) in query.iter_mut() {
        let value = match bar.field {
            SliderField::Audio(field) => get_audio_value(&editor.audio, field),
            SliderField::Graphics(field) => get_graphics_value(&editor.graphics, field),
            SliderField::Controls(field) => get_controls_value(&editor.controls, field),
        };

        style.width = Val::Px(120.0 * value.clamp(0.0, 1.0));

        let intensity = 0.4 + (value.clamp(0.0, 1.0) * 0.6);
        *color = Color::srgb(0.2 * intensity, 0.6 * intensity, 1.0).into();
    }
}

fn get_audio_value(audio: &crate::settings::AudioSettings, field: crate::settings::editor::AudioSettingField) -> f32 {
    match field {
        crate::settings::editor::AudioSettingField::MasterVolume => audio.master_volume,
        crate::settings::editor::AudioSettingField::MusicVolume => audio.music_volume,
        crate::settings::editor::AudioSettingField::SfxVolume => audio.sfx_volume,
        crate::settings::editor::AudioSettingField::NavigationVolume => audio.navigation_volume,
        crate::settings::editor::AudioSettingField::ActivationVolume => audio.activation_volume,
        crate::settings::editor::AudioSettingField::NavigationPitch => audio.navigation_pitch_variation,
        crate::settings::editor::AudioSettingField::ActivationPitch => audio.activation_pitch_variation,
    }
}

fn get_graphics_value(graphics: &crate::settings::GraphicsSettings, field: crate::settings::editor::GraphicsSettingField) -> f32 {
    match field {
        crate::settings::editor::GraphicsSettingField::Fullscreen => if graphics.fullscreen { 1.0 } else { 0.0 },
        crate::settings::editor::GraphicsSettingField::ResolutionWidth => (graphics.resolution_width as f32 / 3840.0).clamp(0.0, 1.0),
        crate::settings::editor::GraphicsSettingField::ResolutionHeight => (graphics.resolution_height as f32 / 2160.0).clamp(0.0, 1.0),
        crate::settings::editor::GraphicsSettingField::Vsync => if graphics.vsync { 1.0 } else { 0.0 },
        crate::settings::editor::GraphicsSettingField::Quality => (graphics.graphics_quality as u8 as f32) / 3.0,
        crate::settings::editor::GraphicsSettingField::FieldOfView => (graphics.field_of_view - 60.0) / 60.0,
        crate::settings::editor::GraphicsSettingField::ShadowQuality => (graphics.shadow_quality as f32) / 3.0,
    }
}

fn get_controls_value(controls: &crate::settings::ControlsSettings, field: crate::settings::editor::ControlsSettingField) -> f32 {
    match field {
        crate::settings::editor::ControlsSettingField::MouseSensitivity => (controls.mouse_sensitivity - 0.1) / 3.9,
        crate::settings::editor::ControlsSettingField::InvertY => if controls.invert_y_axis { 1.0 } else { 0.0 },
        crate::settings::editor::ControlsSettingField::Vibration => if controls.controller_vibration { 1.0 } else { 0.0 },
        crate::settings::editor::ControlsSettingField::AutoRun => if controls.auto_run { 1.0 } else { 0.0 },
        crate::settings::editor::ControlsSettingField::CameraSmoothing => controls.camera_smoothing,
    }
}
