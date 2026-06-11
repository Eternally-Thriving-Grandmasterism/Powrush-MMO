/*!
 * Powrush-MMO Client Settings with Serialization
 *
 * Example of serializing render settings (TAA + Motion Blur) to a human-readable config file.
 * Uses RON format (popular in Bevy community).
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::taa_reprojection::TaaSettings;
use crate::motion_blur::MotionBlurSettings;

/// Main client settings struct that can be saved/loaded.
#[derive(Resource, Serialize, Deserialize, Clone, Debug, Default)]
pub struct ClientSettings {
    pub graphics: GraphicsSettings,
    pub audio: AudioSettings,
    // Add other categories as needed
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct GraphicsSettings {
    pub taa_enabled: bool,
    pub taa_jitter_scale: f32,
    pub motion_blur_enabled: bool,
    pub motion_blur_intensity: f32,
    pub quality_preset: QualityPreset,
    // Add more graphics options here
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

/// Saves client settings to a RON file.
pub fn save_client_settings(settings: &ClientSettings) {
    let ron_string = match ron::to_string(settings) {
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

/// Loads client settings from a RON file, or returns defaults.
pub fn load_client_settings() -> ClientSettings {
    let path = Path::new("config/client_settings.ron");

    if path.exists() {
        match fs::read_to_string(path) {
            Ok(content) => match ron::from_str(&content) {
                Ok(settings) => {
                    info!("[Settings] Loaded client settings from file");
                    return settings;
                }
                Err(e) => {
                    warn!("[Settings] Failed to parse settings file: {}. Using defaults.", e);
                }
            },
            Err(e) => {
                warn!("[Settings] Failed to read settings file: {}. Using defaults.", e);
            },
        }
    }

    // Return sensible defaults aligned with PATSAGi mercy defaults
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

/// Helper to sync TaaSettings from ClientSettings
pub fn sync_taa_settings(
    mut taa: ResMut<TaaSettings>,
    client: Res<ClientSettings>,
) {
    taa.enabled = client.graphics.taa_enabled;
    taa.jitter_scale = client.graphics.taa_jitter_scale;
}

/// Helper to sync MotionBlurSettings from ClientSettings
pub fn sync_motion_blur_settings(
    mut mb: ResMut<MotionBlurSettings>,
    client: Res<ClientSettings>,
) {
    mb.enabled = client.graphics.motion_blur_enabled;
    mb.intensity = client.graphics.motion_blur_intensity;
}
