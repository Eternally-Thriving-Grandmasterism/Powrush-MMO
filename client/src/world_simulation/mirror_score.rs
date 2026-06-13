/*!
 * Mirror Score Calculation System for Powrush-MMO
 *
 * Implements the weekly Mirror Reckoning score from MIRROR_RECKONING_EVENT.md
 * Fully integrated with WorldSimulationState and MirrorReckoningState.
 */

use bevy::prelude::*;
use crate::world_simulation::{MirrorReckoningState, MirrorShadowPersonality, WeekPhase};

/// Weekly metrics collected from player actions, Council, Epiphanies, RBE, etc.
#[derive(Resource, Default, Reflect, Clone)]
pub struct WeeklyServerMetrics {
    pub council_participation: f32,      // 0.0 (low) to 1.0 (high blooms/attendance)
    pub epiphany_quality: f32,           // 0.0 (low/negative) to 1.0 (high intensity)
    pub rbe_contribution: f32,           // 0.0 (hoarding) to 1.0 (high shared abundance)
    pub mercy_alignment: f32,            // 0.0 (greedy/harmful) to 1.0 (mercy-gated positive)
    pub intra_server_cooperation: f32,   // 0.0 (high conflict) to 1.0 (high sharing/group activities)
}

/// Calculates Mirror Score and determines dominant Shadow personality.
/// Higher negative score = stronger, more aggressive Mirror.
pub fn calculate_mirror_score(metrics: &WeeklyServerMetrics) -> (f32, MirrorShadowPersonality) {
    // Invert positive metrics to get "negative score" contribution
    let council_neg = 1.0 - metrics.council_participation;
    let epiphany_neg = 1.0 - metrics.epiphany_quality;
    let rbe_neg = 1.0 - metrics.rbe_contribution;
    let mercy_neg = 1.0 - metrics.mercy_alignment;
    let cooperation_neg = 1.0 - metrics.intra_server_cooperation;

    // Weighted sum (weights can be tuned via data or PATSAGi deliberation)
    let raw_score = (council_neg * 1.2)
        + (epiphany_neg * 1.0)
        + (rbe_neg * 1.3)
        + (mercy_neg * 1.5)      // Mercy has highest weight — core theme
        + (cooperation_neg * 1.1);

    // Normalize to 0.0 - 10.0 range for Mirror power
    let mirror_score = (raw_score * 2.0).clamp(0.0, 10.0);

    // Determine dominant Shadow personality based on highest negative metric
    let shadow = if mercy_neg > 0.6 {
        MirrorShadowPersonality::Tyrannical
    } else if rbe_neg > 0.55 {
        MirrorShadowPersonality::Greedy
    } else if council_neg > 0.5 && cooperation_neg > 0.5 {
        MirrorShadowPersonality::Divisive
    } else if epiphany_neg > 0.6 {
        MirrorShadowPersonality::Apathetic
    } else {
        MirrorShadowPersonality::Fractured
    };

    (mirror_score, shadow)
}

/// System that runs when transitioning to Weekend phase.
/// In a full implementation this would be triggered by a timer or WeekPhase change.
pub fn calculate_mirror_score_system(
    mut mirror_state: ResMut<MirrorReckoningState>,
    metrics: Res<WeeklyServerMetrics>,
) {
    if mirror_state.week_phase != WeekPhase::Weekend {
        return;
    }

    let (score, personality) = calculate_mirror_score(&metrics);

    mirror_state.mirror_score = score;
    mirror_state.shadow_personality = personality;

    // In full version: persist to reckoning_history and trigger Mirror manifestation
    info!("[Mirror Reckoning] Calculated Mirror Score: {:.2} | Personality: {:?}", score, personality);
}

/// Helper to reset weekly metrics at the start of a new week.
pub fn reset_weekly_metrics_system(mut metrics: ResMut<WeeklyServerMetrics>) {
    *metrics = WeeklyServerMetrics::default();
}
