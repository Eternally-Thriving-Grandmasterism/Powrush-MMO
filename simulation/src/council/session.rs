/*!
 * CouncilSession with Weighted Council Influence.
 *
 * Different PATSAGi Councils can now have different influence weights
 * in the final consensus aggregation.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use tracing::info;

use crate::council::decision::CouncilDecision;
use crate::council::event_bus::{CouncilEvent, CouncilEventBus};
use crate::council::proposal::{CouncilProposal, ProposalStatus, ProposalType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct CouncilVote {
    pub council_id: u8,
    pub approved: bool,
    pub mercy_alignment_score: f32,
    pub weight: f32,                    // NEW: influence weight
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilSession {
    pub realm_id: u8,
    pub active_proposals: Vec<CouncilProposal>,
    pub last_session_tick: u64,
    next_proposal_id: u64,
    #[serde(skip)]
    pub event_bus: Option<CouncilEventBus>,
    last_dynamic_threshold: Option<f32>,
    num_parallel_councils: usize,
    council_weights: Vec<f32>,          // NEW: per-council influence weights
}

impl CouncilSession {
    pub fn new(realm_id: u8, current_tick: u64) -> Self {
        let default_councils = 7;
        Self {
            realm_id,
            active_proposals: vec![],
            last_session_tick: current_tick,
            next_proposal_id: current_tick,
            event_bus: None,
            last_dynamic_threshold: None,
            num_parallel_councils: default_councils,
            council_weights: vec![1.0; default_councils], // equal weight by default
        }
    }

    pub fn with_num_parallel_councils(mut self, count: usize) -> Self {
        self.num_parallel_councils = count.max(1);
        self.council_weights = vec![1.0; self.num_parallel_councils];
        self
    }

    /// Set custom influence weights for each council.
    /// Length must match num_parallel_councils.
    pub fn with_council_weights(mut self, weights: Vec<f32>) -> Self {
        if weights.len() == self.num_parallel_councils {
            self.council_weights = weights;
        }
        self
    }

    pub fn with_event_bus(mut self, bus: CouncilEventBus) -> Self {
        self.event_bus = Some(bus);
        self
    }

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

        self.last_dynamic_threshold = Some(dynamic_threshold);

        info!(
            target: "ra_thor::consensus",
            realm_id = self.realm_id,
            average_mercy = average_mercy,
            dynamic_threshold = dynamic_threshold,
            num_councils = self.num_parallel_councils,
            "Ra-Thor weighted multi-council deliberation started"
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
                        let mut votes: Vec<CouncilVote> = Vec::new();
                        let mut total_weight: f32 = 0.0;
                        let mut weighted_approvals: f32 = 0.0;
                        let mut weighted_mas_sum: f32 = 0.0;
                        let mut approving_weight: f32 = 0.0;

                        for council_id in 0..self.num_parallel_councils {
                            let weight = self.council_weights.get(council_id).copied().unwrap_or(1.0);
                            total_weight += weight;

                            // Simulate specialization via slight MAS variation
                            let variation = (council_id as f32 * 0.012) - 0.04;
                            let temp_decision = CouncilDecision::from_resolved_proposal(
                                proposal,
                                mercy_factor,
                                current_tick,
                                self.realm_id,
                            );

                            let base_mas = temp_decision.mercy_alignment_score(None);
                            let council_mas = (base_mas + variation).clamp(0.0, 1.0);

                            let approved = council_mas >= dynamic_threshold;

                            if approved {
                                weighted_approvals += weight;
                                weighted_mas_sum += council_mas * weight;
                                approving_weight += weight;
                            }

                            votes.push(CouncilVote {
                                council_id: council_id as u8,
                                approved,
                                mercy_alignment_score: council_mas,
                                weight,
                            });
                        }

                        // === Weighted Aggregation ===
                        let weighted_approval_ratio = if total_weight > 0.0 {
                            weighted_approvals / total_weight
                        } else {
                            0.0
                        };

                        let avg_weighted_mas = if approving_weight > 0.0 {
                            weighted_mas_sum / approving_weight
                        } else {
                            0.0
                        };

                        // Final consensus rule (weighted)
                        let passes_consensus =
                            weighted_approval_ratio >= (2.0 / 3.0) && avg_weighted_mas >= dynamic_threshold;

                        if passes_consensus {
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
                            weighted_approval_ratio = weighted_approval_ratio,
                            avg_weighted_mas = avg_weighted_mas,
                            dynamic_threshold = dynamic_threshold,
                            passes_consensus = passes_consensus,
                            status = ?proposal.status,
                            "Weighted multi-council aggregation complete"
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
