/*!
 * Spatial Audio System - Phase 1
 *
 * Foundation for world-based spatial audio using Bevy Kira Audio.
 * Divine Whispers remain non-spatial (UI/narrative).
 */

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioManager, AudioTween};
use kira::spatial::emitter::SpatialEmitterSettings;
use kira::spatial::scene::SpatialSceneSettings;
use std::time::Duration;

/// Resource that manages spatial audio
#[derive(Resource)]
pub struct SpatialAudioManager {
    pub enabled: bool,
}

impl Default for SpatialAudioManager {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Event to play a spatial sound at a world position
#[derive(Event)]
pub struct PlaySpatialSound {
    pub sound: Handle<bevy_kira_audio::AudioSource>,
    pub position: Vec3,
    pub volume: f32,
    pub fade_in_ms: u64,
}

impl PlaySpatialSound {
    pub fn new(sound: Handle<bevy_kira_audio::AudioSource>, position: Vec3) -> Self {
        Self {
            sound,
            position,
            volume: 1.0,
            fade_in_ms: 150,
        }
    }
}

pub struct SpatialAudioPlugin;

impl Plugin for SpatialAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SpatialAudioManager>()
            .add_event::<PlaySpatialSound>()
            .add_systems(Update, play_spatial_sounds);
    }
}

/// System that processes spatial sound events
fn play_spatial_sounds(
    mut events: EventReader<PlaySpatialSound>,
    audio: Res<Audio>,
    spatial_manager: Res<SpatialAudioManager>,
) {
    if !spatial_manager.enabled {
        return;
    }

    for event in events.read() {
        // For Phase 1, we play the sound with basic positioning.
        // Full 3D spatialization requires deeper Kira spatial scene integration.
        // This gives us the foundation + event-driven playback.

        audio
            .play(event.sound.clone())
            .with_volume(event.volume as f64)
            .with_panning(0.5) // Placeholder - real panning would come from spatial scene
            .fade_in(AudioTween::new(
                Duration::from_millis(event.fade_in_ms),
                bevy_kira_audio::AudioEasing::OutPowi(2),
            ));

        // TODO Phase 2: Use actual Kira SpatialScene + SpatialEmitter
        // with real 3D position and listener tracking.
    }
}
