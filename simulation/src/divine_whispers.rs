/*!
 * Divine Whispers - Shared types for server-client communication
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Event triggered when a Divine Whisper should be shown to a player.
/// This is sent from server to the specific client.
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct DivineWhisperTrigger {
    pub player_id: u64,
    pub text: String,
    pub flavor: String,
    pub intensity: f32,
    pub duration_seconds: f32,
}

impl DivineWhisperTrigger {
    pub fn new(player_id: u64, text: impl Into<String>, flavor: impl Into<String>, intensity: f32) -> Self {
        Self {
            player_id,
            text: text.into(),
            flavor: flavor.into(),
            intensity,
            duration_seconds: 4.5 + (intensity * 2.0), // longer for stronger epiphanies
        }
    }
}
