use bevy::prelude::*;
use server::hardening::apply_server_hardening;

fn main() {
    // Apply server hardening as early as possible
    apply_server_hardening();

    App::new()
        .add_plugins(DefaultPlugins)
        // ... rest of your server setup
        .run();
}
