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
        app.add_systems(Update, (voice_modulation_system, proximity_voice_system));
    }
}

// Trust-based pitch + volume
fn voice_modulation_system(
    mut audio: ResMut<Audio>,
    players: Query<(&VoicePlayer, &TrustCredits)>,
) {
    for (voice, trust) in &players {
        if voice.speaking {
            let pitch = 0.8 + (trust.0 / 100.0);  // High trust = clearer/higher
            let volume = trust.0 / 100.0;
            // Placeholder — real mic input via cpal in production
            audio.play(/* procedural mercy tone */).with_pitch(pitch).with_volume(volume);
        }
    }
}

// Proximity — louder when close
fn proximity_voice_system(
    players: Query<(&VoicePlayer, &Transform)>,
) {
    let mut iter = players.iter_combinations();
    while let Some([(voice1, trans1), (voice2, trans2)]) = iter.fetch_next() {
        let dist = trans1.translation.distance(trans2.translation);
        let volume = (30.0 - dist.clamp(0.0, 30.0)) / 30.0;  // 30m range
        if voice1.speaking {
            // Adjust playback volume for voice2
            // (real: spatial sink volume)
        }
    }
}
