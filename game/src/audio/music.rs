/*!
 * Dynamic Music System - Gameplay Integration Examples
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

/// Public API to request music state changes from gameplay systems
pub fn request_music_state(
    mut controller: ResMut<MusicController>,
    new_state: MusicStateType,
) {
    if controller.target_state != new_state {
        controller.target_state = new_state;
        controller.transition_timer = 0.0;
    }
}

// ==================== GAMEPLAY INTEGRATION EXAMPLES ====================

/// Call this when combat starts
pub fn on_combat_started(
    mut controller: ResMut<MusicController>,
    // combat_query: Query<...>,
) {
    // TODO: Check number of enemies / boss status to choose Combat vs IntenseCombat vs Boss
    request_music_state(&mut controller, MusicStateType::Combat);
}

/// Call this when combat ends (victory or retreat)
pub fn on_combat_ended(
    mut controller: ResMut<MusicController>,
    // victory: bool,
) {
    // For now we go to Victory briefly, then back to Exploration
    request_music_state(&mut controller, MusicStateType::Victory);

    // After a short delay, return to Exploration (can be improved with a timer)
    // For simplicity, we can have another system reset after Victory
}

/// Call this when player starts harvesting
pub fn on_harvesting_started(mut controller: ResMut<MusicController>) {
    request_music_state(&mut controller, MusicStateType::Harvesting);
}

/// Call this when harvesting ends
pub fn on_harvesting_ended(mut controller: ResMut<MusicController>) {
    request_music_state(&mut controller, MusicStateType::Exploration);
}

/// Call this when entering a council area or narrative moment
pub fn on_council_entered(mut controller: ResMut<MusicController>) {
    request_music_state(&mut controller, MusicStateType::Council);
}

/// Call this when leaving council / narrative
pub fn on_council_exited(mut controller: ResMut<MusicController>) {
    request_music_state(&mut controller, MusicStateType::Exploration);
}

/// Example: Auto-return from Victory state after a few seconds
pub fn handle_victory_timeout(
    mut controller: ResMut<MusicController>,
    time: Res<Time>,
    mut victory_timer: Local<f32>,
) {
    if controller.current_state == MusicStateType::Victory {
        *victory_timer += time.delta_seconds();

        if *victory_timer > 4.0 {
            request_music_state(&mut controller, MusicStateType::Exploration);
            *victory_timer = 0.0;
        }
    } else {
        *victory_timer = 0.0;
    }
}

// ==================== MUSIC PLAYBACK ====================

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

pub fn evaluate_music_state(
    mut controller: ResMut<MusicController>,
) {
    // TODO: Integrate real gameplay data here (combat, health, harvesting, council, etc.)
}
