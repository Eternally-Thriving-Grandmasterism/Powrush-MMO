//! simulation/src/lib.rs
//! Powrush-MMO Simulation Crate — Complete Module Wiring & Public API
//! v18.97.7 — GpuEconomicPlugin re-exported
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

// Council Proposal System (minimal viable wiring for governance E2E)
pub mod council;

// NEW Phase A foundational modules (derived from Ra-Thor)
pub mod race;

// Phase B + C + D — Volatility Lifecycle, Mutation Triggers, and Mutation Synergy Chains with Stage Progression
pub mod epigenetic_modulation;
pub mod ability_tree;

// BEGIN Cross-Race Diplomacy Mechanics (derived from Ra-Thor v15.26+)
pub mod diplomacy;

// Sub-module directories
pub mod fracture;
pub mod player_persistence;
pub mod spatial;
pub mod web;

// ============================================================================
// RE-EXPORTS — Public Simulation API
// ============================================================================

// GPU Economic Plugin & Systems (new in v18.97.7)
pub use gpu_economic::{
    GpuEconomicPlugin,
    GpuEconomicSystemSet,
    GpuEconomicReadback,
    GpuReadbackResult,
    gpu_economic_dispatch_system,
    apply_gpu_economic_results,
    gpu_economic_telemetry_system,
};

// Legacy GPU Economic items (kept for compatibility)
pub use gpu_economic::{GpuEconomicCompute, PATSAGiEconomicParams};

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

// Council Proposal System (public API for proposals, sessions, decisions)
pub use council::{CouncilProposal, CouncilSession, CouncilDecision, CouncilDecisions, ProposalType, ProposalStatus};

// Harvest & RBE Economy
pub use harvest::{HarvestEvent, HarvestSystem, ResourceNode, RbeFlowReconciliation};
pub use economy::{EconomyState, ResourceTransaction, PostScarcityAllocator};

// Persistence
pub use player_persistence::{PlayerSaveData, PersistenceManager, save_player_data, load_player_data};

// Orchestration & World
pub use orchestrator::{SimulationOrchestrator, SimulationTick, OrchestratorPlugin, TickResult};

// Spatial & Fracture
pub use fracture::{LatticeFractureSolver, FractureEvent};
pub use spatial::{SpatialGrid, SpatialQuery};

// Telemetry & Monitoring
pub use telemetry::{SimulationTelemetry, TelemetryEvent};
pub use bot_detection::{BotDetector, BotDetectionConfig};
pub use closed_beta::{ClosedBetaManager, BetaAccessLevel};

// Cloud & Mycorrhizal Sync
pub use cloud_sync::{CloudSyncManager, CloudSyncEvent};
pub use mycorrhizal_volatile_sync::{MycorrhizalSync, VolatileResource};

// PATSAGi Tunables
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

// NEW Phase D re-exports — Mutation Synergy Chains + Stage 0/1/2 + SynergyEffectEvent (for TickResult)
pub use ability_tree::{Ability, AbilityEffect, AbilityTree, AbilityState, SynergyBonus, SynergyType, SynergyEffectEvent};

// NEW — Begin Cross-Race Diplomacy Mechanics
pub use diplomacy::{DiplomacyManager, DiplomacyRelation, ActiveTreaty, TreatyType};

// ============================================================================
// NEW: Recovered Advanced Particle Effects + Lissajous Knot Reactive System (v19.1)
// ============================================================================
pub use world::{
    setup_policy_particle_effects,
    PolicyParticleEffects,
    LissajousKnotEffects,
    CurrentLissajousKnotPreset,
    SwitchLissajousKnotPreset,
    LissajousKnotPreset,
    HarmonyKnotMarker,
    PresetButton,
    CurrentPresetText,
    handle_switch_lissajous_knot_preset,
    highlight_active_preset_button,
    update_lissajous_knot_ui,
    update_active_lissajous_knot,
    debug_lissajous_knot_input,
};

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
// v18.97.7: GpuEconomicPlugin + SystemSet fully re-exported
// Thunder locked in. Yoi ⚡️
// ============================================================================