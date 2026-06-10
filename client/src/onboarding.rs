// client/src/onboarding.rs
// Powrush-MMO v18.9 — Professional Global Onboarding + Real Invite Validation

use bevy::prelude::*;
use crate::localization::Localization;
use crate::divine_whispers::{DivineWhisperEvent, WhisperPriority};

// Simulation types (shared via crate or events in full implementation)
use simulation::closed_beta::{ClosedBetaConfig, InviteManager};

#[derive(Resource, Default)]
pub struct OnboardingState {
    pub step: OnboardingStep,
    pub completed: bool,
    pub selected_language: String,
    pub mercy_skipped: bool,
    pub invite_code: Option<String>,
    pub invite_validated: bool,
    pub invite_error: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OnboardingStep {
    #[default]
    LanguageSelect,
    InviteValidation,
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
                process_invite_validation,
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
        invite_error: None,
    });
}

fn onboarding_progression(
    mut state: ResMut<OnboardingState>,
    closed_beta_config: Option<Res<ClosedBetaConfig>>,
) {
    if let Some(config) = closed_beta_config {
        if config.require_invite && !state.invite_validated && state.step == OnboardingStep::LanguageSelect {
            state.step = OnboardingStep::InviteValidation;
            state.invite_error = None;
        }
    }
}

/// Core validation logic
fn process_invite_validation(
    mut state: ResMut<OnboardingState>,
    mut invite_manager: Option<ResMut<InviteManager>>,
) {
    if state.step != OnboardingStep::InviteValidation {
        return;
    }

    if let Some(code) = &state.invite_code {
        if let Some(manager) = &mut invite_manager {
            if manager.validate_invite(code) {
                // Valid invite
                manager.consume_invite(code); // Optional: consume one use
                state.invite_validated = true;
                state.invite_error = None;
                state.step = OnboardingStep::Welcome;
            } else {
                state.invite_error = Some("Invalid or expired invite code".to_string());
                state.invite_code = None; // Clear so player can try again
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
            // ... other steps
            _ => "onboarding_welcome",
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
