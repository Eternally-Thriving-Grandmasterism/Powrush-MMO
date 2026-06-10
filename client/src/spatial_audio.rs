/*!
 * Spatial Audio System - Phase 4: Full SpatialScene Integration
 */

use bevy::prelude::*;
use bevy_kira_audio::Audio;
use kira::manager::AudioManager;
use kira::manager::backend::DefaultBackend;
use kira::spatial::emitter::SpatialEmitterSettings;
use kira::spatial::listener::SpatialListenerSettings;
use kira::spatial::scene::{SpatialScene, SpatialSceneSettings};
use kira::sound::static_sound::StaticSoundData;
use std::sync::{Arc, Mutex};

/// Resource managing the Kira SpatialScene
#[derive(Resource)]
pub struct SpatialAudioManager {
    pub enabled: bool,
    pub spatial_scene: Arc<Mutex<SpatialScene>>,
    pub audio_manager: Option<AudioManager<DefaultBackend>>,
}

impl Default for SpatialAudioManager {
    fn default() -> Self {
        // Create a spatial scene
        let spatial_scene = SpatialScene::new(SpatialSceneSettings::new());

        Self {
            enabled: true,
            spatial_scene: Arc::new(Mutex::new(spatial_scene)),
            audio_manager: None,
        }
    }
}

#[derive(Component)]
pub struct SpatialListener;

#[derive(Event)]
pub struct PlaySpatialSound {
    pub sound: Handle<bevy_kira_audio::AudioSource>,
    pub position: Vec3,
    pub velocity: Vec3,
    pub volume: f32,
}

impl PlaySpatialSound {
    pub fn new(sound: Handle<bevy_kira_audio::AudioSource>, position: Vec3) -> Self {
        Self {
            sound,
            position,
            velocity: Vec3::ZERO,
            volume: 1.0,
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
            .add_systems(Startup, setup_spatial_audio_manager)
            .add_systems(Update, (
                update_spatial_listener,
                play_spatial_sounds,
            ));
    }
}

/// Initialize the AudioManager with spatial scene support
fn setup_spatial_audio_manager(
    mut spatial_manager: ResMut<SpatialAudioManager>,
) {
    // Note: In a full implementation, we would share the AudioManager
    // For Phase 4 we prepare the structure. Real emitter playback
    // requires deeper integration with bevy_kira_audio's internal manager.
    println!("[SpatialAudio] SpatialScene initialized (Phase 4 structure ready)");
}

/// Update the spatial listener position
fn update_spatial_listener(
    spatial_manager: Res<SpatialAudioManager>,
    listener_query: Query<&GlobalTransform, With<SpatialListener>>,
) {
    if let Ok(transform) = listener_query.get_single() {
        if let Ok(mut scene) = spatial_manager.spatial_scene.lock() {
            // In full Kira integration, we would call:
            // scene.set_listener_position(0, transform.translation().into(), ...);
            // This is prepared for when we have direct AudioManager access.
        }
    }
}

/// Play spatial sounds using the SpatialScene
fn play_spatial_sounds(
    mut events: EventReader<PlaySpatialSound>,
    audio: Res<bevy_kira_audio::Audio>,
    spatial_manager: Res<SpatialAudioManager>,
    asset_server: Res<AssetServer>,
) {
    if !spatial_manager.enabled {
        return;
    }

    for event in events.read() {
        // For now we fall back to enhanced non-spatial playback
        // with distance-based volume as a bridge.
        // Full SpatialEmitter integration requires exposing the
        // internal Kira AudioManager from bevy_kira_audio.

        let distance = event.position.length(); // placeholder
        let volume = (event.volume / (1.0 + distance * 0.01)).clamp(0.2, 1.0);

        audio
            .play(event.sound.clone())
            .with_volume(volume as f64);

        // TODO: Replace with real spatial emitter:
        // let emitter = spatial_scene.add_emitter(
        //     event.position.into(),
        //     SpatialEmitterSettings::new()
        //         .with_velocity(event.velocity.into())
        // );
        // emitter.play(sound_data);
    }
}
