use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;
use crate::emote::EmotePlugin;
use crate::chat::ChatPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO — Chat & Emotes Thriving".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AudioPlugin)
        .add_plugins(VoicePlugin)
        .add_plugins(EmotePlugin)
        .add_plugins(ChatPlugin)  // New
        .insert_resource(LatticeStats::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            mercy_flow_system,
            trust_multiplier_system,
            lattice_expansion_system,
            spawn_particles_system,
            particle_update_system,
            emote_input_system,
            emote_visual_system,
            emote_audio_system,
            chat_input_system,
            chat_render_system,
        ))
        .run();
}
