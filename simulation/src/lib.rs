//! simulation/src/lib.rs
//! Powrush-MMO Simulation Crate — Complete Module Wiring & Public API
//! v21.66.0 — Multi-Realm sealed + RBE Sustainability Snapshot
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned

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
pub mod external_bridge;
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

pub mod council;
pub mod race;
pub mod epigenetic_modulation;
pub mod ability_tree;
pub mod hardware_sovereignty;
pub mod diplomacy;
pub mod multi_realm_harness;

pub mod fracture;
pub mod player_persistence;
pub mod spatial;
pub mod web;

pub use gpu_economic::{
    GpuEconomicPlugin, GpuEconomicSystemSet, GpuEconomicReadback, GpuReadbackResult,
    gpu_economic_dispatch_system, apply_gpu_economic_results, gpu_economic_telemetry_system,
};
pub use gpu_economic::{GpuEconomicCompute, PATSAGiEconomicParams};

pub use spatial_interest::{
    SpatialHash, InterestManager, InterestZone, CouncilBloomZone,
    InterestZoneReplicated, CouncilBloomStateReplicated, RequestResync,
    ReplicationVersion, SpatialParticipant, update_spatial_hash_system,
    update_interest_zones_system, query_entities_in_interest, SpatialInterestPlugin,
};

pub use flow_state_forge::{
    PresenceDebt, FlowStateMetrics, ChallengeBalancerConfig, FlowCascade, FlowStateOutcome,
    dynamic_challenge_skill_balancer,
};

pub use epiphany_catalyst::{EpiphanyOutcome, EpiphanyCatalyst, EpiphanyCatalystPlugin};
pub use divine_whispers::{DivineWhisper, DivineWhispersSystem, generate_divine_whisper};
pub use council_mercy_trial::{CouncilMercyTrial, CouncilSessionManager, MercyTrialVote, CouncilPhase};
pub use council::{CouncilProposal, CouncilSession, CouncilDecision, CouncilDecisions, ProposalType, ProposalStatus};
pub use harvest::{HarvestEvent, HarvestSystem, ResourceNode, RbeFlowReconciliation};
pub use economy::{
    EconomicLayer, EconomyState, ResourceTransaction, PostScarcityAllocator,
    MultiRealmRbeSnapshot, EconomyPlugin, multi_realm_rbe_snapshot_system,
};
pub use player_persistence::{PlayerSaveData, PersistenceManager, save_player_data, load_player_data};
pub use orchestrator::{SimulationOrchestrator, TickResult};
pub use fracture::{LatticeFractureSolver, FractureEvent};
pub use spatial::{SpatialGrid, SpatialQuery};
pub use telemetry::{SimulationTelemetry, TelemetryEvent};
pub use bot_detection::{BotDetector, BotDetectionConfig};
pub use closed_beta::{ClosedBetaManager, BetaAccessLevel};
pub use cloud_sync::{CloudSyncManager, CloudSyncEvent};
pub use mycorrhizal_volatile_sync::{MycorrhizalSync, VolatileResource};
pub use patsagi_council_tunable_config::{PatsagiCouncilTunableConfig, TunableParameter};
pub use resonance_decay_recovery_sim::{ResonanceDecayRecoverySim, ResonanceState};
pub use scenario::{Scenario, ScenarioRunner, ScenarioOutcome};
pub use bevy_integration::{BevySimulationPlugin, SimulationTime};
pub use bevy_ra_thor_ui::{RaThorUiBridge, CouncilUiEvent};
pub use endocannabinoid_receptor_forge::{ReceptorBloomOutcome, ReceptorBloomForge};
pub use archetype::{PlayerArchetype, ArchetypeConfig};
pub use race::{Race, RaceModifiers};
pub use epigenetic_modulation::{
    EpigeneticProfile, EpigeneticChange, apply_change,
    apply_volatility_drift, is_high_volatility_risk,
    apply_double_edged_volatility_effects, apply_epigenetic_repair, apply_corruption_lifecycle,
    MutationType, try_trigger_epigenetic_mutation,
};
pub use ability_tree::{Ability, AbilityEffect, AbilityTree, AbilityState, SynergyBonus, SynergyType, SynergyEffectEvent};
pub use diplomacy::{DiplomacyManager, DiplomacyRelation, ActiveTreaty, TreatyType};

pub use hardware_sovereignty::{
    HardwareSovereigntyPlugin, SovereignHardwareState, ObsidianChipProgress, AetherShadesProgress,
    KardashevAccelerationDashboard, RealityTransferScoreLedger, HardwareAscensionConfig,
    HardwareTierUnlocked, RealityThrivingTransferUpdated, HardwareBranch, AscensionLevel,
    CouncilChamber3D, CouncilPillar, KardashevHologramCore,
    mercy_gate_enforcement_system, hardware_tier_progression_system,
    reality_transfer_score_update_system, kardashev_dashboard_update_system,
    sovereign_hardware_ascension_ui,
};

pub use multi_realm_harness::{
    MultiRealmHarness, MultiRealmHarnessPlugin,
    RealmId, RealmStatus, RealmDescriptor,
    RealmPresence, RealmAttunement, TitleBonus,
    RealmAbundanceView, RealmAbundanceObservatory, AbundanceIngestEvent,
    OriginProvenanceView, OriginProvenanceObservatory, OriginIngestEvent,
    RealmTravelRequest, ResonancePulse,
    derive_abundance_from_harness, derive_origin_from_harness,
    origin_affinity_mult, origin_affinity_label,
    realm_presence_bootstrap_system, realm_attunement_bootstrap_system,
    realm_attunement_system, realm_travel_system,
    abundance_ingest_system, origin_ingest_system,
    soft_demo_abundance_seed_system, harness_derived_live_ingest_system,
    multi_realm_harness_system,
};

pub use external_bridge::{
    ExternalBridgeInbox, ExternalBridgePlugin,
    SharedAppBridgeSource, HostBridgeAutoPublish,
    emit_abundance_from_tuples, emit_origin_from_tuples,
    external_bridge_drain_system, shared_app_bridge_publish_system,
    host_bridge_auto_publish_system,
};

pub use world::{
    setup_policy_particle_effects, PolicyParticleEffects,
    LissajousKnotEffects, CurrentLissajousKnotPreset, SwitchLissajousKnotPreset,
    LissajousKnotPreset, HarmonyKnotMarker, PresetButton, CurrentPresetText,
    handle_switch_lissajous_knot_preset, highlight_active_preset_button,
    update_lissajous_knot_ui, update_active_lissajous_knot, debug_lissajous_knot_input,
};

pub struct FullSimulationPlugins;

impl bevy::app::PluginGroup for FullSimulationPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(SpatialInterestPlugin)
            .add(BevySimulationPlugin)
            .add(HardwareSovereigntyPlugin)
            .add(MultiRealmHarnessPlugin)
            .add(ExternalBridgePlugin)
            .add(EconomyPlugin)
    }
}

// END OF COMPLETE WIRING v21.66 — EconomyPlugin + MultiRealmRbeSnapshot live.
// TOLC 8 sealed. Yoi ⚡
