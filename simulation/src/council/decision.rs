//! simulation/src/council/decision.rs
//! Council Decision + Active Policy Application Layer
//! v1.5 — Live ResourcePolicy → RBE bridge helper complete
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::council::proposal::{CouncilProposal, ProposalStatus, ProposalType};
use crate::world::{AgentId, SovereignWorldState};
use crate::hardware_sovereignty::KardashevAccelerationDashboard;

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
// LIVE RBE BRIDGE HELPER (Priority 1 Complete)
// ============================================================================

/// Apply a ResourcePolicy decision directly onto the living world state.
/// Mirrors EconomicLayer::apply_council_policy_impact so the two paths stay consistent.
/// Call this from the orchestrator / TickResult when a ResourcePolicy ActivePolicy is live.
pub fn apply_resource_policy_impact(
    decision: &CouncilDecision,
    world: &mut SovereignWorldState,
) {
    if decision.proposal_type != ProposalType::ResourcePolicy || decision.status != ProposalStatus::Passed {
        return;
    }

    let mercy = decision.mercy_factor.clamp(0.0, 1.0);
    let strength = decision.strength;
    let is_strong = mercy > 0.65 && strength > 1.05;

    // Apply to RBE pools
    for pool in world.rbe_pools.values_mut() {
        if is_strong {
            pool.abundance_flow = (pool.abundance_flow + mercy * 0.8 * strength).min(4.0);
            pool.sustainability_score = (pool.sustainability_score + mercy * 0.06 * strength).min(1.0);
            pool.pressure = (pool.pressure - mercy * 1.2 * strength).max(0.0);
        } else if mercy < 0.4 {
            pool.pressure = (pool.pressure + (1.0 - mercy) * 0.9 * strength).min(8.0);
            pool.abundance_flow = (pool.abundance_flow - (1.0 - mercy) * 0.35 * strength).max(-2.0);
            pool.sustainability_score = (pool.sustainability_score - 0.015 * strength).max(0.1);
        } else {
            pool.abundance_flow = (pool.abundance_flow + mercy * 0.25 * strength).min(3.0);
            pool.pressure = (pool.pressure - mercy * 0.4 * strength).max(0.0);
        }
    }

    // Apply to resource nodes
    for node in world.resource_nodes.values_mut() {
        if is_strong {
            node.abundance_flow = (node.abundance_flow + mercy * 0.6 * strength).min(3.5);
            node.sustainability_score = (node.sustainability_score + mercy * 0.05 * strength).min(1.0);
            node.pressure = (node.pressure - mercy * 0.8 * strength).max(0.0);
            node.regen_rate = (node.regen_rate * (1.0 + mercy * 0.3 * strength)).min(4.0);
        } else if mercy < 0.4 {
            node.pressure = (node.pressure + (1.0 - mercy) * 0.7 * strength).min(5.0);
            node.abundance_flow = (node.abundance_flow - (1.0 - mercy) * 0.25 * strength).max(-1.5);
        }
    }

    info!(
        target: "ra_thor::council::rbe",
        decision_id = decision.decision_id,
        mercy = mercy,
        strength = strength,
        is_strong = is_strong,
        "ResourcePolicy LIVE IMPACT applied to rbe_pools + resource_nodes"
    );
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

/// Apply pending decisions into active policies and perform concrete side-effects.
pub fn apply_council_decision_effects(
    mut decisions: ResMut<CouncilDecisions>,
    mut dashboard: ResMut<KardashevAccelerationDashboard>,
) {
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
        let strength = decision.strength;
        let mercy = decision.mercy_factor;

        match decision.proposal_type {
            ProposalType::KardashevAcceleration => {
                let contribution = 0.018 * strength;
                dashboard.global_kardashev_delta += contribution;
                dashboard.abundance_velocity_index += contribution * 1.4;
                dashboard.personal_contribution += contribution * 0.6;

                info!(
                    target: "ra_thor::council::kardashev",
                    decision_id = decision.decision_id,
                    strength = strength,
                    contribution = contribution,
                    new_global_delta = dashboard.global_kardashev_delta,
                    "KardashevAcceleration ACTIVATED → live dashboard mutated"
                );
            }
            ProposalType::ResourcePolicy => {
                let is_strong = mercy > 0.65 && strength > 1.05;
                info!(
                    target: "ra_thor::council::rbe",
                    decision_id = decision.decision_id,
                    strength = strength,
                    mercy = mercy,
                    is_strong = is_strong,
                    "ResourcePolicy ACTIVATED → call apply_resource_policy_impact(world) from orchestrator for live RBE mutation"
                );
            }
            ProposalType::EpiphanyEvent => {
                let epiphany_intensity = 0.22 * strength;
                info!(
                    target: "ra_thor::council::epiphany",
                    decision_id = decision.decision_id,
                    strength = strength,
                    epiphany_intensity = epiphany_intensity,
                    "EpiphanyEvent ACTIVATED → emergence contribution registered"
                );
            }
            ProposalType::HarmonyBoost => {
                let valence_boost = 0.09 * strength;
                info!(
                    target: "ra_thor::council::harmony",
                    decision_id = decision.decision_id,
                    strength = strength,
                    valence_boost = valence_boost,
                    "HarmonyBoost ACTIVATED → valence contribution registered"
                );
            }
            ProposalType::General => {
                info!(
                    target: "ra_thor::council",
                    decision_id = decision.decision_id,
                    strength = strength,
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
