use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct VoicePlayer {
    pub position: Vec3,
    pub trust: f32,
    pub speaking: bool,
}

pub struct VoicePlugin;

impl Plugin for VoicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, voice_modulation_system);
    }
}

fn voice_modulation_system(
    mut audio: ResMut<Audio>,
    players: Query<(&VoicePlayer, &TrustCredits)>,
) {
    for (voice, trust) in &players {
        if voice.speaking {
            let pitch = 0.8 + (trust.0 / 100.0);  // Trust = pitch/clarity
            let volume = trust.0 / 100.0;
            audio.play(/* mic */).with_pitch(pitch).with_volume(volume);
        }
    }
}
