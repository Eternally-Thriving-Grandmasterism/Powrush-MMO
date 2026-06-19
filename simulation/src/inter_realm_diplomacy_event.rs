// simulation/src/inter_realm_diplomacy_event.rs
// Updated with auto GraceBlessing trigger after MercifulResolution

use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::world::{Agent, AgentId, SovereignWorldState};
use crate::player_legacy_journal::{LegacyJournalRegistry, LegacyEventType, LegacyThreadId};
use crate::grace_blessing::{GraceBlessing, apply_grace_blessing, BlessingContext, calculate_grace_blessing};

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
        Self { active_events: vec![], historical_events: vec![], realm_monuments: HashMap::new(), global_seed }
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
        outcome: DiplomacyOutcome,
        redemption_score: f32,
        abundance_shared: f32,
        harmony_surge: f32,
        legacy_registry: &mut LegacyJournalRegistry,
        grace_registry: &mut crate::grace_blessing::GraceBlessing, // for future expansion
        agents: &mut Vec<Agent>,
        current_tick: u64,
    ) {
        if let Some(event) = self.active_events.get_mut(event_index) {
            event.outcome = Some(outcome.clone());
            event.redemption_score = redemption_score;
            event.abundance_shared = abundance_shared;
            event.harmony_surge = harmony_surge;

            let is_forgiveness_wave = outcome == DiplomacyOutcome::MercifulResolution;
            event.forgiveness_wave_triggered = is_forgiveness_wave;

            if is_forgiveness_wave {
                let monument_id = (event.realm_a as u64 * 1000) + (event.realm_b as u64) + current_tick;
                event.monument_id = Some(monument_id);
                self.realm_monuments.insert((event.realm_a, event.realm_b), monument_id);

                // === AUTO-TRIGGER GRACE BLESSING after Forgiveness Wave ===
                self.auto_trigger_grace_blessing_after_forgiveness_wave(
                    event,
                    agents,
                    legacy_registry,
                    current_tick,
                );
            }

            let thread_id: LegacyThreadId = (current_tick as u64 * 10007) + (event.realm_a as u64 * 1009) + event.realm_b as u64;
            event.linked_legacy_thread_id = Some(thread_id);

            let resolved = event.clone();
            self.historical_events.push(resolved);
            self.active_events.remove(event_index);
        }
    }

    fn auto_trigger_grace_blessing_after_forgiveness_wave(
        &self,
        event: &InterRealmDiplomacyEvent,
        agents: &mut Vec<Agent>,
        legacy_registry: &mut LegacyJournalRegistry,
        current_tick: u64,
    ) {
        // Find lower-mercy participants and have higher-mercy ones bless them
        let mut sorted: Vec<_> = agents.iter().cloned().collect();
        sorted.sort_by(|a, b| b.mercy_score.partial_cmp(&a.mercy_score).unwrap());

        let mentors: Vec<_> = sorted.iter().filter(|a| a.mercy_score > 65.0).cloned().collect();
        let mentees: Vec<_> = sorted.iter().filter(|a| a.mercy_score < 55.0).cloned().collect();

        for mentor in mentors.iter().take(2) {
            for mentee in mentees.iter().take(3) {
                if mentor.id == mentee.id { continue; }

                // In real system we would have proper &mut access + GraceBlessing component
                // Here we simulate the boost directly for demonstration
                let result = calculate_grace_blessing(
                    mentor.mercy_score,
                    mentee.mercy_score,
                    mentor.archetype_id.clone(),
                    crate::grace_blessing::BlessingContext::PostForgivenessWave,
                    250.0,
                );

                // Apply simulated boost (real version uses apply_grace_blessing with proper components)
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
                    Some("Auto-triggered after Forgiveness Wave".to_string()),
                );
            }
        }
    }
}

pub fn inter_realm_diplomacy_resolution_system(
    mut diplomacy_registry: ResMut<InterRealmDiplomacyRegistry>,
    mut legacy_registry: ResMut<LegacyJournalRegistry>,
    time: Res<Time>,
) {
    let current_tick = time.elapsed_secs() as u64;
    let mut to_resolve = vec![];
    for (i, event) in diplomacy_registry.active_events.iter().enumerate() {
        if event.outcome.is_none() {
            to_resolve.push(i);
        }
    }
    for idx in to_resolve.into_iter().rev() {
        let outcome = if rand::random::<f32>() > 0.25 {
            DiplomacyOutcome::MercifulResolution
        } else {
            DiplomacyOutcome::StableDiplomacy
        };
        let redemption = if outcome == DiplomacyOutcome::MercifulResolution { 0.92 } else { 0.65 };

        // Note: In full system, pass proper agents list and GraceBlessing resource
        diplomacy_registry.resolve_event(
            idx,
            outcome,
            redemption,
            8.5,
            6.2,
            &mut legacy_registry,
            &mut crate::grace_blessing::GraceBlessing::default(),
            &mut vec![], // placeholder - real system passes live agents
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
