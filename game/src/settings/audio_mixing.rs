/*!
 * Dynamic Audio Mixing System
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Powrush-MMO
 */

use bevy::prelude::*;
use bevy::audio::AudioSink;

#[derive(Component, Clone, Copy)]
pub struct DynamicAudio {
    pub category: AudioCategory,
    pub priority: Priority,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
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
        let cat_vol = match category {
            AudioCategory::Music   => self.music,
            AudioCategory::Sfx     => self.sfx,
            AudioCategory::Ui      => self.ui,
            AudioCategory::Voice   => self.voice,
            AudioCategory::Ambient => self.ambient,
        };
        self.master * cat_vol
    }
}

/// System that applies AudioMixer volumes to entities with DynamicAudio + AudioSink
pub fn update_dynamic_audio_volumes(
    mixer: Res<AudioMixer>,
    mut query: Query<(&DynamicAudio, &mut AudioSink)>,
) {
    for (dynamic_audio, mut sink) in query.iter_mut() {
        let final_volume = mixer.get_volume_for_category(dynamic_audio.category);
        sink.set_volume(final_volume);
    }
}

/// Optional: Basic priority-based ducking (simple version)
pub fn apply_priority_ducking(
    mixer: Res<AudioMixer>,
    query: Query<&DynamicAudio>,
) {
    // This can be expanded later for more sophisticated ducking
    // For now, higher priority sounds naturally stand out via volume settings
}
