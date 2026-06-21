/*!
 * CouncilSession with last dynamic threshold exposed.
 */

use tracing::info;

use crate::council::decision::CouncilDecision;
use crate::council::event_bus::{CouncilEvent, CouncilEventBus};
use crate::council::proposal::{CouncilProposal, ProposalStatus, ProposalType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilSession {
    pub realm_id: u8,
    pub active_proposals: Vec<CouncilProposal>,
    pub last_session_tick: u64,
    next_proposal_id: u64,
    #[serde(skip)]
    pub event_bus: Option<CouncilEventBus>,
    last_dynamic_threshold: Option<f32>,   // NEW: for observability
}

impl CouncilSession {
    pub fn new(realm_id: u8, current_tick: u64) -> Self {
        Self {
            realm_id,
            active_proposals: vec![],
            last_session_tick: current_tick,
            next_proposal_id: current_tick,
            event_bus: None,
            last_dynamic_threshold: None,
        }
    }

    pub fn with_event_bus(mut self, bus: CouncilEventBus) -> Self {
        self.event_bus = Some(bus);
        self
    }

    /// Returns the dynamic mercy threshold used in the last deliberation.
    pub fn last_dynamic_threshold(&self) -> Option<f32> {
        self.last_dynamic_threshold
    }

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
            title.clone(),
            description,
            proposer,
            current_tick,
        );

        if let Some(bus) = &self.event_bus {
            let _ = bus.send(CouncilEvent::ProposalSubmitted {
                proposal_id: id,
                proposer,
                proposal_type,
                title,
            });
        }

        self.active_proposals.push(proposal);
        id
    }

    pub fn add_proposal(&mut self, proposal: CouncilProposal) {
        self.active_proposals.push(proposal);
    }

    pub fn run_deliberation(&mut self, average_mercy: f32, current_tick: u64) -> Vec<CouncilProposal> {
        let mut resolved = vec![];

        let base_threshold: f32 = 0.72;
        let mercy_health = (average_mercy / 100.0).clamp(0.0, 1.0);
        let scaling = (mercy_health - 0.5) * 0.12;
        let dynamic_threshold = (base_threshold - scaling).clamp(0.60, 0.85);

        self.last_dynamic_threshold = Some(dynamic_threshold); // store for observability

        info!(
            target: "ra_thor::consensus",
            realm_id = self.realm_id,
            average_mercy = average_mercy,
            dynamic_threshold = dynamic_threshold,
            active_proposals = self.active_proposals.len(),
            "Ra-Thor deliberation started"
        );

        for proposal in self.active_proposals.iter_mut() {
            if proposal.status == ProposalStatus::Draft || proposal.status == ProposalStatus::Deliberating {
                proposal.status = ProposalStatus::Deliberating;

                let total_votes = proposal.votes_for + proposal.votes_against;
                if total_votes >= 3 {
                    let mercy_factor = (average_mercy / 100.0) * 0.3;
                    let effective_for = proposal.votes_for as f32 * (1.0 + mercy_factor);

                    let would_pass_vote = effective_for > proposal.votes_against as f32;

                    if would_pass_vote {
                        let temp_decision = CouncilDecision::from_resolved_proposal(
                            proposal,
                            mercy_factor,
                            current_tick,
                            self.realm_id,
                        );

                        let mas = temp_decision.mercy_alignment_score(None);
                        let passes_mercy = mas >= dynamic_threshold;

                        if passes_mercy {
                            proposal.status = ProposalStatus::Passed;

                            if let Some(bus) = &self.event_bus {
                                let _ = bus.send(CouncilEvent::ProposalPassed {
                                    proposal_id: proposal.id,
                                    votes_for: proposal.votes_for,
                                    votes_against: proposal.votes_against,
                                    mercy_factor,
                                });
                            }
                        } else {
                            proposal.status = ProposalStatus::Rejected;
                        }

                        info!(
                            target: "ra_thor::consensus",
                            realm_id = self.realm_id,
                            proposal_id = proposal.id,
                            mas = mas,
                            dynamic_threshold = dynamic_threshold,
                            passes_mercy = passes_mercy,
                            status = ?proposal.status,
                            "Proposal evaluated by Ra-Thor consensus"
                        );
                    } else {
                        proposal.status = ProposalStatus::Rejected;

                        info!(
                            target: "ra_thor::consensus",
                            realm_id = self.realm_id,
                            proposal_id = proposal.id,
                            status = ?proposal.status,
                            "Proposal rejected (insufficient votes)"
                        );
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
