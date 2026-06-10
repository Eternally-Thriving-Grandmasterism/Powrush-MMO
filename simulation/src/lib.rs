/*!
 * Sovereign Simulation Harness (SSH) for Powrush-MMO
 * 
 * Eternal RBE Validation & Refinement Engine
 * TOLC 8 Mercy Gates as non-bypassable Layer 0
 * 
 * This crate provides a headless, deterministic, time-accelerated simulation
 * framework capable of MMO-scale (100–50,000+ agents) RBE experimentation.
 * 
 * All major transitions pass through Mercy Gates.
 * All valuable prior logic from game/ and engine/ is preserved and elevated.
 */

pub mod world;
pub mod archetype;
pub mod mercy;
pub mod economy;
pub mod orchestrator;
pub mod telemetry;
pub mod scenario;
pub mod harvest;

// Re-exports for convenience
pub use world::SovereignWorldState;
pub use archetype::{Archetype, SovereignArchetypeSystem};
pub use mercy::{MercyGate, PATSAGiCouncilSim};
pub use economy::EconomicLayer;
pub use orchestrator::SovereignSimulationOrchestrator;
pub use telemetry::TelemetryCollector;
pub use scenario::{ScenarioConfig, ScenarioPreset};
pub use harvest::HarvestingSystem;
