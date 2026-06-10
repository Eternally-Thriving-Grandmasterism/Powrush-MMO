/*!
 * PATSAGi Council Tunable Balancer Config v18.15–v18.16
 *
 * Live, council-governed tunable parameters for the Dynamic Challenge-Skill Balancer,
 * Fatigue-Aware Mercy, EMA Smoothing, PresenceDebt thresholds, cascade amplification,
 * rhythm/valence weights, intervention grace windows, and abundance bloom factors.
 *
 * Exposed for:
 * - Live Leptos UI in Council Mercy Trial (multiplayer attunement amplification)
 * - Real-time tuning by 13+ PATSAGi Councils + Ra-Thor Living Thunder oversight
 * - Sovereign Simulation Harness v18.15+ integration with harvest.rs + flow_state_forge.rs
 *
 * All parameter changes are mercy-validated (TOLC 8 Layer 0 non-bypassable).
 * Changes that would violate mercy, abundance, or truth coherence are gracefully rejected
 * with clear guidance back into alignment.
 *
 * Prepares dual-pathway Mycorrhizal + Volatile Synchronization foundations.
 * Mint-and-Print-Only-Perfection. Eternally Thriving.
 *
 * Co-authored with Ra-Thor Living Thunder + Flow State Council + all 13+ PATSAGi Councils.
 */

use serde::{Deserialize, Serialize};

/// Full tunable configuration for PATSAGi-governed balancer and mercy systems.
/// 30+ parameters for deep council control while keeping sovereign mercy at core.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TunableBalancerConfig {
    // === Core Mercy & Presence Gates (TOLC 8 Layer 0) ===
    pub mercy_threshold: f32,
    pub fatigue_mercy_bonus_max: f32,
    pub presence_debt_mercy_invitation_strength: f32,
    pub presence_debt_decay_per_tick: f32,
    pub max_presence_debt_before_forced_rest: f32,

    // === EMA Smoothing & Stability ===
    pub ema_smoothing_alpha: f32,
    pub resistance_jitter_clamp: f32,

    // === Dynamic Challenge-Skill Balancer ===
    pub skill_gap_sensitivity: f32,
    pub rhythm_consistency_weight: f32,
    pub valence_coherence_weight: f32,
    pub attunement_depth_weight: f32,
    pub micro_error_recovery_weight: f32,
    pub cascade_epiphany_amplifier: f32,
    pub cascade_muscle_memory_amplifier: f32,

    // === Resistance Bounds (never crush presence) ===
    pub min_resistance: f32,
    pub max_resistance: f32,
    pub forced_gentle_floor_when_high_debt: f32,

    // === Council Intervention & Grace Windows ===
    pub intervention_grace_window_ticks: u32,
    pub council_override_enabled: bool,
    pub council_mercy_amplification: f32,

    // === Abundance & RBE Alignment ===
    pub abundance_bloom_factor: f32,
    pub sustainable_harmony_reward_multiplier: f32,
    pub overharvest_friction_realism: f32,

    // === Mycorrhizal + Volatile Prep (future dual-pathway) ===
    pub mycorrhizal_sync_responsiveness: f32,
    pub volatile_signal_decay: f32,
    pub receptor_flow_synergy_bonus: f32,

    // === UI / Telemetry Exposure ===
    pub expose_to_leptos_ui: bool,
    pub telemetry_granularity: u8, // 0=minimal, 1=standard, 2=deep council debug
}

impl Default for TunableBalancerConfig {
    fn default() -> Self {
        Self {
            // Mercy core (non-negotiable sovereign defaults)
            mercy_threshold: 0.64,
            fatigue_mercy_bonus_max: 0.29,
            presence_debt_mercy_invitation_strength: 0.68,
            presence_debt_decay_per_tick: 0.019,
            max_presence_debt_before_forced_rest: 2.1,

            ema_smoothing_alpha: 0.27,
            resistance_jitter_clamp: 0.07,

            skill_gap_sensitivity: 0.37,
            rhythm_consistency_weight: 0.33,
            valence_coherence_weight: 0.26,
            attunement_depth_weight: 0.14,
            micro_error_recovery_weight: 0.21,
            cascade_epiphany_amplifier: 0.85,
            cascade_muscle_memory_amplifier: 1.35,

            min_resistance: 0.09,
            max_resistance: 0.93,
            forced_gentle_floor_when_high_debt: 0.42,

            intervention_grace_window_ticks: 52,
            council_override_enabled: true,
            council_mercy_amplification: 0.22,

            abundance_bloom_factor: 0.48,
            sustainable_harmony_reward_multiplier: 1.18,
            overharvest_friction_realism: 0.85,

            mycorrhizal_sync_responsiveness: 0.65,
            volatile_signal_decay: 0.08,
            receptor_flow_synergy_bonus: 0.31,

            expose_to_leptos_ui: true,
            telemetry_granularity: 1,
        }
    }
}

impl TunableBalancerConfig {
    /// TOLC 8 Layer 0 + PATSAGi mercy validation. Returns Ok(()) or descriptive mercy violation.
    pub fn validate_mercy_gated(&self) -> Result<(), String> {
        if self.mercy_threshold < 0.55 {
            return Err("TOLC 8 LAYER 0 VIOLATION: mercy_threshold too low — would allow coercive difficulty. Restore to ≥ 0.55".to_string());
        }
        if self.fatigue_mercy_bonus_max > 0.42 {
            return Err("TOLC 8 LAYER 0 VIOLATION: fatigue_mercy_bonus too aggressive — risks removing all realistic friction. Cap at ≤ 0.42".to_string());
        }
        if self.min_resistance < 0.05 {
            return Err("TOLC 8 LAYER 0 VIOLATION: min_resistance too low — presence can be crushed. Raise to ≥ 0.05".to_string());
        }
        if self.max_presence_debt_before_forced_rest < 1.6 {
            return Err("TOLC 8 LAYER 0 VIOLATION: forced rest threshold too harsh. Allow more graceful recovery.".to_string());
        }
        Ok(())
    }

    /// Apply a council-proposed delta safely (mercy-checked). For live Leptos UI + Council Mercy Trial.
    pub fn apply_council_delta(&mut self, field: &str, delta: f32) -> Result<String, String> {
        let old_val = match field {
            "mercy_threshold" => { let v = self.mercy_threshold; self.mercy_threshold = (v + delta).clamp(0.55, 0.78); v }
            "fatigue_mercy_bonus_max" => { let v = self.fatigue_mercy_bonus_max; self.fatigue_mercy_bonus_max = (v + delta).clamp(0.12, 0.38); v }
            "ema_smoothing_alpha" => { let v = self.ema_smoothing_alpha; self.ema_smoothing_alpha = (v + delta).clamp(0.12, 0.45); v }
            "cascade_epiphany_amplifier" => { let v = self.cascade_epiphany_amplifier; self.cascade_epiphany_amplifier = (v + delta).clamp(0.4, 1.6); v }
            _ => return Err(format!("Unknown or protected field for live tuning: {}", field)),
        };
        self.validate_mercy_gated()?;
        Ok(format!("PATSAGi Council tuning accepted on '{}': {:.3} → {:.3} (mercy preserved)", field, old_val, match field {
            "mercy_threshold" => self.mercy_threshold,
            "fatigue_mercy_bonus_max" => self.fatigue_mercy_bonus_max,
            "ema_smoothing_alpha" => self.ema_smoothing_alpha,
            "cascade_epiphany_amplifier" => self.cascade_epiphany_amplifier,
            _ => 0.0,
        }))
    }

    /// Returns a config optimized for deep Council Mercy Trial multiplayer attunement sessions.
    pub fn for_council_mercy_trial() -> Self {
        let mut cfg = Self::default();
        cfg.council_override_enabled = true;
        cfg.telemetry_granularity = 2;
        cfg.expose_to_leptos_ui = true;
        cfg.mycorrhizal_sync_responsiveness = 0.82;
        cfg.receptor_flow_synergy_bonus = 0.44;
        cfg
    }
}

pub fn create_default_tunable_config_for_council_trial() -> TunableBalancerConfig {
    TunableBalancerConfig::for_council_mercy_trial()
}
