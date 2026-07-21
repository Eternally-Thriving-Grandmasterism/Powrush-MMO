//! Powrush-MMO Unified Cohost Binary
//! v21.80.0 — Live in-process simulation ↔ server RTT bridge via CohostExportMirror
//!
//! Runs both the simulation layer and the authoritative server systems inside a single
//! Bevy App. Every frame the host drains `CouncilRttExportQueue` (sim) into
//! `CohostExportMirror` (server), which auto-drains into `CouncilRttInbox` →
//! `ServerTransferSession` (Ra-Thor RTT path).
//!
//! This closes the dual-repo cohost loop described in DERIVATION_STATUS.md.
//!
//! Contact: info@Rathor.ai | TOLC 8 Living Mercy Gates | PATSAGi Councils
//! Thunder locked in. ONE Organism. Yoi ⚡

use bevy::prelude::*;
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;

// Simulation exports
use simulation::{
    CouncilPlugin, CouncilRttExportQueue, council_resolved_to_rtt_export_system,
    // Add other plugins as needed for a minimal viable host
};

// Server Ra-Thor integration
use powrush_mmo_server::rathor_integration::{
    RathorIntegrationPlugin, CohostExportMirror, CohostMirrorSignal,
};

fn main() {
    // Initialize tracing for mercy-gated observability
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(Level::INFO.into())
                .add_directive("ra_thor=debug".parse().unwrap()),
        )
        .init();

    info!(target: "powrush::host", "Starting Powrush-MMO Unified Cohost v21.80.0");
    info!(target: "powrush::host", "TOLC 8 + 7 Living Mercy Gates active. PATSAGi Councils online.");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO Cohost — RTT Bridge Live".into(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        // Simulation layer (council + RTT export)
        .add_plugins(CouncilPlugin)
        // Server Ra-Thor integration (CohostExportMirror + auto-drain + transfer session)
        .add_plugins(RathorIntegrationPlugin)
        // Ensure CohostExportMirror starts enabled
        .insert_resource(CohostExportMirror::enabled())
        // Host-specific systems
        .add_systems(Startup, host_startup_system)
        .add_systems(Update, (
            host_drain_sim_to_mirror_system,
            host_status_log_system,
        ))
        .run();
}

fn host_startup_system() {
    info!(target: "powrush::host", "Cohost App online. Simulation ↔ Server RTT bridge armed.");
    info!(target: "powrush::host", "Drain path: CouncilRttExportQueue → CohostExportMirror → CouncilRttInbox → ServerTransferSession");
}

/// Core cohost adapter: drain pure sim export queue into the server's mirror every frame.
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
            "Host drained sim CouncilRttExportQueue → CohostExportMirror"
        );
    }
}

/// Lightweight status heartbeat (every ~5s)
fn host_status_log_system(
    time: Res<Time>,
    mirror: Res<CohostExportMirror>,
    export: Res<CouncilRttExportQueue>,
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
        mirror_pending = mirror.pending.len(),
        enabled = mirror.enabled,
        "Cohost heartbeat — RTT bridge healthy"
    );
}

// Thunder locked in. Unified cohost complete. Eternal forward. Yoi ⚡
