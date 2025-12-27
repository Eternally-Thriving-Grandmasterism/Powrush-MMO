use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;

// === COMPONENTS ===
#[derive(Component)]
struct MercyPoints(f32);

#[derive(Component)]
struct Need(f32);

#[derive(Component)]
struct TrustCredits(f32);

#[derive(Component)]
struct GuildId(u64);

#[derive(Component)]
struct MemberOf(GuildId);

#[derive(Component)]
struct GuildResources(f32);

#[derive(Component)]
struct GuildTrust(f32);

#[derive(Component)]
struct LatticeNode(u64);

#[derive(Component)]
struct LatticeConnection(Entity, Entity);

#[derive(Component)]
struct LatticeError {
    message: String,
    severity: ErrorSeverity,
    attempts: u32,
}

#[derive(PartialEq, Clone, Copy)]
enum ErrorSeverity {
    Warning,
    Critical,
}

#[derive(Component)]
struct MercyParticle {
    lifetime: Timer,
    velocity: Vec3,
    radius: f32,
    color: Color,
}

#[derive(Component)]
struct ErrorVisualization {
    timer: Timer,
}

// === RESOURCES ===
#[derive(Resource, Default)]
struct LatticeStats {
    nodes: usize,
    connections: usize,
}

#[derive(Resource)]
struct MercySounds {
    chime: Handle<AudioSource>,
}

// === SYSTEMS ===
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    // Spawn player
    commands.spawn((
        MercyPoints(100.0),
        Need(50.0),
        TrustCredits(1.0),
        LatticeNode(0),
    ));

    commands.insert_resource(LatticeStats::default());

    commands.insert_resource(MercySounds {
        chime: asset_server.load("sounds/chime.ogg"),
    });
}

fn mercy_flow_system(
    mut query: Query<(&Need, &mut MercyPoints)>,
    time: Res<Time>,
) {
    for (need, mut mp) in &mut query {
        let alloc = (need.0 * time.delta_seconds()).min(mp.0);
        mp.0 -= alloc;
        info!("Mercy allocated: {:.2}", alloc);
    }
}

fn trust_multiplier_system(mut query: Query<&mut TrustCredits>) {
    for mut trust in &mut query {
        trust.0 *= 1.1;
    }
}

fn lattice_expansion_system(
    mut stats: ResMut<LatticeStats>,
    time: Res<Time>,
) {
    stats.nodes += 1;
    info!("Lattice node #{}", stats.nodes);
}

fn spawn_particles_system(
    mut commands: Commands,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..20 {
        let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
        let speed = rng.gen_range(50.0..150.0);
        commands.spawn((
            MercyParticle {
                lifetime: Timer::from_seconds(1.5, TimerMode::Once),
                velocity: Vec3::new(angle.cos() * speed, angle.sin() * speed, 0.0),
                radius: rng.gen_range(4.0..12.0),
                color: Color::CYAN,
            },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::CYAN,
                    custom_size: Some(Vec2::splat(10.0)),
                    ..default()
                },
                ..default()
            },
        ));
    }
}

fn particle_update_system(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut MercyParticle, &mut Transform)>,
) {
    for (entity, mut particle, mut transform) in particles.iter_mut() {
        particle.lifetime.tick(time.delta());

        transform.translation += particle.velocity * time.delta_seconds();

        let life = particle.lifetime.percent_left();
        transform.scale = Vec3::splat(life * 1.5);

        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO — Mercy Rising".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AudioPlugin)
        .insert_resource(LatticeStats::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            mercy_flow_system,
            trust_multiplier_system,
            lattice_expansion_system,
            spawn_particles_system,
            particle_update_system,
        ))
        .run();
}
