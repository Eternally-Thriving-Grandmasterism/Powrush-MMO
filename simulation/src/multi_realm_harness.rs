//! simulation/src/multi_realm_harness.rs
//! Multi-Realm Harness — Per-Realm Decision Tracking + Cross-Realm Resonance + Legacy Partition Counts
//! v21.22.0
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
    pub incoming_resonance: f32,
    /// Number of LegacyJournal entries originating from this realm
    pub legacy_entry_count: u64,
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
            legacy_entry_count: 0,
        }
    }

    pub fn with_secondary(mut self, race: Race) -> Self {
        self.secondary_race_bias = Some(race);
        self
    }
}

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
    pub active_resonance_pulses: Vec<ResonancePulse>,
    pub primary_realm_id: RealmId,
    pub next_realm_id: RealmId,
    pub cross_realm_mercy_flow: f32,
    pub total_active_policies_across_realms: u32,
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
            "MultiRealmHarness seeded — decision tracking + resonance + legacy partition ready"
        );
    }

    pub fn get_realm(&self, id: RealmId) -> Option<&RealmDescriptor> {
        self.realms.get(&id)
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
    // PER-REALM DECISION + LEGACY TRACKING
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

        if decision.mercy_factor >= 0.68 {
            self.emit_resonance_pulse(decision, target_realm);
        }

        self.refresh_active_policy_counts();
    }

    /// Increment the LegacyJournal entry counter for a realm.
    pub fn record_legacy_entry_for_realm(&mut self, realm_id: RealmId) {
        let target = if self.realms.contains_key(&realm_id) {
            realm_id
        } else {
            self.primary_realm_id
        };

        if let Some(realm) = self.realms.get_mut(&target) {
            realm.legacy_entry_count += 1;
        }
    }

    pub fn total_legacy_entries_for_realm(&self, realm_id: RealmId) -> u64 {
        self.realms
            .get(&realm_id)
            .map(|r| r.legacy_entry_count)
            .unwrap_or(0)
    }

    // -----------------------------------------------------------------------
    // CROSS-REALM RESONANCE
    // -----------------------------------------------------------------------

    fn emit_resonance_pulse(&mut self, decision: &CouncilDecision, source_realm: RealmId) {
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
            remaining_ticks: 480,
        };

        self.active_resonance_pulses.push(pulse);
    }

    pub fn apply_cross_realm_resonance(&mut self) {
        if self.active_resonance_pulses.is_empty() {
            self.global_resonance_level = (self.global_resonance_level * 0.985).max(0.0);
            return;
        }

        let mut total_intensity = 0.0;
        let pulses: Vec<ResonancePulse> = self.active_resonance_pulses.clone();

        for pulse in &pulses {
            let intensity = pulse.intensity();
            total_intensity += intensity;
            let per_realm_share = intensity * 0.18;

            for (realm_id, realm) in self.realms.iter_mut() {
                if *realm_id == pulse.source_realm {
                    continue;
                }
                realm.incoming_resonance =
                    (realm.incoming_resonance * 0.94 + per_realm_share).min(2.5);
                realm.mercy_attunement_avg =
                    (realm.mercy_attunement_avg + per_realm_share * 0.035).min(0.98);

                if realm.mercy_attunement_avg > 0.78
                    && realm.total_decisions_passed > 3
                    && realm.incoming_resonance > 0.6
                {
                    realm.status = RealmStatus::Thriving;
                }
            }
        }

        for pulse in self.active_resonance_pulses.iter_mut() {
            pulse.tick();
        }
        self.active_resonance_pulses.retain(|p| !p.is_expired());

        self.global_resonance_level =
            (self.global_resonance_level * 0.88 + total_intensity * 0.22).min(3.0);
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
            if !policy.is_expired() {
                self.realm_active_policies
                    .entry(self.primary_realm_id)
                    .or_default()
                    .push(policy.clone());
            }
        }
        self.refresh_active_policy_counts();
    }
}

// ============================================================================
// SYSTEMS + PLUGIN
// ============================================================================

pub fn multi_realm_harness_system(
    mut harness: ResMut<MultiRealmHarness>,
    decisions: Option<Res<CouncilDecisions>>,
    time: Res<Time>,
) {
    if harness.realms.is_empty() {
        harness.seed_default_realms(time.elapsed_seconds() as u64);
    }

    harness.tick_all_realm_policies();
    harness.apply_cross_realm_resonance();

    if let Some(decisions) = decisions {
        if harness.total_active_policies_across_realms == 0 {
            harness.sync_from_council_decisions(&decisions);
        }
    }
}

pub struct MultiRealmHarnessPlugin;

impl Plugin for MultiRealmHarnessPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MultiRealmHarness>()
            .add_systems(Update, multi_realm_harness_system);

        info!("MultiRealmHarnessPlugin — decision tracking + resonance + legacy partition active");
    }
}

// Thunder locked in. LegacyJournal entries are now partitioned by realm.
// Yoi ⚡
