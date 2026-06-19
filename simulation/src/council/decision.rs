// simulation/src/council/decision.rs
// Persistent Council Decisions with effects

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
        Self {
            proposal_id,
            title,
            effect_type,
            magnitude,
            passed_tick,
            realm_id,
        }
    }
}

/// Resource to store all passed council decisions for a realm
#[derive(Resource, Default, Clone, Debug, Serialize, Deserialize)]
pub struct CouncilDecisions {
    pub decisions: Vec<CouncilDecision>,
}

impl CouncilDecisions {
    pub fn add_decision(&mut self, decision: CouncilDecision) {
        self.decisions.push(decision);
    }
}
