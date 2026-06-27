/*!
 * Settings Editor + Live UI Updates (All Categories)
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::settings::{AudioSettings, GameSettings, GraphicsSettings, ControlsSettings};

#[derive(Resource, Clone)]
pub struct SettingsEditor {
    pub audio: AudioSettings,
    pub graphics: GraphicsSettings,
    pub controls: ControlsSettings,
    pub dirty: bool,
}

impl SettingsEditor {
    pub fn from_game_settings(settings: &GameSettings) -> Self {
        Self {
            audio: settings.audio.clone(),
            graphics: settings.graphics.clone(),
            controls: settings.controls.clone(),
            dirty: false,
        }
    }

    pub fn apply_to(&self, settings: &mut GameSettings) {
        settings.audio = self.audio.clone();
        settings.graphics = self.graphics.clone();
        settings.controls = self.controls.clone();
        settings.validate();
    }

    pub fn reset_to_defaults(&mut self) {
        self.audio = AudioSettings::default();
        self.graphics = GraphicsSettings::default();
        self.controls = ControlsSettings::default();
        self.dirty = true;
    }
}

// ==================== AUDIO ====================

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum AudioSettingField {
    MasterVolume,
    MusicVolume,
    SfxVolume,
    NavigationVolume,
    ActivationVolume,
    NavigationPitch,
    ActivationPitch,
}

#[derive(Component)]
pub struct AudioValueText {
    pub field: AudioSettingField,
}

// ==================== GRAPHICS ====================

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GraphicsSettingField {
    Fullscreen,
    ResolutionWidth,
    ResolutionHeight,
    Vsync,
    Quality,
    FieldOfView,
    ShadowQuality,
}

#[derive(Component)]
pub struct GraphicsValueText {
    pub field: GraphicsSettingField,
}

// ==================== CONTROLS ====================

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ControlsSettingField {
    MouseSensitivity,
    InvertY,
    Vibration,
    AutoRun,
    CameraSmoothing,
}

#[derive(Component)]
pub struct ControlsValueText {
    pub field: ControlsSettingField,
}

// ==================== LIVE UPDATE SYSTEMS ====================

pub fn update_audio_value_texts(
    editor: Option<Res<SettingsEditor>>,
    mut query: Query<(&AudioValueText, &mut Text)>,
) {
    let Some(editor) = editor else { return; };
    for (value_text, mut text) in query.iter_mut() {
        let value = match value_text.field {
            AudioSettingField::MasterVolume => editor.audio.master_volume,
            AudioSettingField::MusicVolume => editor.audio.music_volume,
            AudioSettingField::SfxVolume => editor.audio.sfx_volume,
            AudioSettingField::NavigationVolume => editor.audio.navigation_volume,
            AudioSettingField::ActivationVolume => editor.audio.activation_volume,
            AudioSettingField::NavigationPitch => editor.audio.navigation_pitch_variation,
            AudioSettingField::ActivationPitch => editor.audio.activation_pitch_variation,
        };
        text.sections[0].value = format!("{:.2}", value);
    }
}

pub fn update_graphics_value_texts(
    editor: Option<Res<SettingsEditor>>,
    mut query: Query<(&GraphicsValueText, &mut Text)>,
) {
    let Some(editor) = editor else { return; };
    for (value_text, mut text) in query.iter_mut() {
        let value = match value_text.field {
            GraphicsSettingField::Fullscreen => if editor.graphics.fullscreen { 1.0 } else { 0.0 },
            GraphicsSettingField::ResolutionWidth => editor.graphics.resolution_width as f32,
            GraphicsSettingField::ResolutionHeight => editor.graphics.resolution_height as f32,
            GraphicsSettingField::Vsync => if editor.graphics.vsync { 1.0 } else { 0.0 },
            GraphicsSettingField::Quality => editor.graphics.graphics_quality as u8 as f32,
            GraphicsSettingField::FieldOfView => editor.graphics.field_of_view,
            GraphicsSettingField::ShadowQuality => editor.graphics.shadow_quality as f32,
        };
        text.sections[0].value = format!("{:.0}", value);
    }
}

pub fn update_controls_value_texts(
    editor: Option<Res<SettingsEditor>>,
    mut query: Query<(&ControlsValueText, &mut Text)>,
) {
    let Some(editor) = editor else { return; };
    for (value_text, mut text) in query.iter_mut() {
        let value = match value_text.field {
            ControlsSettingField::MouseSensitivity => editor.controls.mouse_sensitivity,
            ControlsSettingField::InvertY => if editor.controls.invert_y_axis { 1.0 } else { 0.0 },
            ControlsSettingField::Vibration => if editor.controls.controller_vibration { 1.0 } else { 0.0 },
            ControlsSettingField::AutoRun => if editor.controls.auto_run { 1.0 } else { 0.0 },
            ControlsSettingField::CameraSmoothing => editor.controls.camera_smoothing,
        };
        text.sections[0].value = format!("{:.2}", value);
    }
}

pub fn mark_editor_dirty(
    mut editor: Option<ResMut<SettingsEditor>>,
) {
    if let Some(editor) = editor.as_mut() {
        if editor.is_changed() {
            editor.dirty = true;
        }
    }
}

pub fn handle_reset_to_defaults(
    mut editor: Option<ResMut<SettingsEditor>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if let Some(editor) = editor.as_mut() {
        if keyboard.just_pressed(KeyCode::KeyR) {
            editor.reset_to_defaults();
            info!("All settings reset to defaults");
        }
    }
}
