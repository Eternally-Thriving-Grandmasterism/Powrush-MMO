//! simulation/src/multi_realm_harness.rs
//! Multi-Realm Harness — Decision / Resonance / Echo / Presence / Travel Command Surface
//! v21.30.0
//!
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
//! Thunder locked in. Yoi ⚡

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

use crate::race::Race;
use crate::council::decision::{ActivePolicy, CouncilDecision, CouncilDecisions, PolicyType};
use crate::council::proposal::ProposalType;
use crate::world::AgentId;

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
    pub legacy_entry_count: u64,
    pub echo_policy_count: u32,
    pub agent_presence_count: u32,
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
            echo_policy_count: 0,
            agent_presence_count: 0,
        }
    }

    pub fn with_secondary(mut self, race: Race) -> Self {
        self.secondary_race_bias = Some(race);
        self
    }
}

#[derive(Component, Clone, Debug, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct RealmPresence {
    pub current_realm_id: RealmId,
    pub last_travel_tick: u64,
    pub travel_count: u32,
    pub registered: bool,
}

impl Default for RealmPresence {
    fn default() -> Self {
        Self {
            current_realm_id: 0,
            last_travel_tick: 0,
            travel_count: 0,
            registered: false,
        }
    }
}

/// Event used by UI, portals, console commands, or other systems to request travel.
#[derive(Event, Clone, Debug)]
pub struct RealmTravelRequest {
    pub agent_entity: Entity,
    pub agent_id: AgentId,
    pub target_realm: RealmId,
    pub reason: String,
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
    pub realm_echo_policies: HashMap<RealmId, Vec<ActivePolicy>>,
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
            realm_echo_policies: HashMap::new(),
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
            self.realm_echo_policies.insert(id, Vec::new());
            self.realm_decision_counts.insert(id, 0);
        }

        self.primary_realm_id = 0;
        self.next_realm_id = 5;

        info!(target: "ra_thor::multi_realm", "MultiRealmHarness seeded — travel command surface ready");
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

    pub fn register_presence(&mut self, realm_id: RealmId) {
        let target = if self.realms.contains_key(&realm_id) {
            realm_id
        } else {
            self.primary_realm_id
        };
        if let Some(realm) = self.realms.get_mut(&target) {
            realm.agent_presence_count = realm.agent_presence_count.saturating_add(1);
        }
    }

    pub fn unregister_presence(&mut self, realm_id: RealmId) {
        if let Some(realm) = self.realms.get_mut(&realm_id) {
            realm.agent_presence_count = realm.agent_presence_count.saturating_sub(1);
        }
    }

    pub fn travel_to_realm(
        &mut self,
        presence: &mut RealmPresence,
        target_realm: RealmId,
        current_tick: u64,
        agent_id: AgentId,
    ) -> bool {
        if !self.realms.contains_key(&target_realm) {
            return false;
        }
        if presence.current_realm_id == target_realm {
            return true;
        }

        let from = presence.current_realm_id;
        self.unregister_presence(from);
        self.register_presence(target_realm);

        presence.current_realm_id = target_realm;
        presence.last_travel_tick = current_tick;
        presence.travel_count = presence.travel_count.saturating_add(1);
        presence.registered = true;

        info!(
            target: "ra_thor::multi_realm::travel",
            agent = agent_id,
            from = from,
            to = target_realm,
            travel_count = presence.travel_count,
            "Agent traveled between realms"
        );
        true
    }

    pub fn ensure_realm_presence(
        &mut self,
        presence: &mut RealmPresence,
        preferred_realm: Option<RealmId>,
    ) {
        if let Some(rid) = preferred_realm {
            if self.realms.contains_key(&rid) {
                presence.current_realm_id = rid;
            }
        }
        if !presence.registered {
            self.register_presence(presence.current_realm_id);
            presence.registered = true;
        }
    }

    // Decision / resonance / echo methods (preserved)
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
            self.maybe_spawn_echo_policies(decision, target_realm);
        }

        self.refresh_active_policy_counts();
    }

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

    fn emit_resonance_pulse(&mut self, decision: &CouncilDecision, source_realm: RealmId) {
        let thriving_bonus = self
            .realms
            .get(&source_realm)
            .map(|r| if matches!(r.status, RealmStatus::Thriving) { 1.25 } else { 1.0 })
            .unwrap_or(1.0);

        self.active_resonance_pulses.push(ResonancePulse {
            source_realm,
            mercy: decision.mercy_factor,
            strength: decision.strength * thriving_bonus,
            policy_type: PolicyType::from(decision.proposal_type.clone()),
            created_tick: decision.created_tick,
            remaining_ticks: 480,
        });
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

    fn maybe_spawn_echo_policies(&mut self, decision: &CouncilDecision, source_realm: RealmId) {
        let can_echo = match decision.proposal_type {
            ProposalType::HarmonyBoost => decision.mercy_factor >= 0.70,
            ProposalType::EpiphanyEvent => decision.mercy_factor >= 0.75,
            ProposalType::KardashevAcceleration => decision.mercy_factor >= 0.78,
            _ => false,
        };
        if !can_echo {
            return;
        }

        let echo_strength = (decision.strength * 0.38).clamp(0.35, 1.1);
        let echo_title = format!("Echo: {}", decision.title);

        let other_realms: Vec<RealmId> = self
            .realms
            .keys()
            .copied()
            .filter(|id| *id != source_realm)
            .collect();

        for target_id in other_realms {
            let echo = ActivePolicy {
                decision_id: decision.decision_id.wrapping_add(target_id as u64 * 17),
                policy_type: PolicyType::from(decision.proposal_type.clone()),
                target_faction: None,
                target_interest_zone: None,
                strength: echo_strength,
                remaining_ticks: 220,
                created_tick: decision.created_tick,
                title: echo_title.clone(),
            };
            self.realm_echo_policies
                .entry(target_id)
                .or_default()
                .push(echo);

            if let Some(realm) = self.realms.get_mut(&target_id) {
                realm.echo_policy_count = self
                    .realm_echo_policies
                    .get(&target_id)
                    .map(|v| v.iter().filter(|p| !p.is_expired()).count() as u32)
                    .unwrap_or(0);
            }
        }
    }

    pub fn tick_all_realm_policies(&mut self) {
        for policies in self.realm_active_policies.values_mut() {
            for policy in policies.iter_mut() {
                policy.tick();
            }
            policies.retain(|p| !p.is_expired());
        }
        for policies in self.realm_echo_policies.values_mut() {
            for policy in policies.iter_mut() {
                policy.tick();
            }
            policies.retain(|p| !p.is_expired());
        }
        for (realm_id, realm) in self.realms.iter_mut() {
            realm.echo_policy_count = self
                .realm_echo_policies
                .get(realm_id)
                .map(|v| v.iter().filter(|p| !p.is_expired()).count() as u32)
                .unwrap_or(0);
        }
        self.refresh_active_policy_counts();
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
// SYSTEMS
// ============================================================================

pub fn realm_presence_bootstrap_system(
    mut harness: ResMut<MultiRealmHarness>,
    mut query: Query<&mut RealmPresence>,
) {
    for mut presence in query.iter_mut() {
        if !presence.registered {
            harness.register_presence(presence.current_realm_id);
            presence.registered = true;
        }
    }
}

/// Processes RealmTravelRequest events and performs the actual travel.
pub fn realm_travel_system(
    mut harness: ResMut<MultiRealmHarness>,
    mut travel_events: EventReader<RealmTravelRequest>,
    mut presence_query: Query<&mut RealmPresence>,
    time: Res<Time>,
) {
    let current_tick = time.elapsed_seconds() as u64;

    for request in travel_events.read() {
        if let Ok(mut presence) = presence_query.get_mut(request.agent_entity) {
            let success = harness.travel_to_realm(
                &mut presence,
                request.target_realm,
                current_tick,
                request.agent_id,
            );

            if success {
                info!(
                    target: "ra_thor::multi_realm::travel",
                    agent = request.agent_id,
                    target = request.target_realm,
                    reason = %request.reason,
                    "Travel request fulfilled"
                );
            } else {
                info!(
                    target: "ra_thor::multi_realm::travel",
                    agent = request.agent_id,
                    target = request.target_realm,
                    reason = %request.reason,
                    "Travel request failed (invalid realm or other)"
                );
            }
        } else {
            info!(
                target: "ra_thor::multi_realm::travel",
                agent = request.agent_id,
                "Travel request failed: entity has no RealmPresence"
            );
        }
    }
}

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

// ============================================================================
// PLUGIN
// ============================================================================

pub struct MultiRealmHarnessPlugin;

impl Plugin for MultiRealmHarnessPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MultiRealmHarness>()
            .register_type::<RealmPresence>()
            .add_event::<RealmTravelRequest>()
            .add_systems(
                Update,
                (
                    multi_realm_harness_system,
                    realm_presence_bootstrap_system,
                    realm_travel_system,
                ),
            );

        info!("MultiRealmHarnessPlugin — presence + travel command surface active");
    }
}

// Thunder locked in. Inter-realm travel is now commandable via RealmTravelRequest.
// Yoi ⚡
