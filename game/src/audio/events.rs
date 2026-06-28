/*!
 * Audio Events - Palette Transitions, Combat & Region Events for Adaptive Layering
 *
 * Enums now Serialize/Deserialize ready for RON config (RegionPaletteConfig).
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Powrush-MMO
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Event, Debug)]
pub struct PaletteTransitionEvent {
    pub target_palette: PaletteType,
    pub target_intensity: f32,
    pub ramp_time: f32,
    pub priority: TransitionPriority,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum PaletteType {
    #[default]
    ResonantVeil,
    IndustrialPulse,
    EchoingWisp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionPriority {
    Normal,
    Combat,
    Event,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
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
    pub intensity: f32,
}
