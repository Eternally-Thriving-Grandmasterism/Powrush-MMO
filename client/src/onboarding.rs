// client/src/onboarding.rs
// Powrush-MMO v17.27 — Production Starter Content & Onboarding Flow
// Mercy-gated, PATSAGi-aligned first-hour delight + tutorial systems
// Builds on Settings, Pause, MercyAnomalyDetector, Interest Management

use bevy::prelude::*;
use crate::divine_whispers_ui::DivineWhisperEvent;

#[derive(Resource, Default)]
pub struct OnboardingState {
    pub step: OnboardingStep,
    pub completed: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OnboardingStep {
    #[default]
    Welcome,
    Movement,
    Harvesting,
    RBEIntro,
    FirstAbundance,
    FactionChoice,
    Complete,
}

pub struct OnboardingPlugin;

impl Plugin for OnboardingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<OnboardingState>()
            .add_systems(Startup, check_first_time_player)
            .add_systems(Update, (
                onboarding_progression,
                trigger_divine_whispers,
            ));
    }
}

fn check_first_time_player(
    mut commands: Commands,
    // TODO: check player profile / localStorage for returning player
) {
    // For new players, start onboarding
    commands.insert_resource(OnboardingState {
        step: OnboardingStep::Welcome,
        completed: false,
    });
}

fn onboarding_progression(
    mut state: ResMut<OnboardingState>,
    // Listen for harvest events, movement, etc.
) {
    // Advance steps based on player actions
    // Example: after first successful harvest → RBEIntro
}

fn trigger_divine_whispers(
    state: Res<OnboardingState>,
    mut whisper_events: EventWriter<DivineWhisperEvent>,
) {
    if state.is_changed() {
        match state.step {
            OnboardingStep::Welcome => {
                whisper_events.send(DivineWhisperEvent {
                    message: "Welcome, Seeker. You have entered the Eternal Flow. Align with mercy and abundance.".to_string(),
                    priority: 1,
                });
            }
            OnboardingStep::Harvesting => {
                whisper_events.send(DivineWhisperEvent {
                    message: "Feel the rhythm of the land. Harvest with intention — sustainability brings abundance.".to_string(),
                    priority: 2,
                });
            }
            // ... other steps
            _ => {}
        }
    }
}

// TODO: Full UI panels for each step, quest log integration, completion rewards
// Mercy-gated: No forced tutorials — player can skip with mercy option
