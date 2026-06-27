/*!
 * Dynamic Music System - Layer Mixing Logic
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::settings::audio_mixing::{AudioMixer, DynamicAudio, AudioCategory, Priority};

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
pub struct MusicController {
    pub current_state: MusicStateType,
    pub target_state: MusicStateType,
    pub intensity: f32,
    pub transition_timer: f32,
    pub transition_duration: f32,
}

impl Default for MusicController {
    fn default() -> Self {
        Self {
            current_state: MusicStateType::Exploration,
            target_state: MusicStateType::Exploration,
            intensity: 0.5,
            transition_timer: 0.0,
            transition_duration: 4.0,
        }
    }
}

/// Request a music state change
pub fn request_music_state(
    mut controller: ResMut<MusicController>,
    new_state: MusicStateType,
) {
    if controller.target_state != new_state {
        controller.target_state = new_state;
        controller.transition_timer = 0.0;
    }
}

/// Manages layered music stems with smooth blending
#[derive(Resource, Default)]
pub struct MusicLayers {
    pub layers: Vec<(MusicLayer, Option<Entity>, f32)>, // (layer, entity, target_volume)
}

pub fn update_music_layers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut controller: ResMut<MusicController>,
    mut music_layers: ResMut<MusicLayers>,
    mixer: Res<AudioMixer>,
    time: Res<Time>,
) {
    // Handle state transition
    if controller.current_state != controller.target_state {
        controller.transition_timer += time.delta_seconds();
        if controller.transition_timer >= controller.transition_duration {
            controller.current_state = controller.target_state;
            controller.transition_timer = 0.0;
        }
    }

    // Define which layers should be active for current state + intensity
    let target_layers = get_active_layers(controller.current_state, controller.intensity);

    // Update or spawn layers
    for (layer, target_volume) in target_layers {
        if let Some((_, entity, current_target)) = music_layers.layers.iter_mut().find(|(l, _, _)| *l == layer) {
            *current_target = target_volume;
        } else {
            // Spawn new layer
            let path = get_layer_path(layer, controller.current_state);
            let music = asset_server.load(path);
            let entity = commands.spawn((
                AudioBundle {
                    source: music,
                    settings: PlaybackSettings::LOOP
                        .with_volume(mixer.music * target_volume),
                },
                DynamicAudio {
                    category: AudioCategory::Music,
                    priority: Priority::High,
                },
            )).id();

            music_layers.layers.push((layer, Some(entity), target_volume));
        }
    }

    // Smoothly update volumes and despawn unused layers
    let mut to_remove = vec![];
    for (i, (layer, entity, target_vol)) in music_layers.layers.iter_mut().enumerate() {
        if let Some(ent) = *entity {
            if let Ok(mut sink) = commands.get_entity(ent) {
                // Smooth blend toward target volume
                // (In real implementation use AudioSink volume lerp)
            }
        }

        // Remove layers that should no longer be active
        if !target_layers.iter().any(|(l, _)| *l == *layer) {
            if let Some(ent) = *entity {
                commands.entity(ent).despawn();
            }
            to_remove.push(i);
        }
    }

    for i in to_remove.into_iter().rev() {
        music_layers.layers.remove(i);
    }
}

/// Returns which layers should be active for a given state and intensity
fn get_active_layers(state: MusicStateType, intensity: f32) -> Vec<(MusicLayer, f32)> {
    let mut layers = vec![];

    // Base layer always present
    layers.push((MusicLayer::Base, 0.8 + intensity * 0.2));

    match state {
        MusicStateType::Exploration | MusicStateType::Harvesting | MusicStateType::Council => {
            if intensity > 0.3 {
                layers.push((MusicLayer::Melody, intensity));
            }
        }
        MusicStateType::Tension => {
            layers.push((MusicLayer::Tension, 0.6 + intensity * 0.4));
        }
        MusicStateType::Combat => {
            layers.push((MusicLayer::Tension, 0.7));
            layers.push((MusicLayer::Percussion, intensity));
        }
        MusicStateType::IntenseCombat | MusicStateType::Boss => {
            layers.push((MusicLayer::Tension, 0.9));
            layers.push((MusicLayer::Percussion, 0.8 + intensity * 0.2));
            layers.push((MusicLayer::Intense, intensity));
        }
        MusicStateType::Victory => {
            layers.push((MusicLayer::Melody, 1.0));
        }
        _ => {}
    }

    layers
}

fn get_layer_path(layer: MusicLayer, state: MusicStateType) -> &'static str {
    match (layer, state) {
        (MusicLayer::Base, _) => "audio/music/layers/base.ogg",
        (MusicLayer::Tension, _) => "audio/music/layers/tension.ogg",
        (MusicLayer::Percussion, _) => "audio/music/layers/percussion.ogg",
        (MusicLayer::Melody, _) => "audio/music/layers/melody.ogg",
        (MusicLayer::Intense, _) => "audio/music/layers/intense.ogg",
        _ => "audio/music/exploration.ogg",
    }
}

// Basic evaluate + single-track fallback (kept for compatibility)
pub fn evaluate_music_state(
    mut controller: ResMut<MusicController>,
) {}

pub fn update_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut controller: ResMut<MusicController>,
    mixer: Res<AudioMixer>,
    time: Res<Time>,
    mut current_music: Local<Option<Entity>>,
) {
    // Fallback single-track system (can be disabled once layered system is stable)
    if controller.current_state != controller.target_state {
        controller.transition_timer += time.delta_seconds();
        if controller.transition_timer >= controller.transition_duration {
            controller.current_state = controller.target_state;
            controller.transition_timer = 0.0;
        }
    }

    let music_path = match controller.current_state {
        MusicStateType::Exploration => "audio/music/exploration.ogg",
        MusicStateType::Tension => "audio/music/tension.ogg",
        MusicStateType::Combat => "audio/music/combat.ogg",
        MusicStateType::IntenseCombat => "audio/music/intense_combat.ogg",
        MusicStateType::Boss => "audio/music/boss.ogg",
        MusicStateType::Harvesting => "audio/music/harvesting.ogg",
        MusicStateType::Council => "audio/music/council.ogg",
        MusicStateType::Victory => "audio/music/victory.ogg",
        MusicStateType::Death => "audio/music/death.ogg",
        MusicStateType::Menu => "audio/music/menu.ogg",
    };

    if current_music.is_none() {
        let music = asset_server.load(music_path);
        let entity = commands.spawn((
            AudioBundle {
                source: music,
                settings: PlaybackSettings::LOOP
                    .with_volume(mixer.music * (0.6 + controller.intensity * 0.4)),
            },
            DynamicAudio {
                category: AudioCategory::Music,
                priority: Priority::High,
            },
        )).id();
        *current_music = Some(entity);
    }
}
