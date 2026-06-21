// simulation/src/council/session.rs
// CouncilSession with support for different proposal types and voting
// Now includes clean submit_proposal API for player/UI submission.

use crate::council::proposal::{CouncilProposal, ProposalStatus, ProposalType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilSession {
    pub realm_id: u8,
    pub active_proposals: Vec<CouncilProposal>,
    pub last_session_tick: u64,
    next_proposal_id: u64,  // Simple ID generator for submissions
}

impl CouncilSession {
    pub fn new(realm_id: u8, current_tick: u64) -> Self {
        Self {
            realm_id,
            active_proposals: vec![],
            last_session_tick: current_tick,
            next_proposal_id: current_tick,  // Start ID from current tick for uniqueness
        }
    }

    /// Clean submission API for players, UI, or external systems.
    /// Creates a properly formed CouncilProposal and adds it to the session.
    /// Status starts as Draft. Ready for deliberation.
    pub fn submit_proposal(
        &mut self,
        proposal_type: ProposalType,
        title: String,
        description: String,
        proposer: crate::world::AgentId,
        current_tick: u64,
    ) -> u64 {
        let id = self.next_proposal_id;
        self.next_proposal_id += 1;

        let proposal = CouncilProposal::new(
            id,
            proposal_type,
            title,
            description,
            proposer,
            current_tick,
        );

        self.active_proposals.push(proposal);
        id
    }

    pub fn add_proposal(&mut self, proposal: CouncilProposal) {
        self.active_proposals.push(proposal);
    }

    pub fn run_deliberation(&mut self, average_mercy: f32, current_tick: u64) -> Vec<CouncilProposal> {
        let mut resolved = vec![];

        for proposal in self.active_proposals.iter_mut() {
            if proposal.status == ProposalStatus::Draft || proposal.status == ProposalStatus::Deliberating {
                proposal.status = ProposalStatus::Deliberating;

                let total_votes = proposal.votes_for + proposal.votes_against;
                if total_votes >= 3 {
                    let mercy_factor = (average_mercy / 100.0) * 0.3;
                    let effective_for = proposal.votes_for as f32 * (1.0 + mercy_factor);

                    if effective_for > proposal.votes_against as f32 {
                        proposal.status = ProposalStatus::Passed;
                    } else {
                        proposal.status = ProposalStatus::Rejected;
                    }
                    resolved.push(proposal.clone());
                }
            }
        }

        self.active_proposals.retain(|p| p.status != ProposalStatus::Passed && p.status != ProposalStatus::Rejected);
        self.last_session_tick = current_tick;
        resolved
    }
}
