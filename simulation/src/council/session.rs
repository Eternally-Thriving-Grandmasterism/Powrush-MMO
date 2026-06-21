/*!
 * CouncilSession with Council Archetype Defaults.
 *
 * Predefined PATSAGi Council archetypes aligned with the 7 Living Mercy Gates.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use tracing::info;

use crate::council::decision::CouncilDecision;
use crate::council::event_bus::{CouncilEvent, CouncilEventBus};
use crate::council::proposal::{CouncilProposal, ProposalStatus, ProposalType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CouncilArchetype {
    Truth,        // Emphasizes truth-seeking and anti-hallucination
    Abundance,    // Focuses on resource flow and shared prosperity
    Harmony,      // Prioritizes systemic balance and sustainability
    Service,      // Values benefit to the many over the few
    Mercy,        // Strong emphasis on compassion and redemption
    Joy,          // Favors decisions that increase overall well-being
    Cosmic,       // Long-term, large-scale, future-oriented view
}

impl CouncilArchetype {
    /// Returns a human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            CouncilArchetype::Truth => "Truth Council",
            CouncilArchetype::Abundance => "Abundance Council",
            CouncilArchetype::Harmony => "Harmony Council",
            CouncilArchetype::Service => "Service Council",
            CouncilArchetype::Mercy => "Mercy Council",
            CouncilArchetype::Joy => "Joy Council",
            CouncilArchetype::Cosmic => "Cosmic Council",
        }
    }

    /// Returns a suggested influence weight for this archetype
    pub fn default_weight(&self) -> f32 {
        match self {
            CouncilArchetype::Truth => 1.15,
            CouncilArchetype::Abundance => 1.10,
            CouncilArchetype::Harmony => 1.05,
            CouncilArchetype::Service => 1.00,
            CouncilArchetype::Mercy => 1.08,
            CouncilArchetype::Joy => 0.95,
            CouncilArchetype::Cosmic => 1.12,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CouncilVote {
    pub council_id: u8,
    pub approved: bool,
    pub mercy_alignment_score: f32,
    pub weight: f32,
    pub archetype: Option<CouncilArchetype>,
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
    council_weights: Vec<f32>,
    council_archetypes: Vec<Option<CouncilArchetype>>, // NEW
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
            council_weights: vec![1.0; default_councils],
            council_archetypes: vec![None; default_councils],
        }
    }

    pub fn with_num_parallel_councils(mut self, count: usize) -> Self {
        self.num_parallel_councils = count.max(1);
        self.council_weights = vec![1.0; self.num_parallel_councils];
        self.council_archetypes = vec![None; self.num_parallel_councils];
        self
    }

    pub fn with_council_weights(mut self, weights: Vec<f32>) -> Self {
        if weights.len() == self.num_parallel_councils {
            self.council_weights = weights;
        }
        self
    }

    /// Assigns the 7 Living Mercy Gate archetypes with sensible default weights.
    pub fn with_archetype_defaults(mut self) -> Self {
        let archetypes = vec![
            Some(CouncilArchetype::Truth),
            Some(CouncilArchetype::Abundance),
            Some(CouncilArchetype::Harmony),
            Some(CouncilArchetype::Service),
            Some(CouncilArchetype::Mercy),
            Some(CouncilArchetype::Joy),
            Some(CouncilArchetype::Cosmic),
        ];

        let count = self.num_parallel_councils.min(archetypes.len());

        for i in 0..count {
            self.council_archetypes[i] = archetypes[i].clone();
            if let Some(archetype) = &archetypes[i] {
                self.council_weights[i] = archetype.default_weight();
            }
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
            "Ra-Thor archetype-weighted deliberation started"
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
                            let archetype = self.council_archetypes.get(council_id).cloned().flatten();
                            total_weight += weight;

                            // Archetype-aware variation (can be expanded later)
                            let variation = match archetype {
                                Some(CouncilArchetype::Truth) => 0.03,
                                Some(CouncilArchetype::Abundance) => 0.02,
                                Some(CouncilArchetype::Harmony) => 0.01,
                                Some(CouncilArchetype::Mercy) => -0.01,
                                _ => 0.0,
                            };

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
                                archetype,
                            });
                        }

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
                            "Archetype-weighted multi-council aggregation complete"
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
