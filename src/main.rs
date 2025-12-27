// [Previous imports...]
use crate::chat::{ChatPlugin, SendChatEvent, VoiceChatEvent};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AudioPlugin)
        .add_plugins(ChatPlugin)  // Enhanced with voice/emote
        .add_event::<SendChatEvent>()
        .add_event::<VoiceChatEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            mercy_flow_system,
            trust_multiplier_system,
            lattice_expansion_system,
            spawn_particles_system,
            particle_update_system,
            chat_input_system,
            emote_system,
            voice_chat_system,
        ))
        .run();
}
