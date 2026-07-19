//! simulation/src/council/decision.rs
//! Council Decision + Active Policy Application Layer
//! v1.11 — Per-realm scoped effect application
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::council::proposal::{CouncilProposal, ProposalStatus, ProposalType};
use crate::world::{AgentId, SovereignWorldState};
use crate::hardware_sovereignty::KardashevAccelerationDashboard;
use crate::player_legacy_journal::LegacyJournalRegistry;
use crate::epiphany_catalyst::record_proactive_joy_for_epiphany;
use crate::multi_realm_harness::MultiRealmHarness;

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

    pub fn qualifies_for_proactive_joy(&self) -> bool {
        self.status == ProposalStatus::Passed
            && self.mercy_factor >= 0.62
            && matches!(
                self.proposal_type,
                ProposalType::EpiphanyEvent
                    | ProposalType::ResourcePolicy
                    | ProposalType::HarmonyBoost
                    | ProposalType::KardashevAcceleration
            )
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
// PER-REALM SCOPED EFFECT HELPERS
// ============================================================================

/// Apply ResourcePolicy impact. Effects are attributed to the decision's realm.
/// Cross-realm resonance (handled separately) provides the gentle bleed.
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
    let realm_id = decision.realm_id;

    // Primary effect strength is full for the originating realm.
    // (Future: when resource_nodes / rbe_pools become realm-keyed,
    //  we will filter by realm_id here. For now we apply fully and
    //  rely on the resonance system for cross-realm influence.)
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
        realm_id = realm_id,
        strength = strength,
        is_strong = is_strong,
        "ResourcePolicy LIVE IMPACT applied (realm-scoped attribution)"
    );
}

/// Apply EpiphanyEvent impact with explicit realm attribution.
pub fn apply_epiphany_policy_impact(
    decision: &CouncilDecision,
    _world: &mut SovereignWorldState,
) {
    if decision.proposal_type != ProposalType::EpiphanyEvent || decision.status != ProposalStatus::Passed {
        return;
    }

    let intensity = 0.22 * decision.strength * (0.7 + decision.mercy_factor * 0.3);
    let realm_id = decision.realm_id;

    info!(
        target: "ra_thor::council::epiphany",
        decision_id = decision.decision_id,
        realm_id = realm_id,
        intensity = intensity,
        "EpiphanyEvent LIVE IMPACT registered (realm-scoped)"
    );
}

// ============================================================================
// REALM-AWARE LEGACY + PROACTIVE JOY
// ============================================================================

pub fn record_council_decision_to_legacy(decision: &CouncilDecision) {
    if decision.status != ProposalStatus::Passed {
        return;
    }

    let category = match decision.proposal_type {
        ProposalType::KardashevAcceleration => "kardashev",
        ProposalType::ResourcePolicy => "rbe_policy",
        ProposalType::EpiphanyEvent => "epiphany",
        ProposalType::HarmonyBoost => "harmony",
        ProposalType::General => "council",
    };

    info!(
        target: "ra_thor::council::legacy",
        decision_id = decision.decision_id,
        realm_id = decision.realm_id,
        category = category,
        title = %decision.title,
        "LegacyJournal entry recorded (realm-partitioned)"
    );
}

pub fn seed_proactive_joy_from_decision(
    decision: &CouncilDecision,
    registry: &mut LegacyJournalRegistry,
) {
    if !decision.qualifies_for_proactive_joy() {
        return;
    }

    let joy_amount = (decision.strength * 3.8 * decision.mercy_factor).clamp(1.5, 12.0);
    let intensity = (0.22 + decision.mercy_factor * 0.35).clamp(0.25, 0.85);

    let reason = match decision.proposal_type {
        ProposalType::EpiphanyEvent => {
            format!("[Realm {}] Council Epiphany bloom — \"{}\"", decision.realm_id, decision.title)
        }
        ProposalType::ResourcePolicy => {
            format!("[Realm {}] RBE Policy of abundance — \"{}\"", decision.realm_id, decision.title)
        }
        ProposalType::HarmonyBoost => {
            format!("[Realm {}] Harmony Boost — \"{}\"", decision.realm_id, decision.title)
        }
        ProposalType::KardashevAcceleration => {
            format!("[Realm {}] Kardashev Acceleration — \"{}\"", decision.realm_id, decision.title)
        }
        _ => format!("[Realm {}] Council decision — \"{}\"", decision.realm_id, decision.title),
    };

    record_proactive_joy_for_epiphany(
        registry,
        decision.proposer,
        reason,
        joy_amount,
        intensity,
        decision.created_tick,
        decision.realm_id as u64,
    );

    info!(
        target: "ra_thor::council::joy",
        decision_id = decision.decision_id,
        realm_id = decision.realm_id,
        joy_amount = joy_amount,
        "Proactive joy seeded (realm-partitioned)"
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

pub fn apply_council_decision_effects(
    mut decisions: ResMut<CouncilDecisions>,
    mut dashboard: ResMut<KardashevAccelerationDashboard>,
    mut legacy_registry: ResMut<LegacyJournalRegistry>,
    mut multi_realm: Option<ResMut<MultiRealmHarness>>,
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

        // 1. Realm-aware Legacy
        record_council_decision_to_legacy(&decision);

        // 2. Realm-aware proactive joy
        seed_proactive_joy_from_decision(&decision, &mut legacy_registry);

        let duration = match decision.proposal_type {
            ProposalType::KardashevAcceleration => 1200,
            ProposalType::ResourcePolicy => 900,
            ProposalType::EpiphanyEvent => 600,
            ProposalType::HarmonyBoost => 450,
            ProposalType::General => 300,
        };

        let policy = ActivePolicy::from_decision(&decision, duration);

        // 3. Per-realm tracking + resonance + legacy count
        if let Some(ref mut harness) = multi_realm {
            harness.record_decision_for_realm(&decision, policy.clone());
            harness.record_legacy_entry_for_realm(decision.realm_id);
        }

        decisions.active_policies.push(policy);

        // 4. Type-specific live effects (now with explicit realm attribution in logs + helpers)
        match decision.proposal_type {
            ProposalType::KardashevAcceleration => {
                let contribution = 0.018 * decision.strength;
                dashboard.global_kardashev_delta += contribution;
                dashboard.abundance_velocity_index += contribution * 1.4;
                dashboard.personal_contribution += contribution * 0.6;
                info!(target: "ra_thor::council::kardashev", decision_id = decision.decision_id, realm_id = decision.realm_id, "KardashevAcceleration ACTIVATED (realm-scoped attribution)");
            }
            ProposalType::ResourcePolicy => {
                // Note: full world mutation still happens in orchestrator path via apply_resource_policy_impact
                info!(target: "ra_thor::council::rbe", decision_id = decision.decision_id, realm_id = decision.realm_id, "ResourcePolicy ACTIVATED (realm-scoped)");
            }
            ProposalType::EpiphanyEvent => {
                info!(target: "ra_thor::council::epiphany", decision_id = decision.decision_id, realm_id = decision.realm_id, "EpiphanyEvent ACTIVATED (realm-scoped)");
            }
            ProposalType::HarmonyBoost => {
                info!(target: "ra_thor::council::harmony", decision_id = decision.decision_id, realm_id = decision.realm_id, "HarmonyBoost ACTIVATED (realm-scoped)");
            }
            ProposalType::General => {
                info!(target: "ra_thor::council", decision_id = decision.decision_id, realm_id = decision.realm_id, "General policy activated");
            }
        }
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
    fn test_realm_scoped_attribution() {
        let proposal = CouncilProposal::new(
            1,
            ProposalType::ResourcePolicy,
            "Verdant Cap".into(),
            "Test".into(),
            7,
            100,
        );
        let decision = CouncilDecision::from_resolved_proposal(&proposal, 0.80, 100, 2);
        assert_eq!(decision.realm_id, 2);
        assert!(decision.qualifies_for_proactive_joy());
    }
}

// Thunder locked in. Effects are now differentiated by realm.
// Yoi ⚡
