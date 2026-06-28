/*!
 * Dynamic Audio Mixing - Tunable Priority Stacking
 */

use bevy::prelude::*;
use bevy::audio::AudioSink;

// ... existing structs ...

impl Default for AudioMixer {
    fn default() -> Self {
        Self {
            // ... existing defaults ...

            // Stacking tuning parameters
            stacking_critical_per_sound: 0.08,
            stacking_high_per_sound: 0.06,
            max_stacking_reduction: 0.35,

            // Category-specific ducking bias (higher = ducks this category more)
            music_ducking_bias: 1.15,
            ambient_ducking_bias: 1.0,
            sfx_ducking_bias: 0.9,
            ui_ducking_bias: 0.6, // UI usually ducks less
        }
    }
}

impl AudioMixer {
    // ... existing methods ...

    /// Calculates stacking multiplier based on how many high-priority sounds are active.
    pub fn calculate_stacking_multiplier(&self, highest: Priority, counts: &[u32; 4]) -> f32 {
        match highest {
            Priority::Critical => {
                let count = counts[Priority::Critical as usize];
                let reduction = (count.saturating_sub(1) as f32 * self.stacking_critical_per_sound)
                    .min(self.max_stacking_reduction);
                1.0 - reduction
            }
            Priority::High => {
                let count = counts[Priority::High as usize];
                let reduction = (count.saturating_sub(1) as f32 * self.stacking_high_per_sound)
                    .min(self.max_stacking_reduction * 0.7);
                1.0 - reduction
            }
            _ => 1.0,
        }
    }
}

/// Updated calculate_target_ducking_level with stacking + category bias
fn calculate_target_ducking_level(
    highest: Priority,
    counts: &[u32; 4],
    mixer: &AudioMixer,
) -> f32 {
    if highest <= Priority::Normal {
        return 1.0;
    }

    let base = mixer.get_ducking_for_priority(Priority::Low, highest);
    let stacking = mixer.calculate_stacking_multiplier(highest, counts);

    base * stacking
}

/// Applies ducking with category bias
fn apply_ducking_to_sound(
    dynamic: &DynamicAudio,
    base_volume: f32,
    current_ducking: f32,
    highest_priority: Priority,
    mixer: &AudioMixer,
) -> f32 {
    if dynamic.priority < highest_priority {
        let priority_duck = mixer.get_ducking_for_priority(dynamic.priority, highest_priority);
        let category_bias = match dynamic.category {
            AudioCategory::Music   => mixer.music_ducking_bias,
            AudioCategory::Ambient => mixer.ambient_ducking_bias,
            AudioCategory::Sfx     => mixer.sfx_ducking_bias,
            AudioCategory::Ui      => mixer.ui_ducking_bias,
            AudioCategory::Voice   => 1.0,
        };

        base_volume * current_ducking * priority_duck * category_bias
    } else {
        base_volume
    }
}
