/*!
 * Sovereign Telemetry + Ra-Thor Transfer Export
 * v21.70.0 — Bevy SimulationTelemetry + Council/RBE → RTT feed
 *
 * In-sim metrics for the Sovereign Simulation Harness, plus offline JSON
 * envelopes matching Ra-Thor `reality-thriving-transfer` contract:
 *   schema: powrush_telemetry_v1 / powrush_telemetry_batch_v1
 *
 * Live path: TelemetryCollector holds a GlobalTransferSession and updates it
 * on every collect_tick / record_tick_result (not profile-only).
 *
 * Consumer: https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor
 *           crates/reality-thriving-transfer
 * Contact: info@Rathor.ai
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tracing::info;

use crate::council::decision::CouncilDecisions;
use crate::economy::MultiRealmRbeSnapshot;

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

// =============================================================================
// Bevy-facing SimulationTelemetry (used by hardware_sovereignty + systems)
// =============================================================================

#[derive(Resource, Debug, Clone)]
pub struct SimulationTelemetry {
    pub events: Vec<(String, f64)>,
    pub last_tick: u64,
    pub export_ready_for_ra_thor: bool,
}

impl Default for SimulationTelemetry {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            last_tick: 0,
            export_ready_for_ra_thor: false,
        }
    }
}

impl SimulationTelemetry {
    pub fn record_event(&mut self, name: &str, value: f64) {
        self.events.push((name.to_string(), value));
        if self.events.len() > 256 {
            let excess = self.events.len() - 256;
            self.events.drain(0..excess);
        }
        self.export_ready_for_ra_thor = true;
    }
}

/// Dual-track collector: local sim metrics + Ra-Thor transfer session.
#[derive(Resource)]
pub struct TelemetryCollector {
    pub current: Telemetry,
    /// Accumulates live counters for Ra-Thor Reality Thriving Transfer export.
    pub transfer_session: GlobalTransferSession,
    /// Seconds of sim time assumed per tick when estimating gameplay_hours.
    pub seconds_per_tick: f64,
}

impl Default for TelemetryCollector {
    fn default() -> Self {
        Self::new("powrush_sim_session")
    }
}

impl TelemetryCollector {
    pub fn new(session_label: impl Into<String>) -> Self {
        Self {
            current: Telemetry::default(),
            transfer_session: GlobalTransferSession::new(session_label),
            seconds_per_tick: 6.0, // 10 ticks ≈ 1 minute of “play”
        }
    }

    /// Primary tick hook — updates local telemetry and live transfer counters.
    pub fn collect_tick(&mut self, world_tick: u64, mercy_flow: f32) {
        self.current.tick = world_tick;
        self.current.average_mercy_flow =
            (self.current.average_mercy_flow * 0.9 + mercy_flow * 0.1).clamp(0.0, 2.0);

        // Live transfer path (not profile-based)
        let hours = (world_tick as f64 * self.seconds_per_tick) / 3600.0;
        self.transfer_session.set_gameplay_hours(hours);
        self.transfer_session
            .ingest_sim_tick(&self.current, 0);
    }

    /// Richer feed from orchestrator TickResult (council / harvest / epiphany).
    pub fn record_tick_result(
        &mut self,
        world_tick: u64,
        mercy_flow: f32,
        council_participants: u32,
        epiphany_impacts: u32,
        harvest_nodes: u32,
        had_errors: bool,
    ) {
        if epiphany_impacts > 0 {
            self.current.epiphany_count =
                self.current.epiphany_count.saturating_add(epiphany_impacts);
        }
        if harvest_nodes > 0 {
            self.current.abundance_blooms =
                self.current.abundance_blooms.saturating_add(1);
            self.current.total_yield_harvested += harvest_nodes as f32 * 0.1;
        }
        if had_errors {
            self.current.stress_events = self.current.stress_events.saturating_add(1);
        }
        if council_participants > 0 {
            self.current.flow_state_entries =
                self.current.flow_state_entries.saturating_add(1);
        }

        self.collect_tick(world_tick, mercy_flow);

        // Extra collaboration signal from council participation
        if council_participants > 0 {
            self.transfer_session
                .counters
                .record_collaboration(council_participants as u64);
            self.transfer_session
                .counters
                .record_ethical_choice((0.7 + council_participants as f64 * 0.02).min(0.99));
        }
    }

    pub fn generate_final_report(&self) -> Telemetry {
        self.current.clone()
    }

    /// Export live-accumulated Ra-Thor JSON (powrush_telemetry_v1).
    pub fn export_transfer_json(&self) -> Result<String, String> {
        self.transfer_session.export_json()
    }

    /// Write transfer JSON to a path (for harness / CI artifacts).
    pub fn write_transfer_json_to(&self, path: impl AsRef<Path>) -> Result<(), String> {
        let json = self.export_transfer_json()?;
        std::fs::write(path.as_ref(), json).map_err(|e| e.to_string())
    }

    pub fn transfer_telemetry(&self) -> PowrushTransferTelemetry {
        self.transfer_session.to_transfer_telemetry()
    }
}

/// Soft system: feeds CouncilDecisions + MultiRealmRbeSnapshot into RTT session.
pub fn council_rbe_to_rtt_feed_system(
    decisions: Option<Res<CouncilDecisions>>,
    rbe: Option<Res<MultiRealmRbeSnapshot>>,
    mut collector: Option<ResMut<TelemetryCollector>>,
    mut sim_tel: Option<ResMut<SimulationTelemetry>>,
) {
    let Some(mut collector) = collector else {
        return;
    };

    if let Some(dec) = decisions {
        if dec.total_passed_count > 0 {
            // Collaboration + ethical signal from council memory
            let n = dec.total_passed_count.min(32);
            collector
                .transfer_session
                .counters
                .record_collaboration(n);
            let mercy_avg = if dec.resolved_history.is_empty() {
                0.7
            } else {
                let sum: f32 = dec
                    .resolved_history
                    .iter()
                    .map(|d| d.mercy_factor)
                    .sum();
                (sum / dec.resolved_history.len() as f32) as f64
            };
            collector
                .transfer_session
                .counters
                .record_ethical_choice(mercy_avg.clamp(0.0, 1.0));
            collector
                .transfer_session
                .counters
                .record_rbe_quality(mercy_avg.clamp(0.0, 1.0));

            if let Some(ref mut st) = sim_tel {
                st.record_event("council_passed_total", dec.total_passed_count as f64);
                st.record_event("council_mercy_avg", mercy_avg);
            }
        }
    }

    if let Some(snap) = rbe {
        if snap.realm_count > 0 {
            collector
                .transfer_session
                .counters
                .record_abundance_velocity(snap.avg_flow.max(0.0) as f64 + 0.5);
            collector
                .transfer_session
                .counters
                .record_rbe_quality(snap.avg_sustainability.clamp(0.0, 1.0) as f64);

            if snap.avg_stress > 0.55 {
                collector.transfer_session.counters.record_resolution(false);
            } else {
                collector.transfer_session.counters.record_resolution(true);
            }

            if let Some(ref mut st) = sim_tel {
                st.record_event("organism_sust", snap.avg_sustainability as f64);
                st.record_event("organism_stress", snap.avg_stress as f64);
                st.record_event("organism_thriving_ratio", snap.thriving_ratio as f64);
            }
        }
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

/// Session counters used to build a transfer snapshot.
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
        if self.rbe_quality_samples.len() > 128 {
            self.rbe_quality_samples.drain(0..64);
        }
    }

    pub fn record_resolution(&mut self, peaceful: bool) {
        self.total_resolutions += 1;
        if peaceful {
            self.peaceful_resolutions += 1;
        }
    }

    pub fn record_ethical_choice(&mut self, score: f64) {
        self.ethical_choice_samples.push(score.clamp(0.0, 1.0));
        if self.ethical_choice_samples.len() > 128 {
            self.ethical_choice_samples.drain(0..64);
        }
    }

    pub fn record_abundance_velocity(&mut self, v: f64) {
        self.abundance_velocity_samples.push(v.max(0.0));
        if self.abundance_velocity_samples.len() > 128 {
            self.abundance_velocity_samples.drain(0..64);
        }
    }

    pub fn record_collaboration(&mut self, n: u64) {
        self.collaboration_events = self.collaboration_events.saturating_add(n);
    }

    pub fn record_adaptation(&mut self, n: u64) {
        self.adaptation_events = self.adaptation_events.saturating_add(n);
    }

    fn mean(samples: &[f64], default: f64) -> f64 {
        if samples.is_empty() {
            default
        } else {
            samples.iter().sum::<f64>() / samples.len() as f64
        }
    }

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

/// Named session accumulator for sim/server loops → Ra-Thor export.
#[derive(Debug, Clone)]
pub struct GlobalTransferSession {
    pub label: String,
    pub counters: SessionTransferCounters,
}

impl GlobalTransferSession {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            counters: SessionTransferCounters::default(),
        }
    }

    pub fn set_gameplay_hours(&mut self, hours: f64) {
        self.counters.gameplay_hours = hours.max(0.0);
    }

    pub fn ingest_sim_tick(&mut self, sim: &Telemetry, collaboration_delta: u64) {
        let mercy = sim.average_mercy_flow.clamp(0.0, 2.0) as f64;
        self.counters.record_rbe_quality((mercy / 2.0).clamp(0.0, 1.0));
        if sim.stress_events > 0 {
            self.counters.record_resolution(false);
        } else {
            self.counters.record_resolution(true);
        }
        let ethics = (0.55 + (sim.epiphany_count as f64 * 0.01).min(0.35)).clamp(0.0, 1.0);
        self.counters.record_ethical_choice(ethics);
        self.counters
            .record_abundance_velocity((0.8 + sim.abundance_blooms as f64 * 0.05).min(1.8));
        self.counters.record_collaboration(collaboration_delta);
        self.counters.record_adaptation(
            (sim.epiphany_count as u64).saturating_add(sim.flow_state_entries as u64),
        );
        let innovation = (sim.receptor_blooms as f64 * 0.03 + sim.flow_state_entries as f64 * 0.02)
            .clamp(0.0, 1.0);
        self.counters.innovation_contribution =
            (self.counters.innovation_contribution * 0.8 + innovation * 0.2).clamp(0.0, 1.0);
    }

    pub fn to_transfer_telemetry(&self) -> PowrushTransferTelemetry {
        self.counters.to_transfer_telemetry()
    }

    pub fn export_json(&self) -> Result<String, String> {
        export_transfer_json(&self.label, &self.to_transfer_telemetry())
    }
}

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

pub fn export_transfer_json(label: &str, telemetry: &PowrushTransferTelemetry) -> Result<String, String> {
    let env = export_transfer_envelope(label, telemetry.clone());
    serde_json::to_string_pretty(&env).map_err(|e| e.to_string())
}

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

pub struct TelemetryPlugin;

impl Plugin for TelemetryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationTelemetry>()
            .init_resource::<TelemetryCollector>()
            .add_systems(Update, council_rbe_to_rtt_feed_system);

        info!("TelemetryPlugin — SimulationTelemetry + Council/RBE → RTT feed active");
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
        let parsed: PowrushTelemetryEnvelope = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.schema, "powrush_telemetry_v1");
    }

    #[test]
    fn collector_live_path_accumulates() {
        let mut c = TelemetryCollector::new("live_sim");
        for tick in 1..=20 {
            c.record_tick_result(tick, 1.2, 3, 1, 2, false);
        }
        let t = c.transfer_telemetry();
        assert!(t.gameplay_hours > 0.0);
        assert!(t.collaboration_events > 0);
        assert!((0.0..=1.0).contains(&t.rbe_decision_quality_avg));
        let json = c.export_transfer_json().unwrap();
        assert!(json.contains("powrush_telemetry_v1"));
        assert!(json.contains("live_sim"));
    }

    #[test]
    fn global_session_ingest_and_export() {
        let mut session = GlobalTransferSession::new("live_test");
        session.set_gameplay_hours(3.0);
        let sim = Telemetry {
            tick: 10,
            total_yield_harvested: 1.0,
            average_mercy_flow: 1.4,
            epiphany_count: 2,
            flow_state_entries: 1,
            receptor_blooms: 1,
            abundance_blooms: 2,
            stress_events: 0,
            custom_metrics: HashMap::new(),
        };
        session.ingest_sim_tick(&sim, 2);
        let json = session.export_json().unwrap();
        assert!(json.contains("powrush_telemetry_v1"));
    }
}
