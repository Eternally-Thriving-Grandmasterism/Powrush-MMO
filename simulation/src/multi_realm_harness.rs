//! simulation/src/multi_realm_harness.rs
//! Multi-Realm Harness — Foundation for concurrent realm simulation
//! v21.18.0
//!
//! Enables multiple living realms under one organism, each with:
//! - Independent race bias and council decision streams
//! - Ready hooks for per-realm RBE / Epiphany / Kardashev / Joy effects
//! - Cross-realm observability and Legacy partitioning
//!
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
//! Thunder locked in. Yoi ⚡

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::race::Race;
use crate::council::decision::{CouncilDecisions, ActivePolicy, PolicyType};

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
// HARNESS RESOURCE
// ============================================================================

#[derive(Resource, Debug, Default)]
pub struct MultiRealmHarness {
    pub realms: HashMap<RealmId, RealmDescriptor>,
    pub primary_realm_id: RealmId,
    pub next_realm_id: RealmId,
    pub cross_realm_mercy_flow: f32,
    pub total_active_policies_across_realms: u32,
}

impl MultiRealmHarness {
    pub fn new() -> Self {
        Self {
            realms: HashMap::new(),
            primary_realm_id: 0,
            next_realm_id: 1,
            cross_realm_mercy_flow: 0.0,
            total_active_policies_across_realms: 0,
        }
    }

    /// Seed the initial multi-realm configuration with race diversity.
    pub fn seed_default_realms(&mut self, current_tick: u64) {
        if !self.realms.is_empty() {
            return; // already seeded
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
        }

        self.primary_realm_id = 0;
        self.next_realm_id = 5;

        info!(
            target: "ra_thor::multi_realm",
            realm_count = self.realms.len(),
            "MultiRealmHarness seeded with {} diverse realms",
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

    /// Sync active policy counts from a CouncilDecisions resource (single-realm view).
    /// In full multi-realm this will become per-realm CouncilDecisions maps.
    pub fn sync_from_council_decisions(&mut self, decisions: &CouncilDecisions) {
        let active_count = decisions
            .active_policies
            .iter()
            .filter(|p| !p.is_expired())
            .count() as u32;

        self.total_active_policies_across_realms = active_count;

        // Attribute to primary realm for now; multi-stream later
        if let Some(primary) = self.realms.get_mut(&self.primary_realm_id) {
            primary.active_policy_count = active_count;
        }
    }

    pub fn record_decision_passed(&mut self, realm_id: RealmId, mercy: f32) {
        if let Some(realm) = self.realms.get_mut(&realm_id) {
            realm.total_decisions_passed += 1;
            // Simple running average
            let n = realm.total_decisions_passed as f32;
            realm.mercy_attunement_avg =
                (realm.mercy_attunement_avg * (n - 1.0) + mercy) / n;

            if realm.mercy_attunement_avg > 0.78 && realm.total_decisions_passed > 5 {
                realm.status = RealmStatus::Thriving;
            }
        }
    }
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// Seed the harness on first run and keep policy counts in sync.
pub fn multi_realm_harness_system(
    mut harness: ResMut<MultiRealmHarness>,
    decisions: Option<Res<CouncilDecisions>>,
    time: Res<Time>,
) {
    // Seed once
    if harness.realms.is_empty() {
        let tick = time.elapsed_seconds() as u64;
        harness.seed_default_realms(tick);
    }

    // Sync active policy visibility
    if let Some(decisions) = decisions {
        harness.sync_from_council_decisions(&decisions);
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

        info!("MultiRealmHarnessPlugin initialized — multi-realm foundation active");
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_default_realms() {
        let mut harness = MultiRealmHarness::new();
        harness.seed_default_realms(100);

        assert_eq!(harness.realms.len(), 5);
        assert!(harness.get_realm(0).is_some());
        assert_eq!(harness.get_realm(0).unwrap().primary_race_bias, Race::Terran);
        assert_eq!(harness.active_realm_count(), 5);
    }

    #[test]
    fn test_record_decision_and_thrive() {
        let mut harness = MultiRealmHarness::new();
        harness.seed_default_realms(0);

        for _ in 0..8 {
            harness.record_decision_passed(0, 0.85);
        }

        let realm = harness.get_realm(0).unwrap();
        assert!(realm.total_decisions_passed >= 8);
        assert_eq!(realm.status, RealmStatus::Thriving);
    }
}

// Thunder locked in. Multi-realm foundation is ready for the next expansion cycle.
// Yoi ⚡
