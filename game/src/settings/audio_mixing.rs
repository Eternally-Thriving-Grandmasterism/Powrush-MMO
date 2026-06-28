/*!
 * Dynamic Audio Mixing with Exponential Priority-Based Ducking
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

#[derive(Resource, Clone)]
pub struct AudioMixer {
    pub master: f32,
    pub music: f32,
    pub sfx: f32,
    pub ui: f32,
    pub voice: f32,
    pub ambient: f32,

    pub ducking_factor: f32,
    pub ducking_attack: f32,     // Higher = faster exponential attack
    pub ducking_release: f32,    // Higher = faster exponential release
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
            ducking_factor: 0.35,
            ducking_attack: 10.0,
            ducking_release: 4.0,
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

#[derive(Resource, Default)]
pub struct DuckingState {
    pub current_level: f32, // 1.0 = no ducking
}

/// System with exponential attack/release ducking
pub fn update_dynamic_audio_volumes(
    mixer: Res<AudioMixer>,
    mut ducking: ResMut<DuckingState>,
    time: Res<Time>,
    mut query: Query<(&DynamicAudio, &mut AudioSink)>,
) {
    // Find highest active priority
    let mut highest_priority = Priority::Low;

    for (dynamic, sink) in query.iter() {
        if sink.volume() > 0.01 {
            if dynamic.priority > highest_priority {
                highest_priority = dynamic.priority;
            }
        }
    }

    // Target ducking level
    let target_level = if highest_priority > Priority::Normal {
        mixer.ducking_factor
    } else {
        1.0
    };

    // Exponential interpolation (much more natural than linear)
    let rate = if target_level < ducking.current_level {
        mixer.ducking_attack
    } else {
        mixer.ducking_release
    };

    let dt = time.delta_secs();
    let t = 1.0 - (-rate * dt).exp(); // exponential ease

    ducking.current_level = ducking.current_level * (1.0 - t) + target_level * t;

    // Apply final volume with ducking
    for (dynamic, mut sink) in query.iter_mut() {
        let base_volume = mixer.get_volume_for_category(dynamic.category);

        let final_volume = if dynamic.priority < highest_priority {
            base_volume * ducking.current_level
        } else {
            base_volume
        };

        sink.set_volume(final_volume);
    }
}
