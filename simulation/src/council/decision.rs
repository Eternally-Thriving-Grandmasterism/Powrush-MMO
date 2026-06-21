// simulation/src/council/decision.rs
// CouncilDecision with full audit fields + proposer for rich audit queries.

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

    // Audit fields
    pub votes_for: u32,
    pub votes_against: u32,
    pub mercy_factor: f32,
    pub deliberation_tick: u64,

    // For proposer-based audit queries
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

/// ECS System with audit logging into world history.
pub fn apply_council_decision_effects(
    mut decisions: ResMut<CouncilDecisions>,
    mut query: Query<&mut crate::world::SovereignWorldState>,
) {
    for decision in &decisions.decisions {
        let effect = decision.effect_type.as_str();
        let mag = decision.magnitude.max(0.1);

        for world in query.iter_mut() {
            match effect {
                "ResourcePolicy" | "resource_policy" => { /* ... effects unchanged ... */ }
                "HarmonyBoost" | "harmony_boost" => { /* ... */ }
                "EpiphanyEvent" | "epiphany_event" => { /* ... */ }
                "General" | "general" => { /* ... */ }
                _ => {}
            }

            world.council_decision_history.push(decision.clone());
        }
    }

    if !decisions.decisions.is_empty() {
        decisions.clear();
    }
}
