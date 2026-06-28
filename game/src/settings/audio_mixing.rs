/*!
 * Dynamic Audio Mixing System with Priority-Based Ducking and Exponential Curves
 *
 * This module provides a professional-grade audio mixing layer for Powrush-MMO.
 *
 * Core Features:
 * - Category-based volume control (Music, SFX, UI, Voice, Ambient)
 * - Priority-based ducking (Low / Normal / High / Critical)
 * - Per-priority ducking amounts
 * - Dynamic exponential attack/release curves (different per priority level)
 * - Smooth real-time interpolation via DuckingState
 * - Fully hot-reloadable via AdaptiveAudioConfig (adaptive_audio.ron)
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Powrush-MMO
 */

use bevy::prelude::*;
use bevy::audio::AudioSink;

/// Component attached to audio entities to enable dynamic mixing and ducking.
#[derive(Component, Clone, Copy, Debug)]
pub struct DynamicAudio {
    /// Which audio category this sound belongs to (affects base volume).
    pub category: AudioCategory,
    /// Playback priority. Higher priority sounds can duck lower priority ones.
    pub priority: Priority,
}

/// Playback priority levels. Higher values can trigger ducking of lower values.
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// High-level audio categories used for volume grouping and mixing.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AudioCategory {
    Music,
    Sfx,
    Ui,
    Voice,
    Ambient,
}

/// Global audio mixer resource.
///
/// Contains master + per-category volumes, plus all Dynamic Ducking Curve parameters.
/// This resource is the single source of truth for runtime mixing behavior.
#[derive(Resource, Clone, Debug)]
pub struct AudioMixer {
    // === Base Volume Controls ===
    pub master: f32,
    pub music: f32,
    pub sfx: f32,
    pub ui: f32,
    pub voice: f32,
    pub ambient: f32,

    // === Per-Priority Ducking Amounts ===
    /// Ducking factor applied to lower-priority sounds when a Critical sound is active.
    pub ducking_critical: f32,
    /// Ducking factor applied when a High priority sound is active.
    pub ducking_high: f32,
    /// Ducking factor applied when a Normal priority sound is active.
    pub ducking_normal: f32,

    // === Dynamic Ducking Curves (Attack / Release per priority) ===
    /// How fast ducking engages when a Critical sound starts playing.
    pub ducking_attack_critical: f32,
    /// How fast ducking releases after Critical sounds stop.
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
    /// Returns the final base volume for a given category (master * category volume).
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

    /// Returns the ducking multiplier to apply based on priority difference.
    pub fn get_ducking_for_priority(&self, current: Priority, highest: Priority) -> f32 {
        if current >= highest {
            return 1.0;
        }
        match highest {
            Priority::Critical => self.ducking_critical,
            Priority::High     => self.ducking_high,
            Priority::Normal   => self.ducking_normal,
            Priority::Low      => 1.0,
        }
    }

    /// Returns (attack_rate, release_rate) for the given triggering priority.
    /// This enables truly dynamic ducking curves per priority level.
    pub fn get_ducking_rates(&self, highest: Priority) -> (f32, f32) {
        match highest {
            Priority::Critical => (self.ducking_attack_critical, self.ducking_release_critical),
            Priority::High     => (self.ducking_attack_high, self.ducking_release_high),
            Priority::Normal   => (self.ducking_attack_normal, self.ducking_release_normal),
            Priority::Low      => (6.0, 3.0),
        }
    }
}

/// Resource that tracks the current interpolated ducking level.
/// Used to achieve smooth exponential attack and release curves.
#[derive(Resource, Default, Debug)]
pub struct DuckingState {
    /// Current ducking multiplier (1.0 = no ducking, lower = more ducked).
    /// Updated every frame with exponential interpolation.
    pub current_level: f32,
}

/// Core mixing system.
/// Applies category volumes + priority-based ducking with smooth exponential curves.
pub fn update_dynamic_audio_volumes(
    mixer: Res<AudioMixer>,
    mut ducking: ResMut<DuckingState>,
    time: Res<Time>,
    mut query: Query<(&DynamicAudio, &mut AudioSink)>,
) {
    // 1. Determine highest active priority
    let mut highest_priority = Priority::Low;

    for (dynamic, sink) in query.iter() {
        if sink.volume() > 0.01 {
            if dynamic.priority > highest_priority {
                highest_priority = dynamic.priority;
            }
        }
    }

    // 2. Calculate target ducking level
    let target_level = if highest_priority > Priority::Low {
        mixer.get_ducking_for_priority(Priority::Low, highest_priority)
    } else {
        1.0
    };

    // 3. Choose dynamic attack/release rate based on triggering priority
    let (attack_rate, release_rate) = mixer.get_ducking_rates(highest_priority);
    let rate = if target_level < ducking.current_level {
        attack_rate
    } else {
        release_rate
    };

    // 4. Exponential interpolation for natural feel
    let dt = time.delta_secs();
    let t = 1.0 - (-rate * dt).exp();
    ducking.current_level = ducking.current_level * (1.0 - t) + target_level * t;

    // 5. Apply final volume to each sound
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
