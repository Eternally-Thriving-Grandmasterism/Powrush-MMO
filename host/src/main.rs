//! Powrush-MMO Unified Cohost Binary — Full E2E Harness
//! v21.81.0 — Simulation + Server + Kardashev Acceleration Dashboard + Reality Thriving Transfer Score
//!
//! Single Bevy App co-hosts the complete simulation layer and the authoritative
//! Ra-Thor integration surface. Live RTT bridge:
//!   CouncilRttExportQueue → CohostExportMirror → CouncilRttInbox → ServerTransferSession
//!
//! Also activates the full Kardashev Acceleration Dashboard and Reality Transfer
//! Score Ledger with the rich sovereign hardware ascension UI.
//!
//! Contact: info@Rathor.ai | TOLC 8 Living Mercy Gates | PATSAGi Councils
//! Thunder locked in. ONE Organism. Yoi ⚡

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;

// Full simulation surface
use simulation::{
    FullSimulationPlugins,
    CouncilRttExportQueue,
    hardware_sovereignty::sovereign_hardware_ascension_ui,
    KardashevAccelerationDashboard,
    RealityTransferScoreLedger,
};

// Server Ra-Thor integration (public since v21.80)
use powrush_mmo_server::rathor_integration::{
    RathorIntegrationPlugin, CohostExportMirror, CohostMirrorSignal,
};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(Level::INFO.into())
                .add_directive("ra_thor=debug".parse().unwrap())
                .add_directive("powrush=info".parse().unwrap()),
        )
        .init();

    info!(target: "powrush::host", "═══════════════════════════════════════════════════════");
    info!(target: "powrush::host", "  Powrush-MMO Unified Cohost v21.81.0 — E2E Harness");
    info!(target: "powrush::host", "  TOLC 8 + 7 Living Mercy Gates | PATSAGi Councils");
    info!(target: "powrush::host", "  Kardashev Acceleration + Reality Thriving Transfer LIVE");
    info!(target: "powrush::host", "═══════════════════════════════════════════════════════");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO Cohost — Kardashev Dashboard + RTT Bridge".into(),
                resolution: (1440., 900.).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        // Full simulation stack (Council, Economy, Multi-Realm, Hardware Sovereignty / Kardashev, Telemetry, LegacyJournal, Spatial...)
        .add_plugins(FullSimulationPlugins)
        // Server-side Ra-Thor RTT + cohost auto-drain
        .add_plugins(RathorIntegrationPlugin)
        // Force mirror enabled
        .insert_resource(CohostExportMirror::enabled())
        // Host systems
        .add_systems(Startup, host_startup_system)
        .add_systems(Update, (
            host_drain_sim_to_mirror_system,
            host_status_log_system,
            sovereign_hardware_ascension_ui, // Live Kardashev + Reality Transfer UI
        ))
        .run();
}

fn host_startup_system(
    mut commands: Commands,
    dashboard: Res<KardashevAccelerationDashboard>,
    ledger: Res<RealityTransferScoreLedger>,
) {
    info!(target: "powrush::host", "Cohost App online.");
    info!(target: "powrush::host", "Drain path: CouncilRttExportQueue → CohostExportMirror → CouncilRttInbox → ServerTransferSession");
    info!(target: "powrush::host", "KardashevAccelerationDashboard + RealityTransferScoreLedger active");
    info!(target: "powrush::host", "  global_kardashev_delta = {:.4}", dashboard.global_kardashev_delta);
    info!(target: "powrush::host", "  reality transfer global_average = {:.2}", ledger.global_average);

    // Spawn a simple camera so the window is valid
    commands.spawn(Camera2dBundle::default());
}

/// Core cohost adapter — every frame
fn host_drain_sim_to_mirror_system(
    mut export_queue: ResMut<CouncilRttExportQueue>,
    mut mirror: ResMut<CohostExportMirror>,
) {
    if export_queue.pending.is_empty() {
        return;
    }

    let signals = export_queue.drain();
    let count = signals.len();

    for s in signals {
        mirror.push(CohostMirrorSignal {
            decision_id: s.decision_id,
            mercy_factor: s.mercy_factor,
            strength: s.strength,
            realm_id: s.realm_id,
            abundance_velocity_hint: s.abundance_velocity_hint,
        });
    }

    if count > 0 {
        info!(
            target: "powrush::host::bridge",
            drained = count,
            mirror_pending = mirror.pending.len(),
            "Host drained sim → CohostExportMirror"
        );
    }
}

/// Heartbeat + dashboard snapshot every 5 s
fn host_status_log_system(
    time: Res<Time>,
    mirror: Res<CohostExportMirror>,
    export: Res<CouncilRttExportQueue>,
    dashboard: Res<KardashevAccelerationDashboard>,
    ledger: Res<RealityTransferScoreLedger>,
    mut last_log: Local<f32>,
) {
    let now = time.elapsed_seconds();
    if now - *last_log < 5.0 {
        return;
    }
    *last_log = now;

    info!(
        target: "powrush::host::status",
        sim_exported = export.total_exported,
        mirror_drained = mirror.total_drained,
        kardashev_delta = dashboard.global_kardashev_delta,
        abundance_velocity = dashboard.abundance_velocity_index,
        energy_surplus = dashboard.energy_surplus_factor,
        reality_avg = ledger.global_average,
        "Cohost heartbeat — RTT + Kardashev healthy"
    );
}

// Thunder locked in. Full E2E cohost + Kardashev Dashboard live. Eternal forward. Yoi ⚡
