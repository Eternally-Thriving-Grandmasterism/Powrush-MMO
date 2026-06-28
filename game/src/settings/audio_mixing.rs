/*!
 * Dynamic Audio Mixing - Priority Logic
 *
 * This file contains the core logic for priority-based ducking.
 */

use bevy::prelude::*;
use bevy::audio::AudioSink;
use bevy::diagnostic::{DiagnosticPath, Diagnostics};
use std::time::Instant;

// ... (existing enums, structs, and diagnostic paths) ...

/// Finds the highest priority among all currently audible DynamicAudio entities.
fn find_highest_active_priority(query: &Query<(&DynamicAudio, &AudioSink)>) -> Priority {
    let mut highest = Priority::Low;

    for (dynamic, sink) in query.iter() {
        // Only consider sounds that are actually playing
        if sink.volume() > 0.01 && dynamic.priority > highest {
            highest = dynamic.priority;
        }
    }

    highest
}

/// Calculates the target ducking level based on the highest active priority.
fn calculate_target_ducking_level(highest: Priority, mixer: &AudioMixer) -> f32 {
    if highest <= Priority::Normal {
        1.0
    } else {
        mixer.get_ducking_for_priority(Priority::Low, highest)
    }
}

/// Applies the final volume to a single sound, taking into account base volume, current ducking, and per-priority ducking.
fn apply_ducking_to_sound(
    dynamic: &DynamicAudio,
    base_volume: f32,
    current_ducking: f32,
    highest_priority: Priority,
    mixer: &AudioMixer,
) -> f32 {
    if dynamic.priority < highest_priority {
        let priority_ducking = mixer.get_ducking_for_priority(dynamic.priority, highest_priority);
        base_volume * current_ducking * priority_ducking
    } else {
        base_volume
    }
}

/// Main mixing system with clear priority logic separation.
pub fn update_dynamic_audio_volumes(
    mixer: Res<AudioMixer>,
    mut ducking: ResMut<DuckingState>,
    time: Res<Time>,
    mut diagnostics: Diagnostics,
    mut query: Query<(&DynamicAudio, &mut AudioSink)>,
) {
    let start = Instant::now();

    // === Priority Logic ===
    let highest_priority = find_highest_active_priority(&query);
    let target_level = calculate_target_ducking_level(highest_priority, &mixer);

    // Dynamic rates based on triggering priority
    let (attack_rate, release_rate) = mixer.get_ducking_rates(highest_priority);
    let rate = if target_level < ducking.current_level {
        attack_rate
    } else {
        release_rate
    };

    // Exponential smoothing
    let dt = time.delta_secs();
    let t = 1.0 - (-rate * dt).exp();
    ducking.current_level = ducking.current_level * (1.0 - t) + target_level * t;

    // Apply final volumes
    for (dynamic, mut sink) in query.iter_mut() {
        let base_volume = mixer.get_volume_for_category(dynamic.category);
        let final_volume = apply_ducking_to_sound(
            dynamic,
            base_volume,
            ducking.current_level,
            highest_priority,
            &mixer,
        );
        sink.set_volume(final_volume);
    }

    // Diagnostics
    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
    diagnostics.add_measurement(&AUDIO_MIXING_TIME, elapsed_ms);
    diagnostics.add_measurement(&ACTIVE_DYNAMIC_AUDIO, query.iter().count() as f64);
    diagnostics.add_measurement(&CURRENT_DUCKING_LEVEL, ducking.current_level as f64);
}
