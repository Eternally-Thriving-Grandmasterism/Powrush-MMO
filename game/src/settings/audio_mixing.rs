/*!
 * Dynamic Audio Mixing
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::settings::editor::SettingsEditor;

#[derive(Component)]
pub struct DynamicAudio {
    pub category: AudioCategory,
    pub priority: crate::settings::editor::Priority,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AudioCategory {
    Music,
    Sfx,
    Ui,
    Voice,
    Ambient,
}

#[derive(Resource, Default, Clone)]
pub struct AudioMixer {
    pub master: f32,
    pub music: f32,
    pub sfx: f32,
    pub ui: f32,
    pub voice: f32,
    pub ambient: f32,
}

impl AudioMixer {
    pub fn get_volume_for_category(&self, category: AudioCategory) -> f32 {
        match category {
            AudioCategory::Music   => self.music,
            AudioCategory::Sfx     => self.sfx,
            AudioCategory::Ui      => self.ui,
            AudioCategory::Voice   => self.voice,
            AudioCategory::Ambient => self.ambient,
        }
    }
}

// ... rest of the file (apply_audio_settings, update_dynamic_audio_volumes, play_dynamic_sound) remains the same
