// simulation/src/inter_realm_diplomacy_event.rs
// InterRealmDiplomacyEvent v20.1 - Complete file with Council integration

use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::world::{Agent, AgentId, RbeResourcePool};
use crate::player_legacy_journal::{LegacyJournalRegistry, LegacyEventType, LegacyThreadId};
use crate::grace_blessing::{GraceBlessing, BlessingContext, calculate_grace_blessing};
use crate::council::decision::CouncilDecisions;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilDeliberationInput {
    pub average_mercy_of_participants: f32,
    pub vote_ratio: f32,
    pub resolution_quality: f32,
    pub dominant_archetype_influence: f32,
}

impl CouncilDeliberationInput {
    pub fn determine_outcome(&self) -> DiplomacyOutcome {
        if self.vote_ratio > 0.78 && self.resolution_quality > 0.75 {
            DiplomacyOutcome::MercifulResolution
        } else if self.vote_ratio > 0.55 {
            DiplomacyOutcome::StableDiplomacy
        } else if self.vote_ratio > 0.35 {
            DiplomacyOutcome::FracturedTension
        } else {
            DiplomacyOutcome::EscalatedConflict
        }
    }

    pub fn calculate_redemption_score(&self) -> f32 {
        let base = (self.vote_ratio * 0.6 + self.resolution_quality * 0.4).clamp(0.0, 1.0);
        let mercy_bonus = (self.average_mercy_of_participants / 100.0) * 0.15;
        (base + mercy_bonus).clamp(0.0, 1.0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiplomacyOutcome {
    MercifulResolution,
    StableDiplomacy,
    FracturedTension,
    EscalatedConflict,
}

#[derive(Clone, Debug, Serialize, Deserialize, Event)]
pub struct InterRealmDiplomacyEvent {
    pub tick: u64,
    pub realm_a: u8,
    pub realm_b: u8,
    pub tension_score: f32,
    pub participating_agents: Vec<AgentId>,
    pub spectator_agents: Vec<AgentId>,
    pub outcome: Option<DiplomacyOutcome>,
    pub forgiveness_wave_triggered: bool,
    pub redemption_score: f32,
    pub abundance_shared: f32,
    pub harmony_surge: f32,
    pub monument_id: Option<u64>,
    pub linked_legacy_thread_id: Option<LegacyThreadId>,
}

#[derive(Resource, Default)]
pub struct InterRealmDiplomacyRegistry {
    pub active_events: Vec<InterRealmDiplomacyEvent>,
    pub historical_events: Vec<InterRealmDiplomacyEvent>,
    pub realm_monuments: HashMap<(u8, u8), u64>,
    pub global_seed: u64,
}

impl InterRealmDiplomacyRegistry {
    pub fn new(global_seed: u64) -> Self {
        Self {
            active_events: vec![],
            historical_events: vec![],
            realm_monuments: HashMap::new(),
            global_seed,
        }
    }

    pub fn trigger_diplomacy_event(
        &mut self,
        realm_a: u8,
        realm_b: u8,
        tension_score: f32,
        participants: Vec<AgentId>,
        spectators: Vec<AgentId>,
        current_tick: u64,
    ) -> InterRealmDiplomacyEvent {
        let event = InterRealmDiplomacyEvent {
            tick: current_tick,
            realm_a,
            realm_b,
            tension_score,
            participating_agents: participants,
            spectator_agents: spectators,
            outcome: None,
            forgiveness_wave_triggered: false,
            redemption_score: 0.0,
            abundance_shared: 0.0,
            harmony_surge: 0.0,
            monument_id: None,
            linked_legacy_thread_id: None,
        };
        self.active_events.push(event.clone());
        event
    }

    pub fn resolve_event(
        &mut self,
        event_index: usize,
        council_input: Option<CouncilDeliberationInput>,
        legacy_registry: &mut LegacyJournalRegistry,
        grace_blessing_resource: &mut GraceBlessing,
        agents: &mut Vec<Agent>,
        rbe_pools: &mut HashMap<u8, RbeResourcePool>,
        current_tick: u64,
    ) {
        if let Some(event) = self.active_events.get_mut(event_index) {
            let (outcome, redemption_score) = if let Some(input) = council_input {
                (input.determine_outcome(), input.calculate_redemption_score())
            } else {
                if rand::random::<f32>() > 0.25 {
                    (DiplomacyOutcome::MercifulResolution, 0.88)
                } else {
                    (DiplomacyOutcome::StableDiplomacy, 0.62)
                }
            };

            event.outcome = Some(outcome.clone());
            event.redemption_score = redemption_score;

            if outcome == DiplomacyOutcome::MercifulResolution {
                let monument_id = (event.realm_a as u64 * 1000) + (event.realm_b as u64) + current_tick;
                event.monument_id = Some(monument_id);
                self.realm_monuments.insert((event.realm_a, event.realm_b), monument_id);

                self.apply_rbe_abundance_sharing(event, rbe_pools, redemption_score);
                self.apply_grace_blessing_cascade(event, agents, legacy_registry, grace_blessing_resource, current_tick);
            }

            let thread_id: LegacyThreadId = (current_tick as u64 * 10007) + (event.realm_a as u64 * 1009) + event.realm_b as u64;
            event.linked_legacy_thread_id = Some(thread_id);

            let resolved = event.clone();
            self.historical_events.push(resolved);
            self.active_events.remove(event_index);
        }
    }

    fn apply_rbe_abundance_sharing(
        &self,
        event: &InterRealmDiplomacyEvent,
        rbe_pools: &mut HashMap<u8, RbeResourcePool>,
        redemption_score: f32,
    ) {
        let shared = 8.0 + (redemption_score * 12.0);
        if let Some(pool) = rbe_pools.get_mut(&event.realm_a) {
            pool.abundance_flow += shared * 0.5;
        }
        if let Some(pool) = rbe_pools.get_mut(&event.realm_b) {
            pool.abundance_flow += shared * 0.5;
        }
    }

    fn apply_grace_blessing_cascade(
        &self,
        event: &InterRealmDiplomacyEvent,
        agents: &mut Vec<Agent>,
        legacy_registry: &mut LegacyJournalRegistry,
        _grace_blessing_resource: &mut GraceBlessing,
        current_tick: u64,
    ) {
        let high_mercy: Vec<_> = agents.iter().filter(|a| a.mercy_score > 65.0).cloned().collect();
        let low_mercy: Vec<_> = agents.iter().filter(|a| a.mercy_score < 55.0).cloned().collect();

        for mentor in high_mercy.iter().take(2) {
            for mentee in low_mercy.iter().take(2) {
                if mentor.id == mentee.id { continue; }
                let result = calculate_grace_blessing(
                    mentor.mercy_score,
                    mentee.mercy_score,
                    mentor.archetype_id.clone(),
                    BlessingContext::PostForgivenessWave,
                    250.0,
                );
                if let Some(m) = agents.iter_mut().find(|a| a.id == mentee.id) {
                    m.mercy_score = (m.mercy_score + result.mentee_mercy_boost).min(99.0);
                }
                legacy_registry.record_event(
                    mentor.id,
                    event.realm_a,
                    LegacyEventType::GraceBlessingGiven {
                        recipient_id: mentee.id,
                        mercy_boost: result.mentee_mercy_boost,
                    },
                    mentor.mercy_score,
                    result.mentor_persistence_gain,
                    result.valence,
                    current_tick,
                    true,
                    Some("Auto after Forgiveness Wave".to_string()),
                );
            }
        }
    }
}

pub fn inter_realm_diplomacy_resolution_system(
    mut diplomacy_registry: ResMut<InterRealmDiplomacyRegistry>,
    mut legacy_registry: ResMut<LegacyJournalRegistry>,
    mut grace_blessing: ResMut<GraceBlessing>,
    time: Res<Time>,
) {
    let current_tick = time.elapsed_secs() as u64;
    let mut to_resolve: Vec<usize> = vec![];
    for (i, event) in diplomacy_registry.active_events.iter().enumerate() {
        if event.outcome.is_none() {
            to_resolve.push(i);
        }
    }
    for idx in to_resolve.into_iter().rev() {
        diplomacy_registry.resolve_event(
            idx,
            None,
            &mut legacy_registry,
            &mut grace_blessing,
            &mut vec![],
            &mut HashMap::new(),
            current_tick,
        );
    }
}

pub struct InterRealmDiplomacyPlugin;

impl Plugin for InterRealmDiplomacyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InterRealmDiplomacyRegistry>()
           .add_event::<InterRealmDiplomacyEvent>()
           .add_systems(Update, inter_realm_diplomacy_resolution_system);
    }
}

// Integration helper (can be expanded)
pub fn get_council_deliberation_input(council_decisions: &CouncilDecisions) -> Option<CouncilDeliberationInput> {
    if council_decisions.decisions.is_empty() {
        return None;
    }
    Some(CouncilDeliberationInput {
        average_mercy_of_participants: 70.0,
        vote_ratio: 0.75,
        resolution_quality: 0.8,
        dominant_archetype_influence: 1.0,
    })
}
