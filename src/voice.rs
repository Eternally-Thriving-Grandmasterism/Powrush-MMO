use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_replicon::prelude::*;
use bevy_spatial::*;

#[derive(Component, Replicated)]
pub struct VoicePlayer {
    pub position: Vec3,
    pub trust: f32,
    pub speaking: bool,
}

pub struct VoicePlugin;

impl Plugin for VoicePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpatialAudioPlugin)
           .add_systems(Update, (
                voice_modulation_system,
                proximity_voice_system,
           ));
    }
}

// Advanced: trust = pitch + clarity
fn voice_modulation_system(
    mut audio: ResMut<Audio<Spatial>>,
    players: Query<(&VoicePlayer, &TrustCredits, &GlobalTransform)>,
    listener: Query<&GlobalTransform, With<AudioListener>>,
) {
    let listener_pos = listener.single().translation();
    for (voice, trust, trans) in &players {
        if voice.speaking {
            let pitch = 0.8 + (trust.0 / 100.0);
            let volume = trust.0 / 100.0;
            let dist = listener_pos.distance(trans.translation());
            let falloff = 1.0 / (1.0 + dist * 0.05);
            audio.play(/* mic */)
                .with_pitch(pitch)
                .with_volume(volume * falloff)
                .with_position(trans.translation());
        }
    }
}

// Proximity: natural falloff + direction
fn proximity_voice_system(
    mut sinks: Query<&mut SpatialAudioSink>,
    listener: Query<&GlobalTransform, With<AudioListener>>,
) {
    let listener_trans = listener.single();
    for mut sink in &mut sinks {
        sink.set_listener_transform(listener_trans);
    }
}
