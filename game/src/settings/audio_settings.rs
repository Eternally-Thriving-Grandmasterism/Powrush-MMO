/*!
 * Audio Settings Category
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub navigation_volume: f32,
    pub activation_volume: f32,
    pub navigation_pitch_variation: f32,
    pub activation_pitch_variation: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.8,
            music_volume: 0.7,
            sfx_volume: 0.8,
            navigation_volume: 0.6,
            activation_volume: 0.8,
            navigation_pitch_variation: 0.03,
            activation_pitch_variation: 0.03,
        }
    }
}

impl AudioSettings {
    pub fn validate(&mut self) {
        self.master_volume = self.master_volume.clamp(0.0, 1.0);
        self.music_volume = self.music_volume.clamp(0.0, 1.0);
        self.sfx_volume = self.sfx_volume.clamp(0.0, 1.0);
        self.navigation_volume = self.navigation_volume.clamp(0.0, 1.0);
        self.activation_volume = self.activation_volume.clamp(0.0, 1.0);

        self.navigation_pitch_variation = self.navigation_pitch_variation.clamp(0.0, 0.2);
        self.activation_pitch_variation = self.activation_pitch_variation.clamp(0.0, 0.2);
    }
}
