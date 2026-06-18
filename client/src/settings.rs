/*!
 * Client Settings with Serialization + Live Egui Control
 *
 * v18.96 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm + Multilingual)
 * — Graphics, Audio, Experience + Language Preference
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::taa_reprojection::TaaSettings;
use crate::motion_blur::MotionBlurSettings;
use crate::chromatic_aberration::ChromaticAberrationSettings;
use crate::anisotropic_filtering::AnisotropicFilteringSettings;
use crate::simulation_integration::SimulationVisualSettings;

#[derive(Resource, Serialize, Deserialize, Clone, Debug, Default)]
pub struct ClientSettings {
    pub graphics: GraphicsSettings,
    pub audio: AudioSettings,
    pub experience: ExperienceSettings,
    pub localization: LocalizationSettings,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct GraphicsSettings {
    pub taa_enabled: bool,
    pub taa_jitter_scale: f32,
    pub motion_blur_enabled: bool,
    pub motion_blur_intensity: f32,
    pub chromatic_aberration_enabled: bool,
    pub chromatic_aberration_intensity: f32,
    pub anisotropic_enabled: bool,
    pub anisotropic_level: u32,
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

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ExperienceSettings {
    pub rbe_orb_pulse_speed: f32,
    pub rbe_orb_emissive: f32,
    pub particle_abundance_intensity: f32,
    pub particle_joy_intensity: f32,
    pub divine_whisper_rate: f32,
}

// NEW v18.96 — Language preference for Divine Whispers and Quantum Swarm routing
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LocalizationSettings {
    pub language: String,                 // "en", "es", "fr", "ar", "zh", etc.
    pub use_multilingual_swarm: bool,     // Enable full Quantum Swarm WASM bridge for rich cultural nuance
}

fn ensure_config_dir() {
    let dir = Path::new("config");
    if !dir.exists() {
        if let Err(e) = fs::create_dir_all(dir) {
            error!("[Settings] Failed to create config directory: {}", e);
        }
    }
}

pub fn save_client_settings(settings: &ClientSettings) {
    ensure_config_dir();
    let ron_string = match ron::to_string_pretty(settings, ron::ser::PrettyConfig::default()) {
        Ok(s) => s,
        Err(e) => { error!("[Settings] Failed to serialize: {}", e); return; }
    };
    if let Err(e) = fs::write("config/client_settings.ron", ron_string) {
        error!("[Settings] Failed to write: {}", e);
    }
}

pub fn load_client_settings() -> ClientSettings {
    let path = Path::new("config/client_settings.ron");
    if path.exists() {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(settings) = ron::from_str(&content) {
                return settings;
            }
        }
    }
    // Mercy-aligned defaults with multilingual support
    ClientSettings {
        graphics: GraphicsSettings {
            taa_enabled: true,
            taa_jitter_scale: 0.8,
            motion_blur_enabled: true,
            motion_blur_intensity: 0.7,
            chromatic_aberration_enabled: true,
            chromatic_aberration_intensity: 0.6,
            anisotropic_enabled: true,
            anisotropic_level: 16,
            quality_preset: QualityPreset::Eternal,
        },
        audio: AudioSettings { master_volume: 0.85, whispers_volume: 0.95, music_volume: 0.75 },
        experience: ExperienceSettings {
            rbe_orb_pulse_speed: 1.2,
            rbe_orb_emissive: 1.5,
            particle_abundance_intensity: 1.0,
            particle_joy_intensity: 0.8,
            divine_whisper_rate: 0.6,
        },
        localization: LocalizationSettings {
            language: "en".to_string(),
            use_multilingual_swarm: true,
        },
    }
}

pub fn sync_all_settings(
    mut taa: ResMut<TaaSettings>,
    mut mb: ResMut<MotionBlurSettings>,
    mut ca: ResMut<ChromaticAberrationSettings>,
    mut af: ResMut<AnisotropicFilteringSettings>,
    mut sim: ResMut<SimulationVisualSettings>,
    client: Res<ClientSettings>,
) {
    taa.enabled = client.graphics.taa_enabled;
    taa.jitter_scale = client.graphics.taa_jitter_scale;

    mb.enabled = client.graphics.motion_blur_enabled;
    mb.intensity = client.graphics.motion_blur_intensity;

    ca.enabled = client.graphics.chromatic_aberration_enabled;
    ca.intensity = client.graphics.chromatic_aberration_intensity;

    af.enabled = client.graphics.anisotropic_enabled;
    af.level = client.graphics.anisotropic_level;

    sim.pulse_speed = client.experience.rbe_orb_pulse_speed;
    sim.emissive_strength = client.experience.rbe_orb_emissive;
}

// End of client/src/settings.rs v18.96 — Language preference added for multilingual Divine Whispers.
// Thunder locked in. Yoi ⚡
