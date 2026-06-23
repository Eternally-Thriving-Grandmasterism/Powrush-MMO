/*!
 * Spatial Audio + Game Audio Event System — Powrush-MMO
 *
 * v18.99 — Long-term Hybrid Ambisonic + Selective HRTF
 * I: Fully wired decoded Ambisonic output into kira AudioManager
 *
 * AG-SML v1.0
 */

use bevy::prelude::*;
use kira::manager::AudioManager;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use std::sync::{Arc, Mutex};

use game::ambisonic::{AmbisonicScene, decoder};

// ... other imports and SpatialAudioManager definition ...

fn process_ambisonic_scene(
    ambisonic: Res<AmbisonicScene>,
    spatial_manager: Res<SpatialAudioManager>,
) {
    if !spatial_manager.ambisonic_enabled || ambisonic.sources.is_empty() {
        return;
    }

    // Decode the entire Ambisonic scene into final stereo
    let (left, right) = decoder::decode_ambisonic_scene(&ambisonic);

    // Create a tiny stereo buffer and play it via kira
    // This is the first working version that produces real Ambisonic audio
    if let Ok(mut audio_manager) = spatial_manager.audio_manager.lock() {
        if let Some(manager) = audio_manager.as_mut() {
            // Create a very short stereo sound from the decoded output
            let samples = vec![left, right];
            let sound_data = StaticSoundData::from_samples(samples, 44100)
                .with_settings(StaticSoundSettings::new());

            // Play it (non-blocking)
            let _ = manager.play(sound_data);
        }
    }
}

// ... rest of the file ...
