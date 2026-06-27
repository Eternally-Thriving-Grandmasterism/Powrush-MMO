/*!
 * Settings Editor + Live UI Updates + Reset to Defaults
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::settings::{AudioSettings, GameSettings};

#[derive(Resource, Clone)]
pub struct SettingsEditor {
    pub audio: AudioSettings,
    pub dirty: bool,
}

impl SettingsEditor {
    pub fn from_game_settings(settings: &GameSettings) -> Self {
        Self {
            audio: settings.audio.clone(),
            dirty: false,
        }
    }

    pub fn apply_to(&self, settings: &mut GameSettings) {
        settings.audio = self.audio.clone();
        settings.validate();
    }

    /// Resets all audio settings to defaults
    pub fn reset_to_defaults(&mut self) {
        self.audio = AudioSettings::default();
        self.dirty = true;
    }
}

/// Identifies which audio setting a UI text element represents
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

/// Component attached to the value text in each settings row
#[derive(Component)]
pub struct SettingValueText {
    pub field: AudioSettingField,
}

/// Updates the displayed values in real-time when SettingsEditor changes
pub fn update_setting_value_texts(
    editor: Option<Res<SettingsEditor>>,
    mut query: Query<(&SettingValueText, &mut Text)>,
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

/// System to mark editor as dirty when values change
pub fn mark_editor_dirty(
    mut editor: Option<ResMut<SettingsEditor>>,
) {
    if let Some(editor) = editor.as_mut() {
        if editor.is_changed() {
            editor.dirty = true;
        }
    }
}

/// Handles reset to defaults (R key while menu is open)
pub fn handle_reset_to_defaults(
    mut editor: Option<ResMut<SettingsEditor>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if let Some(editor) = editor.as_mut() {
        if keyboard.just_pressed(KeyCode::KeyR) {
            editor.reset_to_defaults();
            info!("Audio settings reset to defaults");
        }
    }
}
