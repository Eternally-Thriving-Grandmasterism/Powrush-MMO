//! simulation/src/epigenetic_modulation.rs
//! Epigenetic Modulation & Volatility Lifecycle System
//! Derived from Ra-Thor powrush-mmo-simulator v15.30
//! Harmonized with Powrush-MMO mycorrhizal_volatile_sync, epiphany_catalyst, grace_blessing
//! AG-SML v1.0 | TOLC 8 Living Mercy Gates | PATSAGi aligned

use serde::{Deserialize, Serialize};

/// Core epigenetic state for a player/entity.
/// Volatility acts as a true double-edged sword: high volatility grants temporary power
/// but carries structured risk of backlash and corruption.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpigeneticProfile {
    pub strength: f32,           // Overall epigenetic power / resilience
    pub volatility: f32,         // Instability (0.05–2.0). High = risky power + backlash danger
    pub layer_alignment: f32,
    pub cooperation_score: f64,
    pub corruption: f32,         // Accumulated taint from sustained high volatility (0.0–2.0+)
}

impl Default for EpigeneticProfile {
    fn default() -> Self {
        Self {
            strength: 1.0,
            volatility: 0.65,
            layer_alignment: 0.8,
            cooperation_score: 0.7,
            corruption: 0.0,
        }
    }
}

/// Simple change descriptor for apply_change.
#[derive(Debug, Clone)]
pub struct EpigeneticChange {
    pub strength_delta: f32,
    pub volatility_delta: f32,
    pub cooperation_delta: f64,
}

/// Applies an epigenetic change to the profile.
/// This is the primary mutation point for abilities, synergies, and events.
pub fn apply_change(profile: &mut EpigeneticProfile, change: &EpigeneticChange) {
    profile.strength = (profile.strength + change.strength_delta).clamp(0.2, 4.0);
    profile.volatility = (profile.volatility + change.volatility_delta).clamp(0.05, 2.2);
    profile.cooperation_score = (profile.cooperation_score + change.cooperation_delta).clamp(0.0, 1.5);
}

// ============================================================================
// VOLATILITY LIFECYCLE MECHANICS (Core Double-Edged Sword System)
// ============================================================================

/// Thresholds with hysteresis to prevent rapid state flickering.
pub const VOLATILITY_HIGH_ENTER: f32 = 1.25;
pub const VOLATILITY_HIGH_EXIT: f32 = 1.10;

/// Applies natural + state-modulated volatility drift every tick.
/// High harmony and Resilience synergy strongly suppress upward drift.
pub fn apply_volatility_drift(
    profile: &mut EpigeneticProfile,
    global_harmony: f32,
    has_resilience_synergy: bool,
) {
    let mut drift = 0.007; // Base natural upward entropy

    if global_harmony > 1.8 {
        drift *= 0.45;
    } else if global_harmony > 1.2 {
        drift *= 0.7;
    } else if global_harmony < 0.7 {
        drift *= 1.35;
    }

    if has_resilience_synergy {
        drift *= 0.35; // Strong suppression from Verdant path
    }

    profile.volatility = (profile.volatility + drift).clamp(0.05, 2.2);
}

/// Returns whether the profile is currently in the high-volatility risk state.
/// Uses hysteresis: enters at 1.25, exits only at 1.10.
pub fn is_high_volatility_risk(
    profile: &EpigeneticProfile,
    currently_in_risk: bool,
) -> bool {
    if currently_in_risk {
        profile.volatility >= VOLATILITY_HIGH_EXIT
    } else {
        profile.volatility >= VOLATILITY_HIGH_ENTER
    }
}

/// Applies the double-edged sword effects while in high-volatility risk state.
/// Grants gradual strength gain but introduces backlash risk.
pub fn apply_double_edged_volatility_effects(
    profile: &mut EpigeneticProfile,
    in_high_risk: bool,
    current_tick: u64,
) -> Option<String> {
    if !in_high_risk {
        return None;
    }

    // Power gain (the reward)
    profile.strength = (profile.strength + 0.025).min(3.8);

    // Backlash risk (the danger) - scales with volatility and corruption
    let backlash_chance = 0.18 + (profile.volatility - 1.2) * 0.12 + profile.corruption * 0.08;

    if (current_tick % 35 == 0) && rand::random::<f32>() < backlash_chance.clamp(0.05, 0.65) {
        let severity = if profile.volatility > 1.65 || profile.corruption > 1.1 {
            "Major"
        } else if profile.volatility > 1.35 {
            "Moderate"
        } else {
            "Minor"
        };

        let strength_loss = match severity {
            "Major" => 0.18,
            "Moderate" => 0.11,
            _ => 0.06,
        };
        profile.strength = (profile.strength - strength_loss).max(0.4);
        profile.volatility = (profile.volatility + 0.09).min(2.2);

        if severity == "Major" {
            profile.corruption = (profile.corruption + 0.12).min(2.5);
        }

        return Some(format!("epigenetic_backlash_{}_tick_{}", severity.to_lowercase(), current_tick));
    }

    None
}

/// Applies repair when conditions are favorable (low volatility + Resilience synergy active).
/// Strong repair also slowly cleanses corruption.
pub fn apply_epigenetic_repair(
    profile: &mut EpigeneticProfile,
    has_resilience_synergy: bool,
    global_harmony: f32,
) {
    if profile.volatility < 0.65 && has_resilience_synergy {
        // Strong active repair
        profile.volatility = (profile.volatility - 0.035).max(0.08);
        profile.strength = (profile.strength + 0.04).min(3.5);
        if global_harmony > 1.5 {
            profile.corruption = (profile.corruption - 0.025).max(0.0);
        }
    } else if profile.volatility < 0.85 {
        // Mild natural repair
        profile.volatility = (profile.volatility - 0.012).max(0.08);
    }

    // Gentle harmony-assisted cleansing even at moderate volatility
    if global_harmony > 2.0 && profile.corruption > 0.1 {
        profile.corruption = (profile.corruption - 0.015).max(0.0);
    }
}

/// Applies corruption accumulation from sustained high-risk periods
/// and passive cleansing when conditions improve.
pub fn apply_corruption_lifecycle(
    profile: &mut EpigeneticProfile,
    in_high_risk: bool,
    has_resilience_synergy: bool,
) {
    if in_high_risk {
        profile.corruption = (profile.corruption + 0.004).min(2.8);
    }

    if !in_high_risk && has_resilience_synergy && profile.corruption > 0.05 {
        profile.corruption = (profile.corruption - 0.018).max(0.0);
    }
}

// ============================================================================
// EPIGENETIC MUTATION TRIGGERS (Phase C — Evolutionary Branch Points)
// Derived from Ra-Thor v15.20
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MutationType {
    /// Redemptive / stabilizing path — triggered by high harmony + Resilience synergy
    HarmonicRebirth,
    /// High-risk / high-reward power spike path
    VolatileSurge,
    /// Dangerous long-term consequence from extreme corruption
    CorruptedEcho,
}

impl MutationType {
    pub fn name(&self) -> &'static str {
        match self {
            MutationType::HarmonicRebirth => "Harmonic Rebirth",
            MutationType::VolatileSurge => "Volatile Surge",
            MutationType::CorruptedEcho => "Corrupted Echo",
        }
    }

/// Attempts to trigger a permanent epigenetic mutation when conditions are met.
/// This is the key evolutionary branch point on top of the volatility lifecycle.
/// Returns Some(mutation) if a mutation successfully triggers (one-time per type in practice).
pub fn try_trigger_epigenetic_mutation(
    profile: &EpigeneticProfile,
    in_high_risk: bool,
    has_resilience_synergy: bool,
    global_harmony: f32,
    current_tick: u64,
) -> Option<MutationType> {
    if !in_high_risk {
        return None;
    }

    // Core trigger conditions (corruption + volatility threshold)
    if profile.corruption <= 1.05 || profile.volatility <= 1.40 {
        return None;
    }

    // Check window (every ~55 ticks)
    if current_tick % 55 != 0 {
        return None;
    }

    // Probabilistic trigger (~32% base, scales with corruption)
    let trigger_chance = 0.32 + (profile.corruption - 1.0) * 0.08;
    if rand::random::<f32>() >= trigger_chance.clamp(0.25, 0.65) {
        return None;
    }

    // Priority order: Harmonic Rebirth (redemption) > Corrupted Echo (extreme taint) > Volatile Surge (default risky power)
    if global_harmony > 1.9 && has_resilience_synergy && profile.cooperation_score > 0.85 {
        return Some(MutationType::HarmonicRebirth);
    }

    if profile.corruption > 1.65 {
        return Some(MutationType::CorruptedEcho);
    }

    Some(MutationType::VolatileSurge)
}