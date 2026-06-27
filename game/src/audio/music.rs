/*!
 * Dynamic Music System - Stinger Ducking
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
    pub ducking: f32,           // 0.0 = no duck, 1.0 = full duck
    pub duck_timer: f32,
}

impl Default for MusicController {
    fn default() -> Self {
        Self {
            current_state: MusicStateType::Exploration,
            target_state: MusicStateType::Exploration,
            intensity: 0.5,
            transition_timer: 0.0,
            transition_duration: 4.0,
            ducking: 0.0,
            duck_timer: 0.0,
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

#[derive(Resource, Default)]
pub struct MusicLayers {
    pub layers: Vec<(MusicLayer, Option<Entity>, f32, f32)>,
}

/// Play a stinger and trigger music ducking
pub fn play_music_stinger(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mixer: Res<AudioMixer>,
    mut controller: ResMut<MusicController>,
    stinger_path: &str,
) {
    let stinger = asset_server.load(stinger_path);
    commands.spawn((
        AudioBundle {
            source: stinger,
            settings: PlaybackSettings::ONCE.with_volume(mixer.music * 1.15),
        },
        DynamicAudio {
            category: AudioCategory::Music,
            priority: Priority::Critical,
        },
    ));

    // Trigger ducking for background music layers
    controller.ducking = 0.65; // Duck to ~35% volume
    controller.duck_timer = 2.8; // Duck duration in seconds
}

pub fn update_music_layers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut controller: ResMut<MusicController>,
    mut music_layers: ResMut<MusicLayers>,
    mixer: Res<AudioMixer>,
    time: Res<Time>,
) {
    // Handle ducking timer
    if controller.duck_timer > 0.0 {
        controller.duck_timer -= time.delta_seconds();
        if controller.duck_timer <= 0.0 {
            controller.ducking = 0.0; // Release duck
        }
    }

    // Smooth duck release
    let duck_multiplier = if controller.ducking > 0.0 {
        1.0 - controller.ducking
    } else {
        1.0
    };

    if controller.current_state != controller.target_state {
        controller.transition_timer += time.delta_seconds();
        if controller.transition_timer >= controller.transition_duration {
            controller.current_state = controller.target_state;
            controller.transition_timer = 0.0;
        }
    }

    let target_layers = get_active_layers(controller.current_state, controller.intensity);

    for (layer, target_vol) in target_layers {
        if let Some((_, entity, _, target)) = music_layers.layers.iter_mut().find(|(l, _, _, _)| *l == layer) {
            *target = target_vol;
        } else {
            let path = get_layer_path(layer, controller.current_state);
            let music = asset_server.load(path);
            let entity = commands.spawn((
                AudioBundle {
                    source: music,
                    settings: PlaybackSettings::LOOP.with_volume(0.0),
                },
                DynamicAudio {
                    category: AudioCategory::Music,
                    priority: Priority::High,
                },
            )).id();
            music_layers.layers.push((layer, Some(entity), 0.0, target_vol));
        }
    }

    let lerp_speed = 3.0;
    let mut to_remove = vec![];

    for (i, (layer, entity, current_vol, target_vol)) in music_layers.layers.iter_mut().enumerate() {
        let effective_target = *target_vol * duck_multiplier;
        let new_vol = *current_vol + (effective_target - *current_vol) * lerp_speed * time.delta_seconds();
        *current_vol = new_vol;

        if let Some(ent) = *entity {
            if let Ok(mut sink) = commands.get_entity(ent) {
                // TODO: sink.set_volume(new_vol * mixer.music);
            }
        }

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
    layers.push((MusicLayer::Base, 0., intensity * 0.25));

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
