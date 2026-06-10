// client/src/onboarding.rs
// Powrush-MMO v18.9 — Professional Global Onboarding + Closed Beta Access Control
//
// When ClosedBetaConfig::require_invite is true, players must provide a valid invite
// before proceeding past LanguageSelect.
// Fully mercy-aligned: players can still choose mercy-skip, but closed beta mode can restrict it.

use bevy::prelude::*;
use crate::localization::Localization;
use crate::divine_whispers::{DivineWhisperEvent, WhisperPriority};

// These would normally come from simulation crate via shared types or events
use simulation::closed_beta::{ClosedBetaConfig, InviteManager};

#[derive(Resource, Default)]
pub struct OnboardingState {
    pub step: OnboardingStep,
    pub completed: bool,
    pub selected_language: String,
    pub mercy_skipped: bool,
    pub invite_code: Option<String>,
    pub invite_validated: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OnboardingStep {
    #[default]
    LanguageSelect,
    InviteValidation,      // New step for closed beta
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
                handle_invite_validation,
            ));
    }
}

fn setup_onboarding_with_detection(
    mut commands: Commands,
    mut loc: ResMut<Localization>,
) {
    loc.detect_and_apply();

    commands.insert_resource(OnboardingState {
        step: OnboardingStep::LanguageSelect,
        completed: false,
        selected_language: loc.current_lang.clone(),
        mercy_skipped: false,
        invite_code: None,
        invite_validated: false,
    });
}

fn onboarding_progression(
    mut state: ResMut<OnboardingState>,
    closed_beta_config: Option<Res<ClosedBetaConfig>>,
) {
    // If closed beta mode requires invite and we haven't validated yet, go to InviteValidation
    if let Some(config) = closed_beta_config {
        if config.require_invite && !state.invite_validated && state.step == OnboardingStep::LanguageSelect {
            state.step = OnboardingStep::InviteValidation;
            return;
        }
    }

    // Normal progression logic...
}

fn handle_invite_validation(
    mut state: ResMut<OnboardingState>,
    invite_manager: Option<Res<InviteManager>>,
    // In real implementation: listen for UI input of invite code
) {
    // This is a simplified example. In production you would have a UI input field
    // that sets state.invite_code and then validates it here.

    if state.step == OnboardingStep::InviteValidation {
        if let Some(code) = &state.invite_code {
            if let Some(manager) = invite_manager {
                if manager.validate_invite(code) {
                    state.invite_validated = true;
                    state.step = OnboardingStep::Welcome;
                    // Optionally consume the invite here
                } else {
                    // Invalid invite - stay on validation step or show error
                }
            }
        }
    }
}

fn trigger_contextual_whispers(
    state: Res<OnboardingState>,
    loc: Res<Localization>,
    mut whisper_events: EventWriter<DivineWhisperEvent>,
) {
    if state.is_changed() {
        let key = match state.step {
            OnboardingStep::LanguageSelect => "onboarding_language_select",
            OnboardingStep::InviteValidation => "onboarding_invite_validation",
            OnboardingStep::Welcome => "onboarding_welcome",
            OnboardingStep::RBEPrimer => "onboarding_rbe_primer",
            OnboardingStep::FirstHarvestTutorial => "onboarding_first_harvest",
            OnboardingStep::MercyContribution => "onboarding_mercy_contribution",
            OnboardingStep::SovereignStart => "onboarding_sovereign_start",
            OnboardingStep::Complete => "onboarding_complete",
        };

        let message = loc.t(key);
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

// Note: In a full implementation you would also add UI for entering invite codes
// and connect it to state.invite_code.
