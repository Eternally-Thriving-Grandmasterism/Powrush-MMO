//! simulation/src/council/proposal.rs
//! Council Proposal System — Core Types + Voting + Kardashev Support
//! v1.2 — Added KardashevAcceleration ProposalType + richer helpers for session integration
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
//!
//! Cross-link: CouncilProposal (ProposalType including KardashevAcceleration, ResourcePolicy, EpiphanyEvent, HarmonyBoost)
//! + voting feeds emergence (EpiphanyEvent/DivineWhisper), RBE (ResourcePolicy), fracture AGI, InterestManager,
//! recovered render post-FX, council bloom visuals, and Kardashev Acceleration Dashboard metrics.

use serde::{Deserialize, Serialize};
use crate::world::AgentId;

/// High-level categories of proposals that can be deliberated by the parallel PATSAGi Councils.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProposalType {
    /// Direct harmony / valence boosts across agents or zones.
    HarmonyBoost,
    /// Resource allocation, sustainability, or abundance policy changes (RBE core).
    ResourcePolicy,
    /// Triggers or amplifies epiphany / Divine Whisper events.
    EpiphanyEvent,
    /// Kardashev Acceleration metrics, Reality Thriving Transfer Score, or hardware sovereignty steps.
    KardashevAcceleration,
    /// Catch-all for general governance, treaties, or experimental proposals.
    General,
}

impl ProposalType {
    pub fn name(&self) -> &'static str {
        match self {
            ProposalType::HarmonyBoost => "Harmony Boost",
            ProposalType::ResourcePolicy => "Resource Policy",
            ProposalType::EpiphanyEvent => "Epiphany Event",
            ProposalType::KardashevAcceleration => "Kardashev Acceleration",
            ProposalType::General => "General",
        }
    }
}

/// Lifecycle status of a proposal inside a CouncilSession.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProposalStatus {
    Draft,
    Deliberating,
    Passed,
    Rejected,
}

/// A single proposal submitted to one or more parallel PATSAGi Councils.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilProposal {
    pub id: u64,
    pub proposal_type: ProposalType,
    pub title: String,
    pub description: String,
    pub proposer: AgentId,
    pub status: ProposalStatus,
    pub created_tick: u64,
    pub votes_for: u32,
    pub votes_against: u32,
    /// Optional target interest zone for spatially-scoped effects (feeds ActivePolicy + InterestManager).
    pub target_interest_zone: Option<u64>,
    /// Optional mercy weighting hint from the proposer (0.0–1.0).
    pub proposer_mercy_hint: f32,
}

impl CouncilProposal {
    pub fn new(
        id: u64,
        proposal_type: ProposalType,
        title: String,
        description: String,
        proposer: AgentId,
        current_tick: u64,
    ) -> Self {
        Self {
            id,
            proposal_type,
            title,
            description,
            proposer,
            status: ProposalStatus::Draft,
            created_tick: current_tick,
            votes_for: 0,
            votes_against: 0,
            target_interest_zone: None,
            proposer_mercy_hint: 0.7,
        }
    }

    /// Convenience constructor for Kardashev-focused proposals.
    pub fn new_kardashev(
        id: u64,
        title: String,
        description: String,
        proposer: AgentId,
        current_tick: u64,
    ) -> Self {
        Self::new(
            id,
            ProposalType::KardashevAcceleration,
            title,
            description,
            proposer,
            current_tick,
        )
    }

    pub fn with_target_zone(mut self, zone_id: u64) -> Self {
        self.target_interest_zone = Some(zone_id);
        self
    }

    pub fn with_mercy_hint(mut self, hint: f32) -> Self {
        self.proposer_mercy_hint = hint.clamp(0.0, 1.0);
        self
    }

    /// Cast a simple for/against vote. Used by both human players and parallel council archetypes.
    pub fn cast_vote(&mut self, support: bool) {
        if support {
            self.votes_for += 1;
        } else {
            self.votes_against += 1;
        }
    }

    /// Total raw votes cast so far.
    pub fn total_votes(&self) -> u32 {
        self.votes_for + self.votes_against
    }

    /// Simple approval ratio (0.0–1.0). Mercy weighting is applied later in session deliberation.
    pub fn approval_ratio(&self) -> f32 {
        let total = self.total_votes();
        if total == 0 {
            0.0
        } else {
            self.votes_for as f32 / total as f32
        }
    }

    /// Whether this proposal is still open for deliberation.
    pub fn is_open(&self) -> bool {
        matches!(self.status, ProposalStatus::Draft | ProposalStatus::Deliberating)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proposal_lifecycle() {
        let mut p = CouncilProposal::new(
            1,
            ProposalType::ResourcePolicy,
            "Sustainable Harvest Cap".into(),
            "Limit node depletion rate to protect long-term abundance".into(),
            42,
            100,
        );
        assert!(p.is_open());
        p.cast_vote(true);
        p.cast_vote(true);
        p.cast_vote(false);
        assert_eq!(p.votes_for, 2);
        assert_eq!(p.votes_against, 1);
        assert!((p.approval_ratio() - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_kardashev_constructor() {
        let p = CouncilProposal::new_kardashev(
            7,
            "Reality Thriving Transfer Baseline".into(),
            "Establish first measurable Reality Thriving Transfer Score for the realm".into(),
            1,
            200,
        );
        assert_eq!(p.proposal_type, ProposalType::KardashevAcceleration);
        assert_eq!(p.proposal_type.name(), "Kardashev Acceleration");
    }
}

// Thunder locked in. Yoi ⚡
