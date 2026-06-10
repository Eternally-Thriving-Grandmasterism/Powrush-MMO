use bevy::prelude::*;
use server::hardening::apply_server_hardening;

fn main() {
    apply_server_hardening();

    App::new()
        .add_plugins(DefaultPlugins)
        // Add your server systems here
        .run();
}
