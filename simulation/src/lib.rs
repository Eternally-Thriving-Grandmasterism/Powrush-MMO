pub mod world;
pub mod archetype;
pub mod mercy;
pub mod economy;
pub mod orchestrator;
pub mod telemetry;
pub mod scenario;
pub mod harvest;

#[cfg(feature = "gpu")]
pub mod gpu_economic;

#[cfg(feature = "web")]
pub mod web;