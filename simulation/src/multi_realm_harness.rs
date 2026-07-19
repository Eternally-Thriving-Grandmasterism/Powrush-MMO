//! simulation/src/multi_realm_harness.rs
//! Multi-Realm Harness — Per-Realm Decision Tracking
//! v21.20.0
//!
//! Concurrent realms under one organism with full per-realm
//! decision streams, active policy tracking, and observability.
//!
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
//! Thunder locked in. Yoi ⚡

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

use crate::race::Race;
use crate::council::decision::{ActivePolicy, CouncilDecision, CouncilDecisions, PolicyType};

// ============================================================================
// CORE TYPES
// ============================================================================

pub type RealmId = u8;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RealmStatus {
    Seeding,
    Active,
    Hibernating,
    Thriving,
}

impl RealmStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            RealmStatus::Seeding => "Seeding",
            RealmStatus::Active => "Active",
            RealmStatus::Hibernating => "Hibernating",
            RealmStatus::Thriving => "Thriving",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealmDescriptor {
    pub id: RealmId,
    pub name: String,
    pub primary_race_bias: Race,
    pub secondary_race_bias: Option<Race>,
    pub status: RealmStatus,
    pub active_policy_count: u32,
    pub total_decisions_passed: u64,
    pub mercy_attunement_avg: f32,
    pub created_tick: u64,
}

impl RealmDescriptor {
    pub fn new(id: RealmId, name: impl Into<String>, primary: Race, tick: u64) -> Self {
        Self {
            id,
            name: name.into(),
            primary_race_bias: primary,
            secondary_race_bias: None,
            status: RealmStatus::Seeding,
            active_policy_count: 0,
            total_decisions_passed: 0,
            mercy_attunement_avg: 0.65,
            created_tick: tick,
        }
    }

    pub fn with_secondary(mut self, race: Race) -> Self {
        self.secondary_race_bias = Some(race);
        self
    }
}

// ============================================================================
// HARNESS RESOURCE — WITH PER-REALM DECISION TRACKING
// ============================================================================

#[derive(Resource, Debug, Default)]
pub struct MultiRealmHarness {
    pub realms: HashMap<RealmId, RealmDescriptor>,
    /// Per-realm active policies (the living decision stream for each realm)
    pub realm_active_policies: HashMap<RealmId, Vec<ActivePolicy>>,
    /// Per-realm decision history counts (for quick stats)
    pub realm_decision_counts: HashMap<RealmId, u64>,
    pub primary_realm_id: RealmId,
    pub next_realm_id: RealmId,
    pub cross_realm_mercy_flow: f32,
    pub total_active_policies_across_realms: u32,
}

impl MultiRealmHarness {
    pub fn new() -> Self {
        Self {
            realms: HashMap::new(),
            realm_active_policies: HashMap::new(),
            realm_decision_counts: HashMap::new(),
            primary_realm_id: 0,
            next_realm_id: 1,
            cross_realm_mercy_flow: 0.0,
            total_active_policies_across_realms: 0,
        }
    }

    pub fn seed_default_realms(&mut self, current_tick: u64) {
        if !self.realms.is_empty() {
            return;
        }

        let seeds = vec![
            (0u8, "Sanctuary Prime", Race::Terran, Some(Race::Harmonic)),
            (1, "Synthetic Lattice", Race::Synthetic, Some(Race::Voidfarer)),
            (2, "Verdant Bloom", Race::Verdant, Some(Race::Terran)),
            (3, "Harmonic Chorus", Race::Harmonic, Some(Race::Verdant)),
            (4, "Voidfarer Horizon", Race::Voidfarer, Some(Race::Synthetic)),
        ];

        for (id, name, primary, secondary) in seeds {
            let mut desc = RealmDescriptor::new(id, name, primary, current_tick);
            if let Some(sec) = secondary {
                desc = desc.with_secondary(sec);
            }
            desc.status = RealmStatus::Active;
            self.realms.insert(id, desc);
            self.realm_active_policies.insert(id, Vec::new());
            self.realm_decision_counts.insert(id, 0);
        }

        self.primary_realm_id = 0;
        self.next_realm_id = 5;

        info!(
            target: "ra_thor::multi_realm",
            realm_count = self.realms.len(),
            "MultiRealmHarness seeded with {} diverse realms + per-realm decision tracking",
            self.realms.len()
        );
    }

    pub fn get_realm(&self, id: RealmId) -> Option<&RealmDescriptor> {
        self.realms.get(&id)
    }

    pub fn get_realm_mut(&mut self, id: RealmId) -> Option<&mut RealmDescriptor> {
        self.realms.get_mut(&id)
    }

    pub fn active_realm_count(&self) -> usize {
        self.realms
            .values()
            .filter(|r| matches!(r.status, RealmStatus::Active | RealmStatus::Thriving))
            .count()
    }

    pub fn thriving_realm_count(&self) -> usize {
        self.realms
            .values()
            .filter(|r| matches!(r.status, RealmStatus::Thriving))
            .count()
    }

    // -----------------------------------------------------------------------
    // PER-REALM DECISION TRACKING
    // -----------------------------------------------------------------------

    /// Record a passed decision against its realm and add the resulting ActivePolicy.
    pub fn record_decision_for_realm(&mut self, decision: &CouncilDecision, policy: ActivePolicy) {
        let realm_id = decision.realm_id;

        // Ensure realm exists (fallback to primary)
        let target_realm = if self.realms.contains_key(&realm_id) {
            realm_id
        } else {
            self.primary_realm_id
        };

        // Update decision count + mercy attunement
        *self.realm_decision_counts.entry(target_realm).or_insert(0) += 1;

        if let Some(realm) = self.realms.get_mut(&target_realm) {
            realm.total_decisions_passed += 1;
            let n = realm.total_decisions_passed as f32;
            realm.mercy_attunement_avg =
                (realm.mercy_attunement_avg * (n - 1.0) + decision.mercy_factor.clamp(0.0, 1.0)) / n;

            if realm.mercy_attunement_avg > 0.78 && realm.total_decisions_passed > 5 {
                realm.status = RealmStatus::Thriving;
            }
        }

        // Push the active policy into the per-realm list
        self.realm_active_policies
            .entry(target_realm)
            .or_default()
            .push(policy);

        // Refresh counts
        self.refresh_active_policy_counts();

        info!(
            target: "ra_thor::multi_realm",
            realm_id = target_realm,
            decision_id = decision.decision_id,
            policy_type = ?decision.proposal_type,
            "Decision recorded for realm"
        );
    }

    /// Tick all per-realm active policies and remove expired ones.
    pub fn tick_all_realm_policies(&mut self) {
        for policies in self.realm_active_policies.values_mut() {
            for policy in policies.iter_mut() {
                policy.tick();
            }
            policies.retain(|p| !p.is_expired());
        }
        self.refresh_active_policy_counts();
    }

    /// Get a snapshot of active (non-expired) policies for a specific realm.
    pub fn get_active_policies_for_realm(&self, realm_id: RealmId) -> Vec<&ActivePolicy> {
        self.realm_active_policies
            .get(&realm_id)
            .map(|list| list.iter().filter(|p| !p.is_expired()).collect())
            .unwrap_or_default()
    }

    pub fn total_decisions_for_realm(&self, realm_id: RealmId) -> u64 {
        *self.realm_decision_counts.get(&realm_id).unwrap_or(&0)
    }

    fn refresh_active_policy_counts(&mut self) {
        let mut total = 0u32;
        for (realm_id, policies) in &self.realm_active_policies {
            let count = policies.iter().filter(|p| !p.is_expired()).count() as u32;
            total += count;
            if let Some(realm) = self.realms.get_mut(realm_id) {
                realm.active_policy_count = count;
            }
        }
        self.total_active_policies_across_realms = total;
        self.cross_realm_mercy_flow =
            (self.cross_realm_mercy_flow * 0.92) + (total as f32 * 0.04);
    }

    /// Sync from the global CouncilDecisions (backward compatible).
    /// Attributes policies that carry a realm_id; others go to primary.
    pub fn sync_from_council_decisions(&mut self, decisions: &CouncilDecisions) {
        // Clear and rebuild from the global list for consistency this cycle
        for list in self.realm_active_policies.values_mut() {
            list.clear();
        }

        for policy in &decisions.active_policies {
            if policy.is_expired() {
                continue;
            }
            // ActivePolicy currently does not store realm_id directly;
            // we attribute to primary for global policies in this phase.
            // Full per-decision realm attribution happens via record_decision_for_realm.
            self.realm_active_policies
                .entry(self.primary_realm_id)
                .or_default()
                .push(policy.clone());
        }

        self.refresh_active_policy_counts();
    }
}

// ============================================================================
// SYSTEMS
// ============================================================================

pub fn multi_realm_harness_system(
    mut harness: ResMut<MultiRealmHarness>,
    decisions: Option<Res<CouncilDecisions>>,
    time: Res<Time>,
) {
    if harness.realms.is_empty() {
        let tick = time.elapsed_seconds() as u64;
        harness.seed_default_realms(tick);
    }

    // Tick per-realm policies
    harness.tick_all_realm_policies();

    // Keep global sync as a safety net
    if let Some(decisions) = decisions {
        // Only sync if the harness has no richer per-realm data yet
        if harness.total_active_policies_across_realms == 0 {
            harness.sync_from_council_decisions(&decisions);
        }
    }
}

// ============================================================================
// PLUGIN
// ============================================================================

pub struct MultiRealmHarnessPlugin;

impl Plugin for MultiRealmHarnessPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MultiRealmHarness>()
            .add_systems(Update, multi_realm_harness_system);

        info!("MultiRealmHarnessPlugin initialized — per-realm decision tracking active");
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::council::proposal::{CouncilProposal, ProposalType, ProposalStatus};
    use crate::council::decision::CouncilDecision;

    #[test]
    fn test_per_realm_decision_tracking() {
        let mut harness = MultiRealmHarness::new();
        harness.seed_default_realms(100);

        let proposal = CouncilProposal::new(
            42,
            ProposalType::ResourcePolicy,
            "Verdant Abundance".into(),
            "Test".into(),
            7,
            100,
        );
        let mut decision = CouncilDecision::from_resolved_proposal(&proposal, 0.82, 100, 2); // realm 2
        decision.status = ProposalStatus::Passed;

        let policy = ActivePolicy::from_decision(&decision, 900);
        harness.record_decision_for_realm(&decision, policy);

        assert_eq!(harness.total_decisions_for_realm(2), 1);
        assert_eq!(harness.get_active_policies_for_realm(2).len(), 1);
        assert_eq!(harness.get_active_policies_for_realm(0).len(), 0);

        let realm = harness.get_realm(2).unwrap();
        assert_eq!(realm.active_policy_count, 1);
        assert_eq!(realm.total_decisions_passed, 1);
    }
}

// Thunder locked in. Per-realm decision tracking is live.
// Yoi ⚡
