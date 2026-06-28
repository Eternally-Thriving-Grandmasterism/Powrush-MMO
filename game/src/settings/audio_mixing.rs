/*!
 * Dynamic Audio Mixing with Dynamic Ducking Curves
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

    // Per-priority ducking amounts
    pub ducking_critical: f32,
    pub ducking_high: f32,
    pub ducking_normal: f32,

    // Dynamic ducking curves (attack/release rates per priority)
    pub ducking_attack_critical: f32,
    pub ducking_release_critical: f32,
    pub ducking_attack_high: f32,
    pub ducking_release_high: f32,
    pub ducking_attack_normal: f32,
    pub ducking_release_normal: f32,
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

            ducking_critical: 0.25,
            ducking_high: 0.4,
            ducking_normal: 0.6,

            // Dynamic curves - Critical is more aggressive
            ducking_attack_critical: 14.0,
            ducking_release_critical: 5.0,
            ducking_attack_high: 10.0,
            ducking_release_high: 4.0,
            ducking_attack_normal: 6.0,
            ducking_release_normal: 3.0,
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

    pub fn get_ducking_for_priority(&self, current: Priority, highest: Priority) -> f32 {
        if current >= highest { return 1.0; }
        match highest {
            Priority::Critical => self.ducking_critical,
            Priority::High     => self.ducking_high,
            Priority::Normal   => self.ducking_normal,
            Priority::Low      => 1.0,
        }
    }

    /// Returns dynamic attack/release rates based on the triggering priority
    pub fn get_ducking_rates(&self, highest: Priority) -> (f32, f32) {
        match highest {
            Priority::Critical => (self.ducking_attack_critical, self.ducking_release_critical),
            Priority::High     => (self.ducking_attack_high, self.ducking_release_high),
            Priority::Normal   => (self.ducking_attack_normal, self.ducking_release_normal),
            Priority::Low      => (6.0, 3.0),
        }
    }
}

#[derive(Resource, Default)]
pub struct DuckingState {
    pub current_level: f32,
}

pub fn update_dynamic_audio_volumes(
    mixer: Res<AudioMixer>,
    mut ducking: ResMut<DuckingState>,
    time: Res<Time>,
    mut query: Query<(&DynamicAudio, &mut AudioSink)>,
) {
    let mut highest_priority = Priority::Low;

    for (dynamic, sink) in query.iter() {
        if sink.volume() > 0.01 {
            if dynamic.priority > highest_priority {
                highest_priority = dynamic.priority;
            }
        }
    }

    let target_level = if highest_priority > Priority::Low {
        mixer.get_ducking_for_priority(Priority::Low, highest_priority)
    } else {
        1.0
    };

    // Dynamic rates based on which priority triggered the ducking
    let (attack_rate, release_rate) = mixer.get_ducking_rates(highest_priority);

    let rate = if target_level < ducking.current_level {
        attack_rate
    } else {
        release_rate
    };

    let dt = time.delta_secs();
    let t = 1.0 - (-rate * dt).exp();

    ducking.current_level = ducking.current_level * (1.0 - t) + target_level * t;

    for (dynamic, mut sink) in query.iter_mut() {
        let base_volume = mixer.get_volume_for_category(dynamic.category);
        let ducking_amount = mixer.get_ducking_for_priority(dynamic.priority, highest_priority);

        let final_volume = if dynamic.priority < highest_priority {
            base_volume * ducking.current_level * ducking_amount
        } else {
            base_volume
        };

        sink.set_volume(final_volume);
    }
}
