/*!
 * Adaptive Layering System - Dynamic Ramp Times for Music Palettes, IR/Reverb & Intensity
 *
 * Region weighting via RegionTransitionEvent + biome distance factor + ramp_time_multiplier
 * from BiomeAcousticProfile. calculate_dynamic_ramp_time already supports distance_factor.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Powrush-MMO
 */

use bevy::prelude::*;
use super::events::{PaletteTransitionEvent, PaletteType, TransitionPriority, RegionTransitionEvent};
use super::super::latency_metrics::AudioLatencyMetrics;
use super::super::music::MusicController;
use crate::settings::biome_acoustic::CurrentBiomeAcoustics;

/// Audio contexts...
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AudioContext {
    Exploration,
    Combat,
    SuddenEvent,
    Crafting,
    LongDistanceTravel,
    LargeEvent,
}

/// Emotional weight...
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmotionalWeight {
    Low,
    Medium,
    High,
}

#[derive(Resource, Default)]
pub struct AdaptiveLayeringState { ... } // unchanged from previous

#[derive(Resource, Clone)]
pub struct AdaptiveAudioConfig {
    pub combat_ramp_multiplier: f32,
    pub long_travel_ramp_multiplier: f32,
    pub emotional_high_ramp_multiplier: f32,
    pub max_ramp_time: f32,
    pub min_ramp_time: f32,
    pub combat_ramp_down_multiplier: f32,
    pub default_region_ramp_multiplier: f32,
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
        }
    }
}

// calculate_dynamic_ramp_time unchanged (already uses distance_factor)

// adaptive_layering_system unchanged

fn trigger_palette_crossfade(...) { ... } // unchanged

pub fn request_combat_palette(...) { ... } // unchanged

/// NEW: Region transition handler with biome + distance weighting
/// Listens to RegionTransitionEvent, applies biome.ramp_time_multiplier + distance_factor,
/// then emits PaletteTransitionEvent with Exploration context and calculated ramp.
pub fn region_audio_transition_system(
    mut region_events: EventReader<RegionTransitionEvent>,
    mut palette_writer: EventWriter<PaletteTransitionEvent>,
    current_biome: Res<CurrentBiomeAcoustics>,
    layering_state: Res<AdaptiveLayeringState>,
    config: Res<AdaptiveAudioConfig>,
) {
    for event in region_events.read() {
        let distance_factor = (event.distance / 1000.0).clamp(0.0, 2.0);

        // Biome-specific weighting (data-driven from BiomeAcousticProfile)
        let biome_multiplier = current_biome.active_profile.ramp_time_multiplier;
        let effective_multiplier = config.default_region_ramp_multiplier * biome_multiplier;

        // Use Exploration context for region changes (can extend later with biome->palette mapping)
        let ramp_time = calculate_dynamic_ramp_time(
            AudioContext::Exploration,
            layering_state.current_industrial_intensity,
            EmotionalWeight::Medium,
            distance_factor,
            layering_state.current_world_tension,
        ) * effective_multiplier;

        // Simple palette suggestion based on biome name (extend with proper RegionType later)
        let target_palette = if current_biome.active_profile.name.contains("forest") || current_biome.active_profile.name.contains("wild") {
            PaletteType::ResonantVeil
        } else if current_biome.active_profile.name.contains("industrial") {
            PaletteType::IndustrialPulse
        } else {
            PaletteType::EchoingWisp
        };

        palette_writer.send(PaletteTransitionEvent {
            target_palette,
            target_intensity: 0.6, // region default intensity
            ramp_time: ramp_time.clamp(config.min_ramp_time, config.max_ramp_time),
            priority: TransitionPriority::Normal,
        });

        #[cfg(debug_assertions)]
        info!("[AdaptiveLayering] Region {} -> {} | dist {:.0}m | biome_mult {:.2} | ramp {:.1}s -> {:?}",
              event.from_region, event.to_region, event.distance, biome_multiplier, ramp_time, target_palette);
    }
}
