/*!
 * Spatial Grid UI Navigation System
 *
 * v11 - Settings Versioning
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

/// Versioned settings file wrapper.
/// This allows us to safely evolve the settings format over time.
#[derive(Serialize, Deserialize)]
struct SettingsFile {
    version: u32,
    settings: UiAudioSettings,
}

const CURRENT_SETTINGS_VERSION: u32 = 1;

/// Returns the path to the settings file in the proper user config directory
fn get_settings_path() -> Option<PathBuf> {
    let proj_dirs = directories::ProjectDirs::from("com", "Autonomicity Games", "Powrush-MMO")?;
    let config_dir = proj_dirs.config_dir();

    if !config_dir.exists() {
        if let Err(e) = fs::create_dir_all(config_dir) {
            warn!("Failed to create config directory: {}", e);
            return None;
        }
    }

    Some(config_dir.join("settings.ron"))
}

/// Loads UI audio settings with version checking
pub fn load_ui_settings(mut commands: Commands) {
    if let Some(path) = get_settings_path() {
        if let Ok(content) = fs::read_to_string(&path) {
            // Try to load as versioned SettingsFile first
            if let Ok(file) = ron::from_str::<SettingsFile>(&content) {
                if file.version == CURRENT_SETTINGS_VERSION {
                    commands.insert_resource(file.settings);
                    info!("Loaded settings v{} from {:?}", file.version, path);
                    return;
                } else {
                    warn!(
                        "Settings file version mismatch (found {}, current {}). Using defaults.",
                        file.version, CURRENT_SETTINGS_VERSION
                    );
                }
            }

            // Fallback: try loading old unversioned format (for migration)
            if let Ok(old_settings) = ron::from_str::<UiAudioSettings>(&content) {
                warn!("Loaded legacy unversioned settings. Consider re-saving.");
                commands.insert_resource(old_settings);
                return;
            }
        }
    }

    info!("Using default UI audio settings (v{})", CURRENT_SETTINGS_VERSION);
    commands.insert_resource(UiAudioSettings::default());
}

/// Saves settings with current version
pub fn save_ui_settings(settings: Res<UiAudioSettings>) {
    if settings.is_changed() {
        if let Some(path) = get_settings_path() {
            let file = SettingsFile {
                version: CURRENT_SETTINGS_VERSION,
                settings: settings.clone(),
            };

            if let Ok(serialized) = ron::to_string(&file) {
                if let Err(e) = fs::write(&path, serialized) {
                    warn!("Failed to save settings to {:?}: {}", path, e);
                }
            }
        }
    }
}

// ... (rest of the systems remain unchanged)

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
