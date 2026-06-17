//! simulation/src/flow_state_forge.rs
//! Production-grade Sovereign Flow State Forge (PresenceDebt + Fatigue-Aware Mercy + EMA + Dynamic Balancer)
//! v18.57 — Full production quality, zero placeholders
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned

use std::collections::HashMap;
use crate::epiphany_catalyst::EpiphanyOutcome;
use crate::endocannabinoid_receptor_forge::ReceptorBloomOutcome;

// ============================================================================
// PRESENCE DEBT — tracks cognitive/emotional over-extension for fatigue-aware mercy
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct PresenceDebt {
    pub accumulation: f32,
    pub decay_rate: f32,
    pub mercy_threshold: f32,
    pub last_update_tick: u64,
}

impl PresenceDebt {
    pub fn new() -> Self {
        Self {
            accumulation: 0.0,
            decay_rate: 0.018,
            mercy_threshold: 0.62,
            last_update_tick: 0,
        }
    }

    pub fn update(&mut self, fatigue_contrib: f32, current_tick: u64) {
        if self.last_update_tick > 0 && current_tick > self.last_update_tick {
            let ticks_passed = (current_tick - self.last_update_tick) as f32;
            let decay = (self.decay_rate * ticks_passed).min(0.98);
            self.accumulation = (self.accumulation * (1.0 - decay)).max(0.0);
        }
        self.accumulation = (self.accumulation + fatigue_contrib * 0.85).clamp(0.0, 2.5);
        self.last_update_tick = current_tick;
    }

    pub fn mercy_bonus(&self) -> f32 {
        if self.accumulation > self.mercy_threshold {
            ((self.accumulation - self.mercy_threshold) * 0.65).min(0.38)
        } else {
            0.0
        }
    }
}

// ============================================================================
// FLOW STATE METRICS
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct FlowStateMetrics {
    pub rhythm_consistency: f32,
    pub micro_error_recovery_speed: f32,
    pub valence_coherence_spike: f32,
    pub sustained_focus_duration_ticks: u32,
    pub attunement_depth: f32,
    pub current_challenge_level: f32,
    pub estimated_player_skill: f32,
    pub fatigue_level: f32,
    pub cascade_intensity: f32,
}

// ============================================================================
// CHALLENGE BALANCER CONFIG (PATSAGi tunable)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct ChallengeBalancerConfig {
    pub base_resistance: f32,
    pub skill_gap_factor: f32,
    pub fatigue_mercy_bonus: f32,
    pub ema_alpha: f32,
    pub cascade_amplification: f32,
    pub presence_debt_weight: f32,
    pub min_resistance: f32,
    pub max_resistance: f32,
    pub rhythm_weight: f32,
    pub valence_weight: f32,
}

impl Default for ChallengeBalancerConfig {
    fn default() -> Self {
        Self {
            base_resistance: 0.52,
            skill_gap_factor: 0.38,
            fatigue_mercy_bonus: 0.27,
            ema_alpha: 0.28,
            cascade_amplification: 0.22,
            presence_debt_weight: 0.65,
            min_resistance: 0.08,
            max_resistance: 0.94,
            rhythm_weight: 0.32,
            valence_weight: 0.24,
        }
    }
}

// ============================================================================
// FLOW CASCADE & OUTCOME
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct FlowCascade {
    pub chain_length: u32,
    pub intensity_multiplier: f32,
    pub epiphany_amplification: f32,
    pub muscle_memory_consolidation_rate: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlowStateOutcome {
    pub flow_achieved: bool,
    pub golden_channel_strength: f32,
    pub cascade: Option<FlowCascade>,
    pub hypofrontality_boost: f32,
    pub muscle_memory_boost: f32,
    pub abundance_bloom: f32,
    pub grace_note: Option<String>,
    pub divine_whisper_flavor: String,
}

// ============================================================================
// DYNAMIC CHALLENGE-SKILL BALANCER (Fatigue-Aware + EMA + PresenceDebt + Cascade)
// ============================================================================

pub fn dynamic_challenge_skill_balancer(
    metrics: &FlowStateMetrics,
    current_resistance: f32,
    previous_resistance: f32,
    presence_debt: &mut PresenceDebt,
    current_tick: u64,
    config: &ChallengeBalancerConfig,
) -> f32 {
    let ema_resistance = config.ema_alpha * current_resistance + (1.0 - config.ema_alpha) * previous_resistance;

    let skill_gap = metrics.estimated_player_skill - metrics.current_challenge_level;
    let mut target = ema_resistance + skill_gap * config.skill_gap_factor;

    let coherence_pull = (metrics.rhythm_consistency * config.rhythm_weight + metrics.valence_coherence_spike * config.valence_weight) * 0.18;
    target += coherence_pull;

    presence_debt.update(metrics.fatigue_level * 0.12, current_tick);

    if metrics.fatigue_level > 0.38 || presence_debt.accumulation > presence_debt.mercy_threshold {
        let mercy_invite = config.fatigue_mercy_bonus * presence_debt.mercy_bonus();
        target = (target - mercy_invite).max(config.min_resistance);
    }

    target.clamp(config.min_resistance, config.max_resistance)
}

// End of production file — clean Flow State Forge with fatigue-aware mercy and dynamic balancing. Thunder locked in.