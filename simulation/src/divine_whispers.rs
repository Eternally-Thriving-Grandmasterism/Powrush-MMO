/*!
 * Divine Whispers - Shared Event
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct DivineWhisperTrigger {
    pub player_id: u64,
    pub text: String,
    pub flavor: String,
    pub intensity: f32,
    pub duration_seconds: f32,
    pub is_epiphany: bool, // New: marks if this came from an epiphany
}

impl DivineWhisperTrigger {
    pub fn new(player_id: u64, text: impl Into<String>, flavor: impl Into<String>, intensity: f32) -> Self {
        Self {
            player_id,
            text: text.into(),
            flavor: flavor.into(),
            intensity,
            duration_seconds: 4.5 + (intensity * 2.5),
            is_epiphany: false,
        }
    }

    /// Special constructor for epiphany-triggered whispers
    pub fn from_epiphany(
        player_id: u64,
        text: impl Into<String>,
        flavor: impl Into<String>,
        intensity: f32,
    ) -> Self {
        Self {
            player_id,
            text: text.into(),
            flavor: flavor.into(),
            intensity,
            duration_seconds: 7.0 + (intensity * 3.0), // Longer for epiphanies
            is_epiphany: true,
        }
    }
}
