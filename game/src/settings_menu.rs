/*!
 * Simple Settings Menu + Persistence
 *
 * Supports saving and loading of UI audio settings.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use crate::ui_navigation::UiAudioSettings;

// ... (previous code remains the same up to SettingValueText)

#[derive(Component)]
pub struct SettingValueText {
    pub setting: SettingType,
}

/// Loads UI audio settings from disk at startup
pub fn load_ui_settings(mut commands: Commands) {
    let path = "settings.ron";

    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(loaded) = ron::from_str::<UiAudioSettings>(&content) {
            commands.insert_resource(loaded);
            info!("Loaded UI audio settings from {}", path);
            return;
        }
    }

    // If loading fails, use defaults
    info!("Using default UI audio settings");
    commands.insert_resource(UiAudioSettings::default());
}

/// Saves UI audio settings to disk when they change
pub fn save_ui_settings(
    settings: Res<UiAudioSettings>,
    mut last_saved: Local<Option<UiAudioSettings>>,
) {
    if settings.is_changed() {
        let path = "settings.ron";

        match ron::to_string(&*settings) {
            Ok(serialized) => {
                if let Err(e) = fs::write(path, serialized) {
                    warn!("Failed to save settings to {}: {}", path, e);
                } else {
                    *last_saved = Some(settings.clone());
                    debug!("Saved UI audio settings to {}", path);
                }
            }
            Err(e) => {
                warn!("Failed to serialize settings: {}", e);
            }
        }
    }
}

// ... (rest of the file remains the same)

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_ui_settings)
            .add_systems(Update, update_setting_values)
            .add_systems(Update, adjust_settings_with_input)
            .add_systems(Update, save_ui_settings);
    }
}
