/*!
 * Dynamic Audio Mixing System with Diagnostics
 *
 * Includes performance and state diagnostics using Bevy's Diagnostics system.
 */

use bevy::prelude::*;
use bevy::audio::AudioSink;
use bevy::diagnostic::{DiagnosticPath, Diagnostics};
use std::time::Instant;

// Diagnostic paths
pub const AUDIO_MIXING_TIME: DiagnosticPath = DiagnosticPath::const_new("audio/mixing_time_ms");
pub const ACTIVE_DYNAMIC_AUDIO: DiagnosticPath = DiagnosticPath::const_new("audio/active_dynamic_audio");
pub const CURRENT_DUCKING_LEVEL: DiagnosticPath = DiagnosticPath::const_new("audio/current_ducking_level");

// ... (rest of the existing code: DynamicAudio, Priority, AudioCategory, AudioMixer, DuckingState, etc.)

/// Registers custom audio mixing diagnostics.
/// Call this once during plugin initialization.
pub fn register_audio_diagnostics(mut diagnostics: ResMut<Diagnostics>) {
    diagnostics.register(AUDIO_MIXING_TIME);
    diagnostics.register(ACTIVE_DYNAMIC_AUDIO);
    diagnostics.register(CURRENT_DUCKING_LEVEL);
}

/// Core mixing system with performance diagnostics.
pub fn update_dynamic_audio_volumes(
    mixer: Res<AudioMixer>,
    mut ducking: ResMut<DuckingState>,
    time: Res<Time>,
    mut diagnostics: Diagnostics,
    mut query: Query<(&DynamicAudio, &mut AudioSink)>,
) {
    let start = Instant::now();

    // ... existing logic to find highest_priority and calculate target_level ...

    let (attack_rate, release_rate) = mixer.get_ducking_rates(highest_priority);
    let rate = if target_level < ducking.current_level {
        attack_rate
    } else {
        release_rate
    };

    let dt = time.delta_secs();
    let t = 1.0 - (-rate * dt).exp();
    ducking.current_level = ducking.current_level * (1.0 - t) + target_level * t;

    // Apply volumes
    for (dynamic, mut sink) in query.iter_mut() {
        // ... existing volume logic ...
    }

    // Report diagnostics
    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
    diagnostics.add_measurement(&AUDIO_MIXING_TIME, elapsed_ms);

    let active_count = query.iter().count() as f64;
    diagnostics.add_measurement(&ACTIVE_DYNAMIC_AUDIO, active_count);

    diagnostics.add_measurement(&CURRENT_DUCKING_LEVEL, ducking.current_level as f64);
}
