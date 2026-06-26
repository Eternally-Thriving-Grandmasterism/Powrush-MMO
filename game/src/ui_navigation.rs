/*!
 * Spatial Grid UI Navigation System
 *
 * v10 - Proper AppData / user config paths for settings
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy::input::gamepad::{GamepadButton, Gamepads};
use bevy::audio::PlaybackSettings;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// ... (Focusable, Focused, UiFocus, NavDirection remain the same)

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct UiAudioSettings {
    pub navigation_volume: f32,
    pub activation_volume: f32,
    pub navigation_pitch_variation: f32,
    pub activation_pitch_variation: f32,
}

impl Default for UiAudioSettings {
    fn default() -> Self {
        Self {
            navigation_volume: 0.6,
            activation_volume: 0.8,
            navigation_pitch_variation: 0.03,
            activation_pitch_variation: 0.03,
        }
    }
}

/// Returns the path to the settings file in the proper user config directory
fn get_settings_path() -> Option<PathBuf> {
    let proj_dirs = directories::ProjectDirs::from("com", "Autonomicity Games", "Powrush-MMO")?;
    let config_dir = proj_dirs.config_dir();

    // Ensure the config directory exists
    if !config_dir.exists() {
        if let Err(e) = fs::create_dir_all(config_dir) {
            warn!("Failed to create config directory: {}", e);
            return None;
        }
    }

    Some(config_dir.join("settings.ron"))
}

/// Loads UI audio settings from the proper user config location
pub fn load_ui_settings(mut commands: Commands) {
    if let Some(path) = get_settings_path() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(loaded) = ron::from_str::<UiAudioSettings>(&content) {
                commands.insert_resource(loaded);
                info!("Loaded UI audio settings from {:?}", path);
                return;
            }
        }
    }

    info!("Using default UI audio settings");
    commands.insert_resource(UiAudioSettings::default());
}

/// Saves settings to the proper user config location
pub fn save_ui_settings(settings: Res<UiAudioSettings>) {
    if settings.is_changed() {
        if let Some(path) = get_settings_path() {
            if let Ok(serialized) = ron::to_string(&*settings) {
                if let Err(e) = fs::write(&path, serialized) {
                    warn!("Failed to save settings to {:?}: {}", path, e);
                }
            }
        }
    }
}

// ... (rest of the systems and plugin remain the same)

pub struct UiNavigationPlugin;

impl Plugin for UiNavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiFocus>()
            .add_systems(Startup, load_ui_settings)
            .add_systems(Update, gamepad_ui_navigation)
            .add_systems(Update, apply_focus_visuals)
            .add_systems(Update, activate_focused_button)
            .add_systems(Update, play_focus_change_sound)
            .add_systems(Update, play_button_activate_sound)
            .add_systems(Update, save_ui_settings);
    }
}
