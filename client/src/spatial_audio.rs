/*!
 * Spatial Audio System - Performance Optimized
 */

use bevy::prelude::*;
use kira::manager::AudioManager;
use kira::manager::backend::DefaultBackend;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::spatial::emitter::SpatialEmitterSettings;
use kira::spatial::listener::SpatialListenerSettings;
use kira::spatial::scene::{SpatialScene, SpatialSceneSettings};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Performance-tuned spatial audio manager
#[derive(Resource)]
pub struct SpatialAudioManager {
    pub enabled: bool,
    audio_manager: Arc<Mutex<Option<AudioManager<DefaultBackend>>>>,
    spatial_scene: Arc<Mutex<SpatialScene>>,
    listener_handle: Option<kira::spatial::listener::SpatialListenerHandle>,

    // Performance optimizations
    sound_cache: Arc<Mutex<HashMap<String, Arc<StaticSoundData>>>>,
    max_active_emitters: usize,
    active_emitters: Arc<Mutex<usize>>,
}

impl Default for SpatialAudioManager {
    fn default() -> Self {
        Self {
            enabled: true,
            audio_manager: Arc::new(Mutex::new(None)),
            spatial_scene: Arc::new(Mutex::new(SpatialScene::new(SpatialSceneSettings::new()))),
            listener_handle: None,
            sound_cache: Arc::new(Mutex::new(HashMap::new())),
            max_active_emitters: 32, // Reasonable default
            active_emitters: Arc::new(Mutex::new(0)),
        }
    }
}

impl SpatialAudioManager {
    /// Play a spatial sound with caching and emitter limiting
    pub fn try_play_spatial(
        &self,
        sound_path: &str,
        position: Vec3,
        velocity: Vec3,
        volume: f32,
    ) -> bool {
        if !self.enabled {
            return false;
        }

        // Check emitter limit
        {
            let active = self.active_emitters.lock().unwrap();
            if *active >= self.max_active_emitters {
                return false; // Too many sounds playing
            }
        }

        // Get or load sound (with caching)
        let sound_data = {
            let mut cache = self.sound_cache.lock().unwrap();
            if let Some(cached) = cache.get(sound_path) {
                cached.clone()
            } else {
                match StaticSoundData::from_file(sound_path) {
                    Ok(data) => {
                        let arc_data = Arc::new(data);
                        cache.insert(sound_path.to_string(), arc_data.clone());
                        arc_data
                    }
                    Err(e) => {
                        warn!("Failed to load spatial sound '{}': {}", sound_path, e);
                        return false;
                    }
                }
            }
        };

        // Create emitter
        let emitter_settings = SpatialEmitterSettings::new()
            .with_position(position.into())
            .with_velocity(velocity.into())
            .with_volume(volume);

        if let Ok(mut scene) = self.spatial_scene.lock() {
            match scene.add_emitter(position.into(), emitter_settings) {
                Ok(mut emitter) => {
                    if let Err(e) = emitter.play((*sound_data).clone()) {
                        warn!("Failed to play spatial sound: {}", e);
                        return false;
                    }

                    // Track active emitter count
                    *self.active_emitters.lock().unwrap() += 1;
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

    /// Set maximum number of concurrent spatial sounds
    pub fn set_max_emitters(&mut self, max: usize) {
        self.max_active_emitters = max;
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

fn setup_spatial_audio(
    mut spatial_manager: ResMut<SpatialAudioManager>,
) {
    match AudioManager::<DefaultBackend>::new(Default::default()) {
        Ok(audio_manager) => {
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
            info!("[SpatialAudio] Optimized SpatialScene initialized (max emitters: {})", spatial_manager.max_active_emitters);
        }
        Err(e) => {
            error!("Failed to create AudioManager for spatial audio: {}", e);
            spatial_manager.enabled = false;
        }
    }
}

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

fn handle_play_spatial_sound_events(
    mut events: EventReader<PlaySpatialSound>,
    spatial_manager: Res<SpatialAudioManager>,
) {
    if !spatial_manager.enabled {
        return;
    }

    for event in events.read() {
        spatial_manager.try_play_spatial(
            &event.sound_path,
            event.position,
            event.velocity,
            event.volume,
        );
    }
}
