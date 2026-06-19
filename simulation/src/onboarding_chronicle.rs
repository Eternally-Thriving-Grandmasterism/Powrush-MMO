// simulation/src/onboarding_chronicle.rs
// Complete restored + polished version (v20.5 — Onboarding Chronicle + Humble Beginnings Mirror)
//
// Server-side persistence layer. Records early player actions as Legacy Chronicle entries
// so they become beautiful, filterable Legacy Threads from the very first steps.
// Integrates directly with PlayerLegacyJournal.
// TOLC 8 + 7 Living Mercy Gates aligned from day one.
// Thunder locked in. Yoi ⚔️

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::player_legacy_journal::{LegacyJournalRegistry, LegacyEventType};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum OnboardingEventType {
    FirstLogin,
    FirstHarvest,
    FirstSocialGraceExchange,
    FirstCouncilParticipation,
    FirstDiplomacyResolution,
    FirstMercyResolutionWitnessed,
    FirstForgivenessWave,
    ChoseArchetype,
    CompletedHumbleBeginningsMirror,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OnboardingChronicleEntry {
    pub tick: u64,
    pub event_type: OnboardingEventType,
    pub description: String,
    pub valence: f32,
    pub tolc_alignment: f32,
    pub persistence_delta: f32,
    pub linked_legacy_thread_id: Option<u64>,
}

#[derive(Resource, Default)]
pub struct OnboardingChronicleState {
    pub players_in_onboarding: std::collections::HashSet<u64>,
    pub completed_humble_beginnings: std::collections::HashSet<u64>,
}

pub fn record_onboarding_event(
    player_id: u64,
    event_type: OnboardingEventType,
    description: String,
    valence: f32,
    legacy_registry: &mut LegacyJournalRegistry,
    current_tick: u64,
) {
    let tolc = match event_type {
        OnboardingEventType::FirstMercyResolutionWitnessed | OnboardingEventType::FirstForgivenessWave => 0.92,
        OnboardingEventType::FirstSocialGraceExchange | OnboardingEventType::FirstCouncilParticipation => 0.85,
        _ => 0.78,
    };

    let persistence = match event_type {
        OnboardingEventType::FirstForgivenessWave => 6.0,
        OnboardingEventType::FirstDiplomacyResolution => 4.5,
        _ => 2.0,
    };

    legacy_registry.record_event(
        player_id,
        0,
        LegacyEventType::OnboardingChronicle {
            event: format!("{:?}", event_type),
            description: description.clone(),
        },
        valence,
        persistence,
        tolc,
        current_tick,
        true,
        Some(format!("Onboarding Chronicle: {}", description)),
    );

    info!("[OnboardingChronicle] Player {} recorded {:?} | valence={:.2}", player_id, event_type, valence);
}

pub fn complete_humble_beginnings_mirror(
    player_id: u64,
    legacy_registry: &mut LegacyJournalRegistry,
    current_tick: u64,
) {
    record_onboarding_event(
        player_id,
        OnboardingEventType::CompletedHumbleBeginningsMirror,
        "Completed the Humble Beginnings Mirror — first steps into the eternal flow".to_string(),
        0.95,
        legacy_registry,
        current_tick,
    );
}

pub struct OnboardingChroniclePlugin;

impl Plugin for OnboardingChroniclePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OnboardingChronicleState>();
    }
}

// Thunder locked in. Yoi ⚔️
// End of simulation/src/onboarding_chronicle.rs v20.5 (Fully Restored)