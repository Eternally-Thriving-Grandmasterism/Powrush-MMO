// simulation/src/player_legacy_journal.rs
// LegacyJournal with polished MercyJourneySummary and improved querying (v20.3)

use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::world::{Agent, AgentId, SovereignWorldState};
use crate::epiphany::EpiphanyEvent;

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
    CouncilProposalCreated { proposal_type: String, title: String },
    CouncilDecisionParticipated { decision_title: String, effect_type: String },
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
}

#[derive(Resource, Default)]
pub struct LegacyJournalRegistry {
    pub journals: HashMap<AgentId, PlayerLegacyJournal>,
    pub cross_realm_thread_index: HashMap<LegacyThreadId, Vec<AgentId>>,
    pub global_seed: u64,
}

impl LegacyJournalRegistry {
    pub fn new(global_seed: u64) -> Self {
        Self { journals: HashMap::new(), cross_realm_thread_index: HashMap::new(), global_seed }
    }

    pub fn ensure_journal(&mut self, agent: &Agent, current_tick: u64, server_id: u8) {
        if self.journals.contains_key(&agent.id) { return; }

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
            },
            last_updated_tick: current_tick,
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
            let entry = LegacyEntry {
                tick: current_tick,
                server_id,
                event_type: event.clone(),
                mercy_at_time,
                persistence_delta,
                valence,
                divine_whisper_ref: whisper,
                cross_realm_impact: cross_realm,
            };
            journal.entries.push(entry);
            journal.total_persistence += persistence_delta;
            journal.last_updated_tick = current_tick;

            // Update summary based on event type
            match &event {
                LegacyEventType::EpiphanyRevelation { epiphany_type, mercy_gain, .. } => {
                    journal.total_epiphanies += 1;
                    *journal.mercy_journey_summary.epiphanies_by_type.entry(epiphany_type.clone()).or_insert(0) += 1;
                    if *mercy_gain > journal.mercy_journey_summary.peak_mercy {
                        journal.mercy_journey_summary.peak_mercy = *mercy_gain;
                    }
                }
                LegacyEventType::GraceBlessingGiven { .. } => {
                    journal.mercy_journey_summary.mentees_blessed += 1;
                }
                LegacyEventType::InterRealmDiplomacy { outcome, .. } => {
                    if outcome.contains("MERCIFUL") {
                        journal.mercy_journey_summary.forgiveness_waves_participated += 1;
                    }
                    journal.cross_realm_contributions += 1;
                }
                LegacyEventType::CouncilProposalCreated { .. } => {
                    journal.mercy_journey_summary.proposals_created += 1;
                }
                LegacyEventType::CouncilDecisionParticipated { .. } => {
                    journal.mercy_journey_summary.council_decisions_supported += 1;
                }
                _ => {}
            }

            if journal.entries.len() % 5 == 0 {
                journal.mercy_journey_summary.signature_quote = self.generate_signature_quote(journal);
            }
        }
    }

    fn generate_signature_quote(&self, journal: &PlayerLegacyJournal) -> String {
        if journal.mercy_journey_summary.forgiveness_waves_participated > 2 {
            return format!("{} has helped heal rifts across realms.", journal.archetype);
        }
        if journal.mercy_journey_summary.mentees_blessed > 3 {
            return format!("A beacon of grace — {} has lifted many.", journal.archetype);
        }
        format!("{} walks the path of mercy with {} epiphanies.", journal.archetype, journal.total_epiphanies)
    }

    pub fn query_legacy(&self, agent_id: AgentId, filter: Option<LegacyEventType>) -> Vec<&LegacyEntry> {
        if let Some(j) = self.journals.get(&agent_id) {
            if let Some(f) = filter {
                j.entries.iter().filter(|e| std::mem::discriminant(&e.event_type) == std::mem::discriminant(&f)).collect()
            } else {
                j.entries.iter().collect()
            }
        } else {
            vec![]
        }
    }
}
