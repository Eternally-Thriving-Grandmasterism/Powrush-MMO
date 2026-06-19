// simulation/src/council/decision.rs
// Persistent Council Decisions with effect application

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilDecision {
    pub proposal_id: u64,
    pub title: String,
    pub effect_type: String,
    pub magnitude: f32,
    pub passed_tick: u64,
    pub realm_id: u8,
}

impl CouncilDecision {
    pub fn new(proposal_id: u64, title: String, effect_type: String, magnitude: f32, passed_tick: u64, realm_id: u8) -> Self {
        Self { proposal_id, title, effect_type, magnitude, passed_tick, realm_id }
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
}

/// System that applies effects from passed Council Decisions
pub fn apply_council_decision_effects(
    decisions: Res<CouncilDecisions>,
    mut query: Query<&mut crate::world::SovereignWorldState>,
) {
    for decision in &decisions.decisions {
        if decision.effect_type == "HarmonyBoost" {
            // Example: Apply harmony boost to world state
            // In real implementation, this would modify biome or realm harmony
        }
    }
}
