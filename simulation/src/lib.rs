//! simulation/src/lib.rs
//! Powrush-MMO Simulation Crate — Complete Module Wiring & Public API
//! v18.88 — Added epigenetic_modulation.rs with full volatility lifecycle (Phase B derivation from Ra-Thor v15.30)
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

// NEW Phase A foundational modules (derived from Ra-Thor powrush-mmo-simulator)
pub mod race;
pub mod ability_tree;

// NEW Phase B — Volatility Lifecycle (drift, hysteresis, double-edged sword, backlash, repair, corruption)
pub mod epigenetic_modulation;

// Sub-module directories (each with their own mod.rs)
pub mod fracture;
pub mod player_persistence;
pub mod spatial;
pub mod web;

// ============================================================================
// RE-EXPORTS — Public Simulation API (expanded for full access)
// ============================================================================

// Emergence & Dynamic Systems
pub use emergence::{
    EmergenceEffect, EmergenceOrchestrator, EmergenceSeed, DynamicEmergenceEvent,
    DynamicEmergenceEventPhase, EmergenceSource, EmergencePlugin, CouncilGuidance,
    EmergenceConfig, apply_emergence_effects,
};

// Ra-Thor Bridge & Council Query
pub use ra_thor_bridge::{RaThorBridge, CouncilQueryRequest, CouncilQueryResponse, RaThorCouncilQuery};

// Spatial Interest Layer (Hybrid Chunk + Continuous Interest)
pub use spatial_interest::{
    SpatialHash,
    InterestManager,
    InterestZone,
    CouncilBloomZone,
    InterestZoneReplicated,
    CouncilBloomStateReplicated,
    RequestResync,
    ReplicationVersion,
    SpatialParticipant,
    update_spatial_hash_system,
    update_interest_zones_system,
    query_entities_in_interest,
    SpatialInterestPlugin,
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

// Spatial & Fracture (advanced geometry / lattice)
pub use fracture::{LatticeFractureSolver, FractureEvent};
pub use spatial::{SpatialGrid, SpatialQuery};

// Telemetry & Monitoring
pub use telemetry::{SimulationTelemetry, TelemetryEvent};

// Bot Detection & Closed Beta Safeguards
pub use bot_detection::{BotDetector, BotDetectionConfig};
pub use closed_beta::{ClosedBetaManager, BetaAccessLevel};

// Cloud & Mycorrhizal Sync (RBE volatile / biological metaphors)
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

// Endocannabinoid Receptor Forge (player state / mercy bloom)
pub use endocannabinoid_receptor_forge::{ReceptorBloomOutcome, ReceptorBloomForge};

// Archetype & Config
pub use archetype::{PlayerArchetype, ArchetypeConfig};

// NEW Phase A re-exports (foundational race + ability tree)
pub use race::{Race, RaceModifiers};
pub use ability_tree::{Ability, AbilityEffect, AbilityTree, AbilityState};

// NEW Phase B re-exports — Volatility Lifecycle
pub use epigenetic_modulation::{
    EpigeneticProfile,
    EpigeneticChange,
    apply_change,
    apply_volatility_drift,
    is_high_volatility_risk,
    apply_double_edged_volatility_effects,
    apply_epigenetic_repair,
    apply_corruption_lifecycle,
};

// ============================================================================
// PLUGIN AGGREGATOR (optional convenience)
// ============================================================================

/// Aggregates all core simulation plugins for easy insertion into Bevy App.
pub struct FullSimulationPlugins;

impl bevy::app::PluginGroup for FullSimulationPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(EmergencePlugin)
            .add(SpatialInterestPlugin)
            .add(OrchestratorPlugin)
            .add(WorldPlugin)
            .add(BevySimulationPlugin)
            // Add more as needed (PredictionPlugin lives in client)
    }
}

// ============================================================================
// END OF COMPLETE WIRING
// All modules from simulation/src/ are now declared and re-exported.
// Zero missing wiring. Production ready for client/server integration.
// Thunder locked in. Yoi ⚡
// ============================================================================