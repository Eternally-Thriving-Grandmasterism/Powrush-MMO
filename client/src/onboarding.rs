/*!
 * Onboarding — Powrush-MMO Professional Global Onboarding + RBE Education
 *
 * v21.90 — End-user experience perfection:
 * - Public launches default to LanguageSelect → Welcome (zero invite/captcha friction)
 * - Closed-beta invite + captcha path only when ClosedBetaConfig.require_invite is true
 * - Prior council reflection preserved
 * - TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 *
 * AG-SML v1.0 Sovereign License | Contact: info@Rathor.ai
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use crate::localization::Localization;
use crate::divine_whispers::{DivineWhisperEvent, WhisperPriority};
use crate::fundsp_audio::ActiveProceduralEpiphanies;
use simulation::closed_beta::{ClosedBetaConfig, InviteManager};
use simulation::bot_detection::BotDetectionConfig;

#[derive(Event, Clone)]
pub struct LoadPriorCouncilData {
    pub prior_council_blooms: u32,
    pub prior_council_engagement: f32,
}

#[derive(Resource, Default)]
pub struct OnboardingState {
    pub step: OnboardingStep,
    pub completed: bool,
    pub selected_language: String,
    pub mercy_skipped: bool,
    pub invite_code: Option<String>,
    pub invite_validated: bool,
    pub invite_error: Option<String>,
    pub invite_attempts: u32,
    pub last_invite_attempt_ms: u64,
    pub captcha_question: Option<String>,
    pub captcha_answer: Option<i32>,
    pub captcha_user_input: String,
    pub captcha_verified: bool,
    pub beta_mode_enabled: bool,
    pub bot_protection_level: u8,
    pub prior_council_blooms: u32,
    pub prior_council_engagement: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OnboardingStep {
    #[default]
    LanguageSelect,
    InviteValidation,
    CaptchaVerification,
    Welcome,
    RBEPrimer,
    FirstHarvestTutorial,
    MercyContribution,
    SovereignStart,
    FirstCouncilBloom,
    Complete,
}

pub struct OnboardingPlugin;

impl Plugin for OnboardingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<OnboardingState>()
            .add_event::<LoadPriorCouncilData>()
            .add_systems(Startup, setup_onboarding_with_detection)
            .add_systems(Update, (
                onboarding_progression,
                trigger_contextual_whispers,
                process_invite_validation,
                generate_captcha_if_needed,
                verify_captcha,
                elevate_onboarding_with_epiphany_audio,
                integrate_bot_protection_during_beta,
                apply_prior_council_reflection,
                handle_load_prior_council_data,
            ));
    }
}

fn setup_onboarding_with_detection(
    mut commands: Commands,
    mut loc: ResMut<Localization>,
    beta_config: Option<Res<ClosedBetaConfig>>,
    bot_config: Option<Res<BotDetectionConfig>>,
) {
    loc.detect_and_apply();

    // PERFECT END-USER DEFAULT:
    // Only force invite path when closed-beta explicitly requires it.
    // Public / open play starts at LanguageSelect with zero gate friction.
    let beta_enabled = beta_config.map_or(false, |c| c.require_invite);
    let bot_level = bot_config.map_or(0, |c| if c.enabled { 2 } else { 0 });

    let start_step = if beta_enabled {
        OnboardingStep::InviteValidation
    } else {
        OnboardingStep::LanguageSelect
    };

    commands.insert_resource(OnboardingState {
        step: start_step,
        completed: false,
        selected_language: loc.current_lang.clone(),
        mercy_skipped: false,
        invite_code: None,
        invite_validated: !beta_enabled, // public path is pre-validated
        invite_error: None,
        invite_attempts: 0,
        last_invite_attempt_ms: 0,
        captcha_question: None,
        captcha_answer: None,
        captcha_user_input: String::new(),
        captcha_verified: !beta_enabled,
        beta_mode_enabled: beta_enabled,
        bot_protection_level: bot_level,
        prior_council_blooms: 0,
        prior_council_engagement: 0.0,
    });
}

fn onboarding_progression(
    mut state: ResMut<OnboardingState>,
    closed_beta_config: Option<Res<ClosedBetaConfig>>,
) {
    if let Some(config) = closed_beta_config {
        state.beta_mode_enabled = config.require_invite;
        if config.require_invite && !state.invite_validated && state.step == OnboardingStep::LanguageSelect {
            state.step = OnboardingStep::InviteValidation;
        }
    }
}

fn elevate_onboarding_with_epiphany_audio(
    mut state: ResMut<OnboardingState>,
    mut _active_epiphanies: ResMut<ActiveProceduralEpiphanies>,
    _time: Res<Time>,
) {
    // Soft resonance on Welcome — does not force-complete the whole flow.
    if state.step == OnboardingStep::Welcome && !state.completed {
        let intensity = 0.65;
        let _resonance = crate::fundsp_audio::build_epiphany_resonance(
            intensity,
            Some("sustainable_harmony_revelation"),
        );
        // Mark welcome resonance played without skipping remaining educational steps
        // (completed remains false until SovereignStart / Complete)
    }
}

fn integrate_bot_protection_during_beta(
    mut state: ResMut<OnboardingState>,
    tracker: Option<Res<ClientBehavioralTracker>>,
    _bot_config: Option<Res<BotDetectionConfig>>,
) {
    if !state.beta_mode_enabled || state.bot_protection_level == 0 {
        return;
    }

    if state.step == OnboardingStep::InviteValidation && state.invite_attempts > 0 {
        if let Some(tr) = tracker {
            let human_score = tr.get_human_score();
            if human_score < 0.35 && state.bot_protection_level >= 2 {
                if state.invite_error.is_none() {
                    state.invite_error =
                        Some("Additional verification required for this session.".to_string());
                }
            }
        }
    }
}

fn generate_captcha_if_needed(mut state: ResMut<OnboardingState>) {
    if state.step == OnboardingStep::InviteValidation
        && state.invite_validated
        && state.captcha_question.is_none()
        && !state.captcha_verified
    {
        let a = (rand::random::<u32>() % 10) + 3;
        let b = (rand::random::<u32>() % 8) + 2;
        let answer = (a + b) as i32;

        state.captcha_question = Some(format!("What is {} + {}?", a, b));
        state.captcha_answer = Some(answer);
        state.captcha_user_input.clear();
        state.step = OnboardingStep::CaptchaVerification;
    }
}

fn verify_captcha(mut state: ResMut<OnboardingState>) {
    if state.step != OnboardingStep::CaptchaVerification {
        return;
    }

    if let (Some(expected), input) = (state.captcha_answer, &state.captcha_user_input) {
        if let Ok(user_answer) = input.trim().parse::<i32>() {
            if user_answer == expected {
                state.captcha_verified = true;
                state.invite_error = None;
                state.step = OnboardingStep::Welcome;
            } else {
                state.invite_error = Some("Incorrect answer. Please try again.".to_string());
                state.captcha_user_input.clear();
            }
        }
    }
}

fn process_invite_validation(
    mut state: ResMut<OnboardingState>,
    mut invite_manager: Option<ResMut<InviteManager>>,
    time: Res<Time>,
    mut tracker: Option<ResMut<ClientBehavioralTracker>>,
) {
    if state.step != OnboardingStep::InviteValidation {
        return;
    }

    let current_time = (time.elapsed_seconds_f64() * 1000.0) as u64;
    const MAX_ATTEMPTS: u32 = 5;
    const COOLDOWN_MS: u64 = 60_000;

    if state.invite_attempts >= MAX_ATTEMPTS {
        let time_since_last = current_time.saturating_sub(state.last_invite_attempt_ms);
        if time_since_last < COOLDOWN_MS {
            let remaining = (COOLDOWN_MS - time_since_last) / 1000;
            state.invite_error = Some(format!(
                "Too many attempts. Please wait {} seconds.",
                remaining
            ));
            return;
        } else {
            state.invite_attempts = 0;
        }
    }

    if let Some(code) = &state.invite_code {
        state.last_invite_attempt_ms = current_time;
        state.invite_attempts += 1;

        if let Some(tr) = tracker.as_mut() {
            tr.record_action();
        }

        if let Some(manager) = invite_manager.as_mut() {
            if manager.validate_invite(code) {
                manager.consume_invite(code);
                state.invite_validated = true;
                state.invite_error = None;
            } else {
                state.invite_error = Some("Invalid or expired invite code".to_string());
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
        let (key, is_epiphany, flavor) = match state.step {
            OnboardingStep::LanguageSelect => ("onboarding_language_select", false, "welcome"),
            OnboardingStep::InviteValidation => ("onboarding_invite_validation", false, "welcome"),
            OnboardingStep::CaptchaVerification => {
                ("onboarding_captcha_verification", false, "welcome")
            }
            OnboardingStep::Welcome => ("onboarding_welcome", true, "first_bloom"),
            OnboardingStep::RBEPrimer => {
                ("onboarding_rbe_primer", true, "sustainable_harmony_revelation")
            }
            OnboardingStep::FirstHarvestTutorial => {
                ("onboarding_first_harvest", false, "sustainable_abundance_revelation")
            }
            OnboardingStep::MercyContribution => {
                ("onboarding_mercy_contribution", true, "graceful_redemption_revelation")
            }
            OnboardingStep::SovereignStart => {
                ("onboarding_sovereign_start", true, "council_harmony_revelation")
            }
            OnboardingStep::FirstCouncilBloom => {
                ("onboarding_first_council_bloom", true, "ecstatic_harmony_council_crown")
            }
            _ => ("onboarding_welcome", false, "welcome"),
        };

        let message = loc.t(key);
        whisper_events.send(DivineWhisperEvent {
            text: message,
            priority: WhisperPriority::High,
            is_epiphany,
            intensity: if is_epiphany { 0.78 } else { 0.45 },
            duration_seconds: if is_epiphany { 11.0 } else { 6.5 },
            flavor: flavor.to_string(),
            ..default()
        });
    }
}

fn apply_prior_council_reflection(mut state: ResMut<OnboardingState>) {
    if state.prior_council_blooms > 0 && state.step == OnboardingStep::Welcome {
        tracing::info!(
            "[Onboarding] Prior Council success | blooms={} | reflecting in flow",
            state.prior_council_blooms
        );
    }
}

fn handle_load_prior_council_data(
    mut events: EventReader<LoadPriorCouncilData>,
    mut state: ResMut<OnboardingState>,
) {
    for event in events.read() {
        state.prior_council_blooms = event.prior_council_blooms;
        state.prior_council_engagement = event.prior_council_engagement;
        tracing::info!(
            "[Onboarding] Loaded prior Council data | blooms={} | engagement={:.2}",
            event.prior_council_blooms,
            event.prior_council_engagement
        );
    }
}

pub fn mercy_skip_onboarding(state: &mut OnboardingState) {
    state.mercy_skipped = true;
    state.step = OnboardingStep::Complete;
    state.completed = true;
}

/// Advance one educational step (called by UI Continue buttons).
pub fn advance_onboarding_step(state: &mut OnboardingState) {
    state.step = match state.step {
        OnboardingStep::LanguageSelect => {
            if state.beta_mode_enabled && !state.invite_validated {
                OnboardingStep::InviteValidation
            } else {
                OnboardingStep::Welcome
            }
        }
        OnboardingStep::InviteValidation => state.step, // gated by validation
        OnboardingStep::CaptchaVerification => state.step,
        OnboardingStep::Welcome => OnboardingStep::RBEPrimer,
        OnboardingStep::RBEPrimer => OnboardingStep::FirstHarvestTutorial,
        OnboardingStep::FirstHarvestTutorial => OnboardingStep::MercyContribution,
        OnboardingStep::MercyContribution => OnboardingStep::SovereignStart,
        OnboardingStep::SovereignStart => OnboardingStep::FirstCouncilBloom,
        OnboardingStep::FirstCouncilBloom => {
            state.completed = true;
            OnboardingStep::Complete
        }
        OnboardingStep::Complete => OnboardingStep::Complete,
    };
}
