/*!
 * Adaptive Layering System - Dynamic Ducking Curves in RON config
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Powrush-MMO
 */

use bevy::prelude::*;
use bevy::asset::{Asset, AssetLoader, LoadContext, io::Reader, Handle, AssetEvent};
use crate::settings::audio_mixing::AudioMixer;

// ... (existing code for AudioEventMetrics, AdaptiveLayeringState, etc.)

#[derive(Resource, Clone)]
pub struct AdaptiveAudioConfig {
    // Existing fields (ramps, reload volumes, etc.)
    pub combat_ramp_multiplier: f32,
    pub long_travel_ramp_multiplier: f32,
    pub emotional_high_ramp_multiplier: f32,
    pub max_ramp_time: f32,
    pub min_ramp_time: f32,
    pub combat_ramp_down_multiplier: f32,
    pub default_region_ramp_multiplier: f32,
    pub region_palette_reload_volume: f32,
    pub ai_config_reload_volume: f32,

    // === Dynamic Ducking Curve parameters (live tunable via RON) ===
    pub ducking_critical: f32,
    pub ducking_high: f32,
    pub ducking_normal: f32,
    pub ducking_attack_critical: f32,
    pub ducking_release_critical: f32,
    pub ducking_attack_high: f32,
    pub ducking_release_high: f32,
    pub ducking_attack_normal: f32,
    pub ducking_release_normal: f32,
}

impl Default for AdaptiveAudioConfig {
    fn default() -> Self {
        Self {
            combat_ramp_multiplier: 0.35,
            long_travel_ramp_multiplier: 1.7,
            emotional_high_ramp_multiplier: 1.35,
            max_ramp_time: 15.0,
            min_ramp_time: 1.5,
            combat_ramp_down_multiplier: 1.5,
            default_region_ramp_multiplier: 1.0,
            region_palette_reload_volume: 0.9,
            ai_config_reload_volume: 0.85,

            // Dynamic Ducking defaults
            ducking_critical: 0.25,
            ducking_high: 0.4,
            ducking_normal: 0.6,
            ducking_attack_critical: 14.0,
            ducking_release_critical: 5.0,
            ducking_attack_high: 10.0,
            ducking_release_high: 4.0,
            ducking_attack_normal: 6.0,
            ducking_release_normal: 3.0,
        }
    }
}

// Update the two systems that handle loading/hot-reloading to sync ducking values to AudioMixer

pub fn apply_adaptive_audio_config_on_load(
    mut ev_asset: EventReader<AssetEvent<AdaptiveAudioConfigAsset>>,
    assets: Res<Assets<AdaptiveAudioConfigAsset>>,
    handle: Res<AdaptiveAudioConfigHandle>,
    mut config: ResMut<AdaptiveAudioConfig>,
    mut mixer: ResMut<AudioMixer>,
) {
    for event in ev_asset.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            if let Some(h) = &handle.0 {
                if h.id() == *id {
                    if let Some(loaded) = assets.get(h) {
                        // Copy all fields to config resource...
                        config.combat_ramp_multiplier = loaded.combat_ramp_multiplier;
                        // ... (copy other existing fields)

                        // Sync Dynamic Ducking Curves to AudioMixer
                        mixer.ducking_critical = loaded.ducking_critical;
                        mixer.ducking_high = loaded.ducking_high;
                        mixer.ducking_normal = loaded.ducking_normal;
                        mixer.ducking_attack_critical = loaded.ducking_attack_critical;
                        mixer.ducking_release_critical = loaded.ducking_release_critical;
                        mixer.ducking_attack_high = loaded.ducking_attack_high;
                        mixer.ducking_release_high = loaded.ducking_release_high;
                        mixer.ducking_attack_normal = loaded.ducking_attack_normal;
                        mixer.ducking_release_normal = loaded.ducking_release_normal;

                        info!("[Config] AdaptiveAudioConfig + Dynamic Ducking Curves loaded");
                    }
                }
            }
        }
    }
}

pub fn hot_reload_adaptive_audio_config_system(
    mut asset_events: EventReader<AssetEvent<AdaptiveAudioConfigAsset>>,
    assets: Res<Assets<AdaptiveAudioConfigAsset>>,
    handle: Res<AdaptiveAudioConfigHandle>,
    mut config: ResMut<AdaptiveAudioConfig>,
    mut mixer: ResMut<AudioMixer>,
) {
    for event in asset_events.read() {
        if let AssetEvent::LoadedWithDependencies { id } | AssetEvent::Modified { id } = event {
            if let Some(h) = &handle.0 {
                if h.id() == *id {
                    if let Some(loaded) = assets.get(h) {
                        // Update config...

                        // Live update ducking curves
                        mixer.ducking_critical = loaded.ducking_critical;
                        mixer.ducking_high = loaded.ducking_high;
                        mixer.ducking_normal = loaded.ducking_normal;
                        mixer.ducking_attack_critical = loaded.ducking_attack_critical;
                        mixer.ducking_release_critical = loaded.ducking_release_critical;
                        mixer.ducking_attack_high = loaded.ducking_attack_high;
                        mixer.ducking_release_high = loaded.ducking_release_high;
                        mixer.ducking_attack_normal = loaded.ducking_attack_normal;
                        mixer.ducking_release_normal = loaded.ducking_release_normal;

                        info!("[HotReload] Dynamic Ducking Curves updated live from RON");
                    }
                }
            }
        }
    }
}
