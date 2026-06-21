//! simulation/src/ability_tree.rs
//! Powrush-MMO Foundational Ability Tree System
//! v1.0 — Core Ability, AbilityEffect, AbilityTree with unlock + activation (derived from Ra-Thor)
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Future: branching, synergies, mutations, cross-race chains
//! Purpose: Foundation for race-specific progression, UI state exposure, and evolutionary mechanics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

    /// UI-ready state snapshot (foundation for get_ability_states).
    pub fn get_ability_states(&self, current_tick: u64) -> Vec<AbilityState> {
        // Placeholder: In full version this would be populated from a registry.
        // For foundational, return unlocked with cooldown status.
        self.unlocked.iter().map(|id| {
            let on_cooldown = self.cooldowns.get(id).map_or(false, |&end| current_tick < end);
            let remaining = self.cooldowns.get(id).map_or(0, |&end| end.saturating_sub(current_tick));
            AbilityState {
                id: id.clone(),
                name: id.clone(), // In full version: lookup from registry
                description: String::new(),
                unlocked: true,
                on_cooldown,
                remaining_cooldown_ticks: remaining as u32,
                cooldown_progress: if on_cooldown { 1.0 } else { 0.0 },
            }
        }).collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unlock_and_use() {
        let mut tree = AbilityTree::new();
        assert!(tree.try_unlock_starter("steady_step"));
        assert!(tree.try_use_ability("steady_step", 10, 50));
        assert!(!tree.try_use_ability("steady_step", 20, 50)); // still on cooldown
    }
}
