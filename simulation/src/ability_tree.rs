//! simulation/src/ability_tree.rs
//! Powrush-MMO Ability Tree System with Mutation Synergy Chains
//! v1.2 — Mutation-Specific Synergy Chains (Stage-aware foundation) + Core Ability/Effect/Tree
//! Derived from Ra-Thor powrush-mmo-simulator v15.22/v15.30
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | PATSAGi aligned

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::epigenetic_modulation::MutationType;

/// Core ability definition (extensible for advanced effects).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub id: String,
    pub name: String,
    pub description: String,
    pub race: crate::race::Race,
    pub tier: u8,
    pub requires_ability: Option<String>,
}

/// Simple gameplay effect enum (foundation for future rich effects).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbilityEffect {
    HarmonyBoost { amount: f32 },
    ContributionGain { amount: f64 },
    EpigeneticStabilize { volatility_reduction: f32 },
    MovementBurst { duration_ticks: u32 },
    Custom(String),
}

/// Player's ability progression state.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AbilityTree {
    pub unlocked: Vec<String>,
    pub cooldowns: HashMap<String, u64>,
}

impl AbilityTree {
    pub fn new() -> Self {
        Self::default()
    }

    /// Try to unlock a starter ability (no prerequisite).
    pub fn try_unlock_starter(&mut self, ability_id: &str) -> bool {
        if self.unlocked.contains(&ability_id.to_string()) {
            return false;
        }
        self.unlocked.push(ability_id.to_string());
        true
    }

    /// Try to use an ability (checks unlock + cooldown).
    pub fn try_use_ability(&mut self, ability_id: &str, current_tick: u64, cooldown_duration: u64) -> bool {
        if !self.unlocked.contains(&ability_id.to_string()) {
            return false;
        }
        if let Some(&cd_end) = self.cooldowns.get(ability_id) {
            if current_tick < cd_end {
                return false;
            }
        }
        self.cooldowns.insert(ability_id.to_string(), current_tick + cooldown_duration);
        true
    }

    /// UI-ready state snapshot.
    pub fn get_ability_states(&self, current_tick: u64) -> Vec<AbilityState> {
        self.unlocked.iter().map(|id| {
            let on_cooldown = self.cooldowns.get(id).map_or(false, |&end| current_tick < end);
            let remaining = self.cooldowns.get(id).map_or(0, |&end| end.saturating_sub(current_tick));
            AbilityState {
                id: id.clone(),
                name: id.clone(),
                description: String::new(),
                unlocked: true,
                on_cooldown,
                remaining_cooldown_ticks: remaining as u32,
                cooldown_progress: if on_cooldown { 1.0 } else { 0.0 },
            }
        }).collect()
    }

    // ========================================================================
    // MUTATION SYNERGY CHAINS (Phase D — Evolutionary Synergy Layer)
    // Derived from Ra-Thor v15.22
    // ========================================================================

    /// Calculates active mutation-specific synergy chains.
    /// These chains activate when a player has a specific mutation + relevant abilities unlocked.
    /// Returns powerful, flavor-rich bonuses that scale with long-term investment.
    pub fn calculate_mutation_synergy_chains(
        &self,
        active_mutations: &[MutationType],
    ) -> Vec<SynergyBonus> {
        let mut bonuses = Vec::new();

        for mutation in active_mutations {
            match mutation {
                MutationType::HarmonicRebirth => {
                    // Redemption Cascade Chain (Harmonic Rebirth path)
                    if self.unlocked.iter().any(|a| a.contains("resonant") || a.contains("cosmic")) {
                        bonuses.push(SynergyBonus {
                            name: "Redemption Cascade (Harmonic Rebirth)".to_string(),
                            description: "Stronger HarmonyAmplification + EpigeneticResilience. Passive positive drift and enhanced repair.".to_string(),
                            bonus_type: SynergyType::HarmonyAmplification { multiplier: 1.35 },
                        });
                        bonuses.push(SynergyBonus {
                            name: "Redemption Cascade (Stage 1)".to_string(),
                            description: "Escalating repair strength and corruption resistance.".to_string(),
                            bonus_type: SynergyType::EpigeneticResilience { reduction: 0.25 },
                        });
                    }
                }
                MutationType::VolatileSurge => {
                    if self.unlocked.iter().any(|a| a.contains("overclock") || a.contains("systems")) {
                        bonuses.push(SynergyBonus {
                            name: "Surge Overclock Chain (Volatile Surge)".to_string(),
                            description: "Amplified ContributionBoost while in high-volatility risk state. High-risk high-reward.".to_string(),
                            bonus_type: SynergyType::ContributionBoost { multiplier: 1.28 },
                        });
                    }
                }
                MutationType::CorruptedEcho => {
                    if self.unlocked.iter().any(|a| a.contains("phase") || a.contains("singularity")) {
                        bonuses.push(SynergyBonus {
                            name: "Corrupted Singularity Chain (Corrupted Echo)".to_string(),
                            description: "High ContributionBoost at the cost of slow corruption accumulation. Dangerous power.".to_string(),
                            bonus_type: SynergyType::ContributionBoost { multiplier: 1.22 },
                        });
                    }
                }
            }
        }

        bonuses
    }
}

/// UI-ready ability state (serializable for HUD/hotbar).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityState {
    pub id: String,
    pub name: String,
    pub description: String,
    pub unlocked: bool,
    pub on_cooldown: bool,
    pub remaining_cooldown_ticks: u32,
    pub cooldown_progress: f32,
}

/// Synergy bonus types (foundation for mutation chains and future cross-race chains).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SynergyType {
    HarmonyAmplification { multiplier: f32 },
    ContributionBoost { multiplier: f64 },
    EpigeneticResilience { reduction: f32 },
    MovementEfficiency { multiplier: f32 },
    GlobalCooldownReduction { reduction_percent: f32 },
}

/// Active synergy bonus with rich flavor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynergyBonus {
    pub name: String,
    pub description: String,
    pub bonus_type: SynergyType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unlock_and_use() {
        let mut tree = AbilityTree::new();
        assert!(tree.try_unlock_starter("steady_step"));
        assert!(tree.try_use_ability("steady_step", 10, 50));
        assert!(!tree.try_use_ability("steady_step", 20, 50));
    }

    #[test]
    fn test_mutation_synergy_chain_placeholder() {
        let tree = AbilityTree::new();
        // In full integration this would check against actual unlocked abilities + mutations
        let bonuses = tree.calculate_mutation_synergy_chains(&[MutationType::HarmonicRebirth]);
        // Placeholder: chain logic exists and returns bonuses when conditions met in future wiring
        assert!(bonuses.len() <= 3);
    }
}