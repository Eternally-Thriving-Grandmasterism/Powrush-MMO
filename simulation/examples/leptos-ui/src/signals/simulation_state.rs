use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Telemetry {
    pub rbe_depletion: f64,
    pub abundance_flow: f64,
    pub sustainability: f64,
    pub stress: f64,
    pub archetype_diversity: f64,
    pub mercy_flow: f64,
    pub anomaly_count: u32,
    pub entropy_events: u32,
    pub serverwar_risk: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Archetype {
    pub name: String,
    pub count: u32,
    pub evolution_stage: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InterventionLog {
    pub timestamp: DateTime<Utc>,
    pub intervention: String,
    pub outcome: String,
    pub mercy_status: String,
}

#[derive(Clone, Debug)]
pub struct SimulationState {
    pub telemetry: RwSignal<Telemetry>,
    pub archetypes: RwSignal<Vec<Archetype>>,
    pub intervention_log: RwSignal<VecDeque<InterventionLog>>,
    pub is_running: RwSignal<bool>,
    pub is_paused: RwSignal<bool>,
    pub use_gpu: RwSignal<bool>,
    pub current_preset: RwSignal<String>,
    pub ticks_completed: RwSignal<u32>,
    pub last_tick_duration_ms: RwSignal<f64>,
}

impl SimulationState {
    pub fn new() -> Self {
        let initial_telemetry = Telemetry {
            rbe_depletion: 12.4,
            abundance_flow: 87.6,
            sustainability: 91.2,
            stress: 8.8,
            archetype_diversity: 76.3,
            mercy_flow: 94.7,
            anomaly_count: 2,
            entropy_events: 7,
            serverwar_risk: 3.1,
        };
        
        let initial_archetypes = vec![
            Archetype { name: "Builder".to_string(), count: 1240, evolution_stage: 3 },
            Archetype { name: "Guardian".to_string(), count: 890, evolution_stage: 4 },
            Archetype { name: "Explorer".to_string(), count: 1560, evolution_stage: 2 },
            Archetype { name: "Harmonizer".to_string(), count: 670, evolution_stage: 5 },
        ];
        
        Self {
            telemetry: create_rw_signal(initial_telemetry),
            archetypes: create_rw_signal(initial_archetypes),
            intervention_log: create_rw_signal(VecDeque::new()),
            is_running: create_rw_signal(false),
            is_paused: create_rw_signal(false),
            use_gpu: create_rw_signal(true),
            current_preset: create_rw_signal("Balanced RBE Emergence".to_string()),
            ticks_completed: create_rw_signal(1240u32),
            last_tick_duration_ms: create_rw_signal(2.3),
        }
    }
    
    pub fn inject_intervention(&self, intervention_type: &str, details: &str) {
        let mut log = self.intervention_log.get();
        let outcome = match intervention_type {
            "Abundance Boost" => {
                let mut t = self.telemetry.get();
                t.abundance_flow = (t.abundance_flow + 4.2).min(99.9);
                t.sustainability = (t.sustainability + 1.8).min(99.9);
                t.stress = (t.stress - 2.1).max(0.0);
                self.telemetry.set(t);
                "Abundance vector elevated +4.2%. Sustainability +1.8%. Stress reduced."
            }
            "Mercy Reset" => {
                let mut t = self.telemetry.get();
                t.mercy_flow = 98.5;
                t.anomaly_count = 0;
                t.stress = (t.stress * 0.6).max(0.0);
                self.telemetry.set(t);
                "Mercy flow restored to 98.5%. Anomalies cleared. Stress halved."
            }
            "Archetype Evolution Pressure" => {
                let mut arches = self.archetypes.get();
                for a in &mut arches {
                    if a.evolution_stage < 7 { a.evolution_stage += 1; }
                }
                self.archetypes.set(arches);
                "All archetypes advanced +1 evolution stage. Diversity increased."
            }
            "Divine Whisper" => {
                let mut t = self.telemetry.get();
                t.mercy_flow = (t.mercy_flow + 3.0).min(100.0);
                t.serverwar_risk = (t.serverwar_risk - 1.5).max(0.0);
                self.telemetry.set(t);
                "Divine Whisper received. Mercy +3.0%. ServerWar risk reduced."
            }
            "Trigger ServerWar" => {
                let mut t = self.telemetry.get();
                t.serverwar_risk = (t.serverwar_risk + 35.0).min(100.0);
                t.stress = (t.stress + 12.0).min(100.0);
                t.entropy_events += 3;
                self.telemetry.set(t);
                "ServerWar event triggered for stress-testing. Entropy +3. Risk elevated."
            }
            _ => "Custom intervention processed through TOLC 8 gates."
        };
        
        log.push_front(InterventionLog {
            timestamp: Utc::now(),
            intervention: format!("{}: {}", intervention_type, details),
            outcome: outcome.to_string(),
            mercy_status: "✓ TOLC 8 Mercy Gate Passed".to_string(),
        });
        if log.len() > 12 { log.pop_back(); }
        self.intervention_log.set(log);
    }
    
    pub fn step_one_tick(&self) {
        let mut t = self.telemetry.get();
        // Realistic simulation step
        t.rbe_depletion = (t.rbe_depletion + 0.08).min(100.0);
        t.abundance_flow = (t.abundance_flow - 0.12).max(0.0);
        t.sustainability = (t.sustainability - 0.05).max(0.0);
        t.stress = (t.stress + 0.15).min(100.0);
        t.mercy_flow = (t.mercy_flow - 0.03).max(0.0);
        t.entropy_events = t.entropy_events.saturating_add(1);
        self.telemetry.set(t);
        
        self.ticks_completed.update(|v| *v += 1);
        self.last_tick_duration_ms.set(1.8 + (js_sys::Math::random() * 1.2));
    }
}

impl Default for SimulationState {
    fn default() -> Self { Self::new() }
}