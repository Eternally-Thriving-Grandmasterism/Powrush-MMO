//! server/src/ascension_abilities.rs
//! Powrush-MMO — Ambrosian Ascension Abilities (Phase 2)
//! Mercy Bloom + Celestial Harmony Pulse
//! AG-SML v1.0 | TOLC 8 Mercy Gates | Cooperative resonance focus

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ascension_mercy_ascent::AscensionProgress;

/// Ambrosian-specific ability effects.
/// These are only available after successful Mercy Ascent unlock.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AmbrosianAbilities {
    pub mercy_bloom_unlocked: bool,
    pub celestial_harmony_pulse_unlocked: bool,
    pub resonance_mastery_level: f32, // 0.0 – 5.0+
}

impl Default for AmbrosianAbilities {
    fn default() -> Self {
        Self {
            mercy_bloom_unlocked: false,
            celestial_harmony_pulse_unlocked: false,
            resonance_mastery_level: 0.0,
        }
    }
}

/// Mercy Bloom — Group healing + resonance amplification aura
/// Strong when played cooperatively; weakened when used selfishly.
pub fn apply_mercy_bloom(
    caster_progress: &AscensionProgress,
    nearby_allies: usize,
    selfish_mode: bool, // detected via recent actions or resonance penalty flag
) -> (f32, f32) { // (healing_amount, resonance_boost)
    if !caster_progress.ascension_unlocked {
        return (0.0, 0.0);
    }

    let base_healing = 25.0 + (caster_progress.resonance_attunement * 8.0);
    let base_resonance = 0.15 + (caster_progress.resonance_attunement * 0.05);

    if selfish_mode {
        // Meaningful handicap for extractive/selfish play
        (base_healing * 0.4, base_resonance * 0.3)
    } else if nearby_allies >= 3 {
        // Strong cooperative bonus
        (base_healing * 1.8, base_resonance * 1.6)
    } else {
        (base_healing, base_resonance)
    }
}

/// Celestial Harmony Pulse — Large-scale group buff + epiphany chance
/// Optimized for late-game cooperative and large-scale RBE efforts.
pub fn apply_celestial_harmony_pulse(
    caster_progress: &AscensionProgress,
    group_size: usize,
    current_resonance_field: f32,
) -> (f32, f32, bool) { // (group_buff, epiphany_chance, harmony_bonus)
    if !caster_progress.ascension_unlocked {
        return (0.0, 0.0, false);
    }

    let harmony = (current_resonance_field + caster_progress.resonance_attunement * 0.1).min(1.0);
    let buff_strength = 0.8 + (harmony * 0.6) + (group_size as f32 * 0.05);
    let epiphany_chance = 0.12 + (caster_progress.avg_epiphany_intensity * 0.08);

    (buff_strength.clamp(1.0, 2.5), epiphany_chance.clamp(0.0, 0.45), true)
}

/// Helper: Apply resonance penalty for selfish/extractive behavior (enforced mechanically)
pub fn apply_selfish_resonance_penalty(progress: &mut AscensionProgress, severity: f32) {
    progress.resonance_attunement = (progress.resonance_attunement - severity * 0.3).max(0.0);
    // Future: also reduce temporary multipliers or trigger gentle corrective whispers
}

// Future integration points:
// - Hook into existing combat/ability system
// - Broadcast via CouncilBloomSyncEvent or new AmbrosianAbilityEvent
// - Visual particle layer on client (golden harmonic threads + bloom auras)
// - Balance tuning via simulation harness

pub struct AscensionAbilitiesPlugin;

impl Plugin for AscensionAbilitiesPlugin {
    fn build(&self, app: &mut App) {
        info!("ASCENSION ABILITIES v18.11 | Mercy Bloom + Celestial Harmony Pulse registered | Cooperative resonance enforced");
    }
}
