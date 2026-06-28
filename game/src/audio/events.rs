/*!
 * Audio Events - Including AudioTrigger for gameplay-driven audio
 */

use bevy::prelude::*;
use crate::settings::audio_mixing::{AudioCategory, Priority};

// ... existing events (PaletteTransitionEvent, RegionTransitionEvent, etc.) ...

/// Event emitted by gameplay systems to request audio with a specific priority.
/// This is the primary interface for game logic to influence the Dynamic Audio Mixing system.
#[derive(Event, Debug, Clone)]
pub struct AudioTrigger {
    /// Priority level. Higher priorities can duck lower priority audio.
    pub priority: Priority,

    /// Optional category. If None, the audio system may decide based on context.
    pub category: Option<AudioCategory>,

    /// Optional intensity scalar (0.0 - 1.0).
    pub intensity: Option<f32>,

    /// Optional label for debugging and diagnostics.
    pub label: Option<String>,
}
