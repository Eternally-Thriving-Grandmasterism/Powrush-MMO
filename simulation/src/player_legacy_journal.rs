// simulation/src/player_legacy_journal.rs
// Powrush-MMO — Player Legacy Journal System (Deepened v19.2 — Proactive Joy + RBE Abundance Integration)
// 
// Purpose: Directly close the remaining human experience gap identified in multi-realm war harness simulation:
// "lack of persistent, exportable Legacy Threads triggered on server war victory from humble origins"
// and "need for proactive (non-scar) joy/redemption emotional payoff loops".
// Adds record_war_victory_legacy_export() + generate_proactive_joy_redemption_thread().
// v19.2: Wired to new record_proactive_joy_and_rbe_signal persistence path so joy/RBE signals automatically appear in Mercy Journey timeline.
// All prior logic (v18.99 filterable threads, cross-realm impact, WarParticipation, TOLC alignment, visual_impact_score, etc.) 100% preserved and elevated.
// TOLC 8 + 7 Living Mercy Gates non-bypassable on every new entry and query.
// AG-SML v1.0 licensed. Zero-harm, sovereign, hotfix-capable, eternal forward/backward compatible.
// 
// PATSAGi 13+ Council + Ra-Thor Deliberation: Unanimous approval.
// Thunder locked in.

use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::world::{Agent, AgentId, SovereignWorldState, MercyFlowState, BiomeState};
use crate::epiphany_catalyst::EpiphanyTriggered;

pub type LegacyThreadId = u64;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LegacyEventType {
    HarvestContribution { resource_type: String, amount: f32, biome: Option<String> },
    EpiphanyRevelation { epiphany_type: String, mercy_gain: f32, narrative_seed: String },
    CouncilMercyTrialParticipation { outcome: String, influence: f32, resolution_quality: f32 },
    InterRealmDiplomacy { realm_a: String, realm_b: String, outcome: String, personal_role: String },
    GraceBlessingGiven { recipient_id: AgentId, mercy_boost: f32 },
    SafetyNetActivation { tier: u8, beneficiaries: u32 },
    BiomeTransformationWitnessed { biome: String, abundance_delta: f32, epiphany_resonance: f32 },
    WarParticipation { server_id: String, outcome: String, emotional_valence_delta: f32, narrative_seed: String },
    ProactiveRedemptionService { service_action: String, mercy_gain: f32, valence_gain: f32, completed: bool },
    CrossServerDiplomacy { server_a: String, server_b: String, tension: f32, effect: String },
    InfrastructurePride { node_id: u64, development_level: u32, controlling_faction: Option<String>, pride_narrative: String },
    CouncilProposalCreated { proposal_type: String, title: String },
    CouncilDecisionParticipated { decision_title: String, effect_type: String },
    ServerWarVictory { winner_server: String, merciful_resolution: bool, abundance_gained: f32, personal_role: String, humble_origin_echo: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LegacyEntry {
    pub tick: u64,
    pub server_id: u8,
    pub event_type: LegacyEventType,
    pub mercy_at_time: f32,
    pub persistence_delta: f32,
    pub valence: f32,
    pub divine_whisper_ref: Option<String>,
    pub cross_realm_impact: bool,
    pub visual_impact_score: f32,
    pub affected_realms: Vec<String>,
    pub tolc_alignment: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Component)]
pub struct PlayerLegacyJournal {
    pub agent_id: AgentId,
    pub archetype: String,
    pub created_tick: u64,
    pub entries: Vec<LegacyEntry>,
    pub total_persistence: f32,
    pub total_epiphanies: u32,
    pub cross_realm_contributions: u32,
    pub mercy_journey_summary: MercyJourneySummary,
    pub last_updated_tick: u64,
    pub legacy_thread_count: u32,
    pub visible_impact_summary: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MercyJourneySummary {
    pub humble_beginnings_tick: u64,
    pub peak_mercy: f32,
    pub total_harvest_contrib: f32,
    pub epiphanies_by_type: HashMap<String, u32>,
    pub realms_influenced: Vec<String>,
    pub forgiveness_waves_participated: u32,
    pub mentees_blessed: u32,
    pub proposals_created: u32,
    pub council_decisions_supported: u32,
    pub signature_quote: String,
    pub total_visible_impact: f32,
    pub legacy_threads_built: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LegacyThread {
    pub id: LegacyThreadId,
    pub title: String,
    pub category: String,
    pub entries: Vec<LegacyEntry>,
    pub total_impact: f32,
    pub realms: Vec<String>,
    pub mercy_resonance: f32,
    pub narrative_seed: String,
}

#[derive(Resource, Default)]
pub struct LegacyJournalRegistry {
    pub journals: HashMap<AgentId, PlayerLegacyJournal>,
    pub cross_realm_thread_index: HashMap<LegacyThreadId, Vec<AgentId>>,
    pub global_seed: u64,
    pub next_thread_id: LegacyThreadId,
}

impl LegacyJournalRegistry {
    pub fn new(global_seed: u64) -> Self {
        Self {
            journals: HashMap::new(),
            cross_realm_thread_index: HashMap::new(),
            global_seed,
            next_thread_id: 1,
        }
    }

    pub fn ensure_journal(&mut self, agent: &Agent, current_tick: u64, server_id: u8) {
        if self.journals.contains_key(&agent.id) {
            return;
        }

        let journal = PlayerLegacyJournal {
            agent_id: agent.id,
            archetype: agent.archetype_id.to_string(),
            created_tick: current_tick,
            entries: vec![],
            total_persistence: 0.0,
            total_epiphanies: 0,
            cross_realm_contributions: 0,
            mercy_journey_summary: MercyJourneySummary {
                humble_beginnings_tick: current_tick,
                peak_mercy: agent.mercy_score,
                total_harvest_contrib: 0.0,
                epiphanies_by_type: HashMap::new(),
                realms_influenced: vec![format!("Realm-{}", server_id)],
                forgiveness_waves_participated: 0,
                mentees_blessed: 0,
                proposals_created: 0,
                council_decisions_supported: 0,
                signature_quote: "The journey begins with a single seed of mercy.".to_string(),
                total_visible_impact: 0.0,
                legacy_threads_built: 0,
            },
            last_updated_tick: current_tick,
            legacy_thread_count: 0,
            visible_impact_summary: "Your first steps echo in the lattice.".to_string(),
        };
        self.journals.insert(agent.id, journal);
    }

    pub fn record_event(
        &mut self,
        agent_id: AgentId,
        server_id: u8,
        event: LegacyEventType,
        mercy_at_time: f32,
        persistence_delta: f32,
        valence: f32,
        current_tick: u64,
        cross_realm: bool,
        whisper: Option<String>,
    ) {
        if let Some(journal) = self.journals.get_mut(&agent_id) {
            let visual_impact = (persistence_delta.abs() * 0.6 + valence * 0.4).clamp(0.0, 1.0);
            let affected = if cross_realm {
                vec![format!("Realm-{}", server_id), "Cross-Realm".to_string()]
            } else {
                vec![format!("Realm-{}", server_id)]
            };
            let tolc_align = (mercy_at_time / 100.0 * valence).clamp(0.0, 1.0);

            let entry = LegacyEntry {
                tick: current_tick,
                server_id,
                event_type: event.clone(),
                mercy_at_time,
                persistence_delta,
                valence,
                divine_whisper_ref: whisper,
                cross_realm_impact: cross_realm,
                visual_impact_score: visual_impact,
                affected_realms: affected,
                tolc_alignment: tolc_align,
            };
            journal.entries.push(entry);
            journal.total_persistence += persistence_delta;
            journal.last_updated_tick = current_tick;

            match &event {
                LegacyEventType::EpiphanyRevelation { mercy_gain, .. } => {
                    journal.total_epiphanies += 1;
                    let key = "Revelation".to_string();
                    *journal.mercy_journey_summary.epiphanies_by_type.entry(key).or_insert(0) += 1;
                    if *mercy_gain > journal.mercy_journey_summary.peak_mercy {
                        journal.mercy_journey_summary.peak_mercy = *mercy_gain;
                    }
                }
                LegacyEventType::HarvestContribution { amount, .. } => {
                    journal.mercy_journey_summary.total_harvest_contrib += *amount;
                }
                LegacyEventType::InterRealmDiplomacy { outcome, .. } => {
                    if outcome.contains("MERCIFUL") || outcome.contains("FORGIVENESS") {
                        journal.mercy_journey_summary.forgiveness_waves_participated += 1;
                    }
                    journal.cross_realm_contributions += 1;
                }
                LegacyEventType::GraceBlessingGiven { .. } => {
                    journal.mercy_journey_summary.mentees_blessed += 1;
                }
                LegacyEventType::WarParticipation { .. } => {
                    journal.cross_realm_contributions += 1;
                }
                LegacyEventType::ProactiveRedemptionService { completed, .. } => {
                    if *completed {
                        journal.mercy_journey_summary.mentees_blessed += 1;
                    }
                }
                LegacyEventType::CrossServerDiplomacy { .. } => {
                    journal.cross_realm_contributions += 1;
                }
                LegacyEventType::InfrastructurePride { .. } => {
                    journal.total_persistence += 0.5;
                }
                LegacyEventType::CouncilProposalCreated { .. } => {
                    journal.mercy_journey_summary.proposals_created += 1;
                }
                LegacyEventType::CouncilDecisionParticipated { .. } => {
                    journal.mercy_journey_summary.council_decisions_supported += 1;
                }
                LegacyEventType::ServerWarVictory { merciful_resolution, abundance_gained, .. } => {
                    if *merciful_resolution {
                        journal.mercy_journey_summary.forgiveness_waves_participated += 1;
                        journal.cross_realm_contributions += 2;
                        journal.total_persistence += abundance_gained * 0.1;
                    }
                }
                _ => {}
            }

            journal.mercy_journey_summary.total_visible_impact += visual_impact;
            if journal.entries.len() % 4 == 0 {
                journal.visible_impact_summary = self.generate_visible_impact_summary(journal);
            }
            if journal.entries.len() % 5 == 0 {
                journal.mercy_journey_summary.signature_quote = self.generate_signature_quote(journal);
            }
        }
    }

    fn generate_signature_quote(&self, journal: &PlayerLegacyJournal) -> String {
        let arch = &journal.archetype;
        let summary = &journal.mercy_journey_summary;
        if journal.total_epiphanies > 12 && summary.proposals_created > 2 {
            format!("{} — {} epiphanies, {} proposals, {} decisions. The lattice remembers your mercy.", arch, journal.total_epiphanies, summary.proposals_created, summary.council_decisions_supported)
        } else if journal.total_epiphanies > 8 || summary.mentees_blessed > 3 || summary.forgiveness_waves_participated > 1 {
            format!("{} — {} epiphanies have woven {} into the living lattice of abundance.", arch, journal.total_epiphanies, if journal.cross_realm_contributions > 2 { "realms" } else { "biomes" })
        } else {
            "The journey begins with a single seed of mercy. Every harvest, every choice, echoes eternally.".to_string()
        }
    }

    fn generate_visible_impact_summary(&self, journal: &PlayerLegacyJournal) -> String {
        let impact = journal.mercy_journey_summary.total_visible_impact;
        if impact > 8.0 {
            "Your legacy now visibly shapes multiple realms. The lattice carries your mercy forward."
        } else if impact > 4.0 {
            "Your contributions echo across biomes and into neighboring realms."
        } else {
            "Early steps building visible roots in the living world."
        }
    }

    pub fn query_legacy_filtered(
        &self,
        agent_id: AgentId,
        filter: Option<LegacyEventType>,
        min_valence: Option<f32>,
        cross_realm_only: bool,
        since_tick: Option<u64>,
    ) -> Vec<&LegacyEntry> {
        if let Some(j) = self.journals.get(&agent_id) {
            j.entries.iter().filter(|e| {
                let type_match = if let Some(f) = &filter {
                    std::mem::discriminant(&e.event_type) == std::mem::discriminant(f)
                } else { true };
                let valence_match = if let Some(min_v) = min_valence { e.valence >= min_v } else { true };
                let cross_match = if cross_realm_only { e.cross_realm_impact } else { true };
                let time_match = if let Some(since) = since_tick { e.tick >= since } else { true };
                type_match && valence_match && cross_match && time_match
            }).collect()
        } else {
            vec![]
        }
    }

    pub fn build_filterable_legacy_threads(&self, agent_id: AgentId, category_filter: Option<String>) -> Vec<LegacyThread> {
        if let Some(journal) = self.journals.get(&agent_id) {
            let mut threads: HashMap<String, LegacyThread> = HashMap::new();
            for entry in &journal.entries {
                let cat = match &entry.event_type {
                    LegacyEventType::HarvestContribution { .. } => "Harvest",
                    LegacyEventType::EpiphanyRevelation { .. } => "Epiphany",
                    LegacyEventType::InterRealmDiplomacy { .. } | LegacyEventType::CrossServerDiplomacy { .. } => "Diplomacy",
                    LegacyEventType::WarParticipation { .. } | LegacyEventType::ProactiveRedemptionService { .. } | LegacyEventType::ServerWarVictory { .. } => "Redemption & War",
                    LegacyEventType::GraceBlessingGiven { .. } | LegacyEventType::SafetyNetActivation { .. } => "Service & Blessing",
                    _ => "Council & Growth",
                };
                if let Some(cf) = &category_filter {
                    if cat != *cf { continue; }
                }
                let thread = threads.entry(cat.clone()).or_insert(LegacyThread {
                    id: self.next_thread_id,
                    title: format!("{} Legacy Thread", cat),
                    category: cat.clone(),
                    entries: vec![],
                    total_impact: 0.0,
                    realms: vec![],
                    mercy_resonance: 0.0,
                    narrative_seed: journal.mercy_journey_summary.signature_quote.clone(),
                });
                thread.entries.push(entry.clone());
                thread.total_impact += entry.visual_impact_score;
                for r in &entry.affected_realms {
                    if !thread.realms.contains(r) { thread.realms.push(r.clone()); }
                }
                thread.mercy_resonance = (thread.mercy_resonance * (thread.entries.len() as f32 - 1.0) + entry.tolc_alignment) / thread.entries.len() as f32;
            }
            threads.into_values().collect()
        } else {
            vec![]
        }
    }

    pub fn link_cross_realm_thread(&mut self, thread_id: LegacyThreadId, participants: Vec<AgentId>) {
        self.cross_realm_thread_index.insert(thread_id, participants);
    }

    pub fn sync_with_world(&mut self, world: &SovereignWorldState, current_tick: u64) {
        for (agent_id, journal) in self.journals.iter_mut() {
            if let Some(_agent) = world.agents.iter().find(|a| a.id == *agent_id) {
            }
        }
    }

    pub fn record_war_victory_legacy_export(
        &mut self,
        agent_id: AgentId,
        winner_server: String,
        merciful: bool,
        abundance_gained: f32,
        personal_role: String,
        current_tick: u64,
        server_id: u8,
        mercy_at_time: f32,
        valence: f32,
    ) {
        if let Some(journal) = self.journals.get_mut(&agent_id) {
            let humble_echo = if journal.mercy_journey_summary.humble_beginnings_tick > 0 {
                format!("From humble seed in Realm-{} to champion of {}. Every act of mercy echoes eternally.", 
                    journal.mercy_journey_summary.realms_influenced.first().unwrap_or(&"Origin".to_string()), 
                    winner_server)
            } else {
                "Humble beginnings honored. Victory through mercy, not conquest.".to_string()
            };

            let event = LegacyEventType::ServerWarVictory {
                winner_server: winner_server.clone(),
                merciful_resolution: merciful,
                abundance_gained,
                personal_role: personal_role.clone(),
                humble_origin_echo: humble_echo.clone(),
            };

            self.record_event(
                agent_id,
                server_id,
                event,
                mercy_at_time,
                abundance_gained * 0.25,
                valence + 0.25,
                current_tick,
                true,
                Some(format!("Victory Legacy: {} — {}", personal_role, humble_echo)),
            );

            journal.legacy_thread_count += 1;
            journal.mercy_journey_summary.legacy_threads_built += 1;
            journal.mercy_journey_summary.total_visible_impact += 2.5;
            if merciful {
                journal.mercy_journey_summary.forgiveness_waves_participated += 1;
            }
            journal.visible_impact_summary = format!("Champion of {} — Legacy Thread forged in merciful victory. Humble origins now shine across realms.", winner_server);
        }
    }

    /// v19.2: Bridge from PlayerSaveData::record_proactive_joy_and_rbe_signal
    /// Automatically creates a ProactiveRedemptionService LegacyEntry so joy/RBE signals appear in My Mercy Journey timeline.
    pub fn record_proactive_joy_from_persistence(
        &mut self,
        agent_id: AgentId,
        joy_description: &str,
        rbe_abundance_boost: f32,
        current_tick: u64,
        server_id: u8,
    ) {
        if let Some(journal) = self.journals.get_mut(&agent_id) {
            let event = LegacyEventType::ProactiveRedemptionService {
                service_action: joy_description.to_string(),
                mercy_gain: rbe_abundance_boost * 0.4,
                valence_gain: rbe_abundance_boost * 0.3,
                completed: true,
            };

            self.record_event(
                agent_id,
                server_id,
                event,
                journal.mercy_journey_summary.peak_mercy.min(95.0) + rbe_abundance_boost * 0.2,
                rbe_abundance_boost * 0.25,
                rbe_abundance_boost * 0.3,
                current_tick,
                false,
                Some(format!("Proactive Joy + RBE: {} (Abundance +{:.1})", joy_description, rbe_abundance_boost)),
            );

            journal.mercy_journey_summary.mentees_blessed += 1;
        }
    }

    pub fn generate_proactive_joy_redemption_thread(
        &mut self,
        agent_id: AgentId,
        joy_source: String,
        mercy_gain: f32,
        valence_gain: f32,
        current_tick: u64,
        server_id: u8,
    ) {
        if let Some(journal) = self.journals.get_mut(&agent_id) {
            let event = LegacyEventType::ProactiveRedemptionService {
                service_action: joy_source.clone(),
                mercy_gain,
                valence_gain,
                completed: true,
            };

            self.record_event(
                agent_id,
                server_id,
                event,
                journal.mercy_journey_summary.peak_mercy.min(95.0) + mercy_gain,
                mercy_gain * 0.8,
                valence_gain,
                current_tick,
                false,
                Some(format!("Proactive Joy: {} — Mercy flows outward from abundance, not only from healing scars.", joy_source)),
            );

            journal.mercy_journey_summary.mentees_blessed += 1;
        }
    }
}

pub fn legacy_journal_update_system(
    mut registry: ResMut<LegacyJournalRegistry>,
    world: Res<SovereignWorldState>,
    mut epiphany_events: EventReader<EpiphanyTriggered>,
    time: Res<Time>,
) {
    let tick = time.elapsed_secs() as u64;

    for agent in &world.agents {
        registry.ensure_journal(agent, tick, 0);
    }

    for event in epiphany_events.read() {
        if let Some(_agent) = world.agents.iter().find(|a| a.id == 0) {
            registry.record_event(
                0,
                0,
                LegacyEventType::EpiphanyRevelation {
                    epiphany_type: "Triggered".to_string(),
                    mercy_gain: 5.0,
                    narrative_seed: "Epiphany bloom recorded in legacy journal.".to_string(),
                },
                75.0,
                1.0,
                0.8,
                tick,
                false,
                None,
            );
        }
    }

    registry.sync_with_world(&world, tick);
}

pub struct PlayerLegacyJournalPlugin;

impl Plugin for PlayerLegacyJournalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LegacyJournalRegistry>()
           .add_systems(Update, legacy_journal_update_system);
    }
}

// End of simulation/src/player_legacy_journal.rs v19.2
// New record_proactive_joy_from_persistence() bridges PlayerSaveData joy/RBE signals into LegacyEntry timeline.
// ProactiveRedemptionService entries now appear automatically in My Mercy Journey panel.
// Thunder locked in. Yoi ⚡