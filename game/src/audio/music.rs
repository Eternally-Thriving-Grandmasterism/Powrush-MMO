/*!
 * Dynamic Music System - Core
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
            transition_duration: 3.0,
        }
    }
}

/// Request a music state change (call this from combat, harvesting, etc.)
pub fn request_music_state(
    mut controller: ResMut<MusicController>,
    new_state: MusicStateType,
) {
    if controller.target_state != new_state {
        controller.target_state = new_state;
        controller.transition_timer = 0.0;
    }
}

/// Evaluates current music state (placeholder - expand with real gameplay data)
pub fn evaluate_music_state(
    mut controller: ResMut<MusicController>,
) {
    // TODO: Integrate with combat system, player health, harvesting, council, etc.
}

/// Updates music playback and handles transitions
pub fn update_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut controller: ResMut<MusicController>,
    mixer: Res<AudioMixer>,
    time: Res<Time>,
    mut current_music: Local<Option<Entity>>,
) {
    // Handle smooth state transition
    if controller.current_state != controller.target_state {
        controller.transition_timer += time.delta_seconds();

        if controller.transition_timer >= controller.transition_duration {
            controller.current_state = controller.target_state;
            controller.transition_timer = 0.0;
        }
    }

    let music_path = match controller.current_state {
        MusicStateType::Exploration   => "audio/music/exploration.ogg",
        MusicStateType::Tension       => "audio/music/tension.ogg",
        MusicStateType::Combat        => "audio/music/combat.ogg",
        MusicStateType::IntenseCombat => "audio/music/intense_combat.ogg",
        MusicStateType::Boss          => "audio/music/boss.ogg",
        MusicStateType::Harvesting    => "audio/music/harvesting.ogg",
        MusicStateType::Council       => "audio/music/council.ogg",
        MusicStateType::Victory       => "audio/music/victory.ogg",
        MusicStateType::Death         => "audio/music/death.ogg",
        MusicStateType::Menu          => "audio/music/menu.ogg",
    };

    // Spawn or replace current music track
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
