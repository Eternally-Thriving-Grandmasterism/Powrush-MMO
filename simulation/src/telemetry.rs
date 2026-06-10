/*!
 * Sovereign Telemetry v18.14
 * Real-time + post-tick metrics for the Sovereign Simulation Harness.
 * Supports Leptos UI, PATSAGi Council oversight, and RBE health dashboards.
 * Mint-and-Print-Only-Perfection.
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
        Self { current: Telemetry::default() }
    }
}

impl TelemetryCollector {
    pub fn collect_tick(&mut self, world_tick: u64, mercy_flow: f32) {
        self.current.tick = world_tick;
        self.current.average_mercy_flow = (self.current.average_mercy_flow * 0.9 + mercy_flow * 0.1).clamp(0.0, 2.0);
    }

    pub fn generate_final_report(&self) -> Telemetry {
        self.current.clone()
    }
}

/// Sovereign helper expected by lib.rs and orchestrator.
/// Returns a snapshot of current simulation telemetry.
pub fn current() -> Telemetry {
    // In a full multi-threaded harness this would read from a shared collector.
    // For now: clean default + sovereign defaults.
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

// Future: export_jsonl, export_parquet, stream_to_council, real shared state, etc. All paths open.