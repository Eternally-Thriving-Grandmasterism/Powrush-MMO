/*!
 * Audio Events - Palette Transitions, Combat & Region Events for Adaptive Layering
 *
 * Part of the AdaptiveLayeringSystem for dynamic music/IR/reverb ramps.
 * Integrates with MusicStateType, Kira crossfades, and biome acoustics.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Powrush-MMO
 * Eternal forward/backward compatible. Mercy-gated.
 */

use bevy::prelude::*;

/// Request to transition the music palette / intensity with adaptive ramp
#[derive(Event, Debug)]
pub struct PaletteTransitionEvent {
    pub target_palette: PaletteType,
    pub target_intensity: f32,      // 0.0 - 1.0
    pub ramp_time: f32,             // seconds, calculated dynamically
    pub priority: TransitionPriority,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum PaletteType {
    #[default]
    ResonantVeil,
    IndustrialPulse,
    EchoingWisp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionPriority {
    Normal,
    Combat,
    Event,
}

/// Combat state change for overriding palette weighting
#[derive(Event, Debug)]
pub struct CombatStateChangedEvent {
    pub entering_combat: bool,
    pub intensity: f32, // 0.0-1.0 threat level
}

/// Region transition for distance-aware ramp calculation
#[derive(Event, Debug)]
pub struct RegionTransitionEvent {
    pub from_region: String,
    pub to_region: String,
    pub distance: f32, // meters or normalized
}
