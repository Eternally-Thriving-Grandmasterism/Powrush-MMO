use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use crate::divine_whispers::DivineWhispersPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AudioPlugin)           // Bevy Kira Audio
        .add_plugins(DivineWhispersPlugin)
        // Add other plugins here
        .run();
}
