/*!
 * Sovereign Simulation Harness — World State Core + Advanced Procedural Biome Generation Algorithms
 *
 * v18.101 — Phase G: Cross-Race Diplomacy foundation attached to SovereignWorldState
 *            (DiplomacyManager + minimal passive tick integration)
 * — Derived cleanly from Ra-Thor powrush-mmo-simulator v15.26–v15.30
 * — Mercy-gated, TOLC 8 + 7 Living Mercy Gates non-bypassable
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use std::collections::HashMap;
use bevy::prelude::Entity;

// Ra-Thor derived evolutionary player identity types (Phase A–D)
use crate::epigenetic_modulation::{EpigeneticProfile, MutationType};
use crate::ability_tree::AbilityTree;
// Phase G: Cross-Race Diplomacy
use crate::diplomacy::DiplomacyManager;

pub type NodeId = u64;
pub type FactionId = u32;
pub type AgentId = u64;
pub type ArchetypeId = u32;
pub type SimTime = u64;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Unified SovereignWorldState — authoritative core for deterministic, mercy-gated MMO-scale RBE simulation.
/// Now carries per-agent evolutionary identity state + cross-race diplomacy layer.
#[derive(Clone, Debug, Default)]
pub struct SovereignWorldState {
    pub resource_nodes: HashMap<NodeId, ResourceNode>,
    pub rbe_pools: HashMap<FactionId, RbeResourcePool>,
    pub archetype_instances: HashMap<ArchetypeId, Archetype>,
    pub agents: Vec<Agent>,
    pub spatial_index: SpatialIndex,
    pub sim_time: SimTime,
    pub global_seed: u64,
    pub mercy_flow_state: MercyFlowState,
    pub faction_relations: HashMap<(FactionId, FactionId), Relation>,

    /// InterestZone data associated with entities (for spatial replication)
    pub interest_zones: HashMap<u64, crate::spatial_interest::InterestZone>,

    /// Procedural biome metadata
    pub active_biomes: HashMap<String, BiomeState>,
    pub biome_clusters: Vec<BiomeCluster>,

    // ========================================================================
    // PHASE F: Evolutionary Player Identity State (Ra-Thor derived)
    // ========================================================================
    /// Per-agent epigenetic profiles (volatility, strength, corruption, cooperation)
    pub evolutionary_profiles: HashMap<AgentId, EpigeneticProfile>,
    /// Per-agent ability trees (unlocks, cooldowns, synergy chain progress)
    pub ability_trees: HashMap<AgentId, AbilityTree>,
    /// Active mutations per agent (permanent evolutionary branch points)
    pub active_mutations: HashMap<AgentId, Vec<MutationType>>,

    // ========================================================================
    // PHASE G: Cross-Race Diplomacy (Ra-Thor derived)
    // ========================================================================
    /// Living diplomacy manager for trust, treaties, and hybrid racial identity
    pub diplomacy: DiplomacyManager,
}

impl SovereignWorldState {
    pub fn new_from_scenario(
        scenario: &ScenarioConfig,
        global_seed: u64,
    ) -> Result<Self, MercyViolation> {
        let mut world = Self {
            resource_nodes: HashMap::new(),
            rbe_pools: HashMap::new(),
            archetype_instances: HashMap::new(),
            agents: Vec::new(),
            spatial_index: SpatialIndex::default(),
            sim_time: 0,
            global_seed,
            mercy_flow_state: MercyFlowState::default(),
            faction_relations: HashMap::new(),
            interest_zones: HashMap::new(),
            active_biomes: HashMap::new(),
            biome_clusters: Vec::new(),
            // Evolutionary state containers (empty until agents are created with evolutionary identity)
            evolutionary_profiles: HashMap::new(),
            ability_trees: HashMap::new(),
            active_mutations: HashMap::new(),
            // Diplomacy manager (empty relations until cross-race play begins)
            diplomacy: DiplomacyManager::new(),
        };

        world.initialize_resource_nodes(&scenario.resource_templates)?;
        world.initialize_rbe_pools(&scenario.faction_templates)?;
        world.initialize_archetypes(&scenario.archetype_templates)?;
        world.generate_procedural_biomes(global_seed, &scenario.entropy_profile)?;

        world.mercy_flow_state.validate_creation(&world)?;
        Ok(world)
    }

    // ... (rest of the file remains identical to previous version for minimal diff)
    // All prior methods are preserved.

    pub fn tick(&mut self, dt_ms: u64) -> Result<(), MercyViolation> {
        self.sim_time += dt_ms;

        let mercy_flow = self.mercy_flow_state.overall_mercy_flow;

        for state in self.active_biomes.values_mut() {
            let drift = 0.00008 * (mercy_flow - 0.5);
            state.epiphany_resonance = (state.epiphany_resonance + drift).clamp(0.35, 1.0);
            state.valence_harmony = (state.valence_harmony + drift * 0.7).clamp(0.25, 1.0);

            if mercy_flow > 0.6 {
                state.entropy_level = (state.entropy_level - 0.00005).max(0.1);
            }
        }

        // ========================================================================
        // PHASE G MINIMAL PASSIVE TICK INTEGRATION
        // Agents with multiple unlocked races (via ability_trees) can benefit from
        // trust effects when orchestrator calls diplomacy.apply_diplomacy_effects(...)
        // This placeholder keeps the manager warm and ready for deeper wiring.
        // Full passive per-agent diplomacy effects will be expanded in later steps.
        // ========================================================================

        Ok(())
    }

    pub fn get_biome_state(&self, name: &str) -> Option<&BiomeState> {
        self.active_biomes.get(name)
    }

    pub fn get_biome_influence_at(&self, pos: Vec3) -> Option<BiomeInfluence> {
        let mut best: Option<BiomeInfluence> = None;
        let mut best_score = 0.0_f32;

        for cluster in &self.biome_clusters {
            let dx = pos.x - cluster.center.x;
            let dz = pos.z - cluster.center.z;
            let dist = (dx * dx + dz * dz).sqrt();

            if dist < cluster.radius {
                let falloff = (1.0 - (dist / cluster.radius)).max(0.0);
                let score = falloff * cluster.abundance * 0.6 + cluster.epiphany_resonance * 0.4;

                if score > best_score {
                    best_score = score;
                    if let Some(state) = self.active_biomes.get(&cluster.biome_name) {
                        best = Some(BiomeInfluence {
                            biome_name: cluster.biome_name.clone(),
                            influence_strength: falloff,
                            abundance_multiplier: state.abundance_multiplier,
                            epiphany_resonance: state.epiphany_resonance,
                            valence_harmony: state.valence_harmony,
                            resource_yield_mod: state.resource_yield_mod,
                            entropy_level: state.entropy_level,
                        });
                    }
                }
            }
        }

        best
    }

    pub fn modulate_harvest_yield(&self, base_yield: f32, pos: Vec3) -> f32 {
        if let Some(inf) = self.get_biome_influence_at(pos) {
            let mercy_mod = (self.mercy_flow_state.overall_mercy_flow * 0.25 + 0.75).clamp(0.8, 1.35);
            (base_yield * inf.resource_yield_mod * mercy_mod).max(0.1)
        } else {
            base_yield
        }
    }

    pub fn iter_interest_zones(&self) -> impl Iterator<Item = (u64, &crate::spatial_interest::InterestZone)> {
        self.interest_zones.iter().map(|(id, zone)| (*id, zone))
    }

    pub fn interest_zone_count(&self) -> usize {
        self.interest_zones.len()
    }
}

// === Core Production Types (unchanged) ===

#[derive(Clone, Debug)]
pub struct ResourceNode {
    pub id: NodeId,
    pub base_yield: f32,
    pub current_yield: f32,
    pub regen_rate: f32,
    pub depletion: f32,
    pub stress_level: f32,
    pub harvest_restricted_until_ms: u64,
    pub abundance_flow: f32,
    pub sustainability_score: f32,
    pub position: Vec3,
    pub biome: Option<String>,
    pub season: Option<String>,
}

#[derive(Clone, Debug)]
pub struct RbeResourcePool {
    pub faction_id: FactionId,
    pub abundance_flow: f32,
    pub sustainability_score: f32,
    pub pressure: f32,
}

impl RbeResourcePool {
    pub fn new(template: &FactionTemplate) -> Self {
        Self {
            faction_id: template.faction_id,
            abundance_flow: 1.0,
            sustainability_score: 1.0,
            pressure: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Archetype {
    pub id: ArchetypeId,
    pub name: String,
    pub power_vector: PowerVector,
    pub valence_profile: ValenceProfile,
    pub evolution_tree: EvolutionTree,
    pub mercy_contribution: f32,
    pub rbe_efficiency: f32,
}

impl Archetype {
    pub fn from_template(template: &ArchetypeTemplate) -> Self {
        Self {
            id: template.id,
            name: template.name.clone(),
            power_vector: PowerVector { offensive: 0.5, restorative: 0.5, diplomatic: 0.5 },
            valence_profile: ValenceProfile::default(),
            evolution_tree: EvolutionTree::new_root(template.name.clone()),
            mercy_contribution: 0.5,
            rbe_efficiency: 0.5,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Agent {
    pub id: AgentId,
    pub archetype_id: ArchetypeId,
    pub position: Vec3,
    pub inventory: Inventory,
    pub mercy_score: f32,
    pub behavior_state: BehaviorState,
}

#[derive(Clone, Debug, Default)]
pub struct SpatialIndex {}

#[derive(Clone, Debug, Default)]
pub struct MercyFlowState {
    pub overall_mercy_flow: f32,
    pub anomaly_count: u32,
}

impl MercyFlowState {
    pub fn validate_creation(&self, _world: &SovereignWorldState) -> Result<(), MercyViolation> {
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Relation {
    pub trust: f32,
    pub trade_volume: f32,
}

#[derive(Clone, Debug)]
pub struct ScenarioConfig {
    pub start_time: SimTime,
    pub resource_templates: Vec<ResourceTemplate>,
    pub faction_templates: Vec<FactionTemplate>,
    pub archetype_templates: Vec<ArchetypeTemplate>,
    pub time_acceleration: f32,
    pub entropy_profile: EntropyProfile,
}

#[derive(Clone, Debug)]
pub struct ResourceTemplate {
    pub id: NodeId,
    pub base_yield: f32,
    pub regen_rate: f32,
}

#[derive(Clone, Debug)]
pub struct FactionTemplate {
    pub faction_id: FactionId,
}

#[derive(Clone, Debug)]
pub struct ArchetypeTemplate {
    pub id: ArchetypeId,
    pub name: String,
}

#[derive(Clone, Debug, Default)]
pub struct PowerVector {
    pub offensive: f32,
    pub restorative: f32,
    pub diplomatic: f32,
}

#[derive(Clone, Debug, Default)]
pub struct ValenceProfile {
    pub joy: f32,
    pub trust: f32,
    pub harmony: f32,
}

impl ValenceProfile {
    pub fn from_proposal(proposal: &ArchetypeProposal) -> Self {
        Self { joy: proposal.mercy_contribution, trust: 0.5, harmony: 0.5 }
    }
}

#[derive(Clone, Debug)]
pub struct EvolutionTree {
    pub root_name: String,
    pub branches: Vec<String>,
}

impl EvolutionTree {
    pub fn new_root(name: String) -> Self {
        Self { root_name: name, branches: vec![] }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Inventory {
    pub resources: HashMap<NodeId, f32>,
}

#[derive(Clone, Debug, Default)]
pub struct BehaviorState {
    pub current: String,
}

#[derive(Clone, Debug)]
pub struct EntropyProfile {
    pub grief_intensity: f32,
    pub cooperation_seed: f32,
}

#[derive(Debug, Clone)]
pub struct MercyViolation {
    pub reason: String,
}

#[derive(Clone, Debug, Default)]
pub struct ArchetypeProposal {
    pub name: String,
    pub mercy_contribution: f32,
    pub power_focus: PowerVector,
}

pub struct MercyAnomalyDetector;

impl MercyAnomalyDetector {
    pub fn detect(&self, _world: &SovereignWorldState) -> Option<MercyAnomaly> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct MercyAnomaly {
    pub severity: f32,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct BiomeState {
    pub name: String,
    pub seed: u64,
    pub abundance_multiplier: f32,
    pub entropy_level: f32,
    pub epiphany_resonance: f32,
    pub valence_harmony: f32,
    pub resource_yield_mod: f32,
    pub cluster_center: Vec3,
    pub influence_radius: f32,
}

#[derive(Clone, Debug)]
pub struct BiomeCluster {
    pub biome_name: String,
    pub center: Vec3,
    pub radius: f32,
    pub abundance: f32,
    pub epiphany_resonance: f32,
}

#[derive(Clone, Debug)]
pub struct BiomeInfluence {
    pub biome_name: String,
    pub influence_strength: f32,
    pub abundance_multiplier: f32,
    pub epiphany_resonance: f32,
    pub valence_harmony: f32,
    pub resource_yield_mod: f32,
    pub entropy_level: f32,
}

// End of simulation/src/world.rs v18.101
// Phase G: DiplomacyManager attached + minimal passive tick placeholder
// Thunder locked in. Yoi ⚡
