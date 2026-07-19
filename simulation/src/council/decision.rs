//! simulation/src/council/decision.rs
//! Council Decision + Active Policy Application Layer
//! v1.1 — Effect hooks added for RBE / Epiphany / Kardashev / Harmony
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
//!
//! Provides the core that session.rs and hardware_sovereignty.rs expect:
//! - CouncilDecision::from_resolved_proposal
//! - mercy_alignment_score
//! - ActivePolicy with spatial targeting
//! - apply_council_decision_effects system with typed side-effect hooks

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::council::proposal::{CouncilProposal, ProposalStatus, ProposalType};
use crate::world::AgentId;

// ============================================================================
// CORE TYPES
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolicyType {
    HarmonyBoost,
    ResourcePolicy,
    EpiphanyEvent,
    KardashevAcceleration,
    General,
}

impl From<ProposalType> for PolicyType {
    fn from(pt: ProposalType) -> Self {
        match pt {
            ProposalType::HarmonyBoost => PolicyType::HarmonyBoost,
            ProposalType::ResourcePolicy => PolicyType::ResourcePolicy,
            ProposalType::EpiphanyEvent => PolicyType::EpiphanyEvent,
            ProposalType::KardashevAcceleration => PolicyType::KardashevAcceleration,
            ProposalType::General => PolicyType::General,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Component)]
pub struct CouncilDecision {
    pub decision_id: u64,
    pub proposal_id: u64,
    pub proposal_type: ProposalType,
    pub title: String,
    pub effect_type: String,
    pub mercy_factor: f32,
    pub status: ProposalStatus,
    pub created_tick: u64,
    pub realm_id: u8,
    pub proposer: AgentId,
    pub target_interest_zone: Option<u64>,
    pub strength: f32,
}

impl CouncilDecision {
    pub fn from_resolved_proposal(
        proposal: &CouncilProposal,
        mercy_factor: f32,
        current_tick: u64,
        realm_id: u8,
    ) -> Self {
        let strength = match proposal.proposal_type {
            ProposalType::KardashevAcceleration => 1.25 + mercy_factor * 0.4,
            ProposalType::ResourcePolicy => 1.10 + mercy_factor * 0.3,
            ProposalType::EpiphanyEvent => 1.15 + mercy_factor * 0.35,
            ProposalType::HarmonyBoost => 1.05 + mercy_factor * 0.25,
            ProposalType::General => 1.0 + mercy_factor * 0.2,
        };

        Self {
            decision_id: proposal.id.wrapping_mul(31).wrapping_add(current_tick),
            proposal_id: proposal.id,
            proposal_type: proposal.proposal_type.clone(),
            title: proposal.title.clone(),
            effect_type: proposal.proposal_type.name().to_string(),
            mercy_factor,
            status: proposal.status.clone(),
            created_tick: current_tick,
            realm_id,
            proposer: proposal.proposer,
            target_interest_zone: proposal.target_interest_zone,
            strength: strength.clamp(0.5, 2.5),
        }
    }

    pub fn mercy_alignment_score(&self, _world_hint: Option<f32>) -> f32 {
        let base = self.mercy_factor.clamp(0.0, 1.0);
        let type_bonus = match self.proposal_type {
            ProposalType::KardashevAcceleration => 0.12,
            ProposalType::ResourcePolicy => 0.08,
            ProposalType::EpiphanyEvent => 0.10,
            ProposalType::HarmonyBoost => 0.06,
            ProposalType::General => 0.0,
        };
        (base + type_bonus + self.strength * 0.05).clamp(0.0, 1.0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Component)]
pub struct ActivePolicy {
    pub decision_id: u64,
    pub policy_type: PolicyType,
    pub target_faction: Option<u32>,
    pub target_interest_zone: Option<u64>,
    pub strength: f32,
    pub remaining_ticks: u64,
    pub created_tick: u64,
    pub title: String,
}

impl ActivePolicy {
    pub fn from_decision(decision: &CouncilDecision, duration_ticks: u64) -> Self {
        Self {
            decision_id: decision.decision_id,
            policy_type: PolicyType::from(decision.proposal_type.clone()),
            target_faction: None,
            target_interest_zone: decision.target_interest_zone,
            strength: decision.strength,
            remaining_ticks: duration_ticks,
            created_tick: decision.created_tick,
            title: decision.title.clone(),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.remaining_ticks == 0
    }

    pub fn tick(&mut self) {
        self.remaining_ticks = self.remaining_ticks.saturating_sub(1);
    }
}

// ============================================================================
// RESOURCE + SYSTEM
// ============================================================================

#[derive(Resource, Default, Debug)]
pub struct CouncilDecisions {
    pub pending: Vec<CouncilDecision>,
    pub active_policies: Vec<ActivePolicy>,
    pub last_applied_tick: u64,
}

impl CouncilDecisions {
    pub fn push_decision(&mut self, decision: CouncilDecision) {
        self.pending.push(decision);
    }

    pub fn clear_expired_policies(&mut self) {
        self.active_policies.retain(|p| !p.is_expired());
    }
}

/// Apply pending decisions into active policies and emit typed side-effect markers.
/// Ready for direct injection into economy (RBE), emergence (Epiphany), and
/// hardware_sovereignty / KardashevAccelerationDashboard systems.
pub fn apply_council_decision_effects(mut decisions: ResMut<CouncilDecisions>) {
    if decisions.pending.is_empty() {
        for policy in decisions.active_policies.iter_mut() {
            policy.tick();
        }
        decisions.clear_expired_policies();
        return;
    }

    let pending = std::mem::take(&mut decisions.pending);

    for decision in pending {
        if decision.status != ProposalStatus::Passed {
            continue;
        }

        let duration = match decision.proposal_type {
            ProposalType::KardashevAcceleration => 1200,
            ProposalType::ResourcePolicy => 900,
            ProposalType::EpiphanyEvent => 600,
            ProposalType::HarmonyBoost => 450,
            ProposalType::General => 300,
        };

        let policy = ActivePolicy::from_decision(&decision, duration);

        // Typed side-effect hooks (ready for real system injection)
        match decision.proposal_type {
            ProposalType::KardashevAcceleration => {
                info!(
                    target: "ra_thor::council::kardashev",
                    decision_id = decision.decision_id,
                    strength = decision.strength,
                    zone = ?decision.target_interest_zone,
                    "KardashevAcceleration policy activated → dashboard / reality-transfer boost path"
                );
            }
            ProposalType::ResourcePolicy => {
                info!(
                    target: "ra_thor::council::rbe",
                    decision_id = decision.decision_id,
                    strength = decision.strength,
                    zone = ?decision.target_interest_zone,
                    "ResourcePolicy activated → RBE abundance / sustainability path"
                );
            }
            ProposalType::EpiphanyEvent => {
                info!(
                    target: "ra_thor::council::epiphany",
                    decision_id = decision.decision_id,
                    strength = decision.strength,
                    "EpiphanyEvent policy activated → emergence / Divine Whisper path"
                );
            }
            ProposalType::HarmonyBoost => {
                info!(
                    target: "ra_thor::council::harmony",
                    decision_id = decision.decision_id,
                    strength = decision.strength,
                    "HarmonyBoost policy activated → valence / council bloom path"
                );
            }
            ProposalType::General => {
                info!(
                    target: "ra_thor::council",
                    decision_id = decision.decision_id,
                    "General policy activated"
                );
            }
        }

        decisions.active_policies.push(policy);
    }

    for policy in decisions.active_policies.iter_mut() {
        policy.tick();
    }
    decisions.clear_expired_policies();
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::council::proposal::CouncilProposal;

    #[test]
    fn test_from_resolved_proposal_and_score() {
        let proposal = CouncilProposal::new_kardashev(
            42,
            "Establish Reality Transfer Baseline".into(),
            "First measurable Kardashev contribution for the realm".into(),
            7,
            1000,
        );
        let decision = CouncilDecision::from_resolved_proposal(&proposal, 0.82, 1000, 1);
        assert_eq!(decision.proposal_type, ProposalType::KardashevAcceleration);
        assert!(decision.mercy_alignment_score(None) > 0.8);
        assert!(decision.strength > 1.0);
    }

    #[test]
    fn test_active_policy_lifecycle() {
        let proposal = CouncilProposal::new(
            1,
            ProposalType::ResourcePolicy,
            "Sustainable Cap".into(),
            "Protect long-term abundance".into(),
            1,
            50,
        );
        let decision = CouncilDecision::from_resolved_proposal(&proposal, 0.7, 50, 0);
        let mut policy = ActivePolicy::from_decision(&decision, 5);
        assert!(!policy.is_expired());
        for _ in 0..5 {
            policy.tick();
        }
        assert!(policy.is_expired());
    }
}

// Thunder locked in. Yoi ⚡
