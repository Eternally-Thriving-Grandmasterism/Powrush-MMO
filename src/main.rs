use bevy::prelude::*;
use bevy::log::{info, warn, error};
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
struct ErrorVisualization {
    timer: Timer,
}

#[derive(Component)]
struct TooltipTrigger;

#[derive(Component)]
struct TooltipAnimation {
    timer: Timer,
    pulse_timer: Timer,
}

// === RESOURCES ===
#[derive(Resource, Default)]
struct LatticeStats {
    nodes: usize,
    connections: usize,
}

#[derive(Resource)]
struct MercySounds {
    warning_chime: Handle<AudioSource>,
    critical_hum: Handle<AudioSource>,
    mercy_wave: Handle<AudioSource>,
}

// === SYSTEMS ===
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Spawn player
    commands.spawn((
        MercyPoints(100.0),
        Need(50.0),
        TrustCredits(1.0),
        LatticeNode(0),
    ));

    // Spawn guild
    let guild_entity = commands.spawn((
        GuildId(1),
        GuildResources(0.0),
        GuildTrust(1.0),
    )).id();

    commands.insert_resource(LatticeStats::default());

    // Load sounds (fallback procedural if missing)
    commands.insert_resource(MercySounds {
        warning_chime: asset_server.load("sounds/warning_chime.ogg"),
        critical_hum: asset_server.load("sounds/critical_hum.ogg"),
        mercy_wave: asset_server.load("sounds/mercy_wave.ogg"),
    });
}

fn mercy_flow_system(
    mut query: Query<(&Need, &mut MercyPoints)>,
    time: Res<Time>,
) {
    for (need, mut mp) in &mut query {
        let alloc = (need.0 * time.delta_seconds()).min(mp.0);
        mp.0 -= alloc;
        info!("Mercy Flow: Allocated {:.2}", alloc);
    }
}

fn trust_multiplier_system(mut query: Query<&mut TrustCredits>) {
    for mut trust in &mut query {
        trust.0 *= 1.1;
    }
}

fn guild_contribute_system(
    mut guild_query: Query<(&GuildId, &mut GuildResources)>,
    player_query: Query<(&MemberOf, &MercyPoints)>,
    time: Res<Time>,
) {
    for (member_of, mp) in player_query.iter() {
        if let Ok((_, mut resources)) = guild_query.get_mut(member_of.0.0 as Entity) {
            let contrib = mp.0 * 0.1 * time.delta_seconds();
            resources.0 += contrib;
            info!("Guild contrib +{:.2}", contrib);
        }
    }
}

fn lattice_expansion_system(
    mut stats: ResMut<LatticeStats>,
    new_nodes: Query<&LatticeNode, Added<LatticeNode>>,
    new_connections: Query<&LatticeConnection, Added<LatticeConnection>>,
) {
    let nodes = new_nodes.iter().count();
    let conns = new_connections.iter().count();
    if nodes > 0 || conns > 0 {
        stats.nodes += nodes;
        stats.connections += conns;
        info!("Lattice +{} nodes, +{} connections (total {}/{})", nodes, conns, stats.nodes, stats.connections);
    }
}

fn lattice_error_system(
    mut commands: Commands,
    query: Query<&LatticeNode>,
) {
    if query.iter().count() == 0 {
        error!("Lattice collapsed — zero nodes");
        commands.spawn(LatticeError {
            message: "Zero nodes".to_string(),
            severity: ErrorSeverity::Critical,
            attempts: 0,
        });
    }
}

fn lattice_recovery_system(
    mut commands: Commands,
    mut errors: Query<(Entity, &mut LatticeError)>,
    mut stats: ResMut<LatticeStats>,
) {
    for (entity, mut error) in errors.iter_mut() {
        error.attempts += 1;
        match error.severity {
            ErrorSeverity::Critical if error.message.contains("Zero") => {
                commands.spawn(LatticeNode(stats.nodes as u64 + 1));
                stats.nodes += 1;
                info!("Mercy recovery: Emergency node spawned");
            }
            _ => {}
        }
        if error.attempts >= 3 {
            commands.entity(entity).despawn();
            info!("Error forgiven");
        }
    }
}

fn spawn_error_viz_system(
    mut commands: Commands,
    errors: Query<&LatticeError, Added<LatticeError>>,
) {
    for error in &errors {
        commands.spawn((
            ErrorVisualization { timer: Timer::from_seconds(5.0, TimerMode::Once) },
            TextBundle::from_section(
                &error.message,
                TextStyle { font_size: 40.0, color: Color::RED, ..default() },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                left: Val::Px(100.0),
                ..default()
            }),
        ));
    }
}

fn update_error_viz_system(
    mut commands: Commands,
    time: Res<Time>,
    mut viz: Query<(Entity, &mut ErrorVisualization, &mut Text)>,
) {
    for (entity, mut viz, mut text) in viz.iter_mut() {
        viz.timer.tick(time.delta());
        let alpha = viz.timer.percent_left();
        text.sections[0].style.color = text.sections[0].style.color.with_a(alpha);
        if viz.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn play_sound_system(
    mut commands: Commands,
    sounds: Res<MercySounds>,
    errors: Query<&LatticeError, Added<LatticeError>>,
) {
    for error in &errors {
        let sound = match error.severity {
            ErrorSeverity::Warning => sounds.warning_chime.clone(),
            ErrorSeverity::Critical => sounds.critical_hum.clone(),
        };
        commands.spawn(AudioBundle {
            source: sound,
            settings: PlaybackSettings::ONCE,
        });
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
            guild_contribute_system,
            lattice_expansion_system,
            lattice_error_system,
            lattice_recovery_system,
            spawn_error_viz_system,
            update_error_viz_system,
            play_sound_system,
        ))
        .run();
}
