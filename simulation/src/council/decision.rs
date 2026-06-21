/*!
 * CouncilDecision with post-effect final Mercy Alignment Score.
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

    /// Final Mercy Alignment Score computed *after* effects were applied to the world.
    /// Includes archetype bonuses + real world state deltas when available.
    pub final_mercy_alignment_score: f32,
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
            final_mercy_alignment_score: 0.0,
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
            final_mercy_alignment_score: 0.0,
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

/// ECS System: Applies effects, records to history, computes final MAS with world deltas.
pub fn apply_council_decision_effects(
    mut decisions: ResMut<CouncilDecisions>,
    mut query: Query<&mut crate::world::SovereignWorldState>,
) {
    for decision in &mut decisions.decisions {
        let effect = decision.effect_type.as_str();
        let mag = decision.magnitude.max(0.1);

        for world in query.iter_mut() {
            // Apply effects
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

            // Record to history
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

            // === Compute final MAS with real world state deltas ===
            let avg_sustainability: f32 = world.rbe_pools.values()
                .map(|p| p.sustainability_score)
                .sum::<f32>() / world.rbe_pools.len().max(1) as f32;

            let avg_abundance: f32 = world.rbe_pools.values()
                .map(|p| p.abundance_flow)
                .sum::<f32>() / world.rbe_pools.len().max(1) as f32;

            // Simple but effective delta-aware final score
            let delta_score = (avg_sustainability * 0.5 + avg_abundance * 0.5).clamp(0.3, 1.0);
            let base_mas = decision.mercy_factor.clamp(0.3, 1.0);

            decision.final_mercy_alignment_score = (base_mas * 0.7 + delta_score * 0.3).clamp(0.0, 1.0);
        }
    }

    if !decisions.decisions.is_empty() {
        decisions.clear();
    }
}
