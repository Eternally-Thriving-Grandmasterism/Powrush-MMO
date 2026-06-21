//! simulation/src/race.rs
//! Powrush-MMO Foundational Multi-Race System
//! v1.0 — 5 Sovereign Races (Terran, Synthetic, Harmonic, Verdant, Voidfarer) + RaceModifiers
//! Derived from Ra-Thor powrush-mmo-simulator authoritative reference (v15.x)
//! AG-SML v1.0 | TOLC 8 Living Mercy Gates | PATSAGi + Ra-Thor aligned
//! Purpose: Foundation for branching ability trees, epigenetic modulation, and cross-race diplomacy.

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
}

/// Foundational race-specific modifiers (extendable for movement, contribution, epigenetic, harmony).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceModifiers {
    pub movement_speed: f32,
    pub contribution_multiplier: f32,
    pub epigenetic_strength_bonus: f32,
    pub harmony_affinity: f32,
}

impl RaceModifiers {
    pub fn for_race(race: Race) -> Self {
        match race {
            Race::Terran => Self {
                movement_speed: 1.0,
                contribution_multiplier: 1.1,
                epigenetic_strength_bonus: 0.05,
                harmony_affinity: 1.0,
            },
            Race::Synthetic => Self {
                movement_speed: 1.05,
                contribution_multiplier: 1.15,
                epigenetic_strength_bonus: 0.0,
                harmony_affinity: 0.9,
            },
            Race::Harmonic => Self {
                movement_speed: 0.95,
                contribution_multiplier: 1.0,
                epigenetic_strength_bonus: 0.1,
                harmony_affinity: 1.3,
            },
            Race::Verdant => Self {
                movement_speed: 0.9,
                contribution_multiplier: 1.05,
                epigenetic_strength_bonus: 0.2,
                harmony_affinity: 1.2,
            },
            Race::Voidfarer => Self {
                movement_speed: 1.15,
                contribution_multiplier: 0.95,
                epigenetic_strength_bonus: 0.08,
                harmony_affinity: 0.85,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_modifiers() {
        let terran = RaceModifiers::for_race(Race::Terran);
        assert!(terran.contribution_multiplier > 1.0);
        let verdant = RaceModifiers::for_race(Race::Verdant);
        assert!(verdant.epigenetic_strength_bonus > 0.1);
    }
}
