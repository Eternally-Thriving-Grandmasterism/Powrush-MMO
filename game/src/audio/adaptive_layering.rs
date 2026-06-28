/*!
 * Adaptive Layering System - Asset loading checks for audio feedback
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Powrush-MMO
 */

use bevy::prelude::*;
use bevy::asset::{Asset, AssetLoader, LoadContext, io::Reader, Handle, AssetEvent, LoadState};
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

// === Robust Audio Feedback with Asset Loading Checks ===

fn play_reload_feedback_sound(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mixer: &Res<AudioMixer>,
    path: &str,
    volume_mult: f32,
    pitch: f32,
) {
    let sound_handle: Handle<AudioSource> = asset_server.load(path);

    // Asset loading check
    match asset_server.get_load_state(&sound_handle) {
        Some(LoadState::Loaded) | Some(LoadState::Loading) => {
            commands.spawn((
                AudioBundle {
                    source: sound_handle,
                    settings: PlaybackSettings::ONCE
                        .with_volume(mixer.ui * volume_mult)
                        .with_pitch(pitch),
                },
                DynamicAudio {
                    category: AudioCategory::Music,
                    priority: Priority::High,
                },
            ));
        }
        Some(LoadState::Failed(_)) => {
            warn!("[Audio] Failed to load reload feedback sound: {}", path);
        }
        _ => {
            // Still loading or not yet processed - try again next frame if needed
            // For hot reload feedback, we can silently skip or queue it
        }
    }
}

// Listeners using the safe helper
pub fn on_region_palette_config_reloaded(
    mut events: EventReader<RegionPaletteConfigReloaded>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mixer: Res<AudioMixer>,
    config: Res<AdaptiveAudioConfig>,
) {
    for event in events.read() {
        let pitch = 0.95 + (rand::random::<f32>() * 0.1);
        play_reload_feedback_sound(
            &mut commands,
            &asset_server,
            &mixer,
            "audio/ui/region_palette_reload.ogg",
            config.region_palette_reload_volume,
            pitch,
        );
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
        let pitch = 0.97 + (rand::random::<f32>() * 0.08);
        play_reload_feedback_sound(
            &mut commands,
            &asset_server,
            &mixer,
            "audio/ui/ai_config_reload.ogg",
            config.ai_config_reload_volume,
            pitch,
        );
    }
}

// All previous systems remain
pub fn adaptive_layering_system(...) { /* ... */ }
// ... rest of file
