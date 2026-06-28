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

/// Audio contexts that influence ramp behavior (maps to MusicStateType + region)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AudioContext {
    Exploration,
    Combat,
    SuddenEvent,
    Crafting,
    LongDistanceTravel,
    LargeEvent,
}

/// Emotional weight of a transition or state change
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmotionalWeight {
    Low,
    Medium,
    High,
}

#[derive(Resource, Default)]
pub struct AdaptiveLayeringState {
    pub current_palette: PaletteType,
    pub current_intensity: f32,           // 0.0 - 1.0
    pub target_intensity: f32,
    pub is_transitioning: bool,
    pub current_industrial_intensity: f32, // 0.0 - 100.0 (from metrics/combat)
    pub current_world_tension: f32,        // 0.0 - 1.0
}

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

/// Core dynamic ramp time calculator - hybrid, tunable, data-driven ready
pub fn calculate_dynamic_ramp_time(
    context: AudioContext,
    current_industrial_intensity: f32,
    emotional_weight: EmotionalWeight,
    distance_factor: f32,
    world_tension: f32,
) -> f32 {
    let base: f32 = 6.0;

    match context {
        AudioContext::Combat | AudioContext::SuddenEvent => {
            let ramp = base * 0.35;
            ramp.clamp(1.5, 4.0)
        }
        AudioContext::LongDistanceTravel => {
            let ramp = base * 1.7;
            ramp.clamp(8.0, 15.0)
        }
        _ => {
            let mut ramp = base;

            let intensity_factor = 1.0 + (current_industrial_intensity / 100.0) * 0.5;
            ramp *= intensity_factor;

            if emotional_weight == EmotionalWeight::High {
                ramp *= 1.35;
            }

            if world_tension > 0.7 {
                ramp *= 0.9;
            }

            ramp *= (1.0 + distance_factor * 0.3).clamp(1.0, 2.0);

            ramp.clamp(3.0, 12.0)
        }
    }
}

/// Main system consuming PaletteTransitionEvents and managing state + latency
pub fn adaptive_layering_system(
    mut state: ResMut<AdaptiveLayeringState>,
    mut events: EventReader<PaletteTransitionEvent>,
    time: Res<Time>,
    mut latency_metrics: ResMut<AudioLatencyMetrics>,
    mut music_controller: Option<ResMut<MusicController>>,
) {
    for event in events.read() {
        latency_metrics.record_crossfade_start(time.elapsed_secs());

        state.target_intensity = event.target_intensity;
        state.is_transitioning = true;

        if let Some(ref mut mc) = music_controller {
            // Future: map PaletteType to MusicStateType + set mc.transition_duration = event.ramp_time
        }

        trigger_palette_crossfade(event, &mut state);
    }

    if state.is_transitioning {
        let lerp_speed = 2.0;
        state.current_intensity = state.current_intensity
            + (state.target_intensity - state.current_intensity) * lerp_speed * time.delta_seconds();
        if (state.current_intensity - state.target_intensity).abs() < 0.01 {
            state.current_intensity = state.target_intensity;
            state.is_transitioning = false;
        }
    }
}

fn trigger_palette_crossfade(event: &PaletteTransitionEvent, state: &mut AdaptiveLayeringState) {
    state.current_palette = event.target_palette;

    // Integration point with kira crossfades (now accept ramp_time)
    // Example call site (uncomment when Kira*Controller types are defined):
    // if event.target_palette == PaletteType::IndustrialPulse {
    //     // let mut controller = ... get or res
    //     crate::audio::kira_music::start_music_crossfade(
    //         &mut controller, &new_ir, wetness, mix, source, audio, time, latency, event.ramp_time
    //     );
    // } else {
    //     crate::audio::kira_ambient::start_crossfade(... event.ramp_time);
    // }
    // Same ramp_time guarantees music layers + spatial reverb transition together.

    if event.priority == TransitionPriority::Combat {
        // e.g. force ducking or shorter perceived ramp
    }

    #[cfg(debug_assertions)]
    info!("[AdaptiveLayering] {:?} intensity {:.2} ramp {:.1}s prio {:?}", 
          event.target_palette, event.target_intensity, event.ramp_time, event.priority);
}

pub fn request_combat_palette(
    mut event_writer: EventWriter<PaletteTransitionEvent>,
    layering_state: Res<AdaptiveLayeringState>,
    config: Res<AdaptiveAudioConfig>,
) {
    let ramp_time = calculate_dynamic_ramp_time(
        AudioContext::Combat,
        layering_state.current_industrial_intensity,
        EmotionalWeight::Medium,
        1.0,
        layering_state.current_world_tension,
    );

    event_writer.send(PaletteTransitionEvent {
        target_palette: PaletteType::IndustrialPulse,
        target_intensity: 0.75,
        ramp_time,
        priority: TransitionPriority::Combat,
    });
}

/// Region transition handler with biome + distance weighting
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

        let biome_multiplier = current_biome.active_profile.ramp_time_multiplier;
        let effective_multiplier = config.default_region_ramp_multiplier * biome_multiplier;

        let ramp_time = calculate_dynamic_ramp_time(
            AudioContext::Exploration,
            layering_state.current_industrial_intensity,
            EmotionalWeight::Medium,
            distance_factor,
            layering_state.current_world_tension,
        ) * effective_multiplier;

        let target_palette = if current_biome.active_profile.name.contains("forest") || current_biome.active_profile.name.contains("wild") {
            PaletteType::ResonantVeil
        } else if current_biome.active_profile.name.contains("industrial") {
            PaletteType::IndustrialPulse
        } else {
            PaletteType::EchoingWisp
        };

        palette_writer.send(PaletteTransitionEvent {
            target_palette,
            target_intensity: 0.6,
            ramp_time: ramp_time.clamp(config.min_ramp_time, config.max_ramp_time),
            priority: TransitionPriority::Normal,
        });

        #[cfg(debug_assertions)]
        info!("[AdaptiveLayering] Region {} -> {} | dist {:.0}m | biome_mult {:.2} | ramp {:.1}s -> {:?}",
              event.from_region, event.to_region, event.distance, biome_multiplier, ramp_time, target_palette);
    }
}
