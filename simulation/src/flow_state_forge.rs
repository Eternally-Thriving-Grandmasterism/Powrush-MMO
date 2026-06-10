use serde::{Deserialize, Serialize};

// ... (previous FlowStateMetrics, FlowStateOutcome, check_flow_state, merge_flow_into_epiphany, etc. from v18.13)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeBalancerConfig {
    pub mercy_factor: f32,
    pub growth_factor: f32,
    pub rhythm_bonus: f32,
    pub min_resistance: f32,
    pub max_resistance: f32,
}

impl Default for ChallengeBalancerConfig {
    fn default() -> Self {
        Self {
            mercy_factor: 0.38,
            growth_factor: 0.22,
            rhythm_bonus: 0.07,
            min_resistance: 0.06,
            max_resistance: 1.85,
        }
    }
}

/// Dynamically adjusts harvest/node resistance to keep the player in the golden
/// flow channel (challenge ≈ skill). Mercy-gated, autotelic, and wholesome.
pub fn dynamic_challenge_skill_balancer(
    metrics: &FlowStateMetrics,
    current_resistance: f32,
    config: &ChallengeBalancerConfig,
) -> f32 {
    let gap = metrics.current_challenge_level - metrics.estimated_player_skill;

    let mut adjusted = current_resistance;

    if gap > 0.11 {
        // Player struggling → mercy invitation into flow
        let mercy_pull = (gap * config.mercy_factor).min(0.32);
        adjusted -= mercy_pull;

        if metrics.valence_coherence_spike < 0.42 {
            adjusted -= 0.06; // extra grace when emotional friction is high
        }
    } else if gap < -0.07 {
        // Too easy → gentle growth nudge for autotelic development
        let growth_push = ((-gap) * config.growth_factor).min(0.26);
        adjusted += growth_push;
    }

    // Rhythm reward — high consistency makes entry slightly more forgiving
    if metrics.rhythm_consistency > 0.76 {
        adjusted -= config.rhythm_bonus;
    }

    // Sustained presence bonus
    if metrics.sustained_focus_duration_ticks > 165 {
        adjusted *= 0.955;
    }

    adjusted.clamp(config.min_resistance, config.max_resistance)
}

// Note: Full previous v18.13 content (FlowStateMetrics, check_flow_state, merge_flow_into_epiphany, etc.)
// is assumed present above this addition. In real commit the full file would be the complete updated module.