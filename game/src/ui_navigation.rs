/*!
 * Spatial Grid UI Navigation System
 *
 * v14 - Settings Validation Logic
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

#[derive(Resource, Clone, Serialize, Deserialize, Debug)]
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

impl UiAudioSettings {
    /// Validates and clamps all values to safe ranges.
    /// Logs warnings if any values were out of expected bounds.
    pub fn validate(&mut self) {
        let original = self.clone();

        // Volume must be between 0.0 and 1.0
        self.navigation_volume = self.navigation_volume.clamp(0.0, 1.0);
        self.activation_volume = self.activation_volume.clamp(0.0, 1.0);

        // Pitch variation should be reasonable (max 20% variation)
        self.navigation_pitch_variation = self.navigation_pitch_variation.clamp(0.0, 0.2);
        self.activation_pitch_variation = self.activation_pitch_variation.clamp(0.0, 0.2);

        // Log if any values were corrected
        if original.navigation_volume != self.navigation_volume ||
           original.activation_volume != self.activation_volume ||
           original.navigation_pitch_variation != self.navigation_pitch_variation ||
           original.activation_pitch_variation != self.activation_pitch_variation
        {
            warn!(
                "Settings values were out of valid range and have been clamped. Original: {:?}",
                original
            );
        }
    }
}

/// Versioned settings file wrapper
#[derive(Serialize, Deserialize)]
struct SettingsFile {
    version: u32,
    settings: UiAudioSettings,
}

const CURRENT_SETTINGS_VERSION: u32 = 1;

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

/// Loads settings with validation and strong fallback
pub fn load_ui_settings(mut commands: Commands) {
    if let Some(path) = get_settings_path() {
        if let Ok(content) = fs::read_to_string(&path) {
            // Try versioned format
            if let Ok(mut file) = ron::from_str::<SettingsFile>(&content) {
                if file.version == CURRENT_SETTINGS_VERSION {
                    file.settings.validate();
                    commands.insert_resource(file.settings);
                    info!("Loaded and validated settings v{} from {:?}", file.version, path);
                    return;
                }
            }

            // Fallback: try loading old unversioned format
            if let Ok(mut old_settings) = ron::from_str::<UiAudioSettings>(&content) {
                warn!("Loaded legacy unversioned settings file");
                old_settings.validate();
                commands.insert_resource(old_settings);
                return;
            }
        }
    }

    // Final fallback to defaults
    let mut defaults = UiAudioSettings::default();
    defaults.validate();
    commands.insert_resource(defaults);
    info!("Using default validated UI audio settings");
}

/// Saves settings when changed
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

// ... (rest of the systems remain the same)

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
