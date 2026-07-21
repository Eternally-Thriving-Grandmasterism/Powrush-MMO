//! Server-side Ra-Thor transfer session (powrush_telemetry_v1 / batch_v1).
//! v21.77.0 — Provenance (session_id + exported_at_unix) + batch + offline failsafe
//!
//! Lightweight counters — no dependency on the simulation crate.
//! Feed from high-signal events; export JSON for Ra-Thor Kardashev ingest.
//! Contact: info@Rathor.ai

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

const MAX_SNAPSHOTS: usize = 32;
const MAX_OFFLINE_QUEUE: usize = 16;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PowrushTransferTelemetry {
    pub gameplay_hours: f64,
    pub rbe_decision_quality_avg: f64,
    pub peaceful_resolution_rate: f64,
    pub collaboration_events: u64,
    pub ethical_choice_score: f64,
    pub adaptation_events: u64,
    pub abundance_velocity_signals: f64,
    pub innovation_contribution: f64,
}

/// Single-session envelope. Provenance fields are optional for forward compat
/// (Ra-Thor serde ignores unknown fields; newer Ra-Thor may surface them).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowrushTelemetryEnvelope {
    pub schema: String,
    pub source: String,
    pub label: String,
    /// Stable session identity for council provenance / feedback loops.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// Unix seconds when this envelope was produced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exported_at_unix: Option<u64>,
    /// Monotonic export sequence on this host process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_seq: Option<u64>,
    pub telemetry: PowrushTransferTelemetry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowrushTelemetrySession {
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exported_at_unix: Option<u64>,
    pub telemetry: PowrushTransferTelemetry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowrushTelemetryBatch {
    pub schema: String,
    pub source: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exported_at_unix: Option<u64>,
    pub sessions: Vec<PowrushTelemetrySession>,
}

#[derive(Resource, Debug, Clone)]
pub struct ServerTransferSession {
    pub label: String,
    /// Stable session id (default: label + process start unix).
    pub session_id: String,
    pub started: std::time::Instant,
    pub combat_events: u64,
    pub critical_hits: u64,
    pub treaty_events: u64,
    pub faction_improves: u64,
    pub faction_worsens: u64,
    pub council_passed: u64,
    pub collaboration_extra: u64,
    pub rbe_samples: Vec<f64>,
    pub ethics_samples: Vec<f64>,
    pub abundance_samples: Vec<f64>,
    pub export_path: PathBuf,
    pub batch_export_path: PathBuf,
    pub offline_dir: PathBuf,
    pub export_interval_secs: f32,
    pub last_export_at: f32,
    pub export_count: u64,
    pub batch_export_count: u64,
    pub offline_flush_count: u64,
    pub session_snapshots: Vec<PowrushTelemetrySession>,
    pub offline_queue: Vec<String>,
}

impl Default for ServerTransferSession {
    fn default() -> Self {
        Self::new("server_live_session")
    }
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

impl ServerTransferSession {
    pub fn new(label: impl Into<String>) -> Self {
        let label = label.into();
        let session_id = format!("{}_{}", label, now_unix());
        Self {
            label,
            session_id,
            started: std::time::Instant::now(),
            combat_events: 0,
            critical_hits: 0,
            treaty_events: 0,
            faction_improves: 0,
            faction_worsens: 0,
            council_passed: 0,
            collaboration_extra: 0,
            rbe_samples: Vec::new(),
            ethics_samples: Vec::new(),
            abundance_samples: Vec::new(),
            export_path: PathBuf::from("artifacts/powrush_rtt_latest.json"),
            batch_export_path: PathBuf::from("artifacts/powrush_rtt_batch_latest.json"),
            offline_dir: PathBuf::from("artifacts/rtt_offline"),
            export_interval_secs: 60.0,
            last_export_at: -999.0,
            export_count: 0,
            batch_export_count: 0,
            offline_flush_count: 0,
            session_snapshots: Vec::new(),
            offline_queue: Vec::new(),
        }
    }

    pub fn with_export_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.export_path = path.into();
        self
    }

    pub fn with_session_id(mut self, id: impl Into<String>) -> Self {
        self.session_id = id.into();
        self
    }

    pub fn record_combat(&mut self, was_critical: bool, damage: f32) {
        self.combat_events += 1;
        if was_critical {
            self.critical_hits += 1;
        }
        let peaceful_bias = if was_critical {
            0.45
        } else if damage < 25.0 {
            0.85
        } else {
            0.65
        };
        self.push_sample(&mut self.rbe_samples, peaceful_bias);
        self.push_sample(&mut self.ethics_samples, (peaceful_bias * 0.9).clamp(0.0, 1.0));
    }

    pub fn record_treaty(&mut self) {
        self.treaty_events += 1;
        self.push_sample(&mut self.rbe_samples, 0.92);
        self.push_sample(&mut self.ethics_samples, 0.9);
        self.push_sample(&mut self.abundance_samples, 1.1);
    }

    pub fn record_faction_shift(&mut self, old: f32, new: f32) {
        if new >= old {
            self.faction_improves += 1;
            self.push_sample(&mut self.rbe_samples, 0.88);
            self.push_sample(&mut self.ethics_samples, 0.86);
        } else {
            self.faction_worsens += 1;
            self.push_sample(&mut self.rbe_samples, 0.55);
            self.push_sample(&mut self.ethics_samples, 0.5);
        }
    }

    pub fn record_council_passed(&mut self, mercy_factor: f64) {
        self.council_passed = self.council_passed.saturating_add(1);
        self.collaboration_extra = self.collaboration_extra.saturating_add(1);
        self.push_sample(&mut self.ethics_samples, mercy_factor.clamp(0.0, 1.0));
        self.push_sample(&mut self.rbe_samples, mercy_factor.clamp(0.0, 1.0));
    }

    pub fn record_abundance_velocity(&mut self, v: f64) {
        self.push_sample(&mut self.abundance_samples, v.max(0.0));
    }

    fn push_sample(&self, samples: &mut Vec<f64>, v: f64) {
        samples.push(v);
        if samples.len() > 128 {
            samples.drain(0..64);
        }
    }

    fn mean(samples: &[f64], default: f64) -> f64 {
        if samples.is_empty() {
            default
        } else {
            samples.iter().sum::<f64>() / samples.len() as f64
        }
    }

    pub fn to_transfer_telemetry(&self) -> PowrushTransferTelemetry {
        let hours = self.started.elapsed().as_secs_f64() / 3600.0;
        let total_res = self.combat_events
            + self.treaty_events
            + self.faction_improves
            + self.faction_worsens
            + self.council_passed;
        let peaceful = if total_res == 0 {
            0.7
        } else {
            let good = self.treaty_events
                + self.faction_improves
                + self.council_passed
                + self.combat_events.saturating_sub(self.critical_hits);
            (good as f64 / total_res as f64).clamp(0.0, 1.0)
        };

        PowrushTransferTelemetry {
            gameplay_hours: hours.max(0.0),
            rbe_decision_quality_avg: Self::mean(&self.rbe_samples, 0.7).clamp(0.0, 1.0),
            peaceful_resolution_rate: peaceful.clamp(0.0, 1.0),
            collaboration_events: self.treaty_events
                + self.faction_improves
                + self.council_passed
                + self.collaboration_extra,
            ethical_choice_score: Self::mean(&self.ethics_samples, 0.7).clamp(0.0, 1.0),
            adaptation_events: self.combat_events + self.treaty_events + self.council_passed,
            abundance_velocity_signals: Self::mean(
                &self.abundance_samples,
                (0.9 + self.treaty_events as f64 * 0.05).min(1.8),
            )
            .max(0.0),
            innovation_contribution: (self.faction_improves as f64 * 0.04
                + self.council_passed as f64 * 0.03)
                .clamp(0.0, 1.0),
        }
    }

    fn snapshot_session(&self) -> PowrushTelemetrySession {
        PowrushTelemetrySession {
            label: format!("{}_snap_{}", self.label, self.export_count),
            session_id: Some(format!("{}_{}", self.session_id, self.export_count)),
            exported_at_unix: Some(now_unix()),
            telemetry: self.to_transfer_telemetry(),
        }
    }

    fn push_snapshot(&mut self, snap: PowrushTelemetrySession) {
        self.session_snapshots.push(snap);
        if self.session_snapshots.len() > MAX_SNAPSHOTS {
            let excess = self.session_snapshots.len() - MAX_SNAPSHOTS;
            self.session_snapshots.drain(0..excess);
        }
    }

    pub fn export_json(&self) -> Result<String, String> {
        let env = PowrushTelemetryEnvelope {
            schema: "powrush_telemetry_v1".into(),
            source: "powrush-mmo-server".into(),
            label: self.label.clone(),
            session_id: Some(self.session_id.clone()),
            exported_at_unix: Some(now_unix()),
            export_seq: Some(self.export_count),
            telemetry: self.to_transfer_telemetry(),
        };
        serde_json::to_string_pretty(&env).map_err(|e| e.to_string())
    }

    pub fn export_batch_json(&self) -> Result<String, String> {
        let mut sessions = self.session_snapshots.clone();
        if sessions.is_empty() {
            sessions.push(PowrushTelemetrySession {
                label: self.label.clone(),
                session_id: Some(self.session_id.clone()),
                exported_at_unix: Some(now_unix()),
                telemetry: self.to_transfer_telemetry(),
            });
        }
        let batch = PowrushTelemetryBatch {
            schema: "powrush_telemetry_batch_v1".into(),
            source: "powrush-mmo-server".into(),
            label: format!("{}_batch", self.label),
            exported_at_unix: Some(now_unix()),
            sessions,
        };
        serde_json::to_string_pretty(&batch).map_err(|e| e.to_string())
    }

    fn write_bytes(path: &Path, bytes: &str) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
        }
        std::fs::write(path, bytes).map_err(|e| e.to_string())
    }

    pub fn write_export_to(&self, path: &Path) -> Result<(), String> {
        let json = self.export_json()?;
        Self::write_bytes(path, &json)
    }

    pub fn write_batch_export_to(&self, path: &Path) -> Result<(), String> {
        let json = self.export_batch_json()?;
        Self::write_bytes(path, &json)
    }

    /// Offline queue: drop-oldest when over MAX_OFFLINE_QUEUE (16).
    fn enqueue_offline(&mut self, json: String) {
        self.offline_queue.push(json);
        if self.offline_queue.len() > MAX_OFFLINE_QUEUE {
            let excess = self.offline_queue.len() - MAX_OFFLINE_QUEUE;
            self.offline_queue.drain(0..excess);
        }
        let idx = self.offline_queue.len();
        let path = self.offline_dir.join(format!("queued_{:04}.json", idx));
        let _ = Self::write_bytes(&path, self.offline_queue.last().map(|s| s.as_str()).unwrap_or(""));
    }

    pub fn try_flush_offline_queue(&mut self) -> usize {
        let mut flushed = 0usize;
        while let Some(json) = self.offline_queue.first().cloned() {
            match Self::write_bytes(&self.export_path, &json) {
                Ok(()) => {
                    self.offline_queue.remove(0);
                    flushed += 1;
                    self.offline_flush_count = self.offline_flush_count.saturating_add(1);
                }
                Err(_) => break,
            }
        }
        flushed
    }

    pub fn write_export_cycle(&mut self) -> Result<(), String> {
        let snap = self.snapshot_session();
        self.push_snapshot(snap);

        let _ = self.try_flush_offline_queue();

        let single = self.export_json()?;
        match Self::write_bytes(&self.export_path, &single) {
            Ok(()) => {
                self.export_count = self.export_count.saturating_add(1);
            }
            Err(e) => {
                self.enqueue_offline(single);
                return Err(format!("primary write failed (queued offline): {}", e));
            }
        }

        if let Ok(batch) = self.export_batch_json() {
            if Self::write_bytes(&self.batch_export_path, &batch).is_ok() {
                self.batch_export_count = self.batch_export_count.saturating_add(1);
            }
        }

        Ok(())
    }
}

pub fn server_rtt_export_system(time: Res<Time>, mut transfer: ResMut<ServerTransferSession>) {
    if transfer.export_interval_secs <= 0.0 {
        return;
    }
    let now = time.elapsed_seconds();
    if now - transfer.last_export_at < transfer.export_interval_secs {
        return;
    }

    match transfer.write_export_cycle() {
        Ok(()) => {
            transfer.last_export_at = now;
            info!(
                target: "ra_thor::rtt",
                path = %transfer.export_path.display(),
                batch = %transfer.batch_export_path.display(),
                session_id = %transfer.session_id,
                export_count = transfer.export_count,
                batch_count = transfer.batch_export_count,
                snapshots = transfer.session_snapshots.len(),
                "Server RTT export cycle OK (v1 + batch_v1 + provenance)"
            );
        }
        Err(e) => {
            transfer.last_export_at = now;
            info!(
                target: "ra_thor::rtt",
                error = %e,
                offline_queue = transfer.offline_queue.len(),
                max_offline = MAX_OFFLINE_QUEUE,
                "Server RTT export soft-failed — offline queue retained (drop-oldest at cap)"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_schema_v1_with_provenance() {
        let mut s = ServerTransferSession::new("unit");
        s.record_treaty();
        s.record_combat(false, 10.0);
        s.record_faction_shift(0.2, 0.5);
        s.record_council_passed(0.82);
        let json = s.export_json().unwrap();
        assert!(json.contains("powrush_telemetry_v1"));
        assert!(json.contains("powrush-mmo-server"));
        assert!(json.contains("session_id"));
        assert!(json.contains("exported_at_unix"));
    }

    #[test]
    fn export_batch_v1_schema() {
        let mut s = ServerTransferSession::new("unit_batch");
        s.record_treaty();
        s.push_snapshot(s.snapshot_session());
        s.record_council_passed(0.9);
        s.push_snapshot(s.snapshot_session());
        let json = s.export_batch_json().unwrap();
        assert!(json.contains("powrush_telemetry_batch_v1"));
        assert!(json.contains("sessions"));
        assert!(json.contains("exported_at_unix"));
    }

    #[test]
    fn field_bounds_mercy_gates() {
        let mut s = ServerTransferSession::new("bounds");
        s.record_council_passed(0.95);
        let t = s.to_transfer_telemetry();
        assert!((0.0..=1.0).contains(&t.rbe_decision_quality_avg));
        assert!((0.0..=1.0).contains(&t.ethical_choice_score));
        assert!(t.abundance_velocity_signals >= 0.0);
    }

    #[test]
    fn offline_queue_caps_at_16() {
        let mut s = ServerTransferSession::new("cap");
        for i in 0..20 {
            s.enqueue_offline(format!("{{\"n\":{}}}", i));
        }
        assert_eq!(s.offline_queue.len(), MAX_OFFLINE_QUEUE);
    }
}
