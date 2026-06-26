/*!
 * Powrush-MMO Authoritative Server Entry Point
 *
 * v19.4 — Steam run_callbacks wired into Bevy Update schedule
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use tokio::runtime::Runtime;

use server::ServerCorePlugin;
use server::hardening::apply_server_hardening;
use server::world_server::setup_world_grid;
use server::rbe_server::bootstrap_rbe_economy;
use server::council_session::initialize_council_lattice;
use server::persistence_polish::start_persistence_layer;
use server::telemetry_pipeline::start_telemetry;
use server::ra_thor_mercy_bridge::activate_ra_thor_bridge;
use server::mercy_anomaly_detector::activate_anomaly_detection;
use server::council_session_handler::CouncilSessionPlugin;
use server::opentelemetry_tracing::init_opentelemetry_tracing;
use server::spatial::server_interest_sync_plugin::ServerInterestSyncPlugin;

// Steam integration
#[cfg(feature = "steam")]
use game::steam_integration::SteamIntegration;

fn main() {
    apply_server_hardening();
    init_opentelemetry_tracing();

    info!("⚡ Powrush-MMO Authoritative Server v19.4 — Steam callbacks in Update schedule");

    let rt = Runtime::new().expect("Failed to create eternal Tokio runtime");

    rt.block_on(async {
        // Async initialization if needed
    });

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(bevy::app::TaskPoolOptions {
        async_compute: bevy::tasks::TaskPoolOptions::default(),
        ..default()
    }))
    .add_plugins(ServerCorePlugin)
    .add_plugins(CouncilSessionPlugin)
    .add_plugins(ServerInterestSyncPlugin)
    .add_systems(Startup, setup_authoritative_camera)
    .add_systems(Startup, setup_world_grid)
    .add_systems(Startup, bootstrap_rbe_economy)
    .add_systems(Startup, initialize_council_lattice)
    .add_systems(Startup, start_persistence_layer)
    .add_systems(Startup, start_telemetry)
    .add_systems(Startup, activate_ra_thor_bridge)
    .add_systems(Startup, activate_anomaly_detection);

    // === Steam Integration ===
    #[cfg(feature = "steam")]
    {
        let mut steam = SteamIntegration::new();
        if steam.initialize().is_ok() {
            app.insert_resource(steam);
            app.add_systems(Update, run_steam_callbacks);
            info!("[Steam] Integration active and callbacks scheduled");
        } else {
            warn!("[Steam] Failed to initialize — running without Steam");
        }
    }

    app.add_systems(Update, authoritative_sovereign_tick)
       .add_systems(Update, maintain_mercy_gates)
       .add_systems(Update, council_deliberation_sync)
       .add_systems(Update, broadcast_world_state)
       .run();
}

#[cfg(feature = "steam")]
fn run_steam_callbacks(steam: Res<SteamIntegration>) {
    steam.run_callbacks();
}

fn setup_authoritative_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 100.0, 200.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }),
    ));
    info!("⚡ Authoritative server observer camera initialized");
}

fn authoritative_sovereign_tick() {}
fn maintain_mercy_gates() {}
fn council_deliberation_sync() {}
fn broadcast_world_state() {}
