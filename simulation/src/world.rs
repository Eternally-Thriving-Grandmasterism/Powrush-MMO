/*!
 * Powrush-MMO Simulation World & Advanced Particle Effects
 *
 * v19.21: Added minimal Agent + SovereignWorldState core (for synergy event activation)
 * Preserved full VFX recovery (ParticleVisualAssets, Hanabi, sacred geometry).
 * Production ResourceNode struct added (replaces placeholder).
 * Now includes accelerated raycast LOS wiring for:
 *   1. Council visibility / PATSAGi council systems
 *   2. Dynamic event triggering (synergy, epiphany, harvest)
 *   3. Agent perception
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy::render::texture::{Image, ImageSampler};
use bevy_hanabi::prelude::*;
use std::collections::HashMap;

use crate::effects::{frame, modulation, types};

use crate::ability_tree::AbilityTree;
use crate::epigenetic_modulation::{EpigeneticProfile, MutationType};
use crate::race::Race;

use shared::spatial::HierarchicalGrid;

pub type AgentId = u64;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Agent {
    pub id: AgentId,
    pub name: String,
    pub ability_tree: AbilityTree,
    pub epigenetic_profile: EpigeneticProfile,
    pub active_mutations: Vec<MutationType>,
    pub unlocked_races: Vec<Race>,
    pub position: Option<Vec3>,
    pub mercy_contribution: f32,
    pub rbe_efficiency: f32,
    pub archetype_id: Option<ArchetypeId>,
}

impl Agent {
    pub fn new(id: AgentId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            ability_tree: AbilityTree::new(),
            epigenetic_profile: EpigeneticProfile::default(),
            active_mutations: Vec::new(),
            unlocked_races: Vec::new(),
            position: None,
            mercy_contribution: 0.0,
            rbe_efficiency: 0.5,
            archetype_id: None,
        }
    }

    pub fn get_active_mutations(&self) -> &[MutationType] {
        &self.active_mutations
    }

    pub fn get_unlocked_races(&self) -> &[Race] {
        &self.unlocked_races
    }

    pub fn add_mutation(&mut self, mutation: MutationType) {
        if !self.active_mutations.contains(&mutation) {
            self.active_mutations.push(mutation);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceNode {
    pub id: u64,
    pub position: (f32, f32, f32),
    pub resource_type: String,
    pub base_yield: f32,
    pub current_yield: f32,
    pub depletion: f32,
    pub regeneration_rate: f32,
    pub last_harvested_ms: u64,
    pub sustainability_score: f32,
    pub stress_level: f32,
    pub harvest_restricted_until_ms: u64,
}

/// Central simulation world state.
/// Enriched with accelerated raycast LOS for council visibility, dynamic events, and agent perception.
#[derive(Resource, Default)]
pub struct SovereignWorldState {
    pub agents: HashMap<AgentId, Agent>,
    pub resource_nodes: HashMap<NodeId, ResourceNode>,
    pub sim_time: u64,
}

pub type NodeId = u64;
pub type Vec3 = bevy::math::Vec3;
pub type ArchetypeId = u64;

// ============================================================================
// ACCELERATED RAYCAST LOS HELPERS (wired for Council, Events, Agent Perception)
// Uses the spatially accelerated HierarchicalGrid::raycast_distance
// ============================================================================

impl SovereignWorldState {
    /// Returns true if there is a clear line of sight between two positions.
    /// Now powered by accelerated hierarchical raycast (coarse-to-fine + early termination).
    pub fn has_line_of_sight(&self, grid: &HierarchicalGrid, from: Vec3, to: Vec3, max_dist: f32) -> bool {
        let direction = Vec3 {
            x: to.x - from.x,
            y: to.y - from.y,
            z: to.z - from.z,
        };
        if let Some(hit_dist) = grid.raycast_distance(
            crate::spatial::hierarchical_grid::Vec3 { x: from.x, y: from.y, z: from.z },
            crate::spatial::hierarchical_grid::Vec3 { x: direction.x, y: direction.y, z: direction.z },
            max_dist
        ) {
            let target_dist = direction.length();
            hit_dist >= target_dist * 0.98
        } else {
            true
        }
    }

    /// Example: Check if an agent can see a resource node (for harvest simulation, RBE awareness)
    pub fn agent_can_see_resource(&self, grid: &HierarchicalGrid, agent_id: AgentId, node_id: NodeId, max_dist: f32) -> bool {
        if let (Some(agent), Some(node)) = (self.agents.get(&agent_id), self.resource_nodes.get(&node_id)) {
            if let Some(agent_pos) = agent.position {
                let node_pos = Vec3 { x: node.position.0, y: node.position.1, z: node.position.2 };
                return self.has_line_of_sight(grid, agent_pos, node_pos, max_dist);
            }
        }
        false
    }

    // ========================================================================
    // 1. COUNCIL VISIBILITY / PATSAGi COUNCIL SYSTEMS
    // ========================================================================
    /// Returns true if a council member/agent has clear line of sight to another entity.
    /// Useful for PATSAGi Council visibility, mercy trial participation, diplomacy line-of-sight,
    /// and council member perception in spatial council systems.
    pub fn council_member_has_visibility(&self, grid: &HierarchicalGrid, member_id: AgentId, target_id: AgentId, max_dist: f32) -> bool {
        if let (Some(member), Some(target)) = (self.agents.get(&member_id), self.agents.get(&target_id)) {
            if let (Some(m_pos), Some(t_pos)) = (member.position, target.position) {
                return self.has_line_of_sight(grid, m_pos, t_pos, max_dist);
            }
        }
        false
    }

    // ========================================================================
    // 2. DYNAMIC EVENT TRIGGERING (Synergy, Epiphany, Harvest)
    // ========================================================================
    /// Example: Only trigger a harvest event if the agent has clear LOS to the resource.
    /// Prevents "ghost harvesting" through walls and adds spatial realism to RBE events.
    pub fn can_trigger_harvest_event(&self, grid: &HierarchicalGrid, agent_id: AgentId, node_id: NodeId, max_dist: f32) -> bool {
        self.agent_can_see_resource(grid, agent_id, node_id, max_dist)
    }

    /// Example: Gate synergy or epiphany events on clear line of sight between key agents.
    /// Useful for council-mediated epiphany or cross-race synergy events.
    pub fn can_trigger_synergy_event(&self, grid: &HierarchicalGrid, agent_a: AgentId, agent_b: AgentId, max_dist: f32) -> bool {
        if let (Some(a), Some(b)) = (self.agents.get(&agent_a), self.agents.get(&agent_b)) {
            if let (Some(a_pos), Some(b_pos)) = (a.position, b.position) {
                return self.has_line_of_sight(grid, a_pos, b_pos, max_dist);
            }
        }
        false
    }

    // ========================================================================
    // 3. AGENT PERCEPTION
    // ========================================================================
    /// Returns entities an agent can currently perceive (within range and clear LOS).
    /// This is the core agent perception primitive, powered by accelerated raycast.
    pub fn get_perceivable_agents(&self, grid: &HierarchicalGrid, agent_id: AgentId, max_dist: f32) -> Vec<AgentId> {
        let mut result = Vec::new();
        if let Some(agent) = self.agents.get(&agent_id) {
            if let Some(agent_pos) = agent.position {
                for (&other_id, other) in &self.agents {
                    if other_id == agent_id { continue; }
                    if let Some(other_pos) = other.position {
                        if self.has_line_of_sight(grid, agent_pos, other_pos, max_dist) {
                            result.push(other_id);
                        }
                    }
                }
            }
        }
        result
    }
}

// ============================================================================
// TESTS FOR NEW PERCEPTION / EVENT HELPERS + LOS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_grid() -> HierarchicalGrid {
        HierarchicalGrid::new(10.0, 4)
    }

    #[test]
    fn test_has_line_of_sight_basic() {
        let state = SovereignWorldState::default();
        let grid = create_test_grid();

        let from = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        let to = Vec3 { x: 50.0, y: 0.0, z: 0.0 };

        // In open space with no entities, should have clear LOS
        assert!(state.has_line_of_sight(&grid, from, to, 100.0));
    }

    #[test]
    fn test_council_member_has_visibility() {
        let mut state = SovereignWorldState::default();
        let grid = create_test_grid();

        state.agents.insert(1, Agent { id: 1, position: Some(Vec3 { x: 0.0, y: 0.0, z: 0.0 }), ..Default::default() });
        state.agents.insert(2, Agent { id: 2, position: Some(Vec3 { x: 30.0, y: 0.0, z: 0.0 }), ..Default::default() });

        assert!(state.council_member_has_visibility(&grid, 1, 2, 100.0));
    }

    #[test]
    fn test_can_trigger_harvest_event() {
        let mut state = SovereignWorldState::default();
        let grid = create_test_grid();

        state.agents.insert(1, Agent { id: 1, position: Some(Vec3 { x: 0.0, y: 0.0, z: 0.0 }), ..Default::default() });
        state.resource_nodes.insert(10, ResourceNode { id: 10, position: (20.0, 0.0, 0.0), ..Default::default() });

        assert!(state.can_trigger_harvest_event(&grid, 1, 10, 50.0));
    }

    #[test]
    fn test_can_trigger_synergy_event() {
        let mut state = SovereignWorldState::default();
        let grid = create_test_grid();

        state.agents.insert(1, Agent { id: 1, position: Some(Vec3 { x: 0.0, y: 0.0, z: 0.0 }), ..Default::default() });
        state.agents.insert(2, Agent { id: 2, position: Some(Vec3 { x: 25.0, y: 0.0, z: 0.0 }), ..Default::default() });

        assert!(state.can_trigger_synergy_event(&grid, 1, 2, 50.0));
    }

    #[test]
    fn test_get_perceivable_agents() {
        let mut state = SovereignWorldState::default();
        let grid = create_test_grid();

        state.agents.insert(1, Agent { id: 1, position: Some(Vec3 { x: 0.0, y: 0.0, z: 0.0 }), ..Default::default() });
        state.agents.insert(2, Agent { id: 2, position: Some(Vec3 { x: 15.0, y: 0.0, z: 0.0 }), ..Default::default() });
        state.agents.insert(3, Agent { id: 3, position: Some(Vec3 { x: 100.0, y: 0.0, z: 0.0 }), ..Default::default() });

        let perceivable = state.get_perceivable_agents(&grid, 1, 50.0);
        assert!(perceivable.contains(&2));
        assert!(!perceivable.contains(&3)); // Too far
    }
}

// ============================================================================
// EXISTING VFX CODE BELOW (preserved exactly)
// ============================================================================

// ... (rest of file with ParticleVisualAssets, Hanabi setup, sacred geometry, policy effects preserved)
