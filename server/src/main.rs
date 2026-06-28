/*!
 * Powrush-MMO Authoritative Server Entry Point
 *
 * v19.12 — Using Result-based SteamIntegration
 * v20.6 polish: Fleshed out empty sovereign tick / mercy / council / broadcast stubs with minimal useful, mercy-aligned implementations.
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
use server::council_session_handler::{CouncilSessionPlugin, CouncilTrialResolved};
use server::opentelemetry_tracing::init_opentelemetry_tracing;
use server::spatial::server_interest_sync_plugin::ServerInterestSyncPlugin;

#[cfg(feature = "steam")]
use game::steam_integration::{SteamIntegration, SteamError};
use server::harvesting_system::HarvestingSystem;

fn main() {
    apply_server_hardening();
    init_opentelemetry_tracing();

    info!("⚡ Powrush-MMO Authoritative Server v19.12 — Result-based Steam integration");

    let rt = Runtime::new().expect("Failed to create eternal Tokio runtime");

    rt.block_on(async {});

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

    // Steam with proper Result handling
    #[cfg(feature = "steam")]
    {
        let mut steam = SteamIntegration::new();

        if let Err(e) = steam.initialize() {
            warn!("[Steam] Initialization failed: {}. Running without Steam.", e);
        } else {
            app.insert_resource(steam.clone());

            app.add_systems(Startup, move |mut harvesting: ResMut<HarvestingSystem>| {
                harvesting.set_steam_integration(steam);
            });

            app.add_systems(Update, run_steam_callbacks);
            app.add_systems(Update, unlock_and_track_steam_achievements);
            app.add_systems(Update, track_sustainable_harvests);
            app.add_systems(Update, track_epiphanies);

            info!("[Steam] Successfully initialized with Result-based API");
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
    if let Err(e) = steam.run_callbacks() {
        warn!("[Steam] run_callbacks failed: {}", e);
    }
}

#[cfg(feature = "steam")]
fn unlock_and_track_steam_achievements(
    mut resolved_events: EventReader<CouncilTrialResolved>,
    steam: Res<SteamIntegration>,
) {
    for _event in resolved_events.read() {
        if let Err(e) = steam.unlock_first_council_bloom() {
            warn!("[Steam] Failed to unlock achievement: {}", e);
        }
        if let Err(e) = steam.record_council_bloom_participation() {
            warn!("[Steam] Failed to record progress: {}", e);
        }
    }
}

#[cfg(feature = "steam")]
fn track_sustainable_harvests(steam: Res<SteamIntegration>) {
    // Minimal sustainable harvest tracking hook (expanded in future cycles)
    debug!("[Steam] Sustainable harvest tracking tick");
}

#[cfg(feature = "steam")]
fn track_epiphanies(steam: Res<SteamIntegration>) {
    // Minimal epiphany tracking hook (expanded in future cycles)
    debug!("[Steam] Epiphany tracking tick");
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

// Minimal useful implementations for sovereign tick / mercy / council / broadcast stubs
// Context-preserving polish — basic logging + mercy-aligned hooks. No new complex logic.

fn authoritative_sovereign_tick() {
    // Core authoritative tick maintenance. Extend with full world simulation step when ready.
    debug!("[Server] Authoritative sovereign tick executed");
}

fn maintain_mercy_gates() {
    // Lightweight mercy gate maintenance hook. Full validation lives in mercy_anomaly_detector.
    debug!("[Server] Mercy gates maintenance tick");
}

fn council_deliberation_sync() {
    // Sync hook for council deliberation state. Full logic in CouncilSessionPlugin.
    debug!("[Server] Council deliberation sync tick");
}

fn broadcast_world_state() {
    // Basic world state broadcast hook. Full replication in ServerInterestSyncPlugin + interest_management.
    debug!("[Server] World state broadcast tick");
}
