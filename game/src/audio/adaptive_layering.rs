/*!
 * Adaptive Layering System - Hot Reload for AI Configs
 *
 * AI behavior parameters (aggression, intensity response, region modifiers) now hot-reloadable via RON.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Powrush-MMO
 */

use bevy::prelude::*;
use bevy::asset::{Asset, AssetLoader, LoadContext, io::Reader, Handle, AssetEvent};
use super::events::{PaletteTransitionEvent, PaletteType, TransitionPriority, RegionTransitionEvent, RegionType, CombatStateChangedEvent};
use super::super::latency_metrics::AudioLatencyMetrics;
use super::super::music::{MusicController, MusicStateType};
use crate::settings::biome_acoustic::CurrentBiomeAcoustics;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ... existing enums and core systems (kept compact)

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AudioContext { Exploration, Combat, SuddenEvent, Crafting, LongDistanceTravel, LargeEvent }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmotionalWeight { Low, Medium, High }

#[derive(Resource, Default)]
pub struct AdaptiveLayeringState {
    pub current_palette: PaletteType,
    pub current_intensity: f32,
    pub target_intensity: f32,
    pub is_transitioning: bool,
    pub current_industrial_intensity: f32,
    pub current_world_tension: f32,
}

#[derive(Resource, Clone)]
pub struct AdaptiveAudioConfig { /* ... */ 
    pub combat_ramp_multiplier: f32,
    pub long_travel_ramp_multiplier: f32,
    pub emotional_high_ramp_multiplier: f32,
    pub max_ramp_time: f32,
    pub min_ramp_time: f32,
    pub combat_ramp_down_multiplier: f32,
    pub default_region_ramp_multiplier: f32,
}

impl Default for AdaptiveAudioConfig {
    fn default() -> Self { Self { combat_ramp_multiplier: 0.35, long_travel_ramp_multiplier: 1.7, emotional_high_ramp_multiplier: 1.35, max_ramp_time: 15.0, min_ramp_time: 1.5, combat_ramp_down_multiplier: 1.5, default_region_ramp_multiplier: 1.0 } }
}

pub fn calculate_dynamic_ramp_time(...) -> f32 { /* previous implementation */ 
    let base: f32 = 6.0;
    match context {
        AudioContext::Combat | AudioContext::SuddenEvent => (base * 0.35).clamp(1.5, 4.0),
        AudioContext::LongDistanceTravel => (base * 1.7).clamp(8.0, 15.0),
        _ => {
            let mut ramp = base * (1.0 + (current_industrial_intensity / 100.0) * 0.5);
            if emotional_weight == EmotionalWeight::High { ramp *= 1.35; }
            if world_tension > 0.7 { ramp *= 0.9; }
            ramp *= (1.0 + distance_factor * 0.3).clamp(1.0, 2.0);
            ramp.clamp(3.0, 12.0)
        }
    }
}

// Core systems (abbreviated for this edit - logic is unchanged from previous commits)
pub fn adaptive_layering_system(...) { /* ... */ }
fn trigger_palette_crossfade(...) { /* ... */ }
pub fn request_combat_palette(...) { /* ... */ }
pub fn region_audio_transition_system(...) { /* ... with CurrentRegion tracking */ }
pub fn palette_to_music_mapping_system(...) { /* ... */ }
pub fn feed_combat_intensity(...) { /* ... */ }
pub fn combat_intensity_system(...) { /* ... */ }
pub fn hot_reload_region_palette_system(...) { /* enhanced version with re-apply */ }

// === AI Config with Hot Reload ===

#[derive(Resource, Default, Clone, Debug, Serialize, Deserialize)]
pub struct AIConfig {
    pub base_aggression: f32,
    pub combat_intensity_scale: f32,
    pub region_aggression_modifiers: HashMap<RegionType, f32>,
    pub tension_ramp_rate: f32,
}

impl AIConfig {
    pub fn with_defaults() -> Self {
        let mut mods = HashMap::new();
        mods.insert(RegionType::Industrial, 1.4);
        mods.insert(RegionType::Urban, 1.3);
        mods.insert(RegionType::Forest, 0.85);
        mods.insert(RegionType::Wilderness, 0.9);
        Self {
            base_aggression: 1.0,
            combat_intensity_scale: 1.0,
            region_aggression_modifiers: mods,
            tension_ramp_rate: 0.8,
        }
    }
}

#[derive(Resource, Default)]
pub struct AIConfigHandle(pub Option<Handle<AIConfig>>);

#[derive(Asset, TypePath, Debug, Clone, Serialize, Deserialize, Default)]
pub struct AIConfigAsset {
    pub base_aggression: f32,
    pub combat_intensity_scale: f32,
    pub region_aggression_modifiers: HashMap<RegionType, f32>,
    pub tension_ramp_rate: f32,
}

pub struct AIConfigLoader;

impl AssetLoader for AIConfigLoader {
    type Asset = AIConfigAsset;
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
        let asset: AIConfigAsset = ron::de::from_bytes(&bytes)
            .map_err(|e| anyhow::anyhow!("Failed to parse AIConfig RON: {}", e))?;
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["ron", "ai.ron"]
    }
}

pub fn load_ai_config(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handle = asset_server.load("config/ai_behavior.ron");
    commands.insert_resource(AIConfigHandle(Some(handle)));
}

pub fn hot_reload_ai_config_system(
    mut asset_events: EventReader<AssetEvent<AIConfigAsset>>,
    ai_assets: Res<Assets<AIConfigAsset>>,
    ai_handle: Res<AIConfigHandle>,
    mut ai_config: ResMut<AIConfig>,
) {
    for event in asset_events.read() {
        if let AssetEvent::LoadedWithDependencies { id } | AssetEvent::Modified { id } = event {
            if let Some(handle) = &ai_handle.0 {
                if handle.id() == *id {
                    if let Some(loaded) = ai_assets.get(handle) {
                        ai_config.base_aggression = loaded.base_aggression;
                        ai_config.combat_intensity_scale = loaded.combat_intensity_scale;
                        ai_config.region_aggression_modifiers = loaded.region_aggression_modifiers.clone();
                        ai_config.tension_ramp_rate = loaded.tension_ramp_rate;

                        info!("[HotReload] AIConfig reloaded! aggression={:.2} intensity_scale={:.2}", 
                              ai_config.base_aggression, ai_config.combat_intensity_scale);
                    }
                }
            }
        }
    }
}

// Make AIConfig available as Resource (populated from asset on load)
pub fn apply_ai_config_on_load(
    mut ev_asset: EventReader<AssetEvent<AIConfigAsset>>,
    ai_assets: Res<Assets<AIConfigAsset>>,
    ai_handle: Res<AIConfigHandle>,
    mut ai_config: ResMut<AIConfig>,
) {
    for event in ev_asset.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            if let Some(handle) = &ai_handle.0 {
                if handle.id() == *id {
                    if let Some(loaded) = ai_assets.get(handle) {
                        *ai_config = AIConfig {
                            base_aggression: loaded.base_aggression,
                            combat_intensity_scale: loaded.combat_intensity_scale,
                            region_aggression_modifiers: loaded.region_aggression_modifiers.clone(),
                            tension_ramp_rate: loaded.tension_ramp_rate,
                        };
                    }
                }
            }
        }
    }
}
