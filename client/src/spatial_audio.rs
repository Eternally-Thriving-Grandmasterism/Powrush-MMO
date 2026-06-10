/*!
 * Spatial Audio System - Phase 3: Doppler Effect
 */

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioTween};
use std::time::Duration;

/// Resource that manages spatial audio
#[derive(Resource)]
pub struct SpatialAudioManager {
    pub enabled: bool,
    pub listener_position: Vec3,
    pub listener_velocity: Vec3,
    pub speed_of_sound: f32,
}

impl Default for SpatialAudioManager {
    fn default() -> Self {
        Self {
            enabled: true,
            listener_position: Vec3::ZERO,
            listener_velocity: Vec3::ZERO,
            speed_of_sound: 343.0, // Speed of sound in air (m/s)
        }
    }
}

/// Marks an entity as the spatial audio listener
#[derive(Component)]
pub struct SpatialListener;

/// Event to play a spatial sound at a world position with optional velocity
#[derive(Event)]
pub struct PlaySpatialSound {
    pub sound: Handle<bevy_kira_audio::AudioSource>,
    pub position: Vec3,
    pub velocity: Vec3,           // Source velocity for Doppler
    pub volume: f32,
    pub fade_in_ms: u64,
}

impl PlaySpatialSound {
    pub fn new(sound: Handle<bevy_kira_audio::AudioSource>, position: Vec3) -> Self {
        Self {
            sound,
            position,
            velocity: Vec3::ZERO,
            volume: 1.0,
            fade_in_ms: 150,
        }
    }

    pub fn with_velocity(mut self, velocity: Vec3) -> Self {
        self.velocity = velocity;
        self
    }
}

pub struct SpatialAudioPlugin;

impl Plugin for SpatialAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SpatialAudioManager>()
            .add_event::<PlaySpatialSound>()
            .add_systems(Update, (
                update_listener_position_and_velocity,
                play_spatial_sounds,
            ));
    }
}

/// Tracks both position and velocity of the listener
fn update_listener_position_and_velocity(
    mut spatial_manager: ResMut<SpatialAudioManager>,
    time: Res<Time>,
    listener_query: Query<&GlobalTransform, With<SpatialListener>>,
) {
    if let Ok(transform) = listener_query.get_single() {
        let new_position = transform.translation();

        // Simple velocity estimation from position delta
        let delta = new_position - spatial_manager.listener_position;
        spatial_manager.listener_velocity = delta / time.delta_seconds().max(0.001);

        spatial_manager.listener_position = new_position;
    }
}

/// Plays spatial sounds with distance attenuation + Doppler effect
fn play_spatial_sounds(
    mut events: EventReader<PlaySpatialSound>,
    audio: Res<Audio>,
    spatial_manager: Res<SpatialAudioManager>,
) {
    if !spatial_manager.enabled {
        return;
    }

    let listener_pos = spatial_manager.listener_position;
    let listener_vel = spatial_manager.listener_velocity;
    let speed_of_sound = spatial_manager.speed_of_sound;

    for event in events.read() {
        let source_pos = event.position;
        let source_vel = event.velocity;

        // Distance-based volume
        let distance = source_pos.distance(listener_pos);
        let falloff = (1.0 / (1.0 + distance * 0.012)).clamp(0.15, 1.0);
        let final_volume = event.volume * falloff;

        // Basic panning
        let pan = ((source_pos.x - listener_pos.x) * 0.006).clamp(-0.9, 0.9);

        // === Doppler Effect ===
        let direction = (source_pos - listener_pos).normalize_or_zero();
        let relative_velocity = (source_vel - listener_vel).dot(direction);

        // Doppler shift factor (classic formula)
        let doppler_factor = if relative_velocity.abs() > 0.1 {
            speed_of_sound / (speed_of_sound - relative_velocity).max(1.0)
        } else {
            1.0
        };

        // Clamp to reasonable musical range
        let playback_rate = doppler_factor.clamp(0.6, 1.6);

        audio
            .play(event.sound.clone())
            .with_volume(final_volume as f64)
            .with_panning(((pan + 1.0) * 0.5) as f64)
            .with_playback_rate(playback_rate as f64)
            .fade_in(AudioTween::new(
                Duration::from_millis(event.fade_in_ms),
                bevy_kira_audio::AudioEasing::OutPowi(2),
            ));
    }
}
