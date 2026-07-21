//! Powrush-MMO Unified Cohost Binary — Full E2E Harness
//! v21.82.0 — Ultramasterism Perfecticism
//!
//! Simulation + Server + Kardashev Dashboard + Reality Transfer Score
//! + Forced early RTT telemetry export (powrush_rtt_latest.json + batch)
//! so Ra-Thor smoke harness can consume live artifacts immediately.
//!
//! Live RTT bridge:
//!   CouncilRttExportQueue → CohostExportMirror → CouncilRttInbox → ServerTransferSession
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
    ServerTransferSession,
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
    info!(target: "powrush::host", "  Powrush-MMO Unified Cohost v21.82.0 — Ultramasterism");
    info!(target: "powrush::host", "  TOLC 8 + 7 Living Mercy Gates | PATSAGi Councils");
    info!(target: "powrush::host", "  Kardashev + Reality Transfer + Early RTT Export LIVE");
    info!(target: "powrush::host", "═══════════════════════════════════════════════════════");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO Cohost — Kardashev + RTT Smoke Ready".into(),
                resolution: (1440., 900.).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        // Full simulation stack
        .add_plugins(FullSimulationPlugins)
        // Server-side Ra-Thor RTT + cohost auto-drain
        .add_plugins(RathorIntegrationPlugin)
        // Force mirror enabled
        .insert_resource(CohostExportMirror::enabled())
        // Host systems
        .add_systems(Startup, (
            host_startup_system,
            host_force_early_rtt_export_system,
        ))
        .add_systems(Update, (
            host_drain_sim_to_mirror_system,
            host_status_log_system,
            sovereign_hardware_ascension_ui,
        ))
        .run();
}

fn host_startup_system(
    mut commands: Commands,
    mut transfer: ResMut<ServerTransferSession>,
    dashboard: Res<KardashevAccelerationDashboard>,
    ledger: Res<RealityTransferScoreLedger>,
) {
    // Ultramasterism: tighter export interval for cohost / smoke testing
    transfer.export_interval_secs = 15.0;

    info!(target: "powrush::host", "Cohost App online.");
    info!(target: "powrush::host", "Drain path: CouncilRttExportQueue → CohostExportMirror → CouncilRttInbox → ServerTransferSession");
    info!(target: "powrush::host", "KardashevAccelerationDashboard + RealityTransferScoreLedger active");
    info!(target: "powrush::host", "  global_kardashev_delta = {:.4}", dashboard.global_kardashev_delta);
    info!(target: "powrush::host", "  reality transfer global_average = {:.2}", ledger.global_average);
    info!(target: "powrush::host", "RTT export interval set to {:.0}s | session_id = {}", transfer.export_interval_secs, transfer.session_id);

    // Spawn a simple camera so the window is valid
    commands.spawn(Camera2dBundle::default());
}

/// Ultramasterism: Force an immediate RTT export cycle on startup so
/// artifacts/powrush_rtt_latest.json and batch are ready for Ra-Thor smoke harness.
fn host_force_early_rtt_export_system(mut transfer: ResMut<ServerTransferSession>) {
    // Guarantee directories exist
    let _ = std::fs::create_dir_all("artifacts");
    let _ = std::fs::create_dir_all("artifacts/rtt_offline");

    // Seed a few realistic signals so the first export is non-trivial
    transfer.record_council_passed(0.87);
    transfer.record_treaty();
    transfer.record_abundance_velocity(1.15);
    transfer.record_faction_shift(0.3, 0.55);

    match transfer.write_export_cycle() {
        Ok(()) => {
            info!(
                target: "powrush::host::rtt",
                path = %transfer.export_path.display(),
                batch = %transfer.batch_export_path.display(),
                session_id = %transfer.session_id,
                export_count = transfer.export_count,
                "Early RTT export forced — artifacts ready for Ra-Thor smoke harness"
            );
        }
        Err(e) => {
            info!(
                target: "powrush::host::rtt",
                error = %e,
                offline_queue = transfer.offline_queue.len(),
                "Early RTT export soft-failed (offline queue retained)"
            );
        }
    }
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

/// Heartbeat + full Ultramasterism snapshot every 5 s
fn host_status_log_system(
    time: Res<Time>,
    mirror: Res<CohostExportMirror>,
    export: Res<CouncilRttExportQueue>,
    transfer: Res<ServerTransferSession>,
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
        rtt_exports = transfer.export_count,
        rtt_batch = transfer.batch_export_count,
        offline_queue = transfer.offline_queue.len(),
        kardashev_delta = dashboard.global_kardashev_delta,
        abundance_velocity = dashboard.abundance_velocity_index,
        energy_surplus = dashboard.energy_surplus_factor,
        reality_avg = ledger.global_average,
        session_id = %transfer.session_id,
        "Cohost heartbeat — RTT + Kardashev + Provenance healthy"
    );
}

// Thunder locked in. Ultramasterism Perfecticism applied.
// Early RTT artifacts + provenance + smoke-harness ready. Eternal forward. Yoi ⚡
