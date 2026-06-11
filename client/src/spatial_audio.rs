/*!
 * Spatial Audio System + Game Audio Abstraction
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

use crate::fundsp_audio::{build_epiphany_resonance, ActiveEpiphanyResonance, ActiveProceduralEpiphanies};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SpatialQuality {
    Low,
    #[default]
    Medium,
    High,
}

#[derive(Resource)]
pub struct SpatialAudioManager {
    pub enabled: bool,
    pub quality: SpatialQuality,
    pub hrtf_enabled: bool,
    pub current_hrtf_dataset: Option<String>,
    audio_manager: Arc<Mutex<Option<AudioManager<DefaultBackend>>>>,
    spatial_scene: Arc<Mutex<SpatialScene>>,
    listener_handle: Option<kira::spatial::listener::SpatialListenerHandle>,
    sound_cache: Arc<Mutex<HashMap<String, Arc<StaticSoundData>>>>,
    max_active_emitters: usize,
    active_emitters: Arc<Mutex<usize>>,
}

impl Default for SpatialAudioManager {
    fn default() -> Self {
        Self {
            enabled: true,
            quality: SpatialQuality::default(),
            hrtf_enabled: false,
            current_hrtf_dataset: None,
            audio_manager: Arc::new(Mutex::new(None)),
            spatial_scene: Arc::new(Mutex::new(SpatialScene::new(SpatialSceneSettings::new()))),
            listener_handle: None,
            sound_cache: Arc::new(Mutex::new(HashMap::new())),
            max_active_emitters: 32,
            active_emitters: Arc::new(Mutex::new(0)),
        }
    }
}

impl SpatialAudioManager {
    pub fn set_spatial_quality(&mut self, quality: SpatialQuality) {
        self.quality = quality;
        match quality {
            SpatialQuality::Low => {
                self.hrtf_enabled = false;
                self.max_active_emitters = 16;
            }
            SpatialQuality::Medium => {
                self.hrtf_enabled = false;
                self.max_active_emitters = 32;
            }
            SpatialQuality::High => {
                self.hrtf_enabled = true;
                self.max_active_emitters = 24;
                if self.current_hrtf_dataset.is_none() {
                    let _ = self.preload_hrtf_dataset("mit_kemar");
                }
            }
        }
    }

    pub fn preload_hrtf_dataset(&mut self, dataset_name: &str) -> bool {
        if dataset_name != "mit_kemar" {
            return false;
        }
        self.current_hrtf_dataset = Some(dataset_name.to_string());
        true
    }

    pub fn play_generated_spatial(
        &self,
        samples: Vec<f32>,
        position: Vec3,
        velocity: Vec3,
        volume: f32,
    ) -> bool {
        if !self.enabled || samples.is_empty() {
            return false;
        }

        {
            let active = self.active_emitters.lock().unwrap();
            if *active >= self.max_active_emitters {
                return false;
            }
        }

        let sound_data = StaticSoundData::from_samples(samples, 44100)
            .with_settings(StaticSoundSettings::new());

        let emitter_settings = SpatialEmitterSettings::new()
            .with_position(position.into())
            .with_velocity(velocity.into())
            .with_volume(volume);

        if let Ok(mut scene) = self.spatial_scene.lock() {
            match scene.add_emitter(position.into(), emitter_settings) {
                Ok(mut emitter) => {
                    if let Err(e) = emitter.play(sound_data) {
                        warn!("Failed to play generated spatial sound: {}", e);
                        return false;
                    }
                    *self.active_emitters.lock().unwrap() += 1;
                    true
                }
                Err(e) => {
                    warn!("Failed to create emitter for generated sound: {}", e);
                    false
                }
            }
        } else {
            false
        }
    }

    pub fn try_play_spatial(
        &self,
        sound_path: &str,
        position: Vec3,
        velocity: Vec3,
        volume: f32,
        looped: bool,
    ) -> bool {
        if !self.enabled {
            return false;
        }

        {
            let active = self.active_emitters.lock().unwrap();
            if *active >= self.max_active_emitters {
                return false;
            }
        }

        let sound_data = {
            let mut cache = self.sound_cache.lock().unwrap();
            if let Some(cached) = cache.get(sound_path) {
                cached.clone()
            } else {
                match StaticSoundData::from_file(sound_path) {
                    Ok(data) => {
                        let settings = if looped {
                            kira::sound::static_sound::StaticSoundSettings::new().loop_region(..)
                        } else {
                            kira::sound::static_sound::StaticSoundSettings::new()
                        };
                        let data = data.with_settings(settings);
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

    pub fn set_max_emitters(&mut self, max: usize) {
        self.max_active_emitters = max;
    }
}

#[derive(Component)]
pub struct SpatialListener;

/// High-level game audio events (backend agnostic)
#[derive(Event, Debug, Clone)]
pub enum GameAudioEvent {
    Epiphany {
        position: Vec3,
        intensity: f32,
    },
    Harvest {
        position: Vec3,
        is_sustainable: bool,
    },
}

#[derive(Event, Debug)]
pub struct PlaySpatialSound {
    pub sound_path: String,
    pub position: Vec3,
    pub velocity: Vec3,
    pub volume: f32,
    pub looped: bool,
}

impl PlaySpatialSound {
    pub fn new(sound_path: impl Into<String>, position: Vec3) -> Self {
        Self {
            sound_path: sound_path.into(),
            position,
            velocity: Vec3::ZERO,
            volume: 1.0,
            looped: false,
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

    pub fn looped(mut self) -> Self {
        self.looped = true;
    }
}

pub struct SpatialAudioPlugin;

impl Plugin for SpatialAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SpatialAudioManager>()
            .add_event::<GameAudioEvent>()
            .add_event::<PlaySpatialSound>()
            .add_systems(Startup, setup_spatial_audio)
            .add_systems(Update, (
                update_spatial_listener,
                handle_game_audio_events,
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
                if let Ok(listener_handle) = scene.add_listener(Vec3::ZERO.into(), listener_settings) {
                    spatial_manager.listener_handle = Some(listener_handle);
                }
            }
            *spatial_manager.audio_manager.lock().unwrap() = Some(audio_manager);
            info!("[SpatialAudio] Initialized with nth-degree modulated fundsp resonance");
        }
        Err(e) => {
            error!("Failed to create AudioManager: {}", e);
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
                let _ = scene.set_listener_position(listener_handle.id(), transform.translation().into());
            }
        }
    }
}

/// Epiphany handling with deeply modulated procedural resonance
fn handle_game_audio_events(
    mut game_events: EventReader<GameAudioEvent>,
    mut active_epiphanies: ResMut<crate::fundsp_audio::ActiveProceduralEpiphanies>,
    listener_query: Query<&GlobalTransform, With<SpatialListener>>,
) {
    for event in game_events.read() {
        let sound_position = if let Ok(listener_transform) = listener_query.get_single() {
            listener_transform.translation() + Vec3::new(0.0, 1.5, -6.0)
        } else {
            Vec3::new(0.0, 2.0, -8.0)
        };

        if let GameAudioEvent::Epiphany { intensity, .. } = event {
            if *intensity > 0.35 {
                let (graph, intensity_var) = build_epiphany_resonance(*intensity);

                let total_duration = (1.4 + intensity * 3.0).clamp(1.2, 5.5);

                active_epiphanies.instances.push(
                    crate::fundsp_audio::ActiveEpiphanyResonance {
                        graph,
                        intensity_var,
                        remaining_duration: total_duration,
                        total_duration,
                        chunk_duration: 0.22,
                        position: sound_position,
                    },
                );
            }

            // Sample-based layers
            let volume = (0.55 + intensity * 0.32).clamp(0.45, 0.95);
            let pitch = (0.96 + intensity * 0.07).clamp(0.94, 1.08);

            // (Sample playback remains)
        }

        if let GameAudioEvent::Harvest { .. } = event {
            // Harvest handling
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
            event.looped,
        );
    }
}
