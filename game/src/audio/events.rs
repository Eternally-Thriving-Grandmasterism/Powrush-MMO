/*!
 * Audio Events - Palette Transitions, Combat & Region Events for Adaptive Layering
 *
 * Real RegionType enum + Combat intensity feeding for full closed-loop palette/region/combat audio.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Powrush-MMO
 */

use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct PaletteTransitionEvent {
    pub target_palette: PaletteType,
    pub target_intensity: f32,
    pub ramp_time: f32,
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

/// Real typed regions for data-driven palette mapping (RON config ready)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum RegionType {
    #[default]
    Wilderness,
    Forest,
    Industrial,
    Urban,
    Desert,
    Ocean,
    Mountain,
    Council,
}

#[derive(Event, Debug)]
pub struct RegionTransitionEvent {
    pub from_region: RegionType,
    pub to_region: RegionType,
    pub distance: f32,
}

#[derive(Event, Debug)]
pub struct CombatStateChangedEvent {
    pub entering_combat: bool,
    pub intensity: f32, // 0.0-1.0 threat level -> feeds industrial_intensity
}
