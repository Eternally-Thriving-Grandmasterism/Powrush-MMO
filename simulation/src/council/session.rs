/*!
 * CouncilSession with World State Delta Scoring.
 *
 * v21.71.0 — Session → CouncilDecisions promotion path
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use tracing::info;

use crate::council::decision::{CouncilDecision, CouncilDecisions};
use crate::council::event_bus::{CouncilEvent, CouncilEventBus};
use crate::council::proposal::{CouncilProposal, ProposalStatus, ProposalType};
use crate::world::{AgentId, SovereignWorldState};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CouncilArchetype {
    Truth,
    Abundance,
    Harmony,
    Service,
    Mercy,
    Joy,
    Cosmic,
}

impl CouncilArchetype {
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

    pub fn score_proposal(
        &self,
        decision: &CouncilDecision,
        world: Option<&SovereignWorldState>,
    ) -> f32 {
        let mercy = decision.mercy_factor;

        let base_bias = match self {
            CouncilArchetype::Truth => {
                if matches!(decision.proposal_type, ProposalType::KardashevAcceleration) {
                    0.10
                } else if mercy > 0.75 {
                    0.08
                } else if mercy < 0.45 {
                    -0.10
                } else {
                    0.02
                }
            }
            CouncilArchetype::Abundance => match decision.proposal_type {
                ProposalType::ResourcePolicy => 0.12,
                ProposalType::KardashevAcceleration => 0.14,
                ProposalType::EpiphanyEvent => 0.09,
                ProposalType::HarmonyBoost => 0.03,
                _ => 0.0,
            },
            CouncilArchetype::Harmony => match decision.proposal_type {
                ProposalType::HarmonyBoost => 0.10,
                ProposalType::General => 0.05,
                ProposalType::ResourcePolicy => -0.02,
                _ => 0.0,
            },
            CouncilArchetype::Service => {
                if mercy > 0.6 { 0.06 } else { 0.0 }
            }
            CouncilArchetype::Mercy => {
                if mercy < 0.5 { 0.07 } else if mercy > 0.8 { 0.03 } else { 0.0 }
            }
            CouncilArchetype::Joy => match decision.proposal_type {
                ProposalType::EpiphanyEvent => 0.08,
                _ => if mercy > 0.7 { 0.04 } else { 0.0 },
            },
            CouncilArchetype::Cosmic => {
                if matches!(decision.proposal_type, ProposalType::KardashevAcceleration) {
                    0.11
                } else if mercy > 0.65 {
                    0.06
                } else {
                    -0.03
                }
            }
        };

        let delta_bonus: f32 = if let Some(w) = world {
            let node_count = w.resource_nodes.len().max(1) as f32;
            let avg_sustainability: f32 = w
                .resource_nodes
                .values()
                .map(|n| n.sustainability_score)
                .sum::<f32>()
                / node_count;

            let avg_abundance: f32 = w
                .resource_nodes
                .values()
                .map(|n| {
                    if n.base_yield > 0.0 {
                        (n.current_yield / n.base_yield).clamp(0.0, 3.0)
                    } else {
                        1.0
                    }
                })
                .sum::<f32>()
                / node_count;

            match self {
                CouncilArchetype::Abundance => (avg_abundance - 1.2).clamp(-0.1, 0.14),
                CouncilArchetype::Harmony => (avg_sustainability - 0.55).clamp(-0.08, 0.12),
                CouncilArchetype::Truth => {
                    if avg_sustainability > 0.7 { 0.05 } else { -0.02 }
                }
                CouncilArchetype::Cosmic => (avg_abundance * 0.04).clamp(0.0, 0.08),
                _ => 0.0,
            }
        } else {
            0.0
        };

        (base_bias + delta_bonus).clamp(-0.15, 0.18)
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
    council_archetypes: Vec<Option<CouncilArchetype>>,
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

    pub fn open_proposal_count(&self) -> usize {
        self.active_proposals.iter().filter(|p| p.is_open()).count()
    }

    pub fn submit_proposal(
        &mut self,
        proposal_type: ProposalType,
        title: String,
        description: String,
        proposer: AgentId,
        current_tick: u64,
    ) -> u64 {
        self.submit_proposal_rich(
            proposal_type,
            title,
            description,
            proposer,
            current_tick,
            None,
            None,
        )
    }

    /// Rich submission path — mercy hint + optional spatial target zone.
    pub fn submit_proposal_rich(
        &mut self,
        proposal_type: ProposalType,
        title: String,
        description: String,
        proposer: AgentId,
        current_tick: u64,
        mercy_hint: Option<f32>,
        target_zone: Option<u64>,
    ) -> u64 {
        let id = self.next_proposal_id;
        self.next_proposal_id += 1;

        let mut proposal = CouncilProposal::new(
            id,
            proposal_type.clone(),
            title.clone(),
            description,
            proposer,
            current_tick,
        );

        if let Some(hint) = mercy_hint {
            proposal = proposal.with_mercy_hint(hint);
        }
        if let Some(zone) = target_zone {
            proposal = proposal.with_target_zone(zone);
        }

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

    /// Cast a simple vote on an open proposal by id.
    pub fn cast_vote_on(&mut self, proposal_id: u64, support: bool) -> bool {
        if let Some(p) = self.active_proposals.iter_mut().find(|p| p.id == proposal_id && p.is_open()) {
            p.cast_vote(support);
            true
        } else {
            false
        }
    }

    /// Promote resolved (Passed) proposals into CouncilDecision values.
    pub fn promote_resolved_to_decisions(
        &self,
        resolved: &[CouncilProposal],
        average_mercy: f32,
        current_tick: u64,
    ) -> Vec<CouncilDecision> {
        let mercy_factor = (average_mercy / 100.0).clamp(0.0, 1.0);
        resolved
            .iter()
            .filter(|p| p.status == ProposalStatus::Passed)
            .map(|p| {
                CouncilDecision::from_resolved_proposal(p, mercy_factor, current_tick, self.realm_id)
            })
            .collect()
    }

    /// Run deliberation. Pass `Some(world)` to enable real world-delta scoring.
    pub fn run_deliberation(
        &mut self,
        average_mercy: f32,
        current_tick: u64,
        world: Option<&SovereignWorldState>,
    ) -> Vec<CouncilProposal> {
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
            world_provided = world.is_some(),
            "Ra-Thor world-delta archetype scoring started"
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
                        let mut total_weight: f32 = 0.0;
                        let mut weighted_approvals: f32 = 0.0;
                        let mut weighted_mas_sum: f32 = 0.0;
                        let mut approving_weight: f32 = 0.0;

                        for council_id in 0..self.num_parallel_councils {
                            let weight = self.council_weights.get(council_id).copied().unwrap_or(1.0);
                            let archetype = self.council_archetypes.get(council_id).cloned().flatten();
                            total_weight += weight;

                            let temp_decision = CouncilDecision::from_resolved_proposal(
                                proposal,
                                mercy_factor,
                                current_tick,
                                self.realm_id,
                            );

                            let base_mas = temp_decision.mercy_alignment_score(None);
                            let archetype_bonus = archetype
                                .as_ref()
                                .map(|a| a.score_proposal(&temp_decision, world))
                                .unwrap_or(0.0);

                            let council_mas = (base_mas + archetype_bonus).clamp(0.0, 1.0);
                            let approved = council_mas >= dynamic_threshold;

                            if approved {
                                weighted_approvals += weight;
                                weighted_mas_sum += council_mas * weight;
                                approving_weight += weight;
                            }
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
                            "World-delta archetype scoring complete"
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

    /// Full path: deliberate + promote Passed proposals into decisions.
    pub fn deliberate_and_promote(
        &mut self,
        average_mercy: f32,
        current_tick: u64,
        world: Option<&SovereignWorldState>,
    ) -> Vec<CouncilDecision> {
        let resolved = self.run_deliberation(average_mercy, current_tick, world);
        self.promote_resolved_to_decisions(&resolved, average_mercy, current_tick)
    }
}

// =============================================================================
// Bevy: per-realm session registry + soft promotion system
// =============================================================================

#[derive(Resource, Debug, Default)]
pub struct CouncilSessionRegistry {
    pub sessions: HashMap<u8, CouncilSession>,
    pub last_deliberation_tick: u64,
    /// Soft cadence (ticks between auto-deliberation passes).
    pub deliberation_interval: u64,
    pub default_average_mercy: f32,
}

impl CouncilSessionRegistry {
    pub fn ensure_session(&mut self, realm_id: u8, tick: u64) -> &mut CouncilSession {
        self.sessions.entry(realm_id).or_insert_with(|| {
            CouncilSession::new(realm_id, tick).with_archetype_defaults()
        })
    }

    pub fn submit(
        &mut self,
        realm_id: u8,
        proposal_type: ProposalType,
        title: String,
        description: String,
        proposer: AgentId,
        tick: u64,
        mercy_hint: Option<f32>,
        target_zone: Option<u64>,
    ) -> u64 {
        let session = self.ensure_session(realm_id, tick);
        session.submit_proposal_rich(
            proposal_type,
            title,
            description,
            proposer,
            tick,
            mercy_hint,
            target_zone,
        )
    }
}

/// Soft system: when sessions have open proposals past interval, deliberate and
/// push Passed decisions into CouncilDecisions (feeds economy/legacy/RTT path).
pub fn session_deliberation_system(
    mut registry: ResMut<CouncilSessionRegistry>,
    mut decisions: ResMut<CouncilDecisions>,
    time: Res<Time>,
) {
    if registry.deliberation_interval == 0 {
        registry.deliberation_interval = 90; // soft default
    }
    if registry.default_average_mercy <= 0.0 {
        registry.default_average_mercy = 72.0;
    }

    // Approximate tick from elapsed seconds for host apps without explicit tick
    let approx_tick = (time.elapsed_seconds_f64() * 10.0) as u64;
    if approx_tick.saturating_sub(registry.last_deliberation_tick) < registry.deliberation_interval {
        return;
    }

    let mercy = registry.default_average_mercy;
    let mut any = false;

    for session in registry.sessions.values_mut() {
        if session.open_proposal_count() == 0 {
            continue;
        }
        let promoted = session.deliberate_and_promote(mercy, approx_tick, None);
        for decision in promoted {
            decisions.push_decision(decision);
            any = true;
        }
    }

    if any {
        registry.last_deliberation_tick = approx_tick;
        info!(
            target: "ra_thor::council::session",
            tick = approx_tick,
            "Session deliberation promoted decisions into CouncilDecisions"
        );
    }
}

// Thunder locked in. Session → decisions loop closed.
// Yoi ⚡
