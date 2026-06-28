/*!
 * Adaptive Layering System - Per-config reload volume options
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Powrush-MMO
 */

use bevy::prelude::*;
use bevy::asset::{Asset, AssetLoader, LoadContext, io::Reader, Handle, AssetEvent};
use super::events::{
    PaletteTransitionEvent, PaletteType, TransitionPriority,
    RegionTransitionEvent, RegionType, CombatStateChangedEvent,
    RegionPaletteConfigReloaded, AIConfigReloaded,
};
use super::super::latency_metrics::AudioLatencyMetrics;
use super::super::music::{MusicController, MusicStateType};
use crate::settings::audio_mixing::{AudioMixer, DynamicAudio, AudioCategory, Priority};
use crate::settings::biome_acoustic::CurrentBiomeAcoustics;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Core types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AudioContext { Exploration, Combat, SuddenEvent, Crafting, LongDistanceTravel, LargeEvent }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmotionalWeight { Low, Medium, High }

#[derive(Resource, Default, Debug)]
pub struct AudioEventMetrics { /* ... */ }

#[derive(Resource, Default)]
pub struct AdaptiveLayeringState { /* ... */ }

// Extended with per-config reload volumes
#[derive(Resource, Clone)]
pub struct AdaptiveAudioConfig {
    pub combat_ramp_multiplier: f32,
    pub long_travel_ramp_multiplier: f32,
    pub emotional_high_ramp_multiplier: f32,
    pub max_ramp_time: f32,
    pub min_ramp_time: f32,
    pub combat_ramp_down_multiplier: f32,
    pub default_region_ramp_multiplier: f32,

    // Hot reload audio feedback volumes (multiplier on mixer.ui)
    pub region_palette_reload_volume: f32,
    pub ai_config_reload_volume: f32,
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

            // Sensible defaults for reload feedback
            region_palette_reload_volume: 0.9,
            ai_config_reload_volume: 0.85,
        }
    }
}

pub fn calculate_dynamic_ramp_time(...) -> f32 { /* ... */ }

pub fn adaptive_layering_system(...) { /* ... */ }
fn trigger_palette_crossfade(...) { /* ... */ }
pub fn request_combat_palette(...) { /* ... */ }
pub fn region_audio_transition_system(...) { /* ... */ }
pub fn palette_to_music_mapping_system(...) { /* ... */ }
pub fn feed_combat_intensity(...) { /* ... */ }
pub fn combat_intensity_system(...) { /* ... */ }
pub fn hot_reload_region_palette_system(...) { /* ... */ }
pub fn hot_reload_ai_config_system(...) { /* ... */ }

// === Audio Feedback with Configurable Volume ===

pub fn on_region_palette_config_reloaded(
    mut events: EventReader<RegionPaletteConfigReloaded>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mixer: Res<AudioMixer>,
    config: Res<AdaptiveAudioConfig>,
) {
    for event in events.read() {
        let sound = asset_server.load("audio/ui/region_palette_reload.ogg");

        let pitch_variation = 0.95 + (rand::random::<f32>() * 0.1);
        let volume = mixer.ui * config.region_palette_reload_volume;

        commands.spawn((
            AudioBundle {
                source: sound,
                settings: PlaybackSettings::ONCE
                    .with_volume(volume)
                    .with_pitch(pitch_variation),
            },
            DynamicAudio {
                category: AudioCategory::Music,
                priority: Priority::High,
            },
        ));

        info!("[Audio] RegionPalette reload ({} mappings, vol={:.2})", event.mappings_count, volume);
    }
}

pub fn on_ai_config_reloaded(
    mut events: EventReader<AIConfigReloaded>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mixer: Res<AudioMixer>,
    config: Res<AdaptiveAudioConfig>,
) {
    for event in events.read() {
        let sound = asset_server.load("audio/ui/ai_config_reload.ogg");

        let pitch_variation = 0.97 + (rand::random::<f32>() * 0.08);
        let volume = mixer.ui * config.ai_config_reload_volume;

        commands.spawn((
            AudioBundle {
                source: sound,
                settings: PlaybackSettings::ONCE
                    .with_volume(volume)
                    .with_pitch(pitch_variation),
            },
            DynamicAudio {
                category: AudioCategory::Music,
                priority: Priority::High,
            },
        ));

        info!("[Audio] AIConfig reload (scale={:.2}, vol={:.2})", event.combat_intensity_scale, volume);
    }
}

// Supporting types
#[derive(Resource, Default)]
pub struct CurrentRegion(pub Option<RegionType>);

#[derive(Resource, Default, Clone, Debug, Serialize, Deserialize)]
pub struct AIConfig { /* ... */ }

#[derive(Resource, Default)]
pub struct AIConfigHandle(pub Option<Handle<AIConfigAsset>>);

#[derive(Asset, TypePath, Debug, Clone, Serialize, Deserialize, Default)]
pub struct AIConfigAsset { /* ... */ }

pub struct AIConfigLoader;

impl AssetLoader for AIConfigLoader { /* ... */ }

pub fn load_ai_config(...) { /* ... */ }

#[derive(Asset, TypePath, Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegionPaletteConfig { /* ... */ }

#[derive(Resource, Default)]
pub struct RegionPaletteConfigHandle(pub Option<Handle<RegionPaletteConfig>>);

pub struct RegionPaletteLoader;

impl AssetLoader for RegionPaletteLoader { /* ... */ }

pub fn load_region_palette_config(...) { /* ... */ }
