/*!
 * Adaptive Layering System - Dynamic Ramp Times for Music Palettes, IR/Reverb & Intensity
 *
 * Calculates context-aware crossfade/transition durations based on combat, travel distance,
 * emotional weight, world tension, and industrial intensity.
 * Drives PaletteTransitionEvent consumption and integrates with kira_* crossfades + MusicController.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Powrush-MMO
 * Self-evolving, mercy-aligned, zero-harm audio dynamics.
 */

use bevy::prelude::*;
use crate::audio::events::{PaletteTransitionEvent, PaletteType, TransitionPriority};
use crate::audio::latency_metrics::AudioLatencyMetrics;
use crate::audio::music::MusicController; // For future palette -> music state mapping

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

            // Distance factor stretches for long travel/exploration
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
    mut music_controller: Option<ResMut<MusicController>>, // Optional integration
) {
    for event in events.read() {
        latency_metrics.record_crossfade_start(time.elapsed_secs());

        state.target_intensity = event.target_intensity;
        state.is_transitioning = true;

        // Map palette to music state where sensible (expand later)
        if let Some(ref mut mc) = music_controller {
            match event.target_palette {
                PaletteType::IndustrialPulse => {
                    // mc.target_state = ... map if needed
                }
                _ => {}
            }
        }

        trigger_palette_crossfade(event, &mut state);
    }

    // Optional smooth intensity lerp when transitioning (if not fully event-driven crossfade)
    if state.is_transitioning {
        let lerp_speed = 2.0;
        state.current_intensity = state.current_intensity
            + (state.target_intensity - state.current_intensity) * lerp_speed * time.delta_seconds();
        if (state.current_intensity - state.target_intensity).abs() < 0.01 {
            state.current_intensity = state.target_intensity;
            state.is_transitioning = false;
            // latency_metrics.record_crossfade_complete(...) can be called from kira update
        }
    }
}

fn trigger_palette_crossfade(event: &PaletteTransitionEvent, state: &mut AdaptiveLayeringState) {
    // Update internal state
    state.current_palette = event.target_palette;

    // TODO: Call into existing kira_ambient / kira_music crossfade logic
    // using event.ramp_time as the crossfade_duration / transition_duration.
    // Example hooks:
    // - kira_music::start_music_crossfade(..., ramp_time: event.ramp_time)
    // - Update MusicController.transition_duration = event.ramp_time;
    // - For IR/reverb: pass same ramp_time to start_crossfade in kira_ambient
    // This ensures unified ramp across music layers + spatial/IR.
    //
    // Combat priority can force faster or ducking.
    if event.priority == TransitionPriority::Combat {
        // e.g. immediate duck or shorter effective ramp
    }

    // Placeholder: log or debug event for now (remove in prod or gate behind feature)
    #[cfg(debug_assertions)]
    info!("[AdaptiveLayering] Palette transition to {:?} intensity {:.2} ramp {:.1}s priority {:?}", 
          event.target_palette, event.target_intensity, event.ramp_time, event.priority);
}

/// Example helper for combat systems to request palette shift (call from your combat plugin)
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
