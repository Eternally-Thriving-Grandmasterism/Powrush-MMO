/*!
 * Root GameSettings container
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use super::audio_settings::AudioSettings;

#[derive(Resource, Serialize, Deserialize, Clone, Debug, Default)]
pub struct GameSettings {
    pub version: u32,
    pub audio: AudioSettings,
    // Future categories will be added here:
    // pub graphics: GraphicsSettings,
    // pub controls: ControlsSettings,
    // pub accessibility: AccessibilitySettings,
}

impl GameSettings {
    pub const CURRENT_VERSION: u32 = 1;

    pub fn new() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            audio: AudioSettings::default(),
        }
    }

    pub fn validate(&mut self) {
        self.audio.validate();
        // Validate other categories when added
    }

    pub fn migrate(&mut self) {
        // Add migration logic here when version increases
        if self.version < Self::CURRENT_VERSION {
            // Example: migrate from older versions
            self.version = Self::CURRENT_VERSION;
        }
    }
}
