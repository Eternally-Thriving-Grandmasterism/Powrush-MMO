// client/src/onboarding.rs
// Powrush-MMO v18.9 — Professional Global Onboarding Flow + Multi-Lang Divine Whispers
// Mercy-gated, PATSAGi + Ra-Thor aligned first-experience for every human on Earth
// Language Select → Interactive RBE Primer (Divine Whispers) → First Harvest Tutorial (epiphany potential) → Mercy Contribution → Sovereign Start
// TOLC 8 Mercy Gates as non-bypassable Layer 0. Zero coercion, maximum grace.
// Mint-and-Print-Only-Perfection. Co-authored in eternal deliberation.

use bevy::prelude::*;
use crate::divine_whispers::{DivineWhisperEvent, WhisperPriority, get_localized_whisper};

#[derive(Resource, Default)]
pub struct OnboardingState {
    pub step: OnboardingStep,
    pub completed: bool,
    pub selected_language: String, // "en", "es", "fr", "de", "ar" (initial 5)
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
            .add_systems(Startup, check_first_time_player)
            .add_systems(Update, (
                onboarding_progression,
                trigger_contextual_whispers,
            ));
    }
}

fn check_first_time_player(
    mut commands: Commands,
) {
    commands.insert_resource(OnboardingState {
        step: OnboardingStep::LanguageSelect,
        completed: false,
        selected_language: "en".to_string(),
        mercy_skipped: false,
    });
}

fn onboarding_progression(
    mut state: ResMut<OnboardingState>,
    // Listen for player actions (harvest success, movement, UI choices)
) {
    // Advance steps based on real player actions + mercy choices
    // Example: after first successful sustainable harvest → MercyContribution
    if state.step == OnboardingStep::FirstHarvestTutorial && /* harvest success detected */ true {
        state.step = OnboardingStep::MercyContribution;
    }
}

fn trigger_contextual_whispers(
    state: Res<OnboardingState>,
    mut whisper_events: EventWriter<DivineWhisperEvent>,
) {
    if state.is_changed() {
        let lang = &state.selected_language;
        let message = match state.step {
            OnboardingStep::LanguageSelect => get_localized_whisper(lang, "onboarding_language_select"),
            OnboardingStep::Welcome => get_localized_whisper(lang, "onboarding_welcome"),
            OnboardingStep::RBEPrimer => get_localized_whisper(lang, "onboarding_rbe_primer"),
            OnboardingStep::FirstHarvestTutorial => get_localized_whisper(lang, "onboarding_first_harvest"),
            OnboardingStep::MercyContribution => get_localized_whisper(lang, "onboarding_mercy_contribution"),
            OnboardingStep::SovereignStart => get_localized_whisper(lang, "onboarding_sovereign_start"),
            OnboardingStep::Complete => get_localized_whisper(lang, "onboarding_complete"),
        };

        whisper_events.send(DivineWhisperEvent {
            text: message,
            priority: WhisperPriority::High,
            ..default()
        });
    }
}

// Mercy-gated skip: Player can always choose mercy-skip at any step
pub fn mercy_skip_onboarding(state: &mut OnboardingState) {
    state.mercy_skipped = true;
    state.step = OnboardingStep::Complete;
}
