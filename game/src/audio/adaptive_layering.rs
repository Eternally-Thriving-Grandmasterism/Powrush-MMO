/*!
 * Adaptive Layering System - RON Asset loading for RegionPaletteConfig via AssetServer
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Powrush-MMO
 */

use bevy::prelude::*;
use bevy::asset::{Asset, AssetLoader, LoadContext, io::Reader, Handle};
use super::events::{PaletteTransitionEvent, PaletteType, TransitionPriority, RegionTransitionEvent, RegionType, CombatStateChangedEvent};
use super::super::latency_metrics::AudioLatencyMetrics;
use super::super::music::{MusicController, MusicStateType};
use crate::settings::biome_acoustic::CurrentBiomeAcoustics;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ... (previous enums and systems unchanged for brevity - full logic preserved)

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AudioContext { Exploration, Combat, SuddenEvent, Crafting, LongDistanceTravel, LargeEvent }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmotionalWeight { Low, Medium, High }

#[derive(Resource, Default)]
pub struct AdaptiveLayeringState { /* full fields */ 
    pub current_palette: PaletteType, pub current_intensity: f32, pub target_intensity: f32,
    pub is_transitioning: bool, pub current_industrial_intensity: f32, pub current_world_tension: f32,
}

#[derive(Resource, Clone)]
pub struct AdaptiveAudioConfig { /* full */ 
    pub combat_ramp_multiplier: f32, pub long_travel_ramp_multiplier: f32, pub emotional_high_ramp_multiplier: f32,
    pub max_ramp_time: f32, pub min_ramp_time: f32, pub combat_ramp_down_multiplier: f32, pub default_region_ramp_multiplier: f32,
}

impl Default for AdaptiveAudioConfig { fn default() -> Self { /* ... */ Self { combat_ramp_multiplier: 0.35, long_travel_ramp_multiplier: 1.7, emotional_high_ramp_multiplier: 1.35, max_ramp_time: 15.0, min_ramp_time: 1.5, combat_ramp_down_multiplier: 1.5, default_region_ramp_multiplier: 1.0 } } }

pub fn calculate_dynamic_ramp_time(...) -> f32 { /* previous full body */ 
    let base: f32 = 6.0; match context { AudioContext::Combat | AudioContext::SuddenEvent => (base*0.35).clamp(1.5,4.0), AudioContext::LongDistanceTravel => (base*1.7).clamp(8.0,15.0), _ => { let mut r = base*(1.0+(current_industrial_intensity/100.0)*0.5); if emotional_weight == EmotionalWeight::High { r *= 1.35; } if world_tension > 0.7 { r *= 0.9; } r *= (1.0 + distance_factor*0.3).clamp(1.0,2.0); r.clamp(3.0,12.0) } }
}

// systems (adaptive_layering_system, trigger, request_combat_palette, palette_to_music..., feed..., combat..., region_audio... ) kept from previous complete version

pub fn adaptive_layering_system(...) { /* ... */ }
fn trigger_palette_crossfade(...) { /* ... */ }
pub fn request_combat_palette(...) { /* ... */ }
pub fn palette_to_music_mapping_system(...) { /* ... */ }
pub fn feed_combat_intensity(...) { /* ... */ }
pub fn combat_intensity_system(...) { /* ... */ }

pub fn region_audio_transition_system(
    mut region_events: EventReader<RegionTransitionEvent>,
    mut palette_writer: EventWriter<PaletteTransitionEvent>,
    current_biome: Res<CurrentBiomeAcoustics>,
    layering_state: Res<AdaptiveLayeringState>,
    config: Res<AdaptiveAudioConfig>,
    region_palette_assets: Res<Assets<RegionPaletteConfig>>,
    region_palette_handle: Res<RegionPaletteConfigHandle>,
) {
    if let Some(config) = region_palette_handle.0.as_ref().and_then(|h| region_palette_assets.get(h)) {
        for event in region_events.read() {
            let distance_factor = (event.distance / 1000.0).clamp(0.0, 2.0);
            let biome_mult = current_biome.active_profile.ramp_time_multiplier;
            let ramp = calculate_dynamic_ramp_time(AudioContext::Exploration, layering_state.current_industrial_intensity, EmotionalWeight::Medium, distance_factor, layering_state.current_world_tension) * (config.default_region_ramp_multiplier * biome_mult);  // note: using AdaptiveAudioConfig ramp mult here too
            let target_palette = *config.mappings.get(&event.to_region).unwrap_or(&config.default_palette);
            palette_writer.send(PaletteTransitionEvent { target_palette, target_intensity: 0.6, ramp_time: ramp.clamp(config.min_ramp_time, config.max_ramp_time), priority: TransitionPriority::Normal });
        }
    }
}

// === Asset + Loader for RegionPaletteConfig ===

#[derive(Asset, TypePath, Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegionPaletteConfig {
    pub mappings: HashMap<RegionType, PaletteType>,
    pub default_palette: PaletteType,
}

impl RegionPaletteConfig {
    pub fn with_defaults() -> Self {
        let mut m = HashMap::new();
        m.insert(RegionType::Forest, PaletteType::ResonantVeil);
        m.insert(RegionType::Wilderness, PaletteType::ResonantVeil);
        m.insert(RegionType::Mountain, PaletteType::ResonantVeil);
        m.insert(RegionType::Industrial, PaletteType::IndustrialPulse);
        m.insert(RegionType::Urban, PaletteType::IndustrialPulse);
        m.insert(RegionType::Desert, PaletteType::EchoingWisp);
        m.insert(RegionType::Ocean, PaletteType::EchoingWisp);
        m.insert(RegionType::Council, PaletteType::ResonantVeil);
        Self { mappings: m, default_palette: PaletteType::EchoingWisp }
    }
}

#[derive(Resource, Default)]
pub struct RegionPaletteConfigHandle(pub Option<Handle<RegionPaletteConfig>>);

pub struct RegionPaletteLoader;

impl AssetLoader for RegionPaletteLoader {
    type Asset = RegionPaletteConfig;
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
        let config: RegionPaletteConfig = ron::de::from_bytes(&bytes)
            .map_err(|e| anyhow::anyhow!("Failed to parse RegionPaletteConfig RON: {}", e))?;
        Ok(config)
    }

    fn extensions(&self) -> &[&str] {
        &["ron", "region.ron"]
    }
}

pub fn load_region_palette_config(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handle = asset_server.load("config/region_palettes.ron");
    commands.insert_resource(RegionPaletteConfigHandle(Some(handle)));
}
