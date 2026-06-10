/*!
 * Spatial Audio System - Advanced SpatialScene + Emitters (Option B)
 */

use bevy::prelude::*;
use kira::manager::AudioManager;
use kira::manager::backend::DefaultBackend;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::spatial::emitter::SpatialEmitterSettings;
use kira::spatial::listener::SpatialListenerSettings;
use kira::spatial::scene::{SpatialScene, SpatialSceneSettings};
use std::sync::{Arc, Mutex};

/// Main spatial audio resource with direct Kira AudioManager
#[derive(Resource)]
pub struct SpatialAudioManager {
    pub enabled: bool,
    pub audio_manager: Arc<Mutex<Option<AudioManager<DefaultBackend>>>>,
    pub spatial_scene: Arc<Mutex<SpatialScene>>,
    pub listener_handle: Option<kira::spatial::listener::SpatialListenerHandle>,
}

impl Default for SpatialAudioManager {
    fn default() -> Self {
        let spatial_scene = SpatialScene::new(SpatialSceneSettings::new());

        Self {
            enabled: true,
            audio_manager: Arc::new(Mutex::new(None)),
            spatial_scene: Arc::new(Mutex::new(spatial_scene)),
            listener_handle: None,
        }
    }
}

#[derive(Component)]
pub struct SpatialListener;

#[derive(Event)]
pub struct PlaySpatialSound {
    pub sound_path: String, // Path to the audio file
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
}

pub struct SpatialAudioPlugin;

impl Plugin for SpatialAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SpatialAudioManager>()
            .add_event::<PlaySpatialSound>()
            .add_systems(Startup, setup_spatial_audio)
            .add_systems(Update, (
                update_listener,
                play_spatial_sounds,
            ));
    }
}

/// Initialize AudioManager + SpatialScene
fn setup_spatial_audio(
    mut spatial_manager: ResMut<SpatialAudioManager>,
) {
    let audio_manager = AudioManager::<DefaultBackend>::new(Default::default())
        .expect("Failed to create AudioManager");

    // Add listener to spatial scene
    let listener_settings = SpatialListenerSettings::new();
    if let Ok(mut scene) = spatial_manager.spatial_scene.lock() {
        let listener_handle = scene.add_listener(Vec3::ZERO.into(), listener_settings)
            .expect("Failed to add listener");
        spatial_manager.listener_handle = Some(listener_handle);
    }

    *spatial_manager.audio_manager.lock().unwrap() = Some(audio_manager);

    println!("[SpatialAudio] Advanced SpatialScene initialized with listener");
}

/// Update listener position every frame
fn update_listener(
    spatial_manager: Res<SpatialAudioManager>,
    listener_query: Query<&GlobalTransform, With<SpatialListener>>,
) {
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

/// Play sounds through Spatial Emitters
fn play_spatial_sounds(
    mut events: EventReader<PlaySpatialSound>,
    spatial_manager: Res<SpatialAudioManager>,
    asset_server: Res<AssetServer>,
) {
    if !spatial_manager.enabled {
        return;
    }

    let audio_manager = match spatial_manager.audio_manager.lock().unwrap().as_mut() {
        Some(manager) => manager,
        None => return,
    };

    if let Ok(mut scene) = spatial_manager.spatial_scene.lock() {
        for event in events.read() {
            // Load sound data
            let sound_data = match StaticSoundData::from_file(&event.sound_path) {
                Ok(data) => data,
                Err(e) => {
                    warn!("Failed to load spatial sound {}: {}", event.sound_path, e);
                    continue;
                }
            };

            // Create spatial emitter
            let emitter_settings = SpatialEmitterSettings::new()
                .with_position(event.position.into())
                .with_velocity(event.velocity.into())
                .with_volume(event.volume);

            match scene.add_emitter(event.position.into(), emitter_settings) {
                Ok(mut emitter) => {
                    if let Err(e) = emitter.play(sound_data) {
                        warn!("Failed to play spatial sound: {}", e);
                    }
                }
                Err(e) => {
                    warn!("Failed to create spatial emitter: {}", e);
                }
            }
        }
    }
}
