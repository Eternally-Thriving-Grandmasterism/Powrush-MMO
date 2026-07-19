/*!
 * Sovereign Telemetry + Ra-Thor Transfer Export
 *
 * In-sim metrics for the Sovereign Simulation Harness, plus offline JSON
 * envelopes matching Ra-Thor `reality-thriving-transfer` contract:
 *   schema: powrush_telemetry_v1 / powrush_telemetry_batch_v1
 *
 * Consumer: https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor
 *           crates/reality-thriving-transfer
 * Contact: info@Rathor.ai
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Telemetry {
    pub tick: u64,
    pub total_yield_harvested: f32,
    pub average_mercy_flow: f32,
    pub epiphany_count: u32,
    pub flow_state_entries: u32,
    pub receptor_blooms: u32,
    pub abundance_blooms: u32,
    pub stress_events: u32,
    pub custom_metrics: HashMap<String, f32>,
}

pub struct TelemetryCollector {
    pub current: Telemetry,
}

impl Default for TelemetryCollector {
    fn default() -> Self {
        Self {
            current: Telemetry::default(),
        }
    }
}

impl TelemetryCollector {
    pub fn collect_tick(&mut self, world_tick: u64, mercy_flow: f32) {
        self.current.tick = world_tick;
        self.current.average_mercy_flow =
            (self.current.average_mercy_flow * 0.9 + mercy_flow * 0.1).clamp(0.0, 2.0);
    }

    pub fn generate_final_report(&self) -> Telemetry {
        self.current.clone()
    }
}

/// Sovereign helper expected by lib.rs and orchestrator.
pub fn current() -> Telemetry {
    Telemetry {
        tick: 0,
        total_yield_harvested: 0.0,
        average_mercy_flow: 0.85,
        epiphany_count: 0,
        flow_state_entries: 0,
        receptor_blooms: 0,
        abundance_blooms: 0,
        stress_events: 0,
        custom_metrics: HashMap::new(),
    }
}

// =============================================================================
// Ra-Thor Reality Thriving Transfer contract (powrush_telemetry_v1)
// =============================================================================

/// Canonical fields consumed by Ra-Thor `PowrushTelemetry`.
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowrushTelemetrySession {
    pub label: String,
    pub telemetry: PowrushTransferTelemetry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowrushTelemetryBatch {
    pub schema: String,
    pub source: String,
    pub label: String,
    pub sessions: Vec<PowrushTelemetrySession>,
}

/// Session counters used to build a transfer snapshot (stub until live wiring).
#[derive(Debug, Clone, Default)]
pub struct SessionTransferCounters {
    pub gameplay_hours: f64,
    pub rbe_quality_samples: Vec<f64>,
    pub peaceful_resolutions: u64,
    pub total_resolutions: u64,
    pub collaboration_events: u64,
    pub ethical_choice_samples: Vec<f64>,
    pub adaptation_events: u64,
    pub abundance_velocity_samples: Vec<f64>,
    pub innovation_contribution: f64,
}

impl SessionTransferCounters {
    pub fn record_rbe_quality(&mut self, q: f64) {
        self.rbe_quality_samples.push(q.clamp(0.0, 1.0));
    }

    pub fn record_resolution(&mut self, peaceful: bool) {
        self.total_resolutions += 1;
        if peaceful {
            self.peaceful_resolutions += 1;
        }
    }

    pub fn record_ethical_choice(&mut self, score: f64) {
        self.ethical_choice_samples.push(score.clamp(0.0, 1.0));
    }

    pub fn record_abundance_velocity(&mut self, v: f64) {
        self.abundance_velocity_samples.push(v.max(0.0));
    }

    fn mean(samples: &[f64], default: f64) -> f64 {
        if samples.is_empty() {
            default
        } else {
            samples.iter().sum::<f64>() / samples.len() as f64
        }
    }

    /// Map session counters → Ra-Thor `PowrushTelemetry` fields.
    pub fn to_transfer_telemetry(&self) -> PowrushTransferTelemetry {
        let peaceful_rate = if self.total_resolutions == 0 {
            0.7
        } else {
            self.peaceful_resolutions as f64 / self.total_resolutions as f64
        };

        PowrushTransferTelemetry {
            gameplay_hours: self.gameplay_hours.max(0.0),
            rbe_decision_quality_avg: Self::mean(&self.rbe_quality_samples, 0.6),
            peaceful_resolution_rate: peaceful_rate.clamp(0.0, 1.0),
            collaboration_events: self.collaboration_events,
            ethical_choice_score: Self::mean(&self.ethical_choice_samples, 0.6),
            adaptation_events: self.adaptation_events,
            abundance_velocity_signals: Self::mean(&self.abundance_velocity_samples, 0.9),
            innovation_contribution: self.innovation_contribution.clamp(0.0, 1.0),
        }
    }
}

/// Build a single-session envelope ready for Ra-Thor ingest.
pub fn export_transfer_envelope(
    label: &str,
    telemetry: PowrushTransferTelemetry,
) -> PowrushTelemetryEnvelope {
    PowrushTelemetryEnvelope {
        schema: "powrush_telemetry_v1".into(),
        source: "powrush-mmo".into(),
        label: label.into(),
        telemetry,
    }
}

/// Serialize single session to JSON (Ra-Thor contract).
pub fn export_transfer_json(label: &str, telemetry: &PowrushTransferTelemetry) -> Result<String, String> {
    let env = export_transfer_envelope(label, telemetry.clone());
    serde_json::to_string_pretty(&env).map_err(|e| e.to_string())
}

/// Serialize a batch of sessions.
pub fn export_transfer_batch_json(
    label: &str,
    sessions: Vec<(String, PowrushTransferTelemetry)>,
) -> Result<String, String> {
    let batch = PowrushTelemetryBatch {
        schema: "powrush_telemetry_batch_v1".into(),
        source: "powrush-mmo".into(),
        label: label.into(),
        sessions: sessions
            .into_iter()
            .map(|(l, t)| PowrushTelemetrySession {
                label: l,
                telemetry: t,
            })
            .collect(),
    };
    serde_json::to_string_pretty(&batch).map_err(|e| e.to_string())
}

/// Example high-mercy snapshot (matches Ra-Thor fixture intent).
pub fn example_high_mercy_session() -> PowrushTransferTelemetry {
    PowrushTransferTelemetry {
        gameplay_hours: 86.5,
        rbe_decision_quality_avg: 0.91,
        peaceful_resolution_rate: 0.94,
        collaboration_events: 420,
        ethical_choice_score: 0.89,
        adaptation_events: 175,
        abundance_velocity_signals: 1.55,
        innovation_contribution: 0.81,
    }
}

/// Best-effort map from in-sim `Telemetry` + session hours → transfer fields.
/// Live game systems should prefer `SessionTransferCounters` for accuracy.
pub fn map_sim_telemetry_to_transfer(
    sim: &Telemetry,
    gameplay_hours: f64,
    collaboration_events: u64,
) -> PowrushTransferTelemetry {
    let mercy = sim.average_mercy_flow.clamp(0.0, 2.0) as f64;
    let rbe_q = (mercy / 2.0).clamp(0.0, 1.0);
    let ethics = (0.55 + (sim.epiphany_count as f64 * 0.01).min(0.35)).clamp(0.0, 1.0);
    let peaceful = (0.60 + (1.0 - (sim.stress_events as f64 * 0.02).min(0.4))).clamp(0.0, 1.0);
    let abundance = (0.8 + sim.abundance_blooms as f64 * 0.05).min(1.8);
    let innovation = (sim.receptor_blooms as f64 * 0.03 + sim.flow_state_entries as f64 * 0.02)
        .clamp(0.0, 1.0);

    PowrushTransferTelemetry {
        gameplay_hours: gameplay_hours.max(0.0),
        rbe_decision_quality_avg: rbe_q,
        peaceful_resolution_rate: peaceful,
        collaboration_events,
        ethical_choice_score: ethics,
        adaptation_events: sim.epiphany_count as u64 + sim.flow_state_entries as u64,
        abundance_velocity_signals: abundance,
        innovation_contribution: innovation,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_json_schema_is_v1() {
        let t = example_high_mercy_session();
        let json = export_transfer_json("high_mercy_council_session", &t).unwrap();
        assert!(json.contains("powrush_telemetry_v1"));
        assert!(json.contains("rbe_decision_quality_avg"));
        let parsed: PowrushTelemetryEnvelope = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.schema, "powrush_telemetry_v1");
        assert!(parsed.telemetry.collaboration_events >= 400);
    }

    #[test]
    fn counters_to_transfer_bounds() {
        let mut c = SessionTransferCounters::default();
        c.gameplay_hours = 10.0;
        c.record_rbe_quality(0.9);
        c.record_resolution(true);
        c.record_resolution(true);
        c.record_ethical_choice(0.85);
        c.record_abundance_velocity(1.2);
        c.collaboration_events = 20;
        c.adaptation_events = 5;
        c.innovation_contribution = 0.5;
        let t = c.to_transfer_telemetry();
        assert!((0.0..=1.0).contains(&t.rbe_decision_quality_avg));
        assert!((0.0..=1.0).contains(&t.peaceful_resolution_rate));
        assert!(t.abundance_velocity_signals >= 0.0);
    }

    #[test]
    fn batch_export() {
        let sessions = vec![(
            "high_mercy_council_session".into(),
            example_high_mercy_session(),
        )];
        let json = export_transfer_batch_json("demo_batch", sessions).unwrap();
        assert!(json.contains("powrush_telemetry_batch_v1"));
    }
}
