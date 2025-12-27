// ... previous imports + lobby.rs
use crate::lobby::LobbyPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AudioPlugin)
        .add_plugins(MercyNetPlugin)
        .add_plugins(LobbyPlugin)  // New
        .insert_resource(LatticeStats::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            mercy_flow_system,
            trust_multiplier_system,
            lattice_expansion_system,
            spawn_particles_system,
            particle_update_system,
            lobby_matchmake,
            lobby_ready_system,
            guild_alliance_system,
            lobby_ui_visuals,
        ))
        .run();
}
