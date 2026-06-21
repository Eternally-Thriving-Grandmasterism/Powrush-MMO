/*!
 * CouncilDecision with Mercy Alignment Score calculation.
 *
 * Implements the official Ra-Thor / PATSAGi Mercy Alignment Score formula.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::world::AgentId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilDecision {
    pub proposal_id: u64,
    pub title: String,
    pub effect_type: String,
    pub magnitude: f32,
    pub passed_tick: u64,
    pub realm_id: u8,

    pub votes_for: u32,
    pub votes_against: u32,
    pub mercy_factor: f32,
    pub deliberation_tick: u64,
    pub proposer: AgentId,
}

impl CouncilDecision {
    pub fn new(
        proposal_id: u64,
        title: String,
        effect_type: String,
        magnitude: f32,
        passed_tick: u64,
        realm_id: u8,
    ) -> Self {
        Self {
            proposal_id,
            title,
            effect_type,
            magnitude,
            passed_tick,
            realm_id,
            votes_for: 0,
            votes_against: 0,
            mercy_factor: 0.0,
            deliberation_tick: passed_tick,
            proposer: 0,
        }
    }

    pub fn from_resolved_proposal(
        proposal: &crate::council::CouncilProposal,
        mercy_factor: f32,
        deliberation_tick: u64,
        realm_id: u8,
    ) -> Self {
        Self {
            proposal_id: proposal.id,
            title: proposal.title.clone(),
            effect_type: format!("{:?}", proposal.proposal_type),
            magnitude: 1.0,
            passed_tick: deliberation_tick,
            realm_id,
            votes_for: proposal.votes_for,
            votes_against: proposal.votes_against,
            mercy_factor,
            deliberation_tick,
            proposer: proposal.proposer,
        }
    }

    /// Computes the official Mercy Alignment Score (MAS) for this decision.
    ///
    /// Formula:
    /// MAS = 0.35*V + 0.30*S + 0.20*T + 0.15*A
    ///
    /// Returns a value in [0.0, 1.0]
    pub fn mercy_alignment_score(&self, world: Option<&crate::world::SovereignWorldState>) -> f32 {
        let v = self.vote_mercy();
        let s = self.sustainability_mercy(world);
        let t = self.truth_mercy();
        let a = self.abundance_mercy(world);

        0.35 * v + 0.30 * s + 0.20 * t + 0.15 * a
    }

    /// Vote Mercy component (V)
    fn vote_mercy(&self) -> f32 {
        let total = (self.votes_for + self.votes_against) as f32;
        if total < 1.0 {
            return 0.5; // neutral if no votes
        }
        let ratio = self.votes_for as f32 / total;
        (ratio * self.mercy_factor).clamp(0.0, 1.0)
    }

    /// Sustainability Mercy component (S)
    fn sustainability_mercy(&self, world: Option<&crate::world::SovereignWorldState>) -> f32 {
        // Placeholder: in a full implementation we would compare before/after metrics
        // For now we use a reasonable default based on effect type
        match self.effect_type.as_str() {
            "ResourcePolicy" | "resource_policy" => 0.85,
            "HarmonyBoost" | "harmony_boost" => 0.78,
            "EpiphanyEvent" | "epiphany_event" => 0.72,
            "General" | "general" => 0.65,
            _ => 0.60,
        }
    }

    /// Truth Mercy component (T)
    fn truth_mercy(&self) -> f32 {
        // Currently simple heuristic. Can be expanded with proposer history,
        // harm signals, consistency checks, etc.
        if self.mercy_factor > 0.6 {
            0.88
        } else if self.mercy_factor > 0.4 {
            0.75
        } else {
            0.60
        }
    }

    /// Abundance Mercy component (A)
    fn abundance_mercy(&self, _world: Option<&crate::world::SovereignWorldState>) -> f32 {
        // Placeholder - rewards decisions that increase overall flow
        match self.effect_type.as_str() {
            "ResourcePolicy" | "resource_policy" => 0.82,
            "EpiphanyEvent" | "epiphany_event" => 0.75,
            "HarmonyBoost" | "harmony_boost" => 0.70,
            "General" | "general" => 0.65,
            _ => 0.60,
        }
    }
}

#[derive(Resource, Default, Clone, Debug, Serialize, Deserialize)]
pub struct CouncilDecisions {
    pub decisions: Vec<CouncilDecision>,
}

impl CouncilDecisions {
    pub fn add_decision(&mut self, decision: CouncilDecision) {
        self.decisions.push(decision);
    }

    pub fn clear(&mut self) {
        self.decisions.clear();
    }
}

/// ECS System: Applies effects + records to history + maintains indices.
pub fn apply_council_decision_effects(
    mut decisions: ResMut<CouncilDecisions>,
    mut query: Query<&mut crate::world::SovereignWorldState>,
) {
    for decision in &decisions.decisions {
        let effect = decision.effect_type.as_str();
        let mag = decision.magnitude.max(0.1);

        for world in query.iter_mut() {
            match effect {
                "ResourcePolicy" | "resource_policy" => {
                    for pool in world.rbe_pools.values_mut() {
                        pool.abundance_flow = (pool.abundance_flow + 0.25 * mag).min(3.5);
                        pool.sustainability_score = (pool.sustainability_score + 0.08 * mag).min(1.0);
                        pool.pressure = (pool.pressure * (1.0 - 0.35 * mag)).max(0.0);
                    }
                    for node in world.resource_nodes.values_mut() {
                        node.abundance_flow = (node.abundance_flow + 0.12 * mag).min(3.0);
                        node.sustainability_score = (node.sustainability_score + 0.05 * mag).min(1.0);
                    }
                }
                "HarmonyBoost" | "harmony_boost" => {
                    for pool in world.rbe_pools.values_mut() {
                        pool.sustainability_score = (pool.sustainability_score + 0.06 * mag).min(1.0);
                    }
                }
                "EpiphanyEvent" | "epiphany_event" => {
                    for pool in world.rbe_pools.values_mut() {
                        pool.abundance_flow = (pool.abundance_flow + 0.15 * mag).min(3.5);
                    }
                    for node in world.resource_nodes.values_mut() {
                        node.abundance_flow = (node.abundance_flow + 0.08 * mag).min(3.0);
                    }
                }
                "General" | "general" => {
                    for pool in world.rbe_pools.values_mut() {
                        pool.sustainability_score = (pool.sustainability_score + 0.03 * mag).min(1.0);
                    }
                }
                _ => {}
            }

            let new_index = world.council_decision_history.len();
            world.council_decision_history.push(decision.clone());

            world.council_decision_indices_by_proposer
                .entry(decision.proposer)
                .or_default()
                .push(new_index);

            world.council_decision_indices_by_type
                .entry(decision.effect_type.clone())
                .or_default()
                .push(new_index);
        }
    }

    if !decisions.decisions.is_empty() {
        decisions.clear();
    }
}
