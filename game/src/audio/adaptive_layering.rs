/*!
 * Adaptive Layering System - RON loading for AdaptiveAudioConfig
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

// Runtime config resource (populated from asset)
#[derive(Resource, Clone, Default)]
pub struct AdaptiveAudioConfig {
    pub combat_ramp_multiplier: f32,
    pub long_travel_ramp_multiplier: f32,
    pub emotional_high_ramp_multiplier: f32,
    pub max_ramp_time: f32,
    pub min_ramp_time: f32,
    pub combat_ramp_down_multiplier: f32,
    pub default_region_ramp_multiplier: f32,
    pub region_palette_reload_volume: f32,
    pub ai_config_reload_volume: f32,
}

// Asset version for RON loading
#[derive(Asset, TypePath, Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdaptiveAudioConfigAsset {
    pub combat_ramp_multiplier: f32,
    pub long_travel_ramp_multiplier: f32,
    pub emotional_high_ramp_multiplier: f32,
    pub max_ramp_time: f32,
    pub min_ramp_time: f32,
    pub combat_ramp_down_multiplier: f32,
    pub default_region_ramp_multiplier: f32,
    pub region_palette_reload_volume: f32,
    pub ai_config_reload_volume: f32,
}

#[derive(Resource, Default)]
pub struct AdaptiveAudioConfigHandle(pub Option<Handle<AdaptiveAudioConfigAsset>>);

pub struct AdaptiveAudioConfigLoader;

impl AssetLoader for AdaptiveAudioConfigLoader {
    type Asset = AdaptiveAudioConfigAsset;
    type Settings = ();
    type Error = anyhow::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset: AdaptiveAudioConfigAsset = ron::de::from_bytes(&bytes)
            .map_err(|e| anyhow::anyhow!("Failed to parse AdaptiveAudioConfig RON: {}", e))?;
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["ron", "audio_config.ron"]
    }
}

pub fn load_adaptive_audio_config(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handle = asset_server.load("config/adaptive_audio.ron");
    commands.insert_resource(AdaptiveAudioConfigHandle(Some(handle)));
}

pub fn apply_adaptive_audio_config_on_load(
    mut ev_asset: EventReader<AssetEvent<AdaptiveAudioConfigAsset>>,
    assets: Res<Assets<AdaptiveAudioConfigAsset>>,
    handle: Res<AdaptiveAudioConfigHandle>,
    mut config: ResMut<AdaptiveAudioConfig>,
) {
    for event in ev_asset.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            if let Some(h) = &handle.0 {
                if h.id() == *id {
                    if let Some(loaded) = assets.get(h) {
                        *config = AdaptiveAudioConfig {
                            combat_ramp_multiplier: loaded.combat_ramp_multiplier,
                            long_travel_ramp_multiplier: loaded.long_travel_ramp_multiplier,
                            emotional_high_ramp_multiplier: loaded.emotional_high_ramp_multiplier,
                            max_ramp_time: loaded.max_ramp_time,
                            min_ramp_time: loaded.min_ramp_time,
                            combat_ramp_down_multiplier: loaded.combat_ramp_down_multiplier,
                            default_region_ramp_multiplier: loaded.default_region_ramp_multiplier,
                            region_palette_reload_volume: loaded.region_palette_reload_volume,
                            ai_config_reload_volume: loaded.ai_config_reload_volume,
                        };
                        info!("[Config] AdaptiveAudioConfig loaded from RON");
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
) {
    for event in asset_events.read() {
        if let AssetEvent::LoadedWithDependencies { id } | AssetEvent::Modified { id } = event {
            if let Some(h) = &handle.0 {
                if h.id() == *id {
                    if let Some(loaded) = assets.get(h) {
                        config.combat_ramp_multiplier = loaded.combat_ramp_multiplier;
                        config.long_travel_ramp_multiplier = loaded.long_travel_ramp_multiplier;
                        config.emotional_high_ramp_multiplier = loaded.emotional_high_ramp_multiplier;
                        config.max_ramp_time = loaded.max_ramp_time;
                        config.min_ramp_time = loaded.min_ramp_time;
                        config.combat_ramp_down_multiplier = loaded.combat_ramp_down_multiplier;
                        config.default_region_ramp_multiplier = loaded.default_region_ramp_multiplier;
                        config.region_palette_reload_volume = loaded.region_palette_reload_volume;
                        config.ai_config_reload_volume = loaded.ai_config_reload_volume;

                        info!("[HotReload] AdaptiveAudioConfig updated from RON");
                    }
                }
            }
        }
    }
}

// Other systems and types (abbreviated)
pub fn adaptive_layering_system(...) { /* ... */ }
// ... (all previous systems remain)

#[derive(Resource, Default)]
pub struct CurrentRegion(pub Option<RegionType>);

#[derive(Resource, Default, Clone, Debug, Serialize, Deserialize)]
pub struct AIConfig { /* ... */ }

// ... (other supporting types)
