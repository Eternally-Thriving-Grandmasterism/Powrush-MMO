/*!
 * Audio Events
 */

use bevy::prelude::*;
use crate::settings::audio_mixing::{AudioCategory, Priority};

/// Primary event for gameplay systems to trigger audio with specific mixing behavior.
#[derive(Event, Debug, Clone)]
pub struct AudioTrigger {
    /// Priority level. Higher priorities can duck lower priority audio.
    pub priority: Priority,

    /// Optional category override.
    pub category: Option<AudioCategory>,

    /// Optional intensity (0.0–1.0).
    pub intensity: Option<f32>,

    /// Optional sound path to play. If present, the handler will spawn this sound.
    pub sound_path: Option<String>,

    /// Optional label for debugging.
    pub label: Option<String>,
}
