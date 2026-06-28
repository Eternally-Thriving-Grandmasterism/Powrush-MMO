/*!
 * Dynamic Audio Mixing with Priority-Based Ducking
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

    // Ducking configuration
    pub ducking_factor: f32,           // How much to duck (0.3 = 70% reduction)
    pub ducking_attack: f32,           // How fast ducking engages
    pub ducking_release: f32,          // How fast ducking releases
}

impl Default for AudioMixer {
    fn default() -> Self {
        Self {
            master: 1.0,
            music: 0.8,
            sfx: 1.0,
            ui: 1.0,
            voice: 1.0,
            ambient: 0.7,
            ducking_factor: 0.4,     // Duck to 40% volume
            ducking_attack: 0.2,
            ducking_release: 0.3,
        }
    }
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

/// System that applies volume + priority-based ducking
pub fn update_dynamic_audio_volumes(
    mixer: Res<AudioMixer>,
    mut query: Query<(&DynamicAudio, &mut AudioSink)>,
) {
    // Find the highest priority currently playing
    let mut highest_priority = Priority::Low;

    for (dynamic, sink) in query.iter() {
        if sink.volume() > 0.01 { // Consider it "playing"
            if dynamic.priority > highest_priority {
                highest_priority = dynamic.priority;
            }
        }
    }

    // Apply volumes with ducking
    for (dynamic, mut sink) in query.iter_mut() {
        let base_volume = mixer.get_volume_for_category(dynamic.category);

        let final_volume = if dynamic.priority < highest_priority {
            // Duck lower priority sounds
            base_volume * mixer.ducking_factor
        } else {
            base_volume
        };

        sink.set_volume(final_volume);
    }
}
