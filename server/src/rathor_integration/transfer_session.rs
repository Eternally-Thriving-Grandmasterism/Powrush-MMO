//! Server-side Ra-Thor transfer session (powrush_telemetry_v1).
//! v21.74.0 — Periodic JSON export write path
//!
//! Lightweight counters — no dependency on the simulation crate.
//! Feed from high-signal events; export JSON for Ra-Thor Kardashev ingest.
//! Contact: info@Rathor.ai

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::info;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowrushTelemetryEnvelope {
    pub schema: String,
    pub source: String,
    pub label: String,
    pub telemetry: PowrushTransferTelemetry,
}

#[derive(Resource, Debug, Clone)]
pub struct ServerTransferSession {
    pub label: String,
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
    /// Soft export path (relative or absolute).
    pub export_path: PathBuf,
    /// Seconds between automatic writes (0 = disabled).
    pub export_interval_secs: f32,
    pub last_export_at: f32,
    pub export_count: u64,
}

impl Default for ServerTransferSession {
    fn default() -> Self {
        Self::new("server_live_session")
    }
}

impl ServerTransferSession {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
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
            export_interval_secs: 60.0,
            last_export_at: -999.0,
            export_count: 0,
        }
    }

    pub fn with_export_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.export_path = path.into();
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

    /// Soft feed from simulation council / RBE signals (when host bridges them).
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
            rbe_decision_quality_avg: Self::mean(&self.rbe_samples, 0.7),
            peaceful_resolution_rate: peaceful,
            collaboration_events: self.treaty_events
                + self.faction_improves
                + self.council_passed
                + self.collaboration_extra,
            ethical_choice_score: Self::mean(&self.ethics_samples, 0.7),
            adaptation_events: self.combat_events + self.treaty_events + self.council_passed,
            abundance_velocity_signals: Self::mean(
                &self.abundance_samples,
                (0.9 + self.treaty_events as f64 * 0.05).min(1.8),
            ),
            innovation_contribution: (self.faction_improves as f64 * 0.04
                + self.council_passed as f64 * 0.03)
                .clamp(0.0, 1.0),
        }
    }

    pub fn export_json(&self) -> Result<String, String> {
        let env = PowrushTelemetryEnvelope {
            schema: "powrush_telemetry_v1".into(),
            source: "powrush-mmo-server".into(),
            label: self.label.clone(),
            telemetry: self.to_transfer_telemetry(),
        };
        serde_json::to_string_pretty(&env).map_err(|e| e.to_string())
    }

    /// Write envelope JSON to disk (creates parent dirs).
    pub fn write_export_to(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
        }
        let json = self.export_json()?;
        std::fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn write_export_default(&mut self) -> Result<(), String> {
        self.write_export_to(&self.export_path.clone())?;
        self.export_count = self.export_count.saturating_add(1);
        Ok(())
    }
}

/// Soft periodic export — writes powrush_telemetry_v1 for Ra-Thor ingest.
pub fn server_rtt_export_system(time: Res<Time>, mut transfer: ResMut<ServerTransferSession>) {
    if transfer.export_interval_secs <= 0.0 {
        return;
    }
    let now = time.elapsed_seconds();
    if now - transfer.last_export_at < transfer.export_interval_secs {
        return;
    }
    match transfer.write_export_default() {
        Ok(()) => {
            transfer.last_export_at = now;
            info!(
                target: "ra_thor::rtt",
                path = %transfer.export_path.display(),
                export_count = transfer.export_count,
                "Server RTT export written (powrush_telemetry_v1)"
            );
        }
        Err(e) => {
            // Soft fail — do not panic host
            info!(target: "ra_thor::rtt", error = %e, "Server RTT export skipped");
            transfer.last_export_at = now; // backoff
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_schema_v1() {
        let mut s = ServerTransferSession::new("unit");
        s.record_treaty();
        s.record_combat(false, 10.0);
        s.record_faction_shift(0.2, 0.5);
        s.record_council_passed(0.82);
        let json = s.export_json().unwrap();
        assert!(json.contains("powrush_telemetry_v1"));
        assert!(json.contains("powrush-mmo-server"));
    }
}
