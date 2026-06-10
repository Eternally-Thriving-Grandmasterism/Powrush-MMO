/*!
 * Spatial Audio System - Advanced SpatialScene Integration (Refined)
 *
 * This module uses a dedicated kira::AudioManager to enable full
 * SpatialScene + SpatialEmitter support from Kira.
 *
 * Note: We currently maintain two AudioManagers:
 * - One from bevy_kira_audio (used for Divine Whispers, UI sounds, etc.)
 * - One here for advanced spatial/3D audio features.
 *
 * This is a common pattern when needing deeper Kira functionality.
 */

use bevy::prelude::*;
use kira::manager::AudioManager;
use kira::manager::backend::DefaultBackend;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::spatial::emitter::SpatialEmitterSettings;
use kira::spatial::listener::SpatialListenerSettings;
use kira::spatial::scene::{SpatialScene, SpatialSceneSettings};
use std::sync::{Arc, Mutex};

/// Main resource for advanced spatial audio
#[derive(Resource)]
pub struct SpatialAudioManager {
    pub enabled: bool,
    audio_manager: Arc<Mutex<Option<AudioManager<DefaultBackend>>>>,
    spatial_scene: Arc<Mutex<SpatialScene>>,
    listener_handle: Option<kira::spatial::listener::SpatialListenerHandle>,
}

impl Default for SpatialAudioManager {
    fn default() -> Self {
        Self {
            enabled: true,
            audio_manager: Arc::new(Mutex::new(None)),
            spatial_scene: Arc::new(Mutex::new(SpatialScene::new(SpatialSceneSettings::new()))),
            listener_handle: None,
        }
    }
}

impl SpatialAudioManager {
    /// Try to play a spatial sound. Returns true on success.
    pub fn try_play_spatial(
        &self,
        sound_path: &str,
        position: Vec3,
        velocity: Vec3,
        volume: f32,
    ) -> bool {
        let audio_manager = match self.audio_manager.lock() {
            Ok(guard) => guard,
            Err(_) => return false,
        };

        let audio_manager = match audio_manager.as_ref() {
            Some(manager) => manager,
            None => return false,
        };

        let sound_data = match StaticSoundData::from_file(sound_path) {
            Ok(data) => data,
            Err(e) => {
                warn!("Failed to load spatial sound '{}': {}", sound_path, e);
                return false;
            }
        };

        let emitter_settings = SpatialEmitterSettings::new()
            .with_position(position.into())
            .with_velocity(velocity.into())
            .with_volume(volume);

        if let Ok(mut scene) = self.spatial_scene.lock() {
            match scene.add_emitter(position.into(), emitter_settings) {
                Ok(mut emitter) => {
                    if let Err(e) = emitter.play(sound_data) {
                        warn!("Failed to play spatial sound: {}", e);
                        return false;
                    }
                    true
                }
                Err(e) => {
                    warn!("Failed to create spatial emitter: {}", e);
                    false
                }
            }
        } else {
            false
        }
    }
}

#[derive(Component)]
pub struct SpatialListener;

#[derive(Event, Debug)]
pub struct PlaySpatialSound {
    pub sound_path: String,
    pub position: Vec3,
    pub velocity: Vec3,
    pub volume: f32,
}

impl PlaySpatialSound {
    pub fn new(sound_path: impl Into<String>, position: Vec3) -> Self {
        Self {
            sound_path: sound_path.into(),
            position,
            velocity: Vec3::ZERO,
            volume: 1.0,
        }
    }

    pub fn with_velocity(mut self, velocity: Vec3) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }
}

pub struct SpatialAudioPlugin;

impl Plugin for SpatialAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SpatialAudioManager>()
            .add_event::<PlaySpatialSound>()
            .add_systems(Startup, setup_spatial_audio)
            .add_systems(Update, (
                update_spatial_listener,
                handle_play_spatial_sound_events,
            ));
    }
}

/// Initialize the dedicated AudioManager + SpatialScene
fn setup_spatial_audio(
    mut spatial_manager: ResMut<SpatialAudioManager>,
) {
    match AudioManager::<DefaultBackend>::new(Default::default()) {
        Ok(audio_manager) => {
            // Add listener to the spatial scene
            let listener_settings = SpatialListenerSettings::new();

            if let Ok(mut scene) = spatial_manager.spatial_scene.lock() {
                match scene.add_listener(Vec3::ZERO.into(), listener_settings) {
                    Ok(listener_handle) => {
                        spatial_manager.listener_handle = Some(listener_handle);
                    }
                    Err(e) => {
                        error!("Failed to add spatial listener: {}", e);
                    }
                }
            }

            *spatial_manager.audio_manager.lock().unwrap() = Some(audio_manager);
            info!("[SpatialAudio] Advanced SpatialScene initialized successfully");
        }
        Err(e) => {
            error!("Failed to create AudioManager for spatial audio: {}", e);
            spatial_manager.enabled = false;
        }
    }
}

/// Update listener position every frame
fn update_spatial_listener(
    spatial_manager: Res<SpatialAudioManager>,
    listener_query: Query<&GlobalTransform, With<SpatialListener>>,
) {
    if !spatial_manager.enabled {
        return;
    }

    if let Ok(transform) = listener_query.get_single() {
        if let Some(ref listener_handle) = spatial_manager.listener_handle {
            if let Ok(mut scene) = spatial_manager.spatial_scene.lock() {
                let _ = scene.set_listener_position(
                    listener_handle.id(),
                    transform.translation().into(),
                );
            }
        }
    }
}

/// Handle PlaySpatialSound events
fn handle_play_spatial_sound_events(
    mut events: EventReader<PlaySpatialSound>,
    spatial_manager: Res<SpatialAudioManager>,
) {
    if !spatial_manager.enabled {
        return;
    }

    for event in events.read() {
        if !spatial_manager.try_play_spatial(
            &event.sound_path,
            event.position,
            event.velocity,
            event.volume,
        ) {
            warn!("Failed to play spatial sound: {}", event.sound_path);
        }
    }
}
