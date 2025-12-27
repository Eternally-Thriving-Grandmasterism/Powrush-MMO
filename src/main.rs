use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;

// [All previous components/resources/systems unchanged...]

// Add to App::new()
.add_plugins(LobbyPlugin)

// In setup() — spawn lobby player
commands.spawn((
    LobbyPlayer {
        name: "Sherif".to_string(),
        trust: 1.0,
        ready: false,
    },
    // ... other components
));

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO — Mercy Lobby Ready".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AudioPlugin)
        .add_plugins(LobbyPlugin)  // New
        .insert_resource(LatticeStats::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            mercy_flow_system,
            trust_multiplier_system,
            lattice_expansion_system,
            spawn_particles_system,
            particle_update_system,
            lobby_matchmake,  // New
            lobby_ready_system,  // New
        ))
        .run();
}
