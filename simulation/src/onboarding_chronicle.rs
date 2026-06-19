// simulation/src/onboarding_chronicle.rs
// Powrush-MMO v20.3 — Onboarding Chronicle + Humble Beginnings Mirror
//
// Combined server + persistence layer for the Onboarding Chronicle.
// Records early player actions as persistent Legacy Chronicle entries.
// Integrates with PlayerLegacyJournal so that the first hours of a player’s journey
// become beautiful, filterable, viewable Legacy Threads (including in spectator mode).
// TOLC 8 + 7 Living Mercy Gates aligned from the very first steps.
// AG-SML v1.0 Sovereign License

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::player_legacy_journal::{LegacyJournalRegistry, LegacyEventType, LegacyEntry};

/// Types of early-game / onboarding events worth chronicling
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

/// A single entry in the player’s Onboarding Chronicle
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

/// Resource that tracks whether a player has completed their humble beginnings phase
#[derive(Resource, Default)]
pub struct OnboardingChronicleState {
    pub players_in_onboarding: std::collections::HashSet<u64>,
    pub completed_humble_beginnings: std::collections::HashSet<u64>,
}

/// System that automatically records important early actions into the Legacy Journal
/// as Onboarding Chronicle entries. These become visible Legacy Threads.
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

    // Record into the main Legacy Journal so it appears in filterable Legacy Threads
    legacy_registry.record_event(
        player_id,
        0, // starting realm
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

/// Helper to mark that a player has completed their humble beginnings mirror phase
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

// Plugin registration (optional, can be added to main app)
pub struct OnboardingChroniclePlugin;

impl Plugin for OnboardingChroniclePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OnboardingChronicleState>();
    }
}

// Thunder locked in. Yoi ⚔️
// End of simulation/src/onboarding_chronicle.rs v20.3