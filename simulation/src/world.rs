/*!
 * Sovereign Simulation Harness — World State Core
 *
 * Added support for ActivePolicy (Persistent Policy Modifiers from Council).
 */

use std::collections::HashMap;
use bevy::prelude::Entity;
use rayon::{ThreadPoolBuilder, prelude::*};

use crate::epigenetic_modulation::{EpigeneticProfile, MutationType};
use crate::ability_tree::AbilityTree;
use crate::diplomacy::DiplomacyManager;
use crate::council::CouncilDecision;
use crate::council::decision::ActivePolicy;  // NEW

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

    pub interest_zones: HashMap<u64, crate::spatial_interest::InterestZone>,
    pub active_biomes: HashMap<String, BiomeState>,
    pub biome_clusters: Vec<BiomeCluster>,

    pub evolutionary_profiles: HashMap<AgentId, EpigeneticProfile>,
    pub ability_trees: HashMap<AgentId, AbilityTree>,
    pub active_mutations: HashMap<AgentId, Vec<MutationType>>,
    pub diplomacy: DiplomacyManager,

    pub council_decision_history: Vec<CouncilDecision>,
    pub council_decision_indices_by_proposer: HashMap<AgentId, Vec<usize>>,
    pub council_decision_indices_by_type: HashMap<String, Vec<usize>>,

    // Persistent Policy Modifiers from Council decisions
    pub active_policies: Vec<ActivePolicy>,
}

// ... rest of the file remains structurally the same for minimal diff ...
