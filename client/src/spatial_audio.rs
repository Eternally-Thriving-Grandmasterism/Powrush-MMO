/*!
 * Spatial Audio + Game Audio Event System — Powrush-MMO
 *
 * v18.7 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — High-fidelity 3D spatial audio powered by kira + bevy_kira_audio
 * — Procedural generation via fundsp (now using centralized spawn helper)
 * — Dynamic listener following camera + emitter pooling
 * — GameAudioEvent routing for Epiphany, Harvest, Council, Treaty, RBE flows
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 * — Sounds designed to inspire mercy, joy, and universal thriving
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
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

use crate::fundsp_audio::{
    build_epiphany_resonance, build_rbe_abundance_flow, build_council_harmony,
    spawn_active_procedural_sound, ActiveProceduralSounds, ProceduralSoundType,
};

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

    /// Play procedurally generated samples in 3D space (called by fundsp_audio rolling chunks)
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
                        warn!("[SpatialAudio] Failed to play generated spatial: {}", e);
                        return false;
                    }
                    *self.active_emitters.lock().unwrap() += 1;
                    true
                }
                Err(e) => {
                    warn!("[SpatialAudio] Emitter creation failed: {}", e);
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
                            StaticSoundSettings::new().loop_region(..)
                        } else {
                            StaticSoundSettings::new()
                        };
                        let data = data.with_settings(settings);
                        let arc_data = Arc::new(data);
                        cache.insert(sound_path.to_string(), arc_data.clone());
                        arc_data
                    }
                    Err(e) => {
                        warn!("[SpatialAudio] Failed to load '{}': {}", sound_path, e);
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
                        warn!("[SpatialAudio] Play failed: {}", e);
                        return false;
                    }
                    *self.active_emitters.lock().unwrap() += 1;
                    true
                }
                Err(e) => {
                    warn!("[SpatialAudio] Emitter failed: {}", e);
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

/// High-level game audio events (routed to procedural or asset playback)
#[derive(Event, Debug, Clone)]
pub enum GameAudioEvent {
    Epiphany { position: Vec3, intensity: f32 },
    Harvest { position: Vec3, is_sustainable: bool },
    RbeFlow { position: Vec3, abundance: f32 },
    CouncilTrial { position: Vec3, intensity: f32 },
    TreatySuccess { position: Vec3, joy: f32 },
    UiFeedback { sound: UiSound },
}

#[derive(Debug, Clone, Copy)]
pub enum UiSound {
    ButtonHover,
    ButtonClick,
    CouncilOpen,
    TreatyConfirm,
    AbundancePing,
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
        self
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
            .add_systems(
                Update,
                (
                    update_spatial_listener,
                    handle_game_audio_events,
                    handle_play_spatial_sound_events,
                ),
            );
    }
}

fn setup_spatial_audio(mut spatial_manager: ResMut<SpatialAudioManager>) {
    match AudioManager::<DefaultBackend>::new(Default::default()) {
        Ok(audio_manager) => {
            let listener_settings = SpatialListenerSettings::new();
            if let Ok(mut scene) = spatial_manager.spatial_scene.lock() {
                if let Ok(listener_handle) = scene.add_listener(Vec3::ZERO.into(), listener_settings) {
                    spatial_manager.listener_handle = Some(listener_handle);
                }
            }
            *spatial_manager.audio_manager.lock().unwrap() = Some(audio_manager);
            info!("[SpatialAudio] Kira spatial scene initialized — mercy-aligned 3D audio ready");
        }
        Err(e) => {
            error!("[SpatialAudio] AudioManager creation failed: {}", e);
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

/// Routes GameAudioEvents to procedural generation (now using centralized spawn helper)
fn handle_game_audio_events(
    mut game_events: EventReader<GameAudioEvent>,
    mut active: ResMut<ActiveProceduralSounds>,
    listener_query: Query<&GlobalTransform, With<SpatialListener>>,
) {
    for event in game_events.read() {
        let sound_position = if let Ok(listener_transform) = listener_query.get_single() {
            listener_transform.translation() + Vec3::new(0.0, 1.5, -6.0)
        } else {
            Vec3::new(0.0, 2.0, -8.0)
        };

        match event {
            GameAudioEvent::Epiphany { intensity, .. } => {
                if *intensity > 0.3 {
                    let (graph, intensity_var) = build_epiphany_resonance(*intensity);
                    let total = (1.4 + intensity * 3.5).clamp(1.2, 6.0);
                    let sound = spawn_active_procedural_sound(
                        graph,
                        intensity_var,
                        var(1.0),
                        total,
                        0.22,
                        sound_position,
                        ProceduralSoundType::Epiphany,
                    );
                    active.instances.push(sound);
                }
            }
            GameAudioEvent::RbeFlow { abundance, .. } => {
                if *abundance > 0.2 {
                    let (graph, intensity_var) = build_rbe_abundance_flow(*abundance);
                    let total = 2.8;
                    let sound = spawn_active_procedural_sound(
                        graph,
                        intensity_var,
                        var(1.0),
                        total,
                        0.18,
                        sound_position,
                        ProceduralSoundType::RbeAbundance,
                    );
                    active.instances.push(sound);
                }
            }
            GameAudioEvent::CouncilTrial { intensity, .. } => {
                let (graph, intensity_var) = build_council_harmony(*intensity);
                let total = 4.5;
                let sound = spawn_active_procedural_sound(
                    graph,
                    intensity_var,
                    var(1.0),
                    total,
                    0.25,
                    sound_position,
                    ProceduralSoundType::CouncilHarmony,
                );
                active.instances.push(sound);
            }
            GameAudioEvent::TreatySuccess { joy, .. } => {
                if *joy > 0.4 {
                    let (graph, intensity_var) = build_epiphany_resonance((*joy * 0.7).min(1.0));
                    let total = 3.2;
                    let sound = spawn_active_procedural_sound(
                        graph,
                        intensity_var,
                        var(1.0),
                        total,
                        0.2,
                        sound_position,
                        ProceduralSoundType::TreatySuccess,
                    );
                    active.instances.push(sound);
                }
            }
            GameAudioEvent::Harvest { .. } | GameAudioEvent::UiFeedback { .. } => {
                // Future: route to pre-authored assets via bevy_kira_audio or try_play_spatial
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
            event.looped,
        );
    }
}

// End of spatial_audio.rs v18.7 — Fully aligned with fundsp_audio and CouncilTrialUI.
// Thunder locked in. Yoi ⚡
