//! simulation/src/multi_realm_harness.rs
//! Multi-Realm Harness — Decision / Resonance / Echo / Presence / Travel / Attunement / Titles / Bonuses / Abundance / Origin / Live Ingest
//! v21.53.0 — Harness-Derived Live Ingest (Demo → Live promotion)
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

#[derive(Clone, Debug, Default)]
pub struct TitleBonus {
    pub attunement_gain_mult: f32,
    pub resonance_whisper: f32,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize, Reflect, Default)]
#[reflect(Component)]
pub struct RealmAttunement {
    pub per_realm: HashMap<RealmId, f32>,
    pub total: f32,
    pub peak_realm: Option<RealmId>,
    pub peak_value: f32,
}

impl RealmAttunement {
    pub fn get(&self, realm_id: RealmId) -> f32 {
        *self.per_realm.get(&realm_id).unwrap_or(&0.0)
    }

    pub fn add(&mut self, realm_id: RealmId, amount: f32) {
        let entry = self.per_realm.entry(realm_id).or_insert(0.0);
        *entry = (*entry + amount).min(1.5);
        self.total = (self.total + amount).min(8.0);

        if *entry > self.peak_value {
            self.peak_value = *entry;
            self.peak_realm = Some(realm_id);
        }
    }

    pub fn living_title(&self, current_realm: RealmId) -> String {
        let current = self.get(current_realm);
        let realm_short = match current_realm {
            0 => "Sanctuary",
            1 => "Lattice",
            2 => "Verdant",
            3 => "Chorus",
            4 => "Horizon",
            _ => "Realm",
        };

        let realm_title = if current >= 1.0 {
            format!("Heart of {}", realm_short)
        } else if current >= 0.75 {
            format!("Attuned of {}", realm_short)
        } else if current >= 0.50 {
            format!("Resident of {}", realm_short)
        } else if current >= 0.25 {
            format!("Seeker of {}", realm_short)
        } else {
            String::new()
        };

        let total_honor = if self.total >= 4.0 {
            " • Living Lattice"
        } else if self.total >= 2.5 {
            " • Realm Weaver"
        } else if self.total >= 1.0 {
            " • Multi-Realm Traveler"
        } else {
            ""
        };

        if realm_title.is_empty() && total_honor.is_empty() {
            "Presence accumulating...".to_string()
        } else if realm_title.is_empty() {
            total_honor.trim_start_matches(" • ").to_string()
        } else {
            format!("{}{}", realm_title, total_honor)
        }
    }

    pub fn title_bonus(&self, current_realm: RealmId) -> TitleBonus {
        let current = self.get(current_realm);

        let (att_mult, realm_whisper) = if current >= 1.0 {
            (1.18, 0.004)
        } else if current >= 0.75 {
            (1.12, 0.0025)
        } else if current >= 0.50 {
            (1.07, 0.0015)
        } else if current >= 0.25 {
            (1.03, 0.0008)
        } else {
            (1.0, 0.0)
        };

        let total_whisper = if self.total >= 4.0 {
            0.006
        } else if self.total >= 2.5 {
            0.0035
        } else if self.total >= 1.0 {
            0.0015
        } else {
            0.0
        };

        TitleBonus {
            attunement_gain_mult: att_mult,
            resonance_whisper: realm_whisper + total_whisper,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RealmAbundanceView {
    pub realm_id: RealmId,
    pub node_count: u32,
    pub total_current_yield: f32,
    pub average_sustainability: f32,
    pub average_abundance_flow: f32,
    pub average_stress: f32,
    pub restricted_node_count: u32,
    pub thriving_node_count: u32,
}

impl RealmAbundanceView {
    pub fn from_raw(
        realm_id: RealmId,
        node_count: u32,
        total_current_yield: f32,
        average_sustainability: f32,
        average_abundance_flow: f32,
        average_stress: f32,
        restricted_node_count: u32,
        thriving_node_count: u32,
    ) -> Self {
        Self {
            realm_id,
            node_count,
            total_current_yield,
            average_sustainability,
            average_abundance_flow,
            average_stress,
            restricted_node_count,
            thriving_node_count,
        }
    }

    pub fn is_thriving(&self) -> bool {
        self.node_count > 0
            && self.average_sustainability > 0.72
            && self.average_stress < 0.35
            && self.average_abundance_flow > -0.05
    }

    pub fn health_label(&self) -> &'static str {
        if self.node_count == 0 {
            "Empty"
        } else if self.is_thriving() {
            "Thriving"
        } else if self.average_stress > 0.6 || self.average_sustainability < 0.45 {
            "Stressed"
        } else if self.average_abundance_flow > 0.15 {
            "Abundant"
        } else {
            "Steady"
        }
    }
}

#[derive(Resource, Clone, Debug, Default)]
pub struct RealmAbundanceObservatory {
    pub views: HashMap<RealmId, RealmAbundanceView>,
    pub last_updated_tick: u64,
    pub has_live_data: bool,
}

impl RealmAbundanceObservatory {
    pub fn upsert(&mut self, view: RealmAbundanceView, tick: u64) {
        self.views.insert(view.realm_id, view);
        self.last_updated_tick = tick;
        self.has_live_data = true;
    }

    pub fn ingest_many(&mut self, views: impl IntoIterator<Item = RealmAbundanceView>, tick: u64) {
        for view in views {
            self.views.insert(view.realm_id, view);
        }
        self.last_updated_tick = tick;
        self.has_live_data = true;
    }

    pub fn get(&self, realm_id: RealmId) -> Option<&RealmAbundanceView> {
        self.views.get(&realm_id)
    }

    pub fn all_sorted(&self) -> Vec<&RealmAbundanceView> {
        let mut v: Vec<_> = self.views.values().collect();
        v.sort_by_key(|view| view.realm_id);
        v
    }

    pub fn clear(&mut self) {
        self.views.clear();
        self.has_live_data = false;
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OriginProvenanceView {
    pub realm_id: RealmId,
    pub total_amount: f32,
    pub resource_types: u32,
}

#[derive(Resource, Clone, Debug, Default)]
pub struct OriginProvenanceObservatory {
    pub per_realm: HashMap<RealmId, OriginProvenanceView>,
    pub last_updated_tick: u64,
    pub has_live_data: bool,
}

impl OriginProvenanceObservatory {
    pub fn ingest_many(&mut self, views: impl IntoIterator<Item = OriginProvenanceView>, tick: u64) {
        for view in views {
            self.per_realm.insert(view.realm_id, view);
        }
        self.last_updated_tick = tick;
        self.has_live_data = true;
    }

    pub fn get(&self, realm_id: RealmId) -> Option<&OriginProvenanceView> {
        self.per_realm.get(&realm_id)
    }

    pub fn total_tracked(&self) -> f32 {
        self.per_realm.values().map(|v| v.total_amount).sum()
    }

    pub fn clear(&mut self) {
        self.per_realm.clear();
        self.has_live_data = false;
    }
}

#[derive(Event, Clone, Debug)]
pub struct AbundanceIngestEvent {
    pub views: Vec<RealmAbundanceView>,
    pub tick: u64,
}

#[derive(Event, Clone, Debug)]
pub struct OriginIngestEvent {
    pub views: Vec<OriginProvenanceView>,
    pub tick: u64,
}

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

        info!(target: "ra_thor::multi_realm", "MultiRealmHarness seeded — full multi-realm organism ready");
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

    /// True when the harness shows living activity worth promoting Demo → Live.
    pub fn has_living_activity(&self) -> bool {
        self.total_active_policies_across_realms > 0
            || self.cross_realm_mercy_flow > 0.01
            || self.global_resonance_level > 0.01
            || self.realms.values().any(|r| r.agent_presence_count > 0)
            || self.realms.values().any(|r| r.total_decisions_passed > 0)
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
// PURE DERIVATION HELPERS — live views from living harness metrics
// ============================================================================

/// Derive abundance views from living MultiRealmHarness state.
/// Mercy-aligned: more presence, mercy, and thriving status → healthier nodes.
pub fn derive_abundance_from_harness(harness: &MultiRealmHarness) -> Vec<RealmAbundanceView> {
    let mut views = Vec::new();
    for realm in harness.realms.values() {
        let presence = realm.agent_presence_count as f32;
        let mercy = realm.mercy_attunement_avg.clamp(0.0, 1.0);
        let thriving = matches!(realm.status, RealmStatus::Thriving);
        let policies = realm.active_policy_count as f32;
        let resonance = realm.incoming_resonance.min(2.0);

        let base_nodes = 4u32 + (presence as u32).min(8) + if thriving { 4 } else { 0 };
        let node_count = base_nodes + (policies as u32).min(6);
        let yield_base = 6.0 + presence * 3.5 + mercy * 12.0 + if thriving { 8.0 } else { 0.0 };
        let sust = (0.55 + mercy * 0.35 + if thriving { 0.08 } else { 0.0 }).min(0.98);
        let flow = (-0.05 + mercy * 0.25 + resonance * 0.08).clamp(-0.2, 0.45);
        let stress = (0.45 - mercy * 0.3 - if thriving { 0.1 } else { 0.0 } + (1.0 - sust) * 0.2)
            .clamp(0.05, 0.75);
        let restricted = if stress > 0.55 { 1 } else { 0 };
        let thriving_nodes = ((node_count as f32) * sust * (1.0 - stress)).round() as u32;

        views.push(RealmAbundanceView::from_raw(
            realm.id,
            node_count,
            yield_base,
            sust,
            flow,
            stress,
            restricted,
            thriving_nodes.min(node_count),
        ));
    }
    views.sort_by_key(|v| v.realm_id);
    views
}

/// Derive origin provenance views from living MultiRealmHarness state.
/// Presence + decisions + mercy shape soft harvest weights.
pub fn derive_origin_from_harness(harness: &MultiRealmHarness) -> Vec<OriginProvenanceView> {
    let mut views = Vec::new();
    for realm in harness.realms.values() {
        let presence = realm.agent_presence_count as f32;
        let decisions = realm.total_decisions_passed as f32;
        let mercy = realm.mercy_attunement_avg.clamp(0.0, 1.0);
        let amount = 5.0 + presence * 12.0 + decisions * 1.5 + mercy * 20.0;
        let types = 1u32 + (presence > 0.0) as u32 + (mercy > 0.7) as u32 + (decisions > 2.0) as u32;

        views.push(OriginProvenanceView {
            realm_id: realm.id,
            total_amount: amount,
            resource_types: types,
        });
    }
    views.sort_by_key(|v| v.realm_id);
    views
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

pub fn realm_attunement_system(
    time: Res<Time>,
    mut harness: ResMut<MultiRealmHarness>,
    mut query: Query<(&RealmPresence, &mut RealmAttunement)>,
) {
    let dt = time.delta_seconds();
    let base_gain = 0.012 * dt;

    for (presence, mut attunement) in query.iter_mut() {
        let bonus = attunement.title_bonus(presence.current_realm_id);
        let gain = base_gain * bonus.attunement_gain_mult;

        attunement.add(presence.current_realm_id, gain);

        if bonus.resonance_whisper > 0.0 {
            harness.cross_realm_mercy_flow =
                (harness.cross_realm_mercy_flow + bonus.resonance_whisper * dt).min(2.5);
        }
    }
}

pub fn realm_attunement_bootstrap_system(
    mut commands: Commands,
    query: Query<Entity, (With<RealmPresence>, Without<RealmAttunement>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(RealmAttunement::default());
    }
}

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
            }
        }
    }
}

pub fn abundance_ingest_system(
    mut events: EventReader<AbundanceIngestEvent>,
    mut observatory: ResMut<RealmAbundanceObservatory>,
) {
    for event in events.read() {
        if event.views.is_empty() {
            continue;
        }
        observatory.ingest_many(event.views.clone(), event.tick);
        info!(
            target: "ra_thor::multi_realm::abundance",
            count = event.views.len(),
            tick = event.tick,
            "Live abundance ingested into observatory"
        );
    }
}

pub fn origin_ingest_system(
    mut events: EventReader<OriginIngestEvent>,
    mut observatory: ResMut<OriginProvenanceObservatory>,
) {
    for event in events.read() {
        if event.views.is_empty() {
            continue;
        }
        observatory.ingest_many(event.views.clone(), event.tick);
        info!(
            target: "ra_thor::multi_realm::origin",
            count = event.views.len(),
            tick = event.tick,
            "Live origin provenance ingested"
        );
    }
}

/// Soft demo seed — only when observatories are empty and no live data has arrived yet.
pub fn soft_demo_abundance_seed_system(
    mut abundance: ResMut<RealmAbundanceObservatory>,
    mut origin: ResMut<OriginProvenanceObservatory>,
    harness: Res<MultiRealmHarness>,
    time: Res<Time>,
) {
    if harness.realms.is_empty() {
        return;
    }

    let tick = time.elapsed_seconds() as u64;

    if !abundance.has_live_data && abundance.views.is_empty() {
        let demos = [
            (0u8, 12u32, 28.5f32, 0.88f32, 0.22f32, 0.12f32, 0u32, 9u32),
            (1, 8, 14.2, 0.71, 0.08, 0.28, 1, 4),
            (2, 15, 36.0, 0.91, 0.31, 0.09, 0, 13),
            (3, 6, 11.4, 0.78, 0.15, 0.18, 0, 4),
            (4, 4, 6.8, 0.55, -0.02, 0.42, 1, 1),
        ];

        for (id, n, y, sus, flow, stress, rest, thr) in demos {
            if harness.realms.contains_key(&id) {
                abundance.views.insert(
                    id,
                    RealmAbundanceView::from_raw(id, n, y, sus, flow, stress, rest, thr),
                );
            }
        }
        abundance.last_updated_tick = tick;
        info!(target: "ra_thor::multi_realm::abundance", "Soft demo abundance seeded (will yield to live data)");
    }

    if !origin.has_live_data && origin.per_realm.is_empty() {
        let demos = [
            (0u8, 42.0f32, 3u32),
            (1, 18.5, 2),
            (2, 55.0, 4),
            (3, 12.0, 2),
            (4, 6.5, 1),
        ];

        for (id, amount, types) in demos {
            if harness.realms.contains_key(&id) {
                origin.per_realm.insert(
                    id,
                    OriginProvenanceView {
                        realm_id: id,
                        total_amount: amount,
                        resource_types: types,
                    },
                );
            }
        }
        origin.last_updated_tick = tick;
        info!(target: "ra_thor::multi_realm::origin", "Soft demo origin provenance seeded (will yield to live data)");
    }
}

/// Concrete shared-app tick: promote Demo → Live when harness shows real activity.
/// Emits AbundanceIngestEvent + OriginIngestEvent derived from living harness metrics.
/// Soft refresh ~every 8 seconds once live so the dashboard stays responsive.
pub fn harness_derived_live_ingest_system(
    harness: Res<MultiRealmHarness>,
    abundance: Res<RealmAbundanceObservatory>,
    origin: Res<OriginProvenanceObservatory>,
    mut abundance_writer: EventWriter<AbundanceIngestEvent>,
    mut origin_writer: EventWriter<OriginIngestEvent>,
    time: Res<Time>,
    mut last_emit: Local<f32>,
) {
    if harness.realms.is_empty() || !harness.has_living_activity() {
        return;
    }

    let now = time.elapsed_seconds();
    let already_live = abundance.has_live_data && origin.has_live_data;

    // First promotion: immediate. Subsequent: soft refresh every ~8s.
    if already_live && (now - *last_emit) < 8.0 {
        return;
    }

    let tick = now as u64;
    let abundance_views = derive_abundance_from_harness(&harness);
    let origin_views = derive_origin_from_harness(&harness);

    if !abundance_views.is_empty() {
        abundance_writer.send(AbundanceIngestEvent {
            views: abundance_views,
            tick,
        });
    }
    if !origin_views.is_empty() {
        origin_writer.send(OriginIngestEvent {
            views: origin_views,
            tick,
        });
    }

    *last_emit = now;

    info!(
        target: "ra_thor::multi_realm::live_ingest",
        tick = tick,
        first_promotion = !already_live,
        "Harness-derived live ingest emitted (Demo → Live)"
    );
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
            .init_resource::<RealmAbundanceObservatory>()
            .init_resource::<OriginProvenanceObservatory>()
            .register_type::<RealmPresence>()
            .register_type::<RealmAttunement>()
            .add_event::<RealmTravelRequest>()
            .add_event::<AbundanceIngestEvent>()
            .add_event::<OriginIngestEvent>()
            .add_systems(
                Update,
                (
                    multi_realm_harness_system,
                    realm_presence_bootstrap_system,
                    realm_attunement_bootstrap_system,
                    realm_attunement_system,
                    realm_travel_system,
                    soft_demo_abundance_seed_system,
                    harness_derived_live_ingest_system,
                    abundance_ingest_system,
                    origin_ingest_system,
                ).chain(),
            );

        info!("MultiRealmHarnessPlugin — full organism + harness-derived live ingest active");
    }
}

// Thunder locked in.
// Concrete tick: harness_derived_live_ingest_system promotes Demo → Live on real activity.
// External bridges may still send AbundanceIngestEvent / OriginIngestEvent directly.
// Yoi ⚡
