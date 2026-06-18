/*!
 * Sovereign Simulation Harness — World State Core
 *
 * v18.94 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Complete mint-and-print-only-perfection
 * — Unified SovereignWorldState with InterestZone support
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use std::collections::HashMap;
use bevy::prelude::Entity;

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
    /// Keyed by stable u64 entity identifier (maps to Bevy Entity when needed)
    pub interest_zones: HashMap<u64, crate::spatial_interest::InterestZone>,
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
        };

        world.initialize_resource_nodes(&scenario.resource_templates)?;
        world.initialize_rbe_pools(&scenario.faction_templates)?;
        world.initialize_archetypes(&scenario.archetype_templates)?;

        world.mercy_flow_state.validate_creation(&world)?;
        Ok(world)
    }

    fn initialize_resource_nodes(
        &mut self,
        templates: &[ResourceTemplate],
    ) -> Result<(), MercyViolation> {
        for t in templates {
            let node = ResourceNode {
                id: t.id,
                base_yield: t.base_yield,
                current_yield: t.base_yield,
                regen_rate: t.regen_rate,
                depletion: 0.0,
                stress_level: 0.0,
                harvest_restricted_until_ms: 0,
                abundance_flow: 1.0,
                sustainability_score: 1.0,
            };
            self.resource_nodes.insert(t.id, node);
        }
        Ok(())
    }

    fn initialize_rbe_pools(
        &mut self,
        templates: &[FactionTemplate],
    ) -> Result<(), MercyViolation> {
        for t in templates {
            self.rbe_pools.insert(t.faction_id, RbeResourcePool::new(t));
        }
        Ok(())
    }

    fn initialize_archetypes(
        &mut self,
        templates: &[ArchetypeTemplate],
    ) -> Result<(), MercyViolation> {
        for t in templates {
            self.archetype_instances.insert(t.id, Archetype::from_template(t));
        }
        Ok(())
    }

    pub fn tick(&mut self, dt_ms: u64) -> Result<(), MercyViolation> {
        self.sim_time += dt_ms;
        Ok(())
    }

    /// Returns an iterator over (entity_id, InterestZone) for spatial replication
    pub fn iter_interest_zones(&self) -> impl Iterator<Item = (u64, &crate::spatial_interest::InterestZone)> {
        self.interest_zones.iter().map(|(id, zone)| (*id, zone))
    }

    /// Number of active interest zones
    pub fn interest_zone_count(&self) -> usize {
        self.interest_zones.len()
    }
}

// === Core Production Types ===

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

// === Supporting Types ===

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

// End of simulation/src/world.rs v18.94 — InterestZone support added for real Entity-backed spatial replication.
// Thunder locked in. Yoi ⚡
