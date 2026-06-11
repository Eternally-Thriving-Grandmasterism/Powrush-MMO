/*!
 * Powrush-MMO Client Settings with Serialization
 *
 * Features:
 * - RON serialization for TAA + Motion Blur settings
 * - Automatic creation of config/ directory
 * - Ready to integrate with settings menu
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::taa_reprojection::TaaSettings;
use crate::motion_blur::MotionBlurSettings;

#[derive(Resource, Serialize, Deserialize, Clone, Debug, Default)]
pub struct ClientSettings {
    pub graphics: GraphicsSettings,
    pub audio: AudioSettings,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct GraphicsSettings {
    pub taa_enabled: bool,
    pub taa_jitter_scale: f32,
    pub motion_blur_enabled: bool,
    pub motion_blur_intensity: f32,
    pub quality_preset: QualityPreset,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub enum QualityPreset {
    Seedling,
    FlowGuardian,
    #[default]
    Eternal,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub whispers_volume: f32,
    pub music_volume: f32,
}

/// Ensures the config directory exists.
fn ensure_config_dir() {
    let dir = Path::new("config");
    if !dir.exists() {
        if let Err(e) = fs::create_dir_all(dir) {
            error!("[Settings] Failed to create config directory: {}", e);
        }
    }
}

/// Saves client settings to a RON file (creates config/ dir automatically).
pub fn save_client_settings(settings: &ClientSettings) {
    ensure_config_dir();

    let ron_string = match ron::to_string_pretty(settings, ron::ser::PrettyConfig::default()) {
        Ok(s) => s,
        Err(e) => {
            error!("[Settings] Failed to serialize settings: {}", e);
            return;
        }
    };

    if let Err(e) = fs::write("config/client_settings.ron", ron_string) {
        error!("[Settings] Failed to write settings file: {}", e);
    } else {
        info!("[Settings] Saved client settings to config/client_settings.ron");
    }
}

/// Loads client settings or returns mercy-aligned defaults.
pub fn load_client_settings() -> ClientSettings {
    let path = Path::new("config/client_settings.ron");

    if path.exists() {
        match fs::read_to_string(path) {
            Ok(content) => match ron::from_str(&content) {
                Ok(settings) => {
                    info!("[Settings] Loaded client settings from file");
                    return settings;
                }
                Err(e) => warn!("[Settings] Failed to parse settings file: {}. Using defaults.", e),
            },
            Err(e) => warn!("[Settings] Failed to read settings file: {}. Using defaults.", e),
        }
    }

    // PATSAGi-aligned defaults
    ClientSettings {
        graphics: GraphicsSettings {
            taa_enabled: true,
            taa_jitter_scale: 1.0,
            motion_blur_enabled: true,
            motion_blur_intensity: 1.0,
            quality_preset: QualityPreset::Eternal,
        },
        audio: AudioSettings {
            master_volume: 0.8,
            whispers_volume: 0.9,
            music_volume: 0.7,
        },
    }
}

/// Sync loaded settings to live TAA resource
pub fn sync_taa_settings(
    mut taa: ResMut<TaaSettings>,
    client: Res<ClientSettings>,
) {
    taa.enabled = client.graphics.taa_enabled;
    taa.jitter_scale = client.graphics.taa_jitter_scale;
}

/// Sync loaded settings to live Motion Blur resource
pub fn sync_motion_blur_settings(
    mut mb: ResMut<MotionBlurSettings>,
    client: Res<ClientSettings>,
) {
    mb.enabled = client.graphics.motion_blur_enabled;
    mb.intensity = client.graphics.motion_blur_intensity;
}
