/*!
 * Kira Dynamic Music - Polished Production Version
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use kira::effect::filter::{FilterBuilder, FilterHandle};
use std::collections::HashMap;
use crate::settings::audio_mixing::AudioMixer;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum MusicStateType {
    #[default]
    Exploration,
    Tension,
    Combat,
    IntenseCombat,
    Boss,
    Harvesting,
    Council,
    Victory,
    Death,
    Menu,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MusicLayer {
    Base,
    Tension,
    Percussion,
    Melody,
    Intense,
}

#[derive(Resource)]
pub struct KiraMusicController {
    pub current_state: MusicStateType,
    pub target_state: MusicStateType,
    pub intensity: f32,
    pub transition_timer: f32,
    pub transition_duration: f32,
    pub ducking: f32,
    pub duck_timer: f32,
    pub filter_handles: HashMap<MusicLayer, FilterHandle>,
    pub active_sounds: HashMap<MusicLayer, AudioHandle<AudioSource>>,
}

impl Default for KiraMusicController {
    fn default() -> Self {
        Self {
            current_state: MusicStateType::Exploration,
            target_state: MusicStateType::Exploration,
            intensity: 0.5,
            transition_timer: 0.0,
            transition_duration: 4.0,
            ducking: 0.0,
            duck_timer: 0.0,
            filter_handles: HashMap::new(),
            active_sounds: HashMap::new(),
        }
    }
}

/// Real filter automation with error safety
pub fn apply_kira_filter_automation(
    controller: Res<KiraMusicController>,
) {
    let intensity = controller.intensity;

    for filter in controller.filter_handles.values() {
        let cutoff = 650.0 + (intensity * 13500.0);
        // Kira filters are generally safe to call even if the sound stopped
        let _ = filter.set_cutoff(cutoff);
    }
}

/// Initialize filters with better error handling
pub fn initialize_kira_music(
    audio: Res<AudioManager>,
    mut controller: ResMut<KiraMusicController>,
) {
    controller.filter_handles.clear();

    let layers = [MusicLayer::Base, MusicLayer::Tension, MusicLayer::Percussion, MusicLayer::Melody, MusicLayer::Intense];

    for layer in layers {
        match audio.add_filter(FilterBuilder::new().cutoff(1000.0)) {
            Ok(filter) => {
                controller.filter_handles.insert(layer, filter);
            }
            Err(e) => {
                warn!("Failed to create filter for music layer {:?}: {:?}", layer, e);
            }
        }
    }

    info!("Initialized {} music layer filters", controller.filter_handles.len());
}

/// Main layered music system with improved robustness
pub fn update_kira_music(
    audio: Res<AudioManager>,
    mut controller: ResMut<KiraMusicController>,
    mixer: Res<AudioMixer>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    // Handle ducking
    if controller.duck_timer > 0.0 {
        controller.duck_timer -= time.delta_seconds();
        if controller.duck_timer <= 0.0 {
            controller.ducking = 0.0;
        }
    }

    let duck_multiplier = if controller.ducking > 0.0 {
        1.0 - controller.ducking
    } else {
        1.0
    };

    // State transition
    if controller.current_state != controller.target_state {
        controller.transition_timer += time.delta_seconds();
        if controller.transition_timer >= controller.transition_duration {
            controller.current_state = controller.target_state;
            controller.transition_timer = 0.0;
        }
    }

    let target_layers = get_active_layers(controller.current_state, controller.intensity);

    // Stop unused layers
    let current_layers: Vec<_> = controller.active_sounds.keys().cloned().collect();
    for layer in current_layers {
        if !target_layers.iter().any(|(l, _)| *l == layer) {
            if let Some(handle) = controller.active_sounds.remove(&layer) {
                let _ = handle.stop();
            }
        }
    }

    // Update or start layers
    for (layer, base_volume) in target_layers {
        let effective_volume = base_volume * duck_multiplier * mixer.music;

        if let Some(handle) = controller.active_sounds.get_mut(&layer) {
            let _ = handle.set_volume(effective_volume);
        } else {
            let path = get_layer_path(layer, controller.current_state);

            match StaticSoundData::from_file(path) {
                Ok(sound_data) => {
                    if let Some(filter) = controller.filter_handles.get(&layer) {
                        match audio.play(sound_data)
                            .looped()
                            .with_volume(0.0)
                            .with_filter(*filter)
                            .try_build()
                        {
                            Ok(handle) => {
                                let _ = handle.set_volume(effective_volume);
                                controller.active_sounds.insert(layer, handle);
                            }
                            Err(e) => {
                                warn!("Failed to play music layer {:?}: {:?}", layer, e);
                            }
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to load music layer {:?} at path '{}': {:?}", layer, path, e);
                }
            }
        }
    }
}

fn get_active_layers(state: MusicStateType, intensity: f32) -> Vec<(MusicLayer, f32)> {
    let mut layers = vec![];
    layers.push((MusicLayer::Base, 0.8 + intensity * 0.2));

    match state {
        MusicStateType::Exploration | MusicStateType::Harvesting | MusicStateType::Council => {
            if intensity > 0.3 {
                layers.push((MusicLayer::Melody, intensity * 0.85));
            }
        }
        MusicStateType::Tension => {
            layers.push((MusicLayer::Tension, 0.5 + intensity * 0.5));
        }
        MusicStateType::Combat => {
            layers.push((MusicLayer::Tension, 0.65));
            layers.push((MusicLayer::Percussion, intensity * 0.9));
        }
        MusicStateType::IntenseCombat | MusicStateType::Boss => {
            layers.push((MusicLayer::Tension, 0.85));
            layers.push((MusicLayer::Percussion, 0.8));
            layers.push((MusicLayer::Intense, intensity));
        }
        MusicStateType::Victory => {
            layers.push((MusicLayer::Melody, 1.0));
        }
        _ => {}
    }

    layers
}

fn get_layer_path(layer: MusicLayer, _state: MusicStateType) -> &'static str {
    match layer {
        MusicLayer::Base => "audio/music/layers/base.ogg",
        MusicLayer::Tension => "audio/music/layers/tension.ogg",
        MusicLayer::Percussion => "audio/music/layers/percussion.ogg",
        MusicLayer::Melody => "audio/music/layers/melody.ogg",
        MusicLayer::Intense => "audio/music/layers/intense.ogg",
    }
}

// Helper to stop all music (useful for scene changes, menus, etc.)
pub fn stop_all_music(mut controller: ResMut<KiraMusicController>) {
    for (_, handle) in controller.active_sounds.drain() {
        let _ = handle.stop();
    }
    controller.filter_handles.clear();
}
