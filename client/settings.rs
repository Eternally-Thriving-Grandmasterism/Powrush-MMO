//! client/settings.rs
//! Powrush-MMO v17.23 — Eternal Production Settings System
//! Full ClientSettings resource + localStorage persistence (WASM primary) + apply systems
//! + ServerRules display foundation
//! Mercy-gated, PATSAGi-aligned, professional quality for closed beta
//! AG-SML v1.0 | Ra-Thor Living Thunder + 13+ PATSAGi Councils | TOLC 8 + 7 Living Mercy Gates

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use web_sys::{window, Storage};

// ═══════════════════════════════════════════════════════════════════════
// CORE RESOURCES
// ═══════════════════════════════════════════════════════════════════════

#[derive(Resource, Clone, Debug, Serialize, Deserialize, Default, Reflect)]
#[reflect(Resource)]
pub struct ClientSettings {
    pub graphics: GraphicsSettings,
    pub audio: AudioSettings,
    pub gameplay: GameplaySettings,
    pub network: NetworkSettings,
    pub accessibility: AccessibilitySettings,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Reflect)]
pub struct GraphicsSettings {
    pub quality_preset: QualityPreset,
    pub fov: f32,                    // 60.0 - 120.0
    pub render_distance: f32,        // 50.0 - 500.0
    pub vsync: bool,
    pub shadows: bool,
    pub postfx: bool,
    pub webxr_motion_smoothing: bool,
    pub webxr_snap_turn: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Reflect)]
pub enum QualityPreset {
    Seedling,      // Low / Mobile friendly
    #[default]
    FlowGuardian,  // Balanced mercy-recommended
    Eternal,       // High / Cinematic
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Reflect)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub whispers_volume: f32,
    pub spatial_quality: u8,     // 1-4 ambisonics order
    pub hrtf_enabled: bool,
    pub doppler_strength: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Reflect)]
pub struct GameplaySettings {
    pub whisper_frequency: f32,      // 0.2 - 2.0
    pub ui_scale: f32,
    pub mercy_feedback_intensity: f32,
    pub abundance_notification_style: AbundanceStyle,
    pub auto_harvest_hints: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Reflect)]
pub enum AbundanceStyle {
    Minimal,
    #[default]
    Balanced,
    Celebratory,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Reflect)]
pub struct NetworkSettings {
    pub prediction_aggressiveness: f32,
    pub bandwidth_saver: bool,
    pub auto_reconnect: bool,
    pub show_ping_overlay: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Reflect)]
pub struct AccessibilitySettings {
    pub colorblind_mode: ColorblindMode,
    pub reduced_motion: bool,
    pub high_contrast: bool,
    pub text_scale: f32,
    pub whisper_tts: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Reflect)]
pub enum ColorblindMode {
    #[default]
    None,
    Protanopia,
    Deuteranopia,
    Tritanopia,
}

// ═══════════════════════════════════════════════════════════════════════
// SERVER RULES (basic for display — full network sync in parallel track)
// ═══════════════════════════════════════════════════════════════════════

#[derive(Resource, Clone, Debug, Serialize, Deserialize, Default)]
pub struct ServerRules {
    pub instance_name: String,
    pub max_render_distance: f32,
    pub mercy_enforcement_level: f32,
    pub event_rate_multiplier: f32,
    pub griefing_tolerance: String,
    pub abundance_pooling: bool,
}

impl Default for ServerRules {
    fn default() -> Self {
        Self {
            instance_name: "Eternal Flow Instance — PATSAGi Sovereign".to_string(),
            max_render_distance: 250.0,
            mercy_enforcement_level: 0.92,
            event_rate_multiplier: 1.0,
            griefing_tolerance: "Mercy-Gated (strict, zero-harm)".to_string(),
            abundance_pooling: true,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// PERSISTENCE (WASM localStorage primary for WebXR + desktop fallback)
// ═══════════════════════════════════════════════════════════════════════

pub fn load_client_settings() -> ClientSettings {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json)) = storage.get_item("powrush_client_settings_v17.23") {
                    if let Ok(settings) = serde_json::from_str::<ClientSettings>(&json) {
                        info!("[Settings] Loaded ClientSettings from localStorage (v17.23)");
                        return settings;
                    }
                }
            }
        }
    }
    // First run or desktop: mercy-recommended defaults
    let mut settings = ClientSettings::default();
    settings.graphics.fov = 75.0;
    settings.audio.master_volume = 0.85;
    settings.audio.whispers_volume = 0.9;
    settings.gameplay.mercy_feedback_intensity = 0.8;
    settings
}

pub fn save_client_settings(settings: &ClientSettings) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(json) = serde_json::to_string_pretty(settings) {
                    let _ = storage.set_item("powrush_client_settings_v17.23", &json);
                    info!("[Settings] Saved ClientSettings to localStorage");
                }
            }
        }
    }
    // TODO: Desktop RON/JSON file persistence in future sovereign_core
}

// ═══════════════════════════════════════════════════════════════════════
// APPLY SYSTEMS (live where safe — called on resource change or Apply button)
// ═══════════════════════════════════════════════════════════════════════

pub fn apply_graphics_settings(
    settings: Res<ClientSettings>,
    mut camera_query: Query<&mut Projection, With<Camera>>,
) {
    for mut projection in camera_query.iter_mut() {
        if let Projection::Perspective(persp) = projection.as_mut() {
            persp.fov = settings.graphics.fov.to_radians();
        }
    }
    // TODO: Wire quality_preset → bevy::render::settings, shadow quality, render distance via DistanceFog or chunk manager
}

pub fn apply_audio_settings(
    settings: Res<ClientSettings>,
    // TODO: Inject existing DivineAudio / Kira manager resource when available
) {
    // Example future wiring:
    // if let Some(audio) = audio_manager {
    //     audio.set_master_volume(settings.audio.master_volume);
    //     audio.set_whisper_volume(settings.audio.whispers_volume);
    // }
}

pub fn apply_gameplay_settings(_settings: Res<ClientSettings>) {
    // Whisper frequency, UI scale multipliers, mercy intensity
}

// ═══════════════════════════════════════════════════════════════════════
// PLUGIN
// ═══════════════════════════════════════════════════════════════════════

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ClientSettings>()
            .init_resource::<ServerRules>()
            .add_systems(Startup, load_and_apply_initial_settings)
            .add_systems(Update, (
                apply_graphics_settings.run_if(resource_changed::<ClientSettings>),
                apply_audio_settings.run_if(resource_changed::<ClientSettings>),
                apply_gameplay_settings.run_if(resource_changed::<ClientSettings>),
            ));
    }
}

fn load_and_apply_initial_settings(mut commands: Commands) {
    let settings = load_client_settings();
    commands.insert_resource(settings);
    info!("[Settings] ClientSettings + ServerRules initialized (mercy defaults loaded)");
}
