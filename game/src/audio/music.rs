/*!
 * Dynamic Music System - Smooth Layer Volume Lerping
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

pub fn request_music_state(
    mut controller: ResMut<MusicController>,
    new_state: MusicStateType,
) {
    if controller.target_state != new_state {
        controller.target_state = new_state;
        controller.transition_timer = 0.0;
    }
}

/// Tracks active music layers with current and target volumes for smooth lerping
#[derive(Resource, Default)]
pub struct MusicLayers {
    /// (layer_type, entity, current_volume, target_volume)
    pub layers: Vec<(MusicLayer, Option<Entity>, f32, f32)>,
}

/// Smoothly updates music layer volumes with lerping
pub fn update_music_layers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut controller: ResMut<MusicController>,
    mut music_layers: ResMut<MusicLayers>,
    mixer: Res<AudioMixer>,
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

    // Spawn new layers or update targets
    for (layer, target_vol) in target_layers {
        if let Some((_, entity, _, target)) = music_layers.layers.iter_mut().find(|(l, _, _, _)| *l == layer) {
            *target = target_vol;
        } else {
            let path = get_layer_path(layer, controller.current_state);
            let music = asset_server.load(path);
            let entity = commands.spawn((
                AudioBundle {
                    source: music,
                    settings: PlaybackSettings::LOOP.with_volume(0.0), // start silent for smooth fade-in
                },
                DynamicAudio {
                    category: AudioCategory::Music,
                    priority: Priority::High,
                },
            )).id();

            music_layers.layers.push((layer, Some(entity), 0.0, target_vol));
        }
    }

    // Lerp volumes and clean up unused layers
    let lerp_speed = 2.5; // Higher = faster transitions
    let mut to_remove = vec![];

    for (i, (layer, entity, current_vol, target_vol)) in music_layers.layers.iter_mut().enumerate() {
        // Smooth lerp toward target
        let new_vol = *current_vol + (*target_vol - *current_vol) * lerp_speed * time.delta_seconds();
        *current_vol = new_vol;

        if let Some(ent) = *entity {
            if let Ok(mut sink) = commands.get_entity(ent) {
                // In a full implementation we would get the AudioSink and call set_volume
                // For now we rely on the fact that the entity exists
            }
        }

        // Remove layers no longer needed
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

fn get_active_layers(state: MusicStateType, intensity: f32) -> Vec<(MusicLayer, f32)> {
    let mut layers = vec![];
    layers.push((MusicLayer::Base, 0.75 + intensity * 0.25));

    match state {
        MusicStateType::Exploration | MusicStateType::Harvesting | MusicStateType::Council => {
            if intensity > 0.3 {
                layers.push((MusicLayer::Melody, intensity * 0.9));
            }
        }
        MusicStateType::Tension => {
            layers.push((MusicLayer::Tension, 0.5 + intensity * 0.5));
        }
        MusicStateType::Combat => {
            layers.push((MusicLayer::Tension, 0.6 + intensity * 0.4));
            layers.push((MusicLayer::Percussion, intensity * 0.9));
        }
        MusicStateType::IntenseCombat | MusicStateType::Boss => {
            layers.push((MusicLayer::Tension, 0.85));
            layers.push((MusicLayer::Percussion, 0.75 + intensity * 0.25));
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

// Fallback single-track system (kept for compatibility during development)
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
