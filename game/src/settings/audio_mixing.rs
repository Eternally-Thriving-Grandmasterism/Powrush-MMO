/*!
 * Dynamic Audio Mixing - Dynamic Bias Scaling
 */

use bevy::prelude::*;
use bevy::audio::AudioSink;

// ... existing code ...

impl AudioMixer {
    // ... existing methods ...

    /// Returns a dynamic bias for a category that scales with stacking intensity.
    /// Higher stacking (more high-priority sounds) increases bias on sensitive categories like Music.
    pub fn get_dynamic_category_bias(
        &self,
        category: AudioCategory,
        highest: Priority,
        stacking_multiplier: f32,
    ) -> f32 {
        let base_bias = match category {
            AudioCategory::Music   => self.music_ducking_bias,
            AudioCategory::Ambient => self.ambient_ducking_bias,
            AudioCategory::Sfx     => self.sfx_ducking_bias,
            AudioCategory::Ui      => self.ui_ducking_bias,
            AudioCategory::Voice   => 1.0,
        };

        // Only scale bias when we have significant stacking (multiple high-priority sounds)
        if highest >= Priority::High && stacking_multiplier < 0.85 {
            let scale = (1.0 - stacking_multiplier) * 0.8 + 1.0; // stronger bias when stacking is high
            base_bias * scale
        } else {
            base_bias
        }
    }
}

fn apply_ducking_to_sound(
    dynamic: &DynamicAudio,
    base_volume: f32,
    current_ducking: f32,
    highest_priority: Priority,
    stacking_multiplier: f32,
    mixer: &AudioMixer,
) -> f32 {
    if dynamic.priority < highest_priority {
        let priority_duck = mixer.get_ducking_for_priority(dynamic.priority, highest_priority);
        let dynamic_bias = mixer.get_dynamic_category_bias(
            dynamic.category,
            highest_priority,
            stacking_multiplier,
        );

        base_volume * current_ducking * priority_duck * dynamic_bias
    } else {
        base_volume
    }
}

// Update calculate_target_ducking_level and update_dynamic_audio_volumes
// to pass stacking_multiplier into apply_ducking_to_sound
