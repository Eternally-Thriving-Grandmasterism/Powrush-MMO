//! Server-side Ra-Thor transfer session (powrush_telemetry_v1).
//!
//! Lightweight counters — no dependency on the simulation crate.
//! Feed from high-signal events; export JSON for Ra-Thor Kardashev ingest.
//! Contact: info@Rathor.ai

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub rbe_samples: Vec<f64>,
    pub ethics_samples: Vec<f64>,
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
            rbe_samples: Vec::new(),
            ethics_samples: Vec::new(),
        }
    }

    pub fn record_combat(&mut self, was_critical: bool, damage: f32) {
        self.combat_events += 1;
        if was_critical {
            self.critical_hits += 1;
        }
        // Lower damage / non-crit → slightly higher “peaceful” bias sample
        let peaceful_bias = if was_critical {
            0.45
        } else if damage < 25.0 {
            0.85
        } else {
            0.65
        };
        self.rbe_samples.push(peaceful_bias);
        self.ethics_samples.push((peaceful_bias * 0.9).clamp(0.0, 1.0));
    }

    pub fn record_treaty(&mut self) {
        self.treaty_events += 1;
        self.rbe_samples.push(0.92);
        self.ethics_samples.push(0.9);
    }

    pub fn record_faction_shift(&mut self, old: f32, new: f32) {
        if new >= old {
            self.faction_improves += 1;
            self.rbe_samples.push(0.88);
            self.ethics_samples.push(0.86);
        } else {
            self.faction_worsens += 1;
            self.rbe_samples.push(0.55);
            self.ethics_samples.push(0.5);
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
        let total_res = self.combat_events + self.treaty_events + self.faction_improves + self.faction_worsens;
        let peaceful = if total_res == 0 {
            0.7
        } else {
            let good = self.treaty_events + self.faction_improves + self.combat_events.saturating_sub(self.critical_hits);
            (good as f64 / total_res as f64).clamp(0.0, 1.0)
        };

        PowrushTransferTelemetry {
            gameplay_hours: hours.max(0.0),
            rbe_decision_quality_avg: Self::mean(&self.rbe_samples, 0.7),
            peaceful_resolution_rate: peaceful,
            collaboration_events: self.treaty_events + self.faction_improves,
            ethical_choice_score: Self::mean(&self.ethics_samples, 0.7),
            adaptation_events: self.combat_events + self.treaty_events,
            abundance_velocity_signals: (0.9 + self.treaty_events as f64 * 0.05).min(1.8),
            innovation_contribution: (self.faction_improves as f64 * 0.04).clamp(0.0, 1.0),
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
        let json = s.export_json().unwrap();
        assert!(json.contains("powrush_telemetry_v1"));
        assert!(json.contains("powrush-mmo-server"));
    }
}
