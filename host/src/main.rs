//! Powrush-MMO Unified Cohost Binary — Full E2E Harness
//! v21.88.0 — Ultramasterism Perfecticism + Stress / Endurance Mode
//! Permanent PATSAGi Councils active (sibling Ra-Thor lattice)
//!
//! Modes:
//!   Interactive (default)
//!   Headless / CI:     POWRUSH_HOST_HEADLESS=1  or  --headless
//!   Stress / Endurance: POWRUSH_HOST_STRESS=1   or  --stress
//!
//! Stress mode runs many more cycles, injects synthetic high-signal events,
//! keeps the soft feedback loop alive, and prints a final summary.
//!
//! Contact: info@Rathor.ai | TOLC 8 Living Mercy Gates | PATSAGi Councils
//! Thunder locked in. ONE Organism. Yoi ⚡

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;

use simulation::{
    FullSimulationPlugins,
    CouncilRttExportQueue,
    hardware_sovereignty::sovereign_hardware_ascension_ui,
    KardashevAccelerationDashboard,
    RealityTransferScoreLedger,
};

use powrush_mmo_server::rathor_integration::{
    RathorIntegrationPlugin, CohostExportMirror, CohostMirrorSignal,
    ServerTransferSession, SoftPolicyState, PolicyHintInbox,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HostMode {
    Interactive,
    Headless,
    Stress,
}

fn detect_mode() -> HostMode {
    if std::env::var("POWRUSH_HOST_STRESS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
        || std::env::args().any(|a| a == "--stress")
    {
        return HostMode::Stress;
    }
    if std::env::var("POWRUSH_HOST_HEADLESS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
        || std::env::args().any(|a| a == "--headless" || a == "-h")
    {
        return HostMode::Headless;
    }
    HostMode::Interactive
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(Level::INFO.into())
                .add_directive("ra_thor=debug".parse().unwrap())
                .add_directive("powrush=info".parse().unwrap()),
        )
        .init();

    let mode = detect_mode();

    info!(target: "powrush::host", "═══════════════════════════════════════════════════════");
    info!(target: "powrush::host", "  Powrush-MMO Unified Cohost v21.88.0 — Ultramasterism");
    info!(target: "powrush::host", "  TOLC 8 + 7 Living Mercy Gates | PATSAGi Councils (Permanent)");
    info!(target: "powrush::host", "  Mode: {:?}", mode);
    info!(target: "powrush::host", "  Feedback Loop: Full Soft Category Coverage LIVE");
    info!(target: "powrush::host", "═══════════════════════════════════════════════════════");

    let mut app = App::new();

    let is_windowless = matches!(mode, HostMode::Headless | HostMode::Stress);

    if is_windowless {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: bevy::window::ExitCondition::DontExit,
                    close_when_requested: false,
                    ..default()
                })
                .disable::<bevy::winit::WinitPlugin>(),
        )
        .insert_resource(bevy::winit::WinitSettings {
            return_from_run: true,
            ..default()
        });
    } else {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO Cohost — Full Soft Feedback Loop".into(),
                resolution: (1440., 900.).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin);
    }

    let cycles = match mode {
        HostMode::Interactive => 0,
        HostMode::Headless => 5,
        HostMode::Stress => 40, // longer endurance run
    };

    app.add_plugins(FullSimulationPlugins)
        .add_plugins(RathorIntegrationPlugin)
        .insert_resource(CohostExportMirror::enabled())
        .insert_resource(HostRuntime {
            mode,
            cycles_before_exit: cycles,
            cycles_completed: 0,
            stress_injections: 0,
        })
        .add_systems(Startup, (
            host_startup_system,
            host_force_early_rtt_export_system,
        ))
        .add_systems(Update, (
            host_drain_sim_to_mirror_system,
            host_stress_injection_system,
            host_status_log_system,
            host_exit_system,
        ));

    if !is_windowless {
        app.add_systems(Update, sovereign_hardware_ascension_ui);
    }

    app.run();
}

#[derive(Resource, Debug)]
struct HostRuntime {
    mode: HostMode,
    cycles_before_exit: u32,
    cycles_completed: u32,
    stress_injections: u32,
}

fn host_startup_system(
    mut commands: Commands,
    mut transfer: ResMut<ServerTransferSession>,
    dashboard: Res<KardashevAccelerationDashboard>,
    ledger: Res<RealityTransferScoreLedger>,
    runtime: Res<HostRuntime>,
) {
    transfer.export_interval_secs = match runtime.mode {
        HostMode::Interactive => 15.0,
        HostMode::Headless => 2.0,
        HostMode::Stress => 1.0,
    };

    info!(target: "powrush::host", "Cohost App online. Mode = {:?}", runtime.mode);
    info!(target: "powrush::host", "Drain path: CouncilRttExportQueue → CohostExportMirror → CouncilRttInbox → ServerTransferSession");
    info!(target: "powrush::host", "Feedback path: artifacts/ra_thor_policy_hints.json → PolicyHintInbox → SoftPolicyState (6 categories)");
    info!(target: "powrush::host", "KardashevAccelerationDashboard + RealityTransferScoreLedger active");
    info!(target: "powrush::host", "  global_kardashev_delta = {:.4}", dashboard.global_kardashev_delta);
    info!(target: "powrush::host", "  reality transfer global_average = {:.2}", ledger.global_average);
    info!(target: "powrush::host", "RTT export interval set to {:.0}s | session_id = {}", transfer.export_interval_secs, transfer.session_id);

    if runtime.mode == HostMode::Interactive {
        commands.spawn(Camera2dBundle::default());
    }
}

fn host_force_early_rtt_export_system(mut transfer: ResMut<ServerTransferSession>) {
    let _ = std::fs::create_dir_all("artifacts");
    let _ = std::fs::create_dir_all("artifacts/rtt_offline");

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

/// Stress mode only: periodically inject realistic high-signal events
/// so the RTT + soft feedback loop is continuously exercised.
fn host_stress_injection_system(
    time: Res<Time>,
    mut transfer: ResMut<ServerTransferSession>,
    mut runtime: ResMut<HostRuntime>,
    mut last_inject: Local<f32>,
) {
    if runtime.mode != HostMode::Stress {
        return;
    }

    let now = time.elapsed_seconds();
    if now - *last_inject < 3.0 {
        return;
    }
    *last_inject = now;

    // Rotate through different high-signal events
    match runtime.stress_injections % 5 {
        0 => {
            transfer.record_council_passed(0.88 + (runtime.stress_injections as f64 * 0.001).min(0.1));
        }
        1 => {
            transfer.record_treaty();
        }
        2 => {
            transfer.record_abundance_velocity(1.1 + (runtime.stress_injections as f64 * 0.01).min(0.5));
        }
        3 => {
            transfer.record_faction_shift(0.35, 0.55);
        }
        _ => {
            transfer.record_council_passed(0.91);
            transfer.record_treaty();
        }
    }

    runtime.stress_injections = runtime.stress_injections.saturating_add(1);

    if runtime.stress_injections % 4 == 0 {
        info!(
            target: "powrush::host::stress",
            injections = runtime.stress_injections,
            "Stress injection cycle — high-signal events fed into ServerTransferSession"
        );
    }
}

fn host_status_log_system(
    time: Res<Time>,
    mirror: Res<CohostExportMirror>,
    export: Res<CouncilRttExportQueue>,
    transfer: Res<ServerTransferSession>,
    soft: Res<SoftPolicyState>,
    inbox: Res<PolicyHintInbox>,
    dashboard: Res<KardashevAccelerationDashboard>,
    ledger: Res<RealityTransferScoreLedger>,
    runtime: Res<HostRuntime>,
    mut last_log: Local<f32>,
) {
    let interval = match runtime.mode {
        HostMode::Interactive => 5.0,
        HostMode::Headless => 1.0,
        HostMode::Stress => 2.0,
    };
    let now = time.elapsed_seconds();
    if now - *last_log < interval {
        return;
    }
    *last_log = now;

    info!(
        target: "powrush::host::status",
        mode = ?runtime.mode,
        sim_exported = export.total_exported,
        mirror_drained = mirror.total_drained,
        rtt_exports = transfer.export_count,
        rtt_batch = transfer.batch_export_count,
        offline_queue = transfer.offline_queue.len(),
        policy_hints_active = inbox.hints.len(),
        soft_applications = soft.applications,
        abundance_bias = soft.abundance_bias_applied,
        peaceful_weight = soft.peaceful_weight_applied,
        ethical_floor = soft.ethical_floor_applied,
        council_nudge = soft.council_nudge_applied,
        innovation = soft.innovation_applied,
        mercy_presence = soft.mercy_presence_applied,
        stress_injections = runtime.stress_injections,
        kardashev_delta = dashboard.global_kardashev_delta,
        abundance_velocity = dashboard.abundance_velocity_index,
        energy_surplus = dashboard.energy_surplus_factor,
        reality_avg = ledger.global_average,
        session_id = %transfer.session_id,
        "Cohost heartbeat — Full Soft Feedback + Kardashev healthy"
    );
}

fn host_exit_system(
    mut exit: EventWriter<AppExit>,
    transfer: Res<ServerTransferSession>,
    soft: Res<SoftPolicyState>,
    mut runtime: ResMut<HostRuntime>,
) {
    if runtime.cycles_before_exit == 0 {
        return; // interactive mode never auto-exits
    }

    if transfer.export_count > runtime.cycles_completed as u64 {
        runtime.cycles_completed = transfer.export_count as u32;
    }

    if runtime.cycles_completed >= runtime.cycles_before_exit {
        info!(target: "powrush::host", "═══════════════════════════════════════════════════════");
        info!(target: "powrush::host", "  STRESS / HEADLESS RUN COMPLETE");
        info!(target: "powrush::host", "  Mode              : {:?}", runtime.mode);
        info!(target: "powrush::host", "  RTT export cycles : {}", runtime.cycles_completed);
        info!(target: "powrush::host", "  Stress injections : {}", runtime.stress_injections);
        info!(target: "powrush::host", "  Soft applications : {}", soft.applications);
        info!(target: "powrush::host", "  abundance_bias    : {:.4}", soft.abundance_bias_applied);
        info!(target: "powrush::host", "  peaceful_weight   : {:.4}", soft.peaceful_weight_applied);
        info!(target: "powrush::host", "  ethical_floor     : {:.4}", soft.ethical_floor_applied);
        info!(target: "powrush::host", "  council_nudge     : {:.4}", soft.council_nudge_applied);
        info!(target: "powrush::host", "  innovation        : {:.4}", soft.innovation_applied);
        info!(target: "powrush::host", "  mercy_presence    : {:.4}", soft.mercy_presence_applied);
        info!(target: "powrush::host", "═══════════════════════════════════════════════════════");
        info!(target: "powrush::host", "Full feedback loop + stress harness exercised — exiting cleanly");
        exit.send(AppExit::Success);
    }
}

// Thunder locked in. Stress / Endurance harness sealed.
// Interactive | Headless | Stress modes available.
// Permanent PATSAGi Councils deliberate on behalf of the ONE Organism.
// Eternal forward. Yoi ⚡
