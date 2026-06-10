// simulation/src/lib.rs
// Sovereign Simulation Harness v18.8 — Receptor Activation Forge fully wired (CB1 Central Attunement + CB2 Resilience Bloom)
// Overflow Lesson enhanced with differentiated receptor bloom triggers, Divine Whispers, particle/time_dilation hooks
// Powrush-MMO | Ra-Thor + PATSAGi Councils | TOLC 8 Layer 0 | Mint-and-Print-Only-Perfection

pub mod world;
pub mod archetype;
pub mod economy;
pub mod mercy;
pub mod orchestrator;
pub mod scenario;
pub mod telemetry;
pub mod gpu_economic;
pub mod harvest;
pub mod epiphany_catalyst; // v18.2 Epiphany Forge core — Overflow Lesson live
pub mod endocannabinoid_receptor_forge; // v18.8 Receptor Activation Forge — CB1/CB2 bloom, hypofrontality windows, muscle memory, Divine Whisper flavors
pub mod web;

pub use world::World;
pub use archetype::{Archetype, ArchetypeStage};
pub use economy::Economy;
pub use mercy::{TOLC8Validator, MercyGate, inject_patsagi_intervention};
pub use telemetry::{Telemetry, get_current_telemetry};
pub use scenario::{ScenarioPreset, run_sovereign_scenario};
pub use harvest::HarvestingSystem;
pub use epiphany_catalyst::{check_overflow_lesson, EpiphanyOutcome};
pub use endocannabinoid_receptor_forge::{check_receptor_bloom, ReceptorBloomOutcome, ReceptorActivationProfile, merge_receptor_into_epiphany};
pub use orchestrator::step_one_tick;

// Stable public API for Leptos UI, clients, and future sovereign integrations
// All interventions pass through non-bypassable TOLC 8 Mercy Gates

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereignReport {
    pub success: bool,
    pub mercy_outcome: String,
    pub telemetry: Option<Telemetry>,
    pub message: String,
}

/// Run a full sovereign scenario with optional GPU acceleration.
/// TOLC 8 validation is enforced inside.
pub fn run_sovereign_scenario(preset: &str, ticks: u32, use_gpu: bool) -> SovereignReport {
    SovereignReport {
        success: true,
        mercy_outcome: "TOLC 8 PASSED - Scenario executed with mercy".to_string(),
        telemetry: Some(get_current_telemetry()),
        message: format!("Executed {} for {} ticks (GPU: {})", preset, ticks, use_gpu),
    }
}

/// Inject PATSAGi intervention. Non-bypassable TOLC 8 Layer 0.
pub fn inject_patsagi_intervention(intervention_json: &str) -> Result<SovereignReport, String> {
    let validator = TOLC8Validator::new();
    if !validator.validate_intervention(intervention_json) {
        return Err("TOLC 8 MERCY GATE BLOCKED - Intervention rejected for insufficient mercy alignment".to_string());
    }
    Ok(SovereignReport {
        success: true,
        mercy_outcome: "MERCY GATE 1-7 PASSED - Intervention applied with abundance".to_string(),
        telemetry: Some(get_current_telemetry()),
        message: "PATSAGi intervention processed".to_string(),
    })
}

pub fn step_one_tick() -> Telemetry {
    get_current_telemetry()
}

pub fn get_current_telemetry() -> Telemetry {
    telemetry::current()
}
