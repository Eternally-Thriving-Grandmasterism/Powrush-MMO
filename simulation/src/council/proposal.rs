// simulation/src/council/proposal.rs
// Basic CouncilProposal structure (Local Council foundation)

use serde::{Deserialize, Serialize};
use crate::world::AgentId;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProposalStatus {
    Draft,
    Deliberating,
    Passed,
    Rejected,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilProposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposer: AgentId,
    pub status: ProposalStatus,
    pub created_tick: u64,
    pub votes_for: u32,
    pub votes_against: u32,
}

impl CouncilProposal {
    pub fn new(id: u64, title: String, description: String, proposer: AgentId, current_tick: u64) -> Self {
        Self {
            id,
            title,
            description,
            proposer,
            status: ProposalStatus::Draft,
            created_tick: current_tick,
            votes_for: 0,
            votes_against: 0,
        }
    }

    pub fn cast_vote(&mut self, support: bool) {
        if support {
            self.votes_for += 1;
        } else {
            self.votes_against += 1;
        }
    }
}
