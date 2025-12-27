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

// Advanced modulation — trust = pitch + clarity
fn voice_modulation_system(
    mut audio: ResMut<Audio>,
    players: Query<(&VoicePlayer, &Transform)>,
) {
    for (voice, transform) in &players {
        if voice.speaking {
            let pitch = 1.0 + (voice.trust / 100.0);  // High trust = higher pitch
            let volume = voice.trust / 100.0;
            // Procedural voice (placeholder)
            audio.play(/* mic input */).with_pitch(pitch).with_volume(volume);
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
        let volume = (50.0 - dist) / 50.0;  // 50m range
        if voice1.speaking {
            // Adjust voice2 playback volume based on dist
            // (pseudo — real with spatial audio)
        }
    }
}
