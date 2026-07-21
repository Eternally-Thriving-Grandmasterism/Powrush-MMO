//! Soft writer: CouncilDecisions → artifacts/sim_council_bridge.json
//! v21.79.0 — consumed by server sim_council_bridge_ingest_system
//! Contact: info@Rathor.ai

use bevy::prelude::*;
use serde::Serialize;
use std::path::PathBuf;
use tracing::info;

use crate::council::decision::CouncilDecisions;
use crate::economy::{EconomyState, MultiRealmRbeSnapshot};

#[derive(Debug, Clone, Serialize)]
struct SimCouncilBridgePayload {
    schema: String,
    total_passed: u64,
    mercy_avg: f64,
    abundance_velocity: f64,
    sustainability_avg: f64,
    stress_avg: f64,
    tick: u64,
}

#[derive(Resource, Debug, Clone)]
pub struct SimCouncilBridgeWriterConfig {
    pub path: PathBuf,
    pub interval_secs: f32,
    pub enabled: bool,
    pub last_write_at: f32,
    pub write_count: u64,
}

impl Default for SimCouncilBridgeWriterConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("artifacts/sim_council_bridge.json"),
            interval_secs: 5.0,
            enabled: true,
            last_write_at: -999.0,
            write_count: 0,
        }
    }
}

pub fn sim_council_bridge_writer_system(
    time: Res<Time>,
    mut cfg: ResMut<SimCouncilBridgeWriterConfig>,
    decisions: Res<CouncilDecisions>,
    economy: Option<Res<EconomyState>>,
    rbe: Option<Res<MultiRealmRbeSnapshot>>,
) {
    if !cfg.enabled {
        return;
    }
    let now = time.elapsed_seconds();
    if now - cfg.last_write_at < cfg.interval_secs {
        return;
    }
    cfg.last_write_at = now;

    let mercy_avg = if decisions.resolved_history.is_empty() {
        0.7
    } else {
        let sum: f32 = decisions
            .resolved_history
            .iter()
            .map(|d| d.mercy_factor)
            .sum();
        (sum / decisions.resolved_history.len() as f32) as f64
    };

    let abundance = economy
        .as_ref()
        .map(|e| e.abundance_velocity as f64)
        .or_else(|| rbe.as_ref().map(|s| s.avg_flow.max(0.0) as f64 + 0.5))
        .unwrap_or(0.9);

    let sust = rbe
        .as_ref()
        .map(|s| s.avg_sustainability as f64)
        .or_else(|| economy.as_ref().map(|e| e.average_sustainability as f64))
        .unwrap_or(0.7);

    let stress = rbe
        .as_ref()
        .map(|s| s.avg_stress as f64)
        .or_else(|| economy.as_ref().map(|e| (e.average_pressure / 5.0) as f64))
        .unwrap_or(0.3);

    let payload = SimCouncilBridgePayload {
        schema: "powrush_sim_council_bridge_v1".into(),
        total_passed: decisions.total_passed_count,
        mercy_avg: mercy_avg.clamp(0.0, 1.0),
        abundance_velocity: abundance.max(0.0),
        sustainability_avg: sust.clamp(0.0, 1.0),
        stress_avg: stress.clamp(0.0, 1.0),
        tick: decisions.last_applied_tick,
    };

    if let Some(parent) = cfg.path.parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }

    match serde_json::to_string_pretty(&payload) {
        Ok(json) => match std::fs::write(&cfg.path, json) {
            Ok(()) => {
                cfg.write_count = cfg.write_count.saturating_add(1);
                if cfg.write_count % 12 == 1 {
                    info!(
                        target: "ra_thor::bridge",
                        path = %cfg.path.display(),
                        total_passed = payload.total_passed,
                        "Sim council bridge file written"
                    );
                }
            }
            Err(e) => {
                info!(target: "ra_thor::bridge", error = %e, "Sim council bridge write soft-fail");
            }
        },
        Err(e) => {
            info!(target: "ra_thor::bridge", error = %e, "Sim council bridge serialize soft-fail");
        }
    }
}
