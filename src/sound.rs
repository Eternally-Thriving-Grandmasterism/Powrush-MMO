use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, procedural_sound_system);
    }
}

fn procedural_sound_system(
    mut audio: ResMut<Audio>,
    trust: Query<&TrustCredits>,
) {
    let mut rng = rand::thread_rng();
    for t in &trust {
        if rng.gen_bool(0.001) {
            let pitch = 0.8 + (t.0 / 100.0).min(1.0);
            audio.play(/* procedural chime */).with_pitch(pitch);
        }
    }
}
