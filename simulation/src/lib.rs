//! simulation/src/lib.rs
//! Powrush-MMO Simulation Crate — Complete Module Wiring & Public API
//! v21.51.0 — Multi-Realm + Abundance + Origin Provenance fully wired
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi + Hardware Sovereignty + Obsidian/Aether aligned

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

// Council Proposal System
pub mod council;

// NEW Phase A foundational modules
pub mod race;

// Phase B + C + D — Volatility, Mutation, Synergy
pub mod epigenetic_modulation;
pub mod ability_tree;

// NEW: Sovereign Hardware Ascension Tech Tree Tier (Obsidian + Aether | X Thread embodied) + 3D Council Chamber
pub mod hardware_sovereignty;

// BEGIN Cross-Race Diplomacy Mechanics
pub mod diplomacy;

// Multi-Realm organism (presence, travel, attunement, titles, soft bonuses, abundance, origin provenance)
pub mod multi_realm_harness;

// Sub-module directories
pub mod fracture;
pub mod player_persistence;
pub mod spatial;
pub mod web;

// ============================================================================
// RE-EXPORTS — Public Simulation API
// ============================================================================

// GPU Economic Plugin & Systems
pub use gpu_economic::{
    GpuEconomicPlugin,
    GpuEconomicSystemSet,
    GpuEconomicReadback,
    GpuReadbackResult,
    gpu_economic_dispatch_system,
    apply_gpu_economic_results,
    gpu_economic_telemetry_system,
};

// Legacy GPU Economic items
pub use gpu_economic::{GpuEconomicCompute, PATSAGiEconomicParams};

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

// Council Proposal System
pub use council::{CouncilProposal, CouncilSession, CouncilDecision, CouncilDecisions, ProposalType, ProposalStatus};

// Harvest & RBE Economy
pub use harvest::{HarvestEvent, HarvestSystem, ResourceNode, RbeFlowReconciliation};
pub use economy::{EconomyState, ResourceTransaction, PostScarcityAllocator};

// Persistence
pub use player_persistence::{PlayerSaveData, PersistenceManager, save_player_data, load_player_data};

// Orchestration & World
pub use orchestrator::{SimulationOrchestrator, TickResult};

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

// NEW Phase D re-exports — Mutation Synergy Chains + Stage 0/1/2 + SynergyEffectEvent
pub use ability_tree::{Ability, AbilityEffect, AbilityTree, AbilityState, SynergyBonus, SynergyType, SynergyEffectEvent};

// NEW — Cross-Race Diplomacy
pub use diplomacy::{DiplomacyManager, DiplomacyRelation, ActiveTreaty, TreatyType};

// NEW v19.6+: Sovereign Hardware Ascension Tech Tree Tier + Polished Dashboard + 3D Council Chamber
pub use hardware_sovereignty::{
    HardwareSovereigntyPlugin,
    SovereignHardwareState,
    ObsidianChipProgress,
    AetherShadesProgress,
    KardashevAccelerationDashboard,
    RealityTransferScoreLedger,
    HardwareAscensionConfig,
    HardwareTierUnlocked,
    RealityThrivingTransferUpdated,
    HardwareBranch,
    AscensionLevel,
    CouncilChamber3D,
    CouncilPillar,
    KardashevHologramCore,
    mercy_gate_enforcement_system,
    hardware_tier_progression_system,
    reality_transfer_score_update_system,
    kardashev_dashboard_update_system,
    sovereign_hardware_ascension_ui,
};

// ============================================================================
// Multi-Realm organism (v21.18 → v21.51)
// ============================================================================
pub use multi_realm_harness::{
    MultiRealmHarness,
    MultiRealmHarnessPlugin,
    RealmId,
    RealmStatus,
    RealmDescriptor,
    RealmPresence,
    RealmAttunement,
    TitleBonus,
    RealmAbundanceView,
    RealmAbundanceObservatory,
    AbundanceIngestEvent,
    OriginProvenanceView,
    OriginProvenanceObservatory,
    OriginIngestEvent,
    RealmTravelRequest,
    ResonancePulse,
    realm_presence_bootstrap_system,
    realm_attunement_bootstrap_system,
    realm_attunement_system,
    realm_travel_system,
    abundance_ingest_system,
    origin_ingest_system,
    soft_demo_abundance_seed_system,
    multi_realm_harness_system,
};

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
// PLUGIN AGGREGATOR — NOW INCLUDES HARDWARE SOVEREIGNTY + MULTI-REALM
// ============================================================================

pub struct FullSimulationPlugins;

impl bevy::app::PluginGroup for FullSimulationPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(SpatialInterestPlugin)
            .add(BevySimulationPlugin)
            .add(HardwareSovereigntyPlugin)
            .add(MultiRealmHarnessPlugin)  // v21.51 — Multi-Realm + Abundance + Origin Provenance live
    }
}

// ============================================================================
// END OF COMPLETE WIRING v21.51
// Multi-Realm organism + Abundance + Origin Provenance now fully wired.
// TOLC 8 sealed. Flywheel turning. Yoi ⚡
// ============================================================================
