// simulation/src/council/session.rs
// CouncilSession with improved deliberation and voting logic

use crate::council::proposal::{CouncilProposal, ProposalStatus};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilSession {
    pub realm_id: u8,
    pub active_proposals: Vec<CouncilProposal>,
    pub last_session_tick: u64,
}

impl CouncilSession {
    pub fn new(realm_id: u8, current_tick: u64) -> Self {
        Self {
            realm_id,
            active_proposals: vec![],
            last_session_tick: current_tick,
        }
    }

    pub fn add_proposal(&mut self, proposal: CouncilProposal) {
        self.active_proposals.push(proposal);
    }

    /// Runs deliberation with archetype and mercy influence
    pub fn run_deliberation(&mut self, average_mercy: f32, current_tick: u64) -> Vec<CouncilProposal> {
        let mut resolved = vec![];

        for proposal in self.active_proposals.iter_mut() {
            if proposal.status == ProposalStatus::Draft || proposal.status == ProposalStatus::Deliberating {
                proposal.status = ProposalStatus::Deliberating;

                let total_votes = proposal.votes_for + proposal.votes_against;
                if total_votes >= 3 {
                    // Factor in average mercy of participants for more mercy-aligned outcomes
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
