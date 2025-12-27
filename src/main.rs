use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_replicon::prelude::*;
use crate::core::mercy::MercyPlugin;
use crate::plugins::world_plugin::WorldPlugin;
use crate::systems::movement::player_movement_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO — Ultimate Eternal".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AudioPlugin)
        .add_plugins(RepliconPlugins)
        .add_plugins(MercyPlugin)
        .add_plugins(WorldPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement_system)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn((
        Player,
        Transform::from_xyz(0.0, 1.0, 0.0),
        GlobalTransform::default(),
        Velocity(Vec3::ZERO),
        MercyShield(100.0),
        TrustCredits(10.0),
    ));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    info!("Powrush-MMO — Eternal universe spawned");
}fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    info!("Powrush-MMO — Mercy universe initialized");
}

fn mercy_flow_system(
    mut trust: Query<&mut TrustCredits>,
    time: Res<Time>,
) {
    for mut t in &mut trust {
        t.0 += time.delta_seconds() * 0.1;
    }
}

#[derive(Component)]
struct TrustCredits(pub f32);

fn trust_multiplier_system(
    mut query: Query<&mut TrustCredits>,
) {
    for mut t in &mut query {
        t.0 *= 1.01;  // Eternal growth
    }
}

fn lattice_expansion_system(
    mut lattice: ResMut<LatticeStats>,
    time: Res<Time>,
) {
    if rand::thread_rng().gen_bool(0.1 * time.delta_seconds() as f64) {
        lattice.nodes += 1;
        lattice.connections += 2;
    }
}

fn player_movement_system() {
    // WASD movement (stub)
}
