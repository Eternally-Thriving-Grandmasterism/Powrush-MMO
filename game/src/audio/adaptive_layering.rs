/*!
 * Adaptive Layering System - Emit dedicated hot reload events
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
use crate::settings::biome_acoustic::CurrentBiomeAcoustics;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ... (enums, AdaptiveLayeringState, AdaptiveAudioConfig, calculate_dynamic_ramp_time, etc. unchanged)

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AudioContext { Exploration, Combat, SuddenEvent, Crafting, LongDistanceTravel, LargeEvent }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmotionalWeight { Low, Medium, High }

#[derive(Resource, Default, Debug)]
pub struct AudioEventMetrics { /* ... existing fields and methods ... */ }

#[derive(Resource, Default)]
pub struct AdaptiveLayeringState { /* ... */ }

#[derive(Resource, Clone)]
pub struct AdaptiveAudioConfig { /* ... */ }

impl Default for AdaptiveAudioConfig { fn default() -> Self { /* ... */ } }

pub fn calculate_dynamic_ramp_time(...) -> f32 { /* ... */ }

pub fn adaptive_layering_system(...) { /* ... */ }
fn trigger_palette_crossfade(...) { /* ... */ }
pub fn request_combat_palette(...) { /* ... */ }
pub fn region_audio_transition_system(...) { /* ... */ }
pub fn palette_to_music_mapping_system(...) { /* ... */ }
pub fn feed_combat_intensity(...) { /* ... */ }
pub fn combat_intensity_system(...) { /* ... */ }

// === Hot Reload Systems (now emit dedicated events) ===

pub fn hot_reload_region_palette_system(
    mut asset_events: EventReader<AssetEvent<RegionPaletteConfig>>,
    region_palette_assets: Res<Assets<RegionPaletteConfig>>,
    region_palette_handle: Res<RegionPaletteConfigHandle>,
    current_region: Res<CurrentRegion>,
    mut palette_writer: EventWriter<PaletteTransitionEvent>,
    config: Res<AdaptiveAudioConfig>,
    mut metrics: ResMut<AudioEventMetrics>,
    mut hot_reload_writer: EventWriter<RegionPaletteConfigReloaded>,
    time: Res<Time>,
) {
    for event in asset_events.read() {
        if let AssetEvent::LoadedWithDependencies { id } | AssetEvent::Modified { id } = event {
            if let Some(handle) = &region_palette_handle.0 {
                if handle.id() == *id {
                    metrics.record_region_palette_hot_reload();

                    if let Some(cfg) = region_palette_assets.get(handle) {
                        // Emit dedicated hot reload event
                        hot_reload_writer.send(RegionPaletteConfigReloaded {
                            mappings_count: cfg.mappings.len(),
                            default_palette: cfg.default_palette,
                            timestamp: time.elapsed_secs(),
                        });

                        info!("[HotReload] RegionPaletteConfig updated ({} mappings).", cfg.mappings.len());

                        if let Some(region) = current_region.0 {
                            let target_palette = *cfg.mappings.get(&region).unwrap_or(&cfg.default_palette);
                            palette_writer.send(PaletteTransitionEvent {
                                target_palette,
                                target_intensity: 0.6,
                                ramp_time: 2.0,
                                priority: TransitionPriority::Event,
                            });
                        }
                    }
                }
            }
        }
    }
}

pub fn hot_reload_ai_config_system(
    mut asset_events: EventReader<AssetEvent<AIConfigAsset>>,
    ai_assets: Res<Assets<AIConfigAsset>>,
    ai_handle: Res<AIConfigHandle>,
    mut ai_config: ResMut<AIConfig>,
    mut metrics: ResMut<AudioEventMetrics>,
    mut hot_reload_writer: EventWriter<AIConfigReloaded>,
    time: Res<Time>,
) {
    for event in asset_events.read() {
        if let AssetEvent::LoadedWithDependencies { id } | AssetEvent::Modified { id } = event {
            if let Some(handle) = &ai_handle.0 {
                if handle.id() == *id {
                    metrics.record_ai_config_hot_reload();

                    if let Some(loaded) = ai_assets.get(handle) {
                        ai_config.base_aggression = loaded.base_aggression;
                        ai_config.combat_intensity_scale = loaded.combat_intensity_scale;
                        ai_config.region_aggression_modifiers = loaded.region_aggression_modifiers.clone();
                        ai_config.tension_ramp_rate = loaded.tension_ramp_rate;

                        hot_reload_writer.send(AIConfigReloaded {
                            combat_intensity_scale: loaded.combat_intensity_scale,
                            base_aggression: loaded.base_aggression,
                            timestamp: time.elapsed_secs(),
                        });

                        info!("[HotReload] AIConfig reloaded!");
                    }
                }
            }
        }
    }
}

// AIConfig and RegionPaletteConfig types (unchanged from previous)
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

#[derive(Resource, Default)]
pub struct CurrentRegion(pub Option<RegionType>);
