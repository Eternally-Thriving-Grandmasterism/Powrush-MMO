/*!
 * Full Kira Dynamic Music - Sound Loading + Layered Playback
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use kira::effect::filter::{FilterBuilder, FilterHandle};
use kira::sound::static_sound::StaticSoundData;
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
    /// Store active sound handles so we can control/stop them
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

/// Real filter automation
pub fn apply_kira_filter_automation(
    controller: Res<KiraMusicController>,
) {
    let intensity = controller.intensity;
    for filter in controller.filter_handles.values() {
        let cutoff = 650.0 + (intensity * 13500.0);
        filter.set_cutoff(cutoff);
    }
}

/// Initialize filters and prepare music layers
pub fn initialize_kira_music(
    audio: Res<AudioManager>,
    mut controller: ResMut<KiraMusicController>,
) {
    controller.filter_handles.clear();

    let layers = [MusicLayer::Base, MusicLayer::Tension, MusicLayer::Percussion, MusicLayer::Melody, MusicLayer::Intense];

    for layer in layers {
        if let Ok(filter) = audio.add_filter(FilterBuilder::new().cutoff(1000.0)) {
            controller.filter_handles.insert(layer, filter);
        }
    }
}

/// Main system that manages layered music playback with Kira
pub fn update_kira_music(
    audio: Res<AudioManager>,
    mut controller: ResMut<KiraMusicController>,
    mixer: Res<AudioMixer>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    // Handle state transitions
    if controller.current_state != controller.target_state {
        controller.transition_timer += time.delta_seconds();
        if controller.transition_timer >= controller.transition_duration {
            controller.current_state = controller.target_state;
            controller.transition_timer = 0.0;
        }
    }

    let target_layers = get_active_layers(controller.current_state, controller.intensity);

    // Stop layers that are no longer needed
    let current_layers: Vec<_> = controller.active_sounds.keys().cloned().collect();
    for layer in current_layers {
        if !target_layers.iter().any(|(l, _)| *l == layer) {
            if let Some(handle) = controller.active_sounds.remove(&layer) {
                handle.stop();
            }
        }
    }

    // Start or update needed layers
    for (layer, target_volume) in target_layers {
        let effective_volume = target_volume * (1.0 - controller.ducking) * mixer.music;

        if let Some(handle) = controller.active_sounds.get_mut(&layer) {
            // Update volume (Kira handles support volume changes)
            handle.set_volume(effective_volume);
        } else {
            // Load and play new layer
            let path = get_layer_path(layer, controller.current_state);
            if let Ok(sound_data) = StaticSoundData::from_file(path) {
                if let Some(filter) = controller.filter_handles.get(&layer) {
                    // Play with filter attached
                    if let Ok(handle) = audio.play(sound_data)
                        .looped()
                        .with_volume(0.0)
                        .with_filter(*filter)
                        .try_build() 
                    {
                        // Fade in
                        handle.set_volume(effective_volume);
                        controller.active_sounds.insert(layer, handle);
                    }
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
