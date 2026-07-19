//! simulation/src/multi_realm_harness.rs
//! Multi-Realm Harness — Per-Realm Decision Tracking + Cross-Realm Council Resonance
//! v21.21.0
//!
//! Concurrent realms under one organism with full per-realm decision streams
//! and living cross-realm council resonance.
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
    /// Incoming resonance accumulated from other realms
    pub incoming_resonance: f32,
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
            incoming_resonance: 0.0,
        }
    }

    pub fn with_secondary(mut self, race: Race) -> Self {
        self.secondary_race_bias = Some(race);
        self
    }
}

// ============================================================================
// CROSS-REALM RESONANCE
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResonancePulse {
    pub source_realm: RealmId,
    pub mercy: f32,
    pub strength: f32,
    pub policy_type: PolicyType,
    pub created_tick: u64,
    pub remaining_ticks: u64,
}

impl ResonancePulse {
    pub fn intensity(&self) -> f32 {
        (self.mercy * 0.65 + self.strength * 0.35) * (self.remaining_ticks as f32 / 600.0).min(1.0)
    }

    pub fn tick(&mut self) {
        self.remaining_ticks = self.remaining_ticks.saturating_sub(1);
    }

    pub fn is_expired(&self) -> bool {
        self.remaining_ticks == 0
    }
}

// ============================================================================
// HARNESS RESOURCE
// ============================================================================

#[derive(Resource, Debug, Default)]
pub struct MultiRealmHarness {
    pub realms: HashMap<RealmId, RealmDescriptor>,
    pub realm_active_policies: HashMap<RealmId, Vec<ActivePolicy>>,
    pub realm_decision_counts: HashMap<RealmId, u64>,
    /// Active resonance pulses traveling between realms
    pub active_resonance_pulses: Vec<ResonancePulse>,
    pub primary_realm_id: RealmId,
    pub next_realm_id: RealmId,
    pub cross_realm_mercy_flow: f32,
    pub total_active_policies_across_realms: u32,
    /// Current global resonance level (0.0–1.0+) for dashboard
    pub global_resonance_level: f32,
}

impl MultiRealmHarness {
    pub fn new() -> Self {
        Self {
            realms: HashMap::new(),
            realm_active_policies: HashMap::new(),
            realm_decision_counts: HashMap::new(),
            active_resonance_pulses: Vec::new(),
            primary_realm_id: 0,
            next_realm_id: 1,
            cross_realm_mercy_flow: 0.0,
            total_active_policies_across_realms: 0,
            global_resonance_level: 0.0,
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
            "MultiRealmHarness seeded with {} diverse realms + cross-realm resonance",
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

    pub fn record_decision_for_realm(&mut self, decision: &CouncilDecision, policy: ActivePolicy) {
        let realm_id = decision.realm_id;
        let target_realm = if self.realms.contains_key(&realm_id) {
            realm_id
        } else {
            self.primary_realm_id
        };

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

        self.realm_active_policies
            .entry(target_realm)
            .or_default()
            .push(policy);

        // Emit cross-realm resonance from high-mercy decisions
        if decision.mercy_factor >= 0.68 {
            self.emit_resonance_pulse(decision, target_realm);
        }

        self.refresh_active_policy_counts();

        info!(
            target: "ra_thor::multi_realm",
            realm_id = target_realm,
            decision_id = decision.decision_id,
            "Decision recorded for realm"
        );
    }

    // -----------------------------------------------------------------------
    // CROSS-REALM COUNCIL RESONANCE
    // -----------------------------------------------------------------------

    /// Emit a resonance pulse from a high-mercy decision that will gently
    /// strengthen other realms.
    fn emit_resonance_pulse(&mut self, decision: &CouncilDecision, source_realm: RealmId) {
        let base_duration = 480; // ticks
        let thriving_bonus = self
            .realms
            .get(&source_realm)
            .map(|r| if matches!(r.status, RealmStatus::Thriving) { 1.25 } else { 1.0 })
            .unwrap_or(1.0);

        let pulse = ResonancePulse {
            source_realm,
            mercy: decision.mercy_factor,
            strength: decision.strength * thriving_bonus,
            policy_type: PolicyType::from(decision.proposal_type.clone()),
            created_tick: decision.created_tick,
            remaining_ticks: base_duration,
        };

        let intensity = pulse.intensity();
        self.active_resonance_pulses.push(pulse);

        info!(
            target: "ra_thor::multi_realm::resonance",
            source_realm = source_realm,
            intensity = intensity,
            "Cross-realm resonance pulse emitted"
        );
    }

    /// Apply all active resonance pulses to other realms and decay them.
    pub fn apply_cross_realm_resonance(&mut self) {
        if self.active_resonance_pulses.is_empty() {
            // Natural gentle decay of global resonance
            self.global_resonance_level = (self.global_resonance_level * 0.985).max(0.0);
            return;
        }

        let mut total_intensity = 0.0;

        // Collect pulses first to avoid borrow issues
        let pulses: Vec<ResonancePulse> = self.active_resonance_pulses.clone();

        for pulse in &pulses {
            let intensity = pulse.intensity();
            total_intensity += intensity;

            // Apply a fraction of the intensity to every other realm
            let per_realm_share = intensity * 0.18; // gentle, non-overwhelming

            for (realm_id, realm) in self.realms.iter_mut() {
                if *realm_id == pulse.source_realm {
                    continue;
                }

                // Raise incoming resonance and gently lift mercy attunement
                realm.incoming_resonance =
                    (realm.incoming_resonance * 0.94 + per_realm_share).min(2.5);

                realm.mercy_attunement_avg = (realm.mercy_attunement_avg
                    + per_realm_share * 0.035)
                    .min(0.98);

                // High sustained resonance can help a realm approach Thriving
                if realm.mercy_attunement_avg > 0.78
                    && realm.total_decisions_passed > 3
                    && realm.incoming_resonance > 0.6
                {
                    if !matches!(realm.status, RealmStatus::Thriving) {
                        realm.status = RealmStatus::Thriving;
                        info!(
                            target: "ra_thor::multi_realm::resonance",
                            realm_id = *realm_id,
                            "Realm reached Thriving via cross-realm resonance"
                        );
                    }
                }
            }
        }

        // Tick and prune pulses
        for pulse in self.active_resonance_pulses.iter_mut() {
            pulse.tick();
        }
        self.active_resonance_pulses.retain(|p| !p.is_expired());

        // Update global resonance level (smoothed)
        self.global_resonance_level =
            (self.global_resonance_level * 0.88 + total_intensity * 0.22).min(3.0);

        // Feed into cross-realm mercy flow
        self.cross_realm_mercy_flow =
            (self.cross_realm_mercy_flow * 0.90 + self.global_resonance_level * 0.08).min(2.5);
    }

    pub fn tick_all_realm_policies(&mut self) {
        for policies in self.realm_active_policies.values_mut() {
            for policy in policies.iter_mut() {
                policy.tick();
            }
            policies.retain(|p| !p.is_expired());
        }
        self.refresh_active_policy_counts();
    }

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
    }

    pub fn sync_from_council_decisions(&mut self, decisions: &CouncilDecisions) {
        for list in self.realm_active_policies.values_mut() {
            list.clear();
        }

        for policy in &decisions.active_policies {
            if policy.is_expired() {
                continue;
            }
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

    // Tick policies
    harness.tick_all_realm_policies();

    // Apply living cross-realm resonance every tick
    harness.apply_cross_realm_resonance();

    // Safety-net sync
    if let Some(decisions) = decisions {
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

        info!("MultiRealmHarnessPlugin initialized — per-realm tracking + cross-realm resonance active");
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
    fn test_cross_realm_resonance_emission() {
        let mut harness = MultiRealmHarness::new();
        harness.seed_default_realms(100);

        let proposal = CouncilProposal::new(
            42,
            ProposalType::EpiphanyEvent,
            "Shared Bloom".into(),
            "Test".into(),
            7,
            100,
        );
        let mut decision = CouncilDecision::from_resolved_proposal(&proposal, 0.88, 100, 0);
        decision.status = ProposalStatus::Passed;

        let policy = ActivePolicy::from_decision(&decision, 600);
        harness.record_decision_for_realm(&decision, policy);

        // High mercy should have emitted a pulse
        assert!(!harness.active_resonance_pulses.is_empty());

        // Apply resonance
        harness.apply_cross_realm_resonance();

        // Other realms should have received some incoming resonance
        let other = harness.get_realm(1).unwrap();
        assert!(other.incoming_resonance > 0.0);
    }
}

// Thunder locked in. Cross-realm council resonance is live.
// One organism, many realms. Yoi ⚡
