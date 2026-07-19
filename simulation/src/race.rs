//! simulation/src/race.rs
//! Powrush-MMO Foundational Multi-Race System
//! v1.1 — 5 Sovereign Races (Terran, Synthetic, Harmonic, Verdant, Voidfarer) + RaceModifiers + Starter Ability Registries
//! Derived from Ra-Thor powrush-mmo-simulator authoritative reference (v15.x)
//! AG-SML v1.0 | TOLC 8 Living Mercy Gates | PATSAGi + Ra-Thor aligned
//! Purpose: Foundation for branching ability trees, epigenetic modulation, and cross-race diplomacy.
//! Completes DERIVATION_STATUS Phase A. Feeds ability_tree synergy chains + orchestrator TickResult.

use serde::{Deserialize, Serialize};

/// The five sovereign races of Powrush.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Race {
    Terran,
    Synthetic,
    Harmonic,
    Verdant,
    Voidfarer,
}

impl Race {
    pub fn all() -> [Race; 5] {
        [Race::Terran, Race::Synthetic, Race::Harmonic, Race::Verdant, Race::Voidfarer]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Race::Terran => "Terran",
            Race::Synthetic => "Synthetic",
            Race::Harmonic => "Harmonic",
            Race::Verdant => "Verdant",
            Race::Voidfarer => "Voidfarer",
        }
    }

    /// Canonical starter abilities for this race (unlocked on spawn).
    /// These seed the ability_tree and enable immediate synergy chain progression.
    pub fn starter_abilities(&self) -> &'static [&'static str] {
        match self {
            Race::Terran => &["steady_step", "community_bond", "fortress_resolve"],
            Race::Synthetic => &["systems_overclock", "precision_calibration", "adaptive_matrix"],
            Race::Harmonic => &["resonant_jump", "cosmic_attunement", "harmony_pulse"],
            Race::Verdant => &["mycelial_root", "epigenetic_bloom", "resilience_weave"],
            Race::Voidfarer => &["void_phase", "singularity_glimpse", "dimensional_shift"],
        }
    }
}

/// Foundational race-specific modifiers (extendable for movement, contribution, epigenetic, harmony).
/// Directly influence ability_tree stage progression, epigenetic volatility drift, and RBE efficiency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceModifiers {
    pub movement_speed: f32,
    pub contribution_multiplier: f32,
    pub epigenetic_strength_bonus: f32,
    pub harmony_affinity: f32,
    /// Bonus to Reality Thriving Transfer Score accumulation (Kardashev layer).
    pub thriving_transfer_bonus: f32,
}

impl RaceModifiers {
    pub fn for_race(race: Race) -> Self {
        match race {
            Race::Terran => Self {
                movement_speed: 1.0,
                contribution_multiplier: 1.1,
                epigenetic_strength_bonus: 0.05,
                harmony_affinity: 1.0,
                thriving_transfer_bonus: 0.08,
            },
            Race::Synthetic => Self {
                movement_speed: 1.05,
                contribution_multiplier: 1.15,
                epigenetic_strength_bonus: 0.0,
                harmony_affinity: 0.9,
                thriving_transfer_bonus: 0.12,
            },
            Race::Harmonic => Self {
                movement_speed: 0.95,
                contribution_multiplier: 1.0,
                epigenetic_strength_bonus: 0.1,
                harmony_affinity: 1.3,
                thriving_transfer_bonus: 0.15,
            },
            Race::Verdant => Self {
                movement_speed: 0.9,
                contribution_multiplier: 1.05,
                epigenetic_strength_bonus: 0.2,
                harmony_affinity: 1.2,
                thriving_transfer_bonus: 0.18,
            },
            Race::Voidfarer => Self {
                movement_speed: 1.15,
                contribution_multiplier: 0.95,
                epigenetic_strength_bonus: 0.08,
                harmony_affinity: 0.85,
                thriving_transfer_bonus: 0.10,
            },
        }
    }
}

/// Helper: seed an AbilityTree with a race's starter abilities (idempotent).
pub fn seed_starter_abilities(tree: &mut crate::ability_tree::AbilityTree, race: Race) {
    for ability_id in race.starter_abilities() {
        tree.try_unlock_starter(ability_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ability_tree::AbilityTree;

    #[test]
    fn test_race_modifiers() {
        let terran = RaceModifiers::for_race(Race::Terran);
        assert!(terran.contribution_multiplier > 1.0);
        let verdant = RaceModifiers::for_race(Race::Verdant);
        assert!(verdant.epigenetic_strength_bonus > 0.1);
        assert!(verdant.thriving_transfer_bonus > 0.1);
    }

    #[test]
    fn test_starter_abilities_seed() {
        let mut tree = AbilityTree::new();
        seed_starter_abilities(&mut tree, Race::Harmonic);
        assert!(tree.unlocked.contains(&"resonant_jump".to_string()));
        assert!(tree.unlocked.contains(&"cosmic_attunement".to_string()));
        assert_eq!(tree.unlocked.len(), 3);
    }
}
