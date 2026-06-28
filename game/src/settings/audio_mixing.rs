/*!
 * Dynamic Audio Mixing - Smoothing Filters
 */

use bevy::prelude::*;
use bevy::audio::AudioSink;

// Add a simple per-sound smoothing filter

/// Applies a simple exponential smoothing filter to a target volume.
/// This prevents abrupt volume jumps when ducking changes rapidly.
pub fn smooth_volume(current: f32, target: f32, smoothing: f32, dt: f32) -> f32 {
    current + (target - current) * (1.0 - (-smoothing * dt).exp())
}

// Extend AudioMixer with a global smoothing factor for final volumes
impl Default for AudioMixer {
    fn default() -> Self {
        Self {
            // ... existing fields ...
            volume_smoothing: 8.0, // Higher = faster smoothing
        }
    }
}

pub fn update_dynamic_audio_volumes(
    mixer: Res<AudioMixer>,
    mut ducking: ResMut<DuckingState>,
    time: Res<Time>,
    mut diagnostics: Diagnostics,
    mut query: Query<(&DynamicAudio, &mut AudioSink)>,
) {
    // ... existing logic to calculate highest_priority, target_level, etc. ...

    let dt = time.delta_secs();

    for (dynamic, mut sink) in query.iter_mut() {
        let base_volume = mixer.get_volume_for_category(dynamic.category);
        let target_volume = apply_ducking_to_sound(
            dynamic,
            base_volume,
            ducking.current_level,
            highest_priority,
            stacking_multiplier,
            &mixer,
        );

        // Apply smoothing filter to final volume
        let current_volume = sink.volume();
        let smoothed = smooth_volume(current_volume, target_volume, mixer.volume_smoothing, dt);

        sink.set_volume(smoothed);
    }

    // ... diagnostics ...
}
