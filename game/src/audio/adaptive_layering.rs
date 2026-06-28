/*!
 * Adaptive Layering System - Full closed loop: RegionType + Palette ↔ MusicStateType + Combat intensity feed + data-driven region palettes
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Powrush-MMO
 */

use bevy::prelude::*;
use super::events::{PaletteTransitionEvent, PaletteType, TransitionPriority, RegionTransitionEvent, RegionType, CombatStateChangedEvent};
use super::super::latency_metrics::AudioLatencyMetrics;
use super::super::music::{MusicController, MusicStateType};
use crate::settings::biome_acoustic::CurrentBiomeAcoustics;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AudioContext {
    Exploration, Combat, SuddenEvent, Crafting, LongDistanceTravel, LargeEvent,
}

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

pub fn calculate_dynamic_ramp_time(
    context: AudioContext,
    current_industrial_intensity: f32,
    emotional_weight: EmotionalWeight,
    distance_factor: f32,
    world_tension: f32,
) -> f32 {
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
        if let Some(ref mut mc) = music_controller { /* mapping happens in dedicated system */ }
        trigger_palette_crossfade(event, &mut state);
    }
    if state.is_transitioning {
        let lerp_speed = 2.0;
        state.current_intensity += (state.target_intensity - state.current_intensity) * lerp_speed * time.delta_seconds();
        if (state.current_intensity - state.target_intensity).abs() < 0.01 {
            state.current_intensity = state.target_intensity;
            state.is_transitioning = false;
        }
    }
}

fn trigger_palette_crossfade(event: &PaletteTransitionEvent, state: &mut AdaptiveLayeringState) {
    state.current_palette = event.target_palette;
    #[cfg(debug_assertions)]
    info!("[Adaptive] Palette {:?} intensity {:.2} ramp {:.1}s", event.target_palette, event.target_intensity, event.ramp_time);
}

pub fn request_combat_palette(
    mut event_writer: EventWriter<PaletteTransitionEvent>,
    layering_state: Res<AdaptiveLayeringState>,
    config: Res<AdaptiveAudioConfig>,
) {
    let ramp = calculate_dynamic_ramp_time(AudioContext::Combat, layering_state.current_industrial_intensity, EmotionalWeight::Medium, 1.0, layering_state.current_world_tension);
    event_writer.send(PaletteTransitionEvent { target_palette: PaletteType::IndustrialPulse, target_intensity: 0.75, ramp_time: ramp, priority: TransitionPriority::Combat });
}

pub fn region_audio_transition_system(
    mut region_events: EventReader<RegionTransitionEvent>,
    mut palette_writer: EventWriter<PaletteTransitionEvent>,
    current_biome: Res<CurrentBiomeAcoustics>,
    layering_state: Res<AdaptiveLayeringState>,
    config: Res<AdaptiveAudioConfig>,
) {
    for event in region_events.read() {
        let distance_factor = (event.distance / 1000.0).clamp(0.0, 2.0);
        let biome_mult = current_biome.active_profile.ramp_time_multiplier;
        let ramp = calculate_dynamic_ramp_time(AudioContext::Exploration, layering_state.current_industrial_intensity, EmotionalWeight::Medium, distance_factor, layering_state.current_world_tension) * (config.default_region_ramp_multiplier * biome_mult);

        let target_palette = match event.to_region {
            RegionType::Forest | RegionType::Wilderness | RegionType::Mountain => PaletteType::ResonantVeil,
            RegionType::Industrial | RegionType::Urban => PaletteType::IndustrialPulse,
            _ => PaletteType::EchoingWisp,
        };
        palette_writer.send(PaletteTransitionEvent {
            target_palette,
            target_intensity: 0.6,
            ramp_time: ramp.clamp(config.min_ramp_time, config.max_ramp_time),
            priority: TransitionPriority::Normal,
        });
    }
}

/// PaletteType → MusicStateType mapping (drives music controller with adaptive ramp)
pub fn palette_to_music_mapping_system(
    mut palette_events: EventReader<PaletteTransitionEvent>,
    mut music: ResMut<MusicController>,
) {
    for event in palette_events.read() {
        let new_state = match event.target_palette {
            PaletteType::IndustrialPulse => MusicStateType::Combat,
            PaletteType::ResonantVeil => MusicStateType::Exploration,
            PaletteType::EchoingWisp => MusicStateType::Tension,
        };
        if music.target_state != new_state {
            music.target_state = new_state;
            music.transition_duration = event.ramp_time;
        }
        music.intensity = event.target_intensity.clamp(0.3, 1.2);
    }
}

/// Combat systems call this to feed industrial_intensity (used by ramp calc)
pub fn feed_combat_intensity(mut state: ResMut<AdaptiveLayeringState>, intensity: f32) {
    state.current_industrial_intensity = (intensity * 100.0).clamp(0.0, 100.0);
    if intensity > 0.6 { state.current_world_tension = (intensity * 0.8).clamp(0.0, 1.0); }
}

pub fn combat_intensity_system(
    mut combat_events: EventReader<CombatStateChangedEvent>,
    mut state: ResMut<AdaptiveLayeringState>,
) {
    for ev in combat_events.read() {
        if ev.entering_combat {
            feed_combat_intensity(state, ev.intensity);
        } else {
            state.current_industrial_intensity *= 0.6;
            state.current_world_tension *= 0.7;
        }
    }
}
