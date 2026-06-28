/*!
 * Dynamic Audio Mixing - Priority Stacking Logic
 */

use bevy::prelude::*;
use bevy::audio::AudioSink;

// ... existing code ...

/// Counts how many sounds are currently active at each priority level.
fn count_active_priorities(query: &Query<(&DynamicAudio, &AudioSink)>) -> [u32; 4] {
    let mut counts = [0u32; 4];

    for (dynamic, sink) in query.iter() {
        if sink.volume() > 0.01 {
            let idx = dynamic.priority as usize;
            counts[idx] += 1;
        }
    }

    counts
}

/// Calculates target ducking level with support for priority stacking.
/// More high/critical sounds = stronger ducking.
fn calculate_target_ducking_level(
    highest: Priority,
    counts: &[u32; 4],
    mixer: &AudioMixer,
) -> f32 {
    if highest <= Priority::Normal {
        return 1.0;
    }

    let base = mixer.get_ducking_for_priority(Priority::Low, highest);

    // Stacking factor: more high-priority sounds = deeper ducking
    let stack_multiplier = match highest {
        Priority::Critical => {
            let critical_count = counts[Priority::Critical as usize];
            // Each additional critical sound increases ducking strength
            1.0 - (critical_count.saturating_sub(1) as f32 * 0.08).min(0.35)
        }
        Priority::High => {
            let high_count = counts[Priority::High as usize];
            1.0 - (high_count.saturating_sub(1) as f32 * 0.06).min(0.25)
        }
        _ => 1.0,
    };

    (base * stack_multiplier).max(0.1) // never duck completely to silence
}

pub fn update_dynamic_audio_volumes(
    mixer: Res<AudioMixer>,
    mut ducking: ResMut<DuckingState>,
    time: Res<Time>,
    mut diagnostics: Diagnostics,
    mut query: Query<(&DynamicAudio, &mut AudioSink)>,
) {
    let start = Instant::now();

    let highest_priority = find_highest_active_priority(&query);
    let counts = count_active_priorities(&query);
    let target_level = calculate_target_ducking_level(highest_priority, &counts, &mixer);

    // ... rest of exponential smoothing and volume application ...

    // (keep existing smoothing + apply_ducking_to_sound logic)
}
