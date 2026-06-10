/*!
 * Spatial Audio System - Phase 2: Listener Tracking
 */

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioTween};
use std::time::Duration;

/// Resource that manages spatial audio
#[derive(Resource)]
pub struct SpatialAudioManager {
    pub enabled: bool,
    pub listener_position: Vec3,
}

impl Default for SpatialAudioManager {
    fn default() -> Self {
        Self {
            enabled: true,
            listener_position: Vec3::ZERO,
        }
    }
}

/// Marks an entity as the spatial audio listener (usually the camera or player)
#[derive(Component)]
pub struct SpatialListener;

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
            .add_systems(Update, (
                update_listener_position,
                play_spatial_sounds,
            ));
    }
}

/// Updates the listener position from any entity with SpatialListener (usually camera)
fn update_listener_position(
    mut spatial_manager: ResMut<SpatialAudioManager>,
    listener_query: Query<&GlobalTransform, With<SpatialListener>>,
) {
    if let Ok(transform) = listener_query.get_single() {
        spatial_manager.listener_position = transform.translation();
    }
}

/// Processes spatial sound events with basic distance-based attenuation
fn play_spatial_sounds(
    mut events: EventReader<PlaySpatialSound>,
    audio: Res<Audio>,
    spatial_manager: Res<SpatialAudioManager>,
) {
    if !spatial_manager.enabled {
        return;
    }

    let listener_pos = spatial_manager.listener_position;

    for event in events.read() {
        // Simple distance-based volume attenuation
        let distance = event.position.distance(listener_pos);
        let falloff = (1.0 / (1.0 + distance * 0.01)).clamp(0.1, 1.0);
        let final_volume = event.volume * falloff;

        // Basic panning based on x position relative to listener
        let pan = ((event.position.x - listener_pos.x) * 0.005).clamp(-1.0, 1.0);

        audio
            .play(event.sound.clone())
            .with_volume(final_volume as f64)
            .with_panning(((pan + 1.0) * 0.5) as f64) // Convert -1..1 to 0..1
            .fade_in(AudioTween::new(
                Duration::from_millis(event.fade_in_ms),
                bevy_kira_audio::AudioEasing::OutPowi(2),
            ));
    }
}
