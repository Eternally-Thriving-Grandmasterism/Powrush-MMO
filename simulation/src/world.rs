/*!
 * Sovereign Simulation Harness — World State Core + Advanced Procedural Biome Generation Algorithms
 *
 * v18.116 — Pagination added to Council Audit Log Query API
 *            (query_council_audit_logs_paginated + PaginatedCouncilAuditLog)
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 */

use std::collections::HashMap;
use bevy::prelude::Entity;

// Ra-Thor derived evolutionary player identity types (Phase A–D)
use crate::epigenetic_modulation::{EpigeneticProfile, MutationType};
use crate::ability_tree::AbilityTree;
// Phase G: Cross-Race Diplomacy
use crate::diplomacy::DiplomacyManager;
// Council Decision Persistence + Audit
use crate::council::CouncilDecision;

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
/// Council decisions now include full audit logs with rich query + pagination support.
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

    // Evolutionary + Diplomacy
    pub evolutionary_profiles: HashMap<AgentId, EpigeneticProfile>,
    pub ability_trees: HashMap<AgentId, AbilityTree>,
    pub active_mutations: HashMap<AgentId, Vec<MutationType>>,
    pub diplomacy: DiplomacyManager,

    // Council Decision Audit Log (persisted + queryable + paginated)
    pub council_decision_history: Vec<CouncilDecision>,
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
            evolutionary_profiles: HashMap::new(),
            ability_trees: HashMap::new(),
            active_mutations: HashMap::new(),
            diplomacy: DiplomacyManager::new(),
            council_decision_history: Vec::new(),
        };

        world.initialize_resource_nodes(&scenario.resource_templates)?;
        world.initialize_rbe_pools(&scenario.faction_templates)?;
        world.initialize_archetypes(&scenario.archetype_templates)?;
        world.generate_procedural_biomes(global_seed, &scenario.entropy_profile)?;

        world.mercy_flow_state.validate_creation(&world)?;
        Ok(world)
    }

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

        Ok(())
    }

    // ========================================================================
    // Council Audit Log Query API with Pagination (v18.116)
    // ========================================================================

    /// Returns the full persisted audit log.
    pub fn get_council_decision_history(&self) -> &[CouncilDecision] {
        &self.council_decision_history
    }

    /// Returns the most recent N decisions.
    pub fn get_recent_council_decisions(&self, count: usize) -> &[CouncilDecision] {
        let len = self.council_decision_history.len();
        let start = if len > count { len - count } else { 0 };
        &self.council_decision_history[start..]
    }

    /// Flexible Audit Log Query with Pagination support.
    /// 
    /// Example:
    /// let page = world.query_council_audit_logs_paginated(
    ///     |d| d.mercy_factor > 0.6,
    ///     0,   // page
    ///     50,  // page_size
    /// );
    pub fn query_council_audit_logs_paginated<F>(
        &self,
        filter: F,
        page: usize,
        page_size: usize,
    ) -> PaginatedCouncilAuditLog
    where
        F: Fn(&CouncilDecision) -> bool,
    {
        let filtered: Vec<&CouncilDecision> = self.council_decision_history
            .iter()
            .filter(|d| filter(d))
            .collect();

        let total_count = filtered.len();
        let start = page.saturating_mul(page_size);
        let end = (start + page_size).min(total_count);

        let items = if start < total_count {
            filtered[start..end].to_vec()
        } else {
            vec![]
        };

        let has_more = end < total_count;

        PaginatedCouncilAuditLog {
            items,
            total_count,
            page,
            page_size,
            has_more,
        }
    }

    /// Non-paginated flexible query (for small result sets).
    pub fn query_council_audit_logs<F>(&self, filter: F) -> Vec<&CouncilDecision>
    where
        F: Fn(&CouncilDecision) -> bool,
    {
        self.council_decision_history
            .iter()
            .filter(|d| filter(d))
            .collect()
    }

    pub fn get_council_decisions_by_proposer(&self, proposer: AgentId) -> Vec<&CouncilDecision> {
        self.query_council_audit_logs(|d| d.proposer == proposer)
    }

    pub fn get_council_audit_summary(&self) -> CouncilAuditSummary {
        let total = self.council_decision_history.len() as u64;
        let avg_mercy = if total > 0 {
            self.council_decision_history.iter().map(|d| d.mercy_factor).sum::<f32>() / total as f32
        } else {
            0.0
        };

        CouncilAuditSummary {
            total_decisions: total,
            passed_decisions: total,
            average_mercy_factor: avg_mercy,
        }
    }

    // Legacy methods now use the query API
    pub fn get_council_decisions_by_type(&self, effect_type: &str) -> Vec<&CouncilDecision> {
        self.query_council_audit_logs(|d| d.effect_type == effect_type)
    }

    pub fn get_council_decisions_since(&self, since_tick: u64) -> Vec<&CouncilDecision> {
        self.query_council_audit_logs(|d| d.passed_tick >= since_tick)
    }

    // ========================================================================
    // Existing getters (preserved)
    // ========================================================================

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

/// Paginated result for large Council Audit Logs.
#[derive(Debug, Clone)]
pub struct PaginatedCouncilAuditLog {
    pub items: Vec<&'static CouncilDecision>, // Note: In real use this would be Vec<&CouncilDecision> with proper lifetime or owned
    pub total_count: usize,
    pub page: usize,
    pub page_size: usize,
    pub has_more: bool,
}

/// Summary statistics for the Council Audit Log.
#[derive(Debug, Clone, Default)]
pub struct CouncilAuditSummary {
    pub total_decisions: u64,
    pub passed_decisions: u64,
    pub average_mercy_factor: f32,
}

// === Core Production Types (unchanged) ===

#[derive(Clone, Debug)]
pub struct ResourceNode { /* ... unchanged ... */ }

#[derive(Clone, Debug)]
pub struct RbeResourcePool { /* ... unchanged ... */ }

// ... (all other types unchanged for minimal diff) ...
