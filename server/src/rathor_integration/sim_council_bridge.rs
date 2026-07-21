//! Soft file bridge: simulation council totals → ServerTransferSession
//! v21.76.0 — pure JSON, zero crate cycle
//!
//! Schema: `powrush_sim_council_bridge_v1`
//! Default path: `artifacts/sim_council_bridge.json`
//! Written by simulation TelemetryPlugin; read here.
//! Contact: info@Rathor.ai

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::info;

use super::transfer_session::ServerTransferSession;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimCouncilBridgePayload {
    pub schema: String,
    #[serde(default)]
    pub total_passed: u64,
    #[serde(default)]
    pub mercy_avg: f64,
    #[serde(default)]
    pub abundance_velocity: f64,
    #[serde(default)]
    pub sustainability_avg: f64,
    #[serde(default)]
    pub stress_avg: f64,
    #[serde(default)]
    pub tick: u64,
}

impl Default for SimCouncilBridgePayload {
    fn default() -> Self {
        Self {
            schema: "powrush_sim_council_bridge_v1".into(),
            total_passed: 0,
            mercy_avg: 0.7,
            abundance_velocity: 0.9,
            sustainability_avg: 0.7,
            stress_avg: 0.3,
            tick: 0,
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct SimCouncilBridgeConfig {
    pub path: PathBuf,
    pub interval_secs: f32,
    pub enabled: bool,
    pub last_poll_at: f32,
    /// Last total_passed we already applied (delta-only ingest).
    pub last_ingested_passed: u64,
    pub ingest_count: u64,
}

impl Default for SimCouncilBridgeConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("artifacts/sim_council_bridge.json"),
            interval_secs: 5.0,
            enabled: true,
            last_poll_at: -999.0,
            last_ingested_passed: 0,
            ingest_count: 0,
        }
    }
}

/// Apply pure totals from sim (delta on total_passed).
pub fn apply_sim_council_bridge(
    transfer: &mut ServerTransferSession,
    payload: &SimCouncilBridgePayload,
    last_passed: &mut u64,
) -> u64 {
    if payload.schema != "powrush_sim_council_bridge_v1" {
        return 0;
    }

    let mut applied = 0u64;
    if payload.total_passed > *last_passed {
        let delta = payload.total_passed - *last_passed;
        // Cap per-poll application to avoid spikes after long offline gaps
        let apply_n = delta.min(16);
        let mercy = payload.mercy_avg.clamp(0.0, 1.0);
        for _ in 0..apply_n {
            transfer.record_council_passed(mercy);
        }
        *last_passed = payload.total_passed;
        applied = apply_n;
    }

    if payload.abundance_velocity > 0.0 {
        transfer.record_abundance_velocity(payload.abundance_velocity.max(0.0));
    }

    let sust = payload.sustainability_avg.clamp(0.0, 1.0);
    if sust > 0.0 {
        transfer.rbe_samples.push(sust);
        if transfer.rbe_samples.len() > 128 {
            transfer.rbe_samples.drain(0..64);
        }
    }

    if payload.stress_avg > 0.55 {
        transfer.rbe_samples.push(0.55);
        if transfer.rbe_samples.len() > 128 {
            transfer.rbe_samples.drain(0..64);
        }
    }

    applied
}

/// Soft poll of sim bridge file → server transfer session.
pub fn sim_council_bridge_ingest_system(
    time: Res<Time>,
    mut cfg: ResMut<SimCouncilBridgeConfig>,
    mut transfer: ResMut<ServerTransferSession>,
) {
    if !cfg.enabled {
        return;
    }
    let now = time.elapsed_seconds();
    if now - cfg.last_poll_at < cfg.interval_secs {
        return;
    }
    cfg.last_poll_at = now;

    let bytes = match std::fs::read_to_string(&cfg.path) {
        Ok(b) => b,
        Err(_) => return, // soft — sim not writing yet / offline
    };

    let payload: SimCouncilBridgePayload = match serde_json::from_str(&bytes) {
        Ok(p) => p,
        Err(e) => {
            info!(target: "ra_thor::bridge", error = %e, "sim council bridge parse soft-fail");
            return;
        }
    };

    let applied = apply_sim_council_bridge(
        &mut transfer,
        &payload,
        &mut cfg.last_ingested_passed,
    );
    if applied > 0 || payload.total_passed > 0 {
        cfg.ingest_count = cfg.ingest_count.saturating_add(1);
        info!(
            target: "ra_thor::bridge",
            applied = applied,
            total_passed = payload.total_passed,
            mercy_avg = payload.mercy_avg,
            tick = payload.tick,
            "Sim council bridge ingested into ServerTransferSession"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_apply_council_passed() {
        let mut transfer = ServerTransferSession::new("bridge_test");
        let mut last = 0u64;
        let payload = SimCouncilBridgePayload {
            schema: "powrush_sim_council_bridge_v1".into(),
            total_passed: 3,
            mercy_avg: 0.88,
            abundance_velocity: 1.2,
            sustainability_avg: 0.8,
            stress_avg: 0.2,
            tick: 42,
        };
        let applied = apply_sim_council_bridge(&mut transfer, &payload, &mut last);
        assert_eq!(applied, 3);
        assert_eq!(last, 3);
        assert_eq!(transfer.council_passed, 3);
        // second apply with same total → 0 delta
        let applied2 = apply_sim_council_bridge(&mut transfer, &payload, &mut last);
        assert_eq!(applied2, 0);
    }
}
