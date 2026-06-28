/*!
 * Adaptive Layering System - Audio feedback on hot reload
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
pub fn hot_reload_region_palette_system(...) { /* ... */ }
pub fn hot_reload_ai_config_system(...) { /* ... */ }

// === Audio Feedback on Hot Reload ===

pub fn on_region_palette_config_reloaded(
    mut events: EventReader<RegionPaletteConfigReloaded>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mixer: Res<AudioMixer>,
) {
    for event in events.read() {
        // Play a short UI/notification stinger for config reload feedback
        let sound = asset_server.load("audio/ui/config_reload.ogg"); // Replace with real asset
        commands.spawn((
            AudioBundle {
                source: sound,
                settings: PlaybackSettings::ONCE.with_volume(mixer.ui * 0.9),
            },
            DynamicAudio {
                category: AudioCategory::UI, // or SFX if you have it
                priority: Priority::High,
            },
        ));

        info!("[Audio] Played reload feedback for RegionPaletteConfig ({} mappings)", event.mappings_count);
    }
}

pub fn on_ai_config_reloaded(
    mut events: EventReader<AIConfigReloaded>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mixer: Res<AudioMixer>,
) {
    for event in events.read() {
        let sound = asset_server.load("audio/ui/config_reload.ogg"); // Same or different stinger
        commands.spawn((
            AudioBundle {
                source: sound,
                settings: PlaybackSettings::ONCE.with_volume(mixer.ui * 0.85),
            },
            DynamicAudio {
                category: AudioCategory::UI,
                priority: Priority::High,
            },
        ));

        info!("[Audio] Played reload feedback for AIConfig (scale={:.2})", event.combat_intensity_scale);
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
