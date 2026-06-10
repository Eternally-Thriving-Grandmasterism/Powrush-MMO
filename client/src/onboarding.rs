// client/src/onboarding.rs
// Powrush-MMO v18.9 — Professional Global Onboarding Flow + Realtime Language Detection
// Auto-detects language on first launch using Localization system
// Language Select → Interactive RBE Primer → First Harvest Tutorial → Mercy Contribution → Sovereign Start

use bevy::prelude::*;
use crate::localization::Localization;
use crate::divine_whispers::{DivineWhisperEvent, WhisperPriority};

#[derive(Resource, Default)]
pub struct OnboardingState {
    pub step: OnboardingStep,
    pub completed: bool,
    pub selected_language: String,
    pub mercy_skipped: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OnboardingStep {
    #[default]
    LanguageSelect,
    Welcome,
    RBEPrimer,
    FirstHarvestTutorial,
    MercyContribution,
    SovereignStart,
    Complete,
}

pub struct OnboardingPlugin;

impl Plugin for OnboardingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<OnboardingState>()
            .add_systems(Startup, setup_onboarding_with_detection)
            .add_systems(Update, (
                onboarding_progression,
                trigger_contextual_whispers,
            ));
    }
}

fn setup_onboarding_with_detection(
    mut commands: Commands,
    mut loc: ResMut<Localization>,
) {
    // Run realtime language detection at startup
    loc.detect_and_apply();

    commands.insert_resource(OnboardingState {
        step: OnboardingStep::LanguageSelect,
        completed: false,
        selected_language: loc.current_lang.clone(),
        mercy_skipped: false,
    });
}

fn onboarding_progression(
    mut state: ResMut<OnboardingState>,
) {
    // Advance based on player actions
}

fn trigger_contextual_whispers(
    state: Res<OnboardingState>,
    loc: Res<Localization>,
    mut whisper_events: EventWriter<DivineWhisperEvent>,
) {
    if state.is_changed() {
        let message = loc.t(match state.step {
            OnboardingStep::LanguageSelect => "onboarding_language_select",
            OnboardingStep::Welcome => "onboarding_welcome",
            OnboardingStep::RBEPrimer => "onboarding_rbe_primer",
            OnboardingStep::FirstHarvestTutorial => "onboarding_first_harvest",
            OnboardingStep::MercyContribution => "onboarding_mercy_contribution",
            OnboardingStep::SovereignStart => "onboarding_sovereign_start",
            OnboardingStep::Complete => "onboarding_complete",
        });

        whisper_events.send(DivineWhisperEvent {
            text: message,
            priority: WhisperPriority::High,
            ..default()
        });
    }
}

pub fn mercy_skip_onboarding(state: &mut OnboardingState) {
    state.mercy_skipped = true;
    state.step = OnboardingStep::Complete;
}
