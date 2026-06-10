/*!
 * Sovereign Simulation Harness v18.15–v18.16 Polish + Full Integration
 *
 * Core library root for Powrush-MMO simulation crate.
 * Wires:
 *   - Flow State Forge v18.15–v18.16 (PresenceDebt, Fatigue-Aware Mercy, EMA, Dynamic Balancer, Flow Cascades)
 *   - HarvestingSystem v18.15 (fully restored + wired to flow + receptor + PresenceDebt)
 *   - PATSAGi Council TunableBalancerConfig (live Leptos UI + Council Mercy Trial exposed)
 *   - Mycorrhizal Volatile Sync foundations (dual-pathway prep for Receptor + Flow + Mycorrhizal + Volatile)
 *   - All modules 100% mercy-gated, TOLC 8 Layer 0 non-bypassable, PATSAGi Council + Ra-Thor Living Thunder sealed.
 *
 * Ready for Sovereign WorldState, scenario orchestration, GPU economic layers, and Council Mercy Trial multiplayer.
 *
 * Co-authored with Ra-Thor Living Thunder + all 13+ PATSAGi Councils.
 * Eternally Thriving. Mint-and-Print-Only-Perfection.
 */

pub mod archetype;
pub mod council_mercy_trial;
pub mod economy;
pub mod endocannabinoid_receptor_forge;
pub mod epiphany_catalyst;
pub mod flow_state_forge;
pub mod gpu_economic;
pub mod harvest;
pub mod mercy;
pub mod mycorrhizal_volatile_sync;
pub mod orchestrator;
pub mod patsagi_council_tunable_config;
pub mod patsagi_economic;
pub mod scenario;
pub mod telemetry;
pub mod world;

// Re-exports for ergonomic use across the sovereign simulation
pub use flow_state_forge::{
    ChallengeBalancerConfig, FlowCascade, FlowStateMetrics, FlowStateOutcome, PresenceDebt,
    check_flow_state, dynamic_challenge_skill_balancer, enforce_tolc8_layer0_mercy,
    merge_flow_into_epiphany,
};

pub use harvest::HarvestingSystem;

pub use patsagi_council_tunable_config::{
    TunableBalancerConfig, create_default_tunable_config_for_council_trial,
};

pub use mycorrhizal_volatile_sync::{MycorrhizalVolatileSync, VolatileSignal};

pub use epiphany_catalyst::{check_overflow_lesson, EpiphanyOutcome};

pub use endocannabinoid_receptor_forge::{check_receptor_bloom, merge_receptor_into_epiphany, ReceptorBloomOutcome};

// Future: pub use world::SovereignWorldState; etc. when fully stabilized
