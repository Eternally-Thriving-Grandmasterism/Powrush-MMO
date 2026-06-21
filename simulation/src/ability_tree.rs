//! simulation/src/ability_tree.rs
//! Powrush-MMO Ability Tree System with Mutation Synergy Chains + Stage 0/1/2 + Cross-Race Chain Synergy
//! v1.7 — Event Emission for Synergy Effects Added
//! Derived from Ra-Thor powrush-mmo-simulator v15.30
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | PATSAGi aligned

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::epigenetic_modulation::{MutationType, EpigeneticProfile};

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
    /// Tracks raw progress points for each mutation synergy chain (maturing over sustained play).
    pub chain_progress: HashMap<String, u32>,
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
    // MUTATION SYNERGY CHAINS + STAGE 0/1/2 PROGRESSION (Phase D Extension)
    // Derived from Ra-Thor v15.23
    // ========================================================================

    /// Progresses a mutation synergy chain based on current simulation state.
    /// High harmony, contribution, and stability accelerate maturation.
    /// Poor conditions cause slow natural decay.
    pub fn progress_chain_stages(&mut self, chain_key: &str, harmony: f32, contribution: f32, volatility: f32) {
        let progress = self.chain_progress.entry(chain_key.to_string()).or_insert(0);
        let mut delta: u32 = 0;

        if harmony > 1.8 {
            delta += 2;
        } else if harmony > 1.2 {
            delta += 1;
        }
        if contribution > 10.0 {
            delta += 1;
        }
        if volatility < 0.6 {
            delta += 1; // stability bonus
        }

        if delta == 0 && *progress > 0 {
            // slow decay under poor conditions
            *progress = progress.saturating_sub(1);
        } else {
            *progress = (*progress + delta).min(200); // cap at stage 2 threshold
        }
    }

    /// Returns current stage (0, 1, or 2) for a given chain.
    pub fn get_chain_stage(&self, chain_key: &str) -> u8 {
        let p = self.chain_progress.get(chain_key).copied().unwrap_or(0);
        if p >= 150 { 2 } else if p >= 75 { 1 } else { 0 }
    }

    /// Calculates active mutation-specific synergy chains.
    /// Now fully stage-aware: bonuses escalate as chains mature (Stage 0 → 1 → 2).
    pub fn calculate_mutation_synergy_chains(
        &self,
        active_mutations: &[MutationType],
    ) -> Vec<SynergyBonus> {
        let mut bonuses = Vec::new();

        for mutation in active_mutations {
            match mutation {
                MutationType::HarmonicRebirth => {
                    let stage = self.get_chain_stage("redemption_cascade");
                    if self.unlocked.iter().any(|a| a.contains("resonant") || a.contains("cosmic")) {
                        let (mult, name, desc) = match stage {
                            2 => (1.55, "Redemption Cascade (Stage 2 — Mastered)", "Maximum redemptive power: powerful ongoing epigenetic healing + harmony mastery."),
                            1 => (1.40, "Redemption Cascade (Stage 1)", "Escalating repair strength and corruption resistance."),
                            _ => (1.25, "Redemption Cascade (Stage 0)", "Foundational harmony repair + corruption resistance."),
                        };
                        bonuses.push(SynergyBonus {
                            name: name.to_string(),
                            description: desc.to_string(),
                            bonus_type: SynergyType::HarmonyAmplification { multiplier: mult },
                            stage,
                        });
                        bonuses.push(SynergyBonus {
                            name: format!("Redemption Cascade (Stage {})", stage),
                            description: "EpigeneticResilience scaling with chain maturity.".to_string(),
                            bonus_type: SynergyType::EpigeneticResilience { reduction: 0.20 + (stage as f32 * 0.08) },
                            stage,
                        });
                    }
                }
                MutationType::VolatileSurge => {
                    let stage = self.get_chain_stage("surge_overclock");
                    if self.unlocked.iter().any(|a| a.contains("overclock") || a.contains("systems")) {
                        let mult = 1.20 + (stage as f64 * 0.12);
                        bonuses.push(SynergyBonus {
                            name: format!("Surge Overclock Chain (Stage {})", stage),
                            description: "Amplified ContributionBoost while in high-volatility risk state. Scales with maturity.".to_string(),
                            bonus_type: SynergyType::ContributionBoost { multiplier: mult },
                            stage,
                        });
                    }
                }
                MutationType::CorruptedEcho => {
                    let stage = self.get_chain_stage("corrupted_singularity");
                    if self.unlocked.iter().any(|a| a.contains("phase") || a.contains("singularity")) {
                        let mult = 1.15 + (stage as f64 * 0.10);
                        bonuses.push(SynergyBonus {
                            name: format!("Corrupted Singularity Chain (Stage {})", stage),
                            description: "High ContributionBoost at the cost of slow corruption accumulation. Dangerous power path.".to_string(),
                            bonus_type: SynergyType::ContributionBoost { multiplier: mult },
                            stage,
                        });
                    }
                }
            }
        }

        bonuses
    }

    // ========================================================================
    // CROSS-RACE (HYBRID) CHAIN SYNERGY (Phase G Extension)
    // Derived from Ra-Thor v15.24
    // Rewards true multi-race identity builds with stronger hybrid bonuses.
    // ========================================================================

    /// Calculates cross-race (hybrid) synergy chains.
    /// Activated when an entity has abilities spanning multiple races + relevant mutations.
    /// These bonuses are stronger than single-race chains and create emergent hybrid playstyles.
    pub fn calculate_cross_race_synergy_chains(
        &self,
        active_mutations: &[MutationType],
        unlocked_races: &[crate::race::Race],
    ) -> Vec<SynergyBonus> {
        let mut bonuses = Vec::new();

        // Hybrid 1: Harmonic Rebirth + Terran abilities → Allied Resonance
        if active_mutations.contains(&MutationType::HarmonicRebirth)
            && unlocked_races.contains(&crate::race::Race::Terran)
            && self.unlocked.iter().any(|a| a.contains("steady") || a.contains("community") || a.contains("fortress"))
        {
            let stage = self.get_chain_stage("allied_resonance_cross");
            let mult = 1.30 + (stage as f32 * 0.15);
            bonuses.push(SynergyBonus {
                name: format!("Allied Resonance Cross-Chain (Stage {})", stage),
                description: "Hybrid Terran-Harmonic resonance. Stronger group harmony, shared epigenetic stability, and cross-race cooperation bonuses.".to_string(),
                bonus_type: SynergyType::HarmonyAmplification { multiplier: mult },
                stage,
            });
        }

        // Hybrid 2: Volatile Surge + Voidfarer abilities → Chaotic Void
        if active_mutations.contains(&MutationType::VolatileSurge)
            && unlocked_races.contains(&crate::race::Race::Voidfarer)
            && self.unlocked.iter().any(|a| a.contains("void") || a.contains("phase") || a.contains("singularity"))
        {
            let stage = self.get_chain_stage("chaotic_void_cross");
            let mult = 1.25 + (stage as f64 * 0.18);
            bonuses.push(SynergyBonus {
                name: format!("Chaotic Void Cross-Chain (Stage {})", stage),
                description: "Hybrid Volatile-Voidfarer path. High-risk contribution spikes with dimensional instability flavor.".to_string(),
                bonus_type: SynergyType::ContributionBoost { multiplier: mult },
                stage,
            });
        }

        // Hybrid 3: Corrupted Echo + Synthetic abilities → Corrupted Tech
        if active_mutations.contains(&MutationType::CorruptedEcho)
            && unlocked_races.contains(&crate::race::Race::Synthetic)
            && self.unlocked.iter().any(|a| a.contains("overclock") || a.contains("systems"))
        {
            let stage = self.get_chain_stage("corrupted_tech_hybrid");
            let mult = 1.22 + (stage as f64 * 0.14);
            bonuses.push(SynergyBonus {
                name: format!("Corrupted Tech Hybrid Chain (Stage {})", stage),
                description: "Hybrid Corrupted-Synthetic path. Innovation gains with managed epigenetic cost and tech-corruption synergy.".to_string(),
                bonus_type: SynergyType::ContributionBoost { multiplier: mult },
                stage,
            });
        }

        bonuses
    }

    // ========================================================================
    // MECHANICAL SYNERGY BONUS APPLICATION + EVENT EMISSION (v1.7)
    // Applies real effects + returns structured events for observability (tracing, UI, client sync).
    // ========================================================================

    /// Applies the mechanical effects of active synergy bonuses directly to the agent's epigenetic profile.
    /// Returns a list of `SynergyEffectEvent` describing exactly what changed — ready for structured emission.
    /// Called every tick from the production evolutionary processing loop.
    pub fn apply_synergy_bonuses_to_profile(
        &self,
        profile: &mut EpigeneticProfile,
        synergies: &[SynergyBonus],
    ) -> Vec<SynergyEffectEvent> {
        let mut events = Vec::new();

        for bonus in synergies {
            let stage_scale: f32 = match bonus.stage {
                0 => 1.0,
                1 => 1.65,
                2 => 2.6,
                _ => 1.0,
            };

            let before_vol = profile.volatility;
            let before_str = profile.strength;
            let before_coop = profile.cooperation_score;

            match &bonus.bonus_type {
                SynergyType::HarmonyAmplification { multiplier } => {
                    let scaled = *multiplier * stage_scale;
                    profile.cooperation_score = (profile.cooperation_score + (scaled * 0.008) as f64).min(1.0);
                    profile.volatility = (profile.volatility - (scaled * 0.015)).max(0.05);
                }
                SynergyType::EpigeneticResilience { reduction } => {
                    let scaled = *reduction * stage_scale;
                    profile.volatility = (profile.volatility - scaled * 0.6).max(0.05);
                    profile.strength = (profile.strength + scaled * 0.4).min(3.5);
                }
                SynergyType::ContributionBoost { multiplier } => {
                    let scaled = *multiplier as f32 * stage_scale;
                    profile.cooperation_score = (profile.cooperation_score + (scaled * 0.006) as f64).min(1.0);
                    if profile.volatility > 1.0 {
                        profile.strength = (profile.strength + (scaled * 0.012)).min(3.5);
                    }
                }
                SynergyType::MovementEfficiency { multiplier: _ } => {}
                SynergyType::GlobalCooldownReduction { reduction_percent: _ } => {}
            }

            // Emit structured event describing the change
            events.push(SynergyEffectEvent {
                chain_name: bonus.name.clone(),
                stage: bonus.stage,
                bonus_type: format!("{:?}", bonus.bonus_type),
                volatility_delta: profile.volatility - before_vol,
                strength_delta: profile.strength - before_str,
                cooperation_delta: profile.cooperation_score - before_coop,
            });
        }

        events
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

/// Structured event emitted when synergy bonuses are applied.
/// Enables rich observability, tracing, future UI sync, and client replication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynergyEffectEvent {
    pub chain_name: String,
    pub stage: u8,
    pub bonus_type: String,
    pub volatility_delta: f32,
    pub strength_delta: f32,
    pub cooperation_delta: f64,
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
/// `stage` enables explicit stage-scaled multipliers in apply_synergy_bonuses_to_profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynergyBonus {
    pub name: String,
    pub description: String,
    pub bonus_type: SynergyType,
    pub stage: u8,
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
    fn test_mutation_synergy_chain_with_stages() {
        let mut tree = AbilityTree::new();
        tree.try_unlock_starter("resonant_jump");
        tree.try_unlock_starter("cosmic_attunement");
        tree.progress_chain_stages("redemption_cascade", 2.1, 15.0, 0.4);
        let bonuses = tree.calculate_mutation_synergy_chains(&[MutationType::HarmonicRebirth]);
        assert!(!bonuses.is_empty());
        let stage = tree.get_chain_stage("redemption_cascade");
        assert!(stage >= 1);
    }

    #[test]
    fn test_cross_race_chain_activation() {
        let mut tree = AbilityTree::new();
        tree.try_unlock_starter("steady_step");
        tree.try_unlock_starter("community_bond");
        let bonuses = tree.calculate_cross_race_synergy_chains(
            &[MutationType::HarmonicRebirth],
            &[crate::race::Race::Terran, crate::race::Race::Harmonic],
        );
        assert!(!bonuses.is_empty());
    }
}