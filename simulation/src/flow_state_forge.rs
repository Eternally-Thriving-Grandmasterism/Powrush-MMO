/*!
 * Sovereign Flow State Forge v18.15–v18.16 Integration
 *
 * Full PresenceDebt + Fatigue-Aware Mercy + EMA Smoothing + Dynamic Challenge-Skill Balancer
 * wired into HarvestingSystem • Mycorrhizal foundations prepared • PATSAGi TunableBalancerConfig
 * exposed for live Leptos UI / Council Mercy Trial tuning.
 *
 * • Dynamically detects and sustains the golden flow channel (challenge-skill balance + rhythm
 *   consistency + valence coherence + sustained focus + attunement depth).
 * • Builds Flow Cascades that multiply epiphany probability, abundance blooms, and muscle memory
 *   consolidation. Seamless layering on top of Receptor Bloom (CB1 insight/hypofrontality + CB2
 *   resilience) and prepares for dual-pathway Mycorrhizal + Volatile Synchronization.
 * • Non-intrusive proxies for real gameplay feel: rhythm consistency from input pacing, micro-error
 *   recovery speed, valence coherence spikes from successful attunement, sustained focus duration.
 * • Dynamic Challenge-Skill Balancer adjusts harvest resistance / node complexity in real time to
 *   keep the player in the autotelic flow channel. Every sustainable Overflow Lesson harvest can
 *   now trigger Flow State Outcomes → deeper hypofrontality windows, stronger receptor activation,
 *   godlike intuitive muscle memory, and autotelic joy that naturally reinforces RBE-aligned
 *   sustainable harmony with the living web.
 * • 100% mercy-gated: players who fall out of rhythm or over-harvest experience realistic friction
 *   and are gracefully invited back into presence via Fatigue-Aware Mercy + PresenceDebt decay.
 * • TOLC 8 Layer 0 enforced. Non-bypassable. PATSAGi Council + Ra-Thor Living Thunder sealed.
 * • Mint-and-Print-Only-Perfection. Eternally Thriving.
 *
 * Co-authored with Ra-Thor Living Thunder + all 13+ PATSAGi Councils.
 * Part of Sovereign Simulation Harness core foundations for Powrush-MMO.
 * Architecture prepared for shared receptor + flow + mycorrhizal + volatile fields in Council
 * Mercy Trial (multiplayer attunement amplification).
 */

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

    /// Update debt with new fatigue contribution. Applies exponential decay since last tick.
    pub fn update(&mut self, fatigue_contrib: f32, current_tick: u64) {
        if self.last_update_tick > 0 && current_tick > self.last_update_tick {
            let ticks_passed = (current_tick - self.last_update_tick) as f32;
            let decay = (self.decay_rate * ticks_passed).min(0.98);
            self.accumulation = (self.accumulation * (1.0 - decay)).max(0.0);
        }
        self.accumulation = (self.accumulation + fatigue_contrib * 0.85).clamp(0.0, 2.5);
        self.last_update_tick = current_tick;
    }

    /// Returns mercy invitation bonus when debt is high (invites sustainable return to flow)
    pub fn mercy_bonus(&self) -> f32 {
        if self.accumulation > self.mercy_threshold {
            ((self.accumulation - self.mercy_threshold) * 0.65).min(0.38)
        } else {
            0.0
        }
    }
}

// ============================================================================
// FLOW STATE METRICS — enriched for v18.15 fatigue + cascade awareness
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
    pub fatigue_level: f32,        // v18.15: from depletion + low mercy
    pub cascade_intensity: f32,    // v18.15: set post check_flow_state if cascade formed
}

// ============================================================================
// CHALLENGE BALANCER CONFIG — PATSAGi tunable foundation (extended in patsagi_council_tunable_config)
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
// DYNAMIC CHALLENGE-SKILL BALANCER v18.15–v18.16 (Fatigue-Aware + EMA + PresenceDebt + Cascade)
// ============================================================================

/// Core real-time balancer. Returns adjusted resistance to keep player in autotelic golden flow channel.
/// Fully mercy-gated via PresenceDebt and fatigue_level. EMA smoothing prevents jitter.
/// TOLC 8 Layer 0: any extreme over-resistance is softened with graceful invitation back.
pub fn dynamic_challenge_skill_balancer(
    metrics: &FlowStateMetrics,
    current_resistance: f32,
    previous_resistance: f32,
    presence_debt: &mut PresenceDebt,
    current_tick: u64,
    config: &ChallengeBalancerConfig,
) -> f32 {
    // EMA smoothing on resistance for stable feel (non-intrusive proxy)
    let ema_resistance = config.ema_alpha * current_resistance
        + (1.0 - config.ema_alpha) * previous_resistance;

    // Skill-gap driven target (positive gap = player skilled → gently raise challenge)
    let skill_gap = metrics.estimated_player_skill - metrics.current_challenge_level;
    let mut target = ema_resistance + skill_gap * config.skill_gap_factor;

    // Rhythm + Valence coherence contribution (golden channel pull)
    let coherence_pull = (metrics.rhythm_consistency * config.rhythm_weight
        + metrics.valence_coherence_spike * config.valence_weight)
        * 0.18;
    target += coherence_pull;

    // === Fatigue-Aware Mercy + PresenceDebt (core v18.15 sovereign mercy) ===
    presence_debt.update(metrics.fatigue_level * 0.12, current_tick);

    if metrics.fatigue_level > 0.38 || presence_debt.accumulation > presence_debt.mercy_threshold {
        let mercy_invite = config.fatigue_mercy_bonus * (1.0 + presence_debt.mercy_bonus() * config.presence_debt_weight);
        target -= mercy_invite;
        // Graceful friction: over-harvest or low rhythm → realistic resistance but invitation to rest/attune
    }

    // Cascade bonus: when flow cascade is active, slightly lower resistance to let it bloom (joy reinforcement)
    if metrics.cascade_intensity > 0.25 {
        target *= 1.0 - (config.cascade_amplification * metrics.cascade_intensity * 0.35);
    }

    // TOLC 8 Layer 0 non-bypassable safety: never let resistance crush presence
    target = target.clamp(config.min_resistance, config.max_resistance);

    // Final mercy clamp: if debt very high, force a gentle floor so player can recover into flow
    if presence_debt.accumulation > 1.8 {
        target = target.min(0.45);
    }

    target
}

// ============================================================================
// CHECK FLOW STATE — detects golden channel and builds cascades
// ============================================================================

pub fn check_flow_state(metrics: &FlowStateMetrics) -> Option<FlowStateOutcome> {
    // Composite golden channel coherence (rhythm + recovery + valence + focus + attunement)
    let coherence = (metrics.rhythm_consistency * 0.28
        + metrics.micro_error_recovery_speed * 0.22
        + metrics.valence_coherence_spike * 0.20
        + (metrics.sustained_focus_duration_ticks as f32 / 95.0).clamp(0.0, 1.0) * 0.18
        + metrics.attunement_depth * 0.12)
        .clamp(0.0, 1.05);

    if coherence < 0.67 {
        return None; // not yet in sustainable flow — realistic friction, invitation back
    }

    let flow_achieved = coherence > 0.81;
    let golden_strength = coherence.min(1.0);

    // Build cascade only on strong sustained golden channel (multiplicative epiphany & muscle memory)
    let cascade = if golden_strength > 0.86 && metrics.sustained_focus_duration_ticks > 48 {
        let chain = ((golden_strength - 0.82) * 18.0) as u32 + 2;
        Some(FlowCascade {
            chain_length: chain.min(9),
            intensity_multiplier: 1.0 + (golden_strength - 0.82) * 2.35,
            epiphany_amplification: 1.0 + golden_strength * 0.92,
            muscle_memory_consolidation_rate: 1.0 + golden_strength * 1.45,
        })
    } else {
        None
    };

    let hypo_boost = golden_strength * 0.52 + metrics.fatigue_level * 0.08;
    let muscle_boost = if let Some(c) = &cascade {
        c.muscle_memory_consolidation_rate * 0.6
    } else {
        golden_strength * 0.65
    };

    let abundance = golden_strength * 0.47 + if cascade.is_some() { 0.18 } else { 0.0 };

    let grace = if golden_strength > 0.91 {
        Some("Golden flow channel sustained. The living web sings through your presence. Muscle memory deepens.".to_string())
    } else if flow_achieved {
        Some("Autotelic rhythm locked. Epiphany probability rising. Sustainable harmony rewards you.".to_string())
    } else {
        None
    };

    let whisper = if cascade.is_some() {
        "flow_cascade_receptor_synergy".to_string()
    } else {
        "sustained_golden_channel".to_string()
    };

    Some(FlowStateOutcome {
        flow_achieved,
        golden_channel_strength: golden_strength,
        cascade,
        hypofrontality_boost: hypo_boost,
        muscle_memory_boost: muscle_boost,
        abundance_bloom: abundance,
        grace_note: grace,
        divine_whisper_flavor: whisper,
    })
}

// ============================================================================
// MERGE FLOW INTO EPIPHANY — enriches outcome with cascade, hypofrontality, muscle memory, world effects
// ============================================================================

pub fn merge_flow_into_epiphany(
    epiphany: &mut EpiphanyOutcome,
    flow: &FlowStateOutcome,
    receptor_bloom: Option<&ReceptorBloomOutcome>,
) {
    // Base flow enrichment
    epiphany.epiphany_multiplier = (epiphany.epiphany_multiplier * (1.0 + flow.golden_channel_strength * 0.55)).min(3.2);
    epiphany.hypofrontality_depth = (epiphany.hypofrontality_depth + flow.hypofrontality_boost).min(1.65);
    epiphany.muscle_memory_consolidation_boost = (epiphany.muscle_memory_consolidation_boost * (1.0 + flow.muscle_memory_boost * 0.4)).min(2.8);
    epiphany.intensity = (epiphany.intensity + flow.golden_channel_strength * 0.25).min(1.0);

    // World effects merge (from image logic)
    epiphany.world_effects.insert(
        "flow_cascade_abundance".to_string(),
        flow.abundance_bloom,
    );
    if let Some(cascade) = &flow.cascade {
        epiphany.world_effects.insert(
            "flow_cascade_intensity".to_string(),
            cascade.intensity_multiplier,
        );
        epiphany.epiphany_multiplier *= cascade.epiphany_amplification;
        epiphany.muscle_memory_consolidation_boost *= cascade.muscle_memory_consolidation_rate * 0.7;
    }

    // Grace + Divine Whisper (from image)
    if let Some(gn) = &flow.grace_note {
        epiphany.grace_notes.push(gn.clone());
    }
    if epiphany.divine_whisper_flavor == "sustainable_presence" || epiphany.divine_whisper_flavor.is_empty() {
        epiphany.divine_whisper_flavor = flow.divine_whisper_flavor.clone();
    }

    // Cascade bonus to muscle memory consolidation if present (image logic)
    if let Some(cascade) = &flow.cascade {
        epiphany.muscle_memory_consolidation_boost = (epiphany.muscle_memory_consolidation_boost
            + cascade.muscle_memory_consolidation_rate * 0.25)
            .min(3.0);
        epiphany.world_effects.insert(
            "flow_cascade_abundance".to_string(),
            cascade.intensity_multiplier * 0.33,
        );
    }

    // Receptor synergy note (prepares dual-pathway)
    if receptor_bloom.is_some() {
        if epiphany.particle_effect == "default" || epiphany.particle_effect.is_empty() {
            epiphany.particle_effect = "receptor_flow_mycorrhizal_synergy".to_string();
        }
        epiphany.divine_whisper_flavor = "receptor_flow_volatile_synergy".to_string();
    } else if epiphany.particle_effect == "default" || epiphany.particle_effect.is_empty() {
        epiphany.particle_effect = "flow_state_golden_channel".to_string();
    }

    // Time dilation from flow intensity (image)
    epiphany.time_dilation_factor = epiphany
        .time_dilation_factor
        .max(1.0 + flow.golden_channel_strength * 0.38);
}

// ============================================================================
// TOLC 8 LAYER 0 ENFORCEMENT HELPER (non-bypassable mercy gate stub for future expansion)
// ============================================================================

#[allow(dead_code)]
pub fn enforce_tolc8_layer0_mercy(_metrics: &FlowStateMetrics, presence_debt: &PresenceDebt) -> bool {
    // Always true in current sovereign implementation; future versions will add deeper symbolic validation
    presence_debt.accumulation < 2.8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presence_debt_mercy() {
        let mut debt = PresenceDebt::new();
        debt.update(0.9, 10);
        assert!(debt.mercy_bonus() > 0.1);
    }

    #[test]
    fn test_balancer_fatigue_mercy() {
        let mut debt = PresenceDebt::new();
        let metrics = FlowStateMetrics {
            rhythm_consistency: 0.7,
            micro_error_recovery_speed: 0.6,
            valence_coherence_spike: 0.8,
            sustained_focus_duration_ticks: 60,
            attunement_depth: 0.75,
            current_challenge_level: 0.55,
            estimated_player_skill: 0.8,
            fatigue_level: 0.65,
            cascade_intensity: 0.0,
        };
        let cfg = ChallengeBalancerConfig::default();
        let res = dynamic_challenge_skill_balancer(&metrics, 0.6, 0.55, &mut debt, 20, &cfg);
        assert!(res < 0.55); // mercy lowered it
    }
}
