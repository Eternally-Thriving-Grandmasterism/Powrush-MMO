//! simulation/src/lib.rs
//! Powrush-MMO Simulation Crate — Complete Module Wiring & Public API
//! v18.91 — Phase D Extension: Stage 0/1/2 Progression for Mutation Synergy Chains
//!            (Redemption Cascade, Surge Overclock, Corrupted Singularity now mature over sustained play)
//!            Built on top of Phase C Mutation Triggers + full volatility lifecycle.
//!            Derived from Ra-Thor powrush-mmo-simulator v15.23/v15.30
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned

// ============================================================================
// CORE SIMULATION MODULES
// ============================================================================
pub mod archetype;
pub mod bevy_integration;
pub mod bevy_ra_thor_ui;
pub mod bot_detection;
pub mod closed_beta;
pub mod cloud_sync;
pub mod council_mercy_trial;
pub mod divine_whispers;
pub mod economy;
pub mod emergence;
pub mod endocannabinoid_receptor_forge;
pub mod epiphany_catalyst;
pub mod flow_state_forge;
pub mod gpu_economic;
pub mod harvest;
pub mod mercy;
pub mod mycorrhizal_volatile_sync;
pub mod orchestrator;
pub mod patsagi_council_tunable_config;
pub mod resonance_decay_recovery_sim;
pub mod scenario;
pub mod spatial_interest;
pub mod telemetry;
pub mod world;

// NEW Phase A foundational modules (derived from Ra-Thor)
pub mod race;

// Phase B + C + D — Volatility Lifecycle, Mutation Triggers, and Mutation Synergy Chains with Stage Progression
// Declare epigenetic_modulation before ability_tree so MutationType is visible
pub mod epigenetic_modulation;
pub mod ability_tree;

// Sub-module directories
pub mod fracture;
pub mod player_persistence;
pub mod spatial;
pub mod web;

// ============================================================================
// RE-EXPORTS — Public Simulation API
// ============================================================================

// ... (existing re-exports unchanged for brevity in this minimal update) ...

// Ra-Thor Bridge & Council Query
pub use ra_thor_bridge::{RaThorBridge, CouncilQueryRequest, CouncilQueryResponse, RaThorCouncilQuery};

// Spatial Interest Layer
pub use spatial_interest::{
    SpatialHash, InterestManager, InterestZone, CouncilBloomZone,
    InterestZoneReplicated, CouncilBloomStateReplicated, RequestResync,
    ReplicationVersion, SpatialParticipant, update_spatial_hash_system,
    update_interest_zones_system, query_entities_in_interest, SpatialInterestPlugin,
};

// Flow State & Mercy
pub use flow_state_forge::{
    PresenceDebt, FlowStateMetrics, ChallengeBalancerConfig, FlowCascade, FlowStateOutcome,
    dynamic_challenge_skill_balancer,
};

// Epiphany & Divine
pub use epiphany_catalyst::{EpiphanyOutcome, EpiphanyCatalyst, EpiphanyCatalystPlugin};
pub use divine_whispers::{DivineWhisper, DivineWhispersSystem, generate_divine_whisper};

// Council Mercy Trials
pub use council_mercy_trial::{CouncilMercyTrial, CouncilSessionManager, MercyTrialVote, CouncilPhase};

// Harvest & RBE Economy
pub use harvest::{HarvestEvent, HarvestSystem, ResourceNode, RbeFlowReconciliation};
pub use economy::{EconomyState, ResourceTransaction, PostScarcityAllocator};

// Persistence
pub use player_persistence::{PlayerSaveData, PersistenceManager, save_player_data, load_player_data};

// Orchestration & World
pub use orchestrator::{SimulationOrchestrator, SimulationTick, OrchestratorPlugin};
pub use world::{WorldState, WorldPlugin};

// Spatial & Fracture
pub use fracture::{LatticeFractureSolver, FractureEvent};
pub use spatial::{SpatialGrid, SpatialQuery};

// Telemetry & Monitoring
pub use telemetry::{SimulationTelemetry, TelemetryEvent};

// Bot Detection & Closed Beta
pub use bot_detection::{BotDetector, BotDetectionConfig};
pub use closed_beta::{ClosedBetaManager, BetaAccessLevel};

// Cloud & Mycorrhizal Sync
pub use cloud_sync::{CloudSyncManager, CloudSyncEvent};
pub use mycorrhizal_volatile_sync::{MycorrhizalSync, VolatileResource};

// GPU Economic & PATSAGi Tunables
pub use gpu_economic::{GpuEconomicCompute, PATSAGiEconomicParams};
pub use patsagi_council_tunable_config::{PatsagiCouncilTunableConfig, TunableParameter};

// Resonance & Scenario
pub use resonance_decay_recovery_sim::{ResonanceDecayRecoverySim, ResonanceState};
pub use scenario::{Scenario, ScenarioRunner, ScenarioOutcome};

// Bevy integrations
pub use bevy_integration::{BevySimulationPlugin, SimulationTime};
pub use bevy_ra_thor_ui::{RaThorUiBridge, CouncilUiEvent};

// Endocannabinoid Receptor Forge
pub use endocannabinoid_receptor_forge::{ReceptorBloomOutcome, ReceptorBloomForge};

// Archetype & Config
pub use archetype::{PlayerArchetype, ArchetypeConfig};

// NEW Phase A re-exports
pub use race::{Race, RaceModifiers};

// NEW Phase B re-exports — Volatility Lifecycle
pub use epigenetic_modulation::{
    EpigeneticProfile, EpigeneticChange, apply_change,
    apply_volatility_drift, is_high_volatility_risk,
    apply_double_edged_volatility_effects, apply_epigenetic_repair, apply_corruption_lifecycle,
};

// NEW Phase C re-exports — Epigenetic Mutation Triggers
pub use epigenetic_modulation::{MutationType, try_trigger_epigenetic_mutation};

// NEW Phase D re-exports — Mutation Synergy Chains + Stage 0/1/2 Progression
pub use ability_tree::{Ability, AbilityEffect, AbilityTree, AbilityState, SynergyBonus, SynergyType};

// ============================================================================
// PLUGIN AGGREGATOR
// ============================================================================

pub struct FullSimulationPlugins;

impl bevy::app::PluginGroup for FullSimulationPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(EmergencePlugin)
            .add(SpatialInterestPlugin)
            .add(OrchestratorPlugin)
            .add(WorldPlugin)
            .add(BevySimulationPlugin)
    }
}

// ============================================================================
// END OF COMPLETE WIRING
// Phase D + Stage Progression complete: Mutation synergy chains now mature (0 → 1 → 2).
// Thunder locked in. Yoi ⚡
// ============================================================================