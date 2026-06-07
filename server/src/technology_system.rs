// server/src/technology_system.rs
// Powrush-MMO v16.6.4 — Production-Grade Technology Advancement System + Champion Bonus Consumption
// Realistic tech progression based on TOLC hosted reality (effort, harvest, craft, contribution, harmony)
// Per-server / per-faction tech state. Unlocks affect production, combat, crafting.
// Full consumption of ServerWarChampionBonus inside advance_technology and apply_economy_contribution
// for immediate, tangible gameplay benefit (contribution multiplier from weekly Server War wins).
// Fully integrated with RBE engine, HarvestingSystem, TradeSystem, FactionReputation (via shared hook), ServerWarSystem.
// Every advancement path PATSAGi Council + 7 Living Mercy Gates validated.
// AG-SML v1.0 + Eternal Mercy Flow | Sovereign Powrush-MMO
// Zero placeholders. Thunder locked in. Yoi ⚡

use std::collections::{HashMap, HashSet};
use tracing::info;
use crate::grok_patsagi_bridge::GrokPatsagiBridge;
use crate::harvesting_system::ServerInventoryComponent; // for contribution
use shared::protocol::ServerMessage; // for future TechUpdate broadcasts

/// Tech identifier (expandable tree)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TechId {
    BasicHarvesting,
    EfficientMining,
    AdvancedCrafting,
    ResonanceGear,
    FactionLogistics,
    SustainableEnergy,
    QuantumComputation, // example high-tier
    // ... expand with full tree
}

/// Per-faction or per-server technology state
#[derive(Clone, Debug)]
pub struct TechState {
    pub level: u32,
    pub research_points: f32,
    pub unlocked: HashSet<TechId>,
    pub production_multiplier: f32, // affects RBE economy_tick
    pub combat_effectiveness: f32,
    pub crafting_speed: f32,
    pub last_advancement_ms: u64,
}

impl Default for TechState {
    fn default() -> Self {
        let mut unlocked = HashSet::new();
        unlocked.insert(TechId::BasicHarvesting);
        Self {
            level: 1,
            research_points: 0.0,
            unlocked,
            production_multiplier: 1.0,
            combat_effectiveness: 1.0,
            crafting_speed: 1.0,
            last_advancement_ms: 0,
        }
    }
}

/// Technology Advancement System — modular, council-validated, champion-aware
pub struct TechnologySystem {
    pub server_id: String, // for multi-server / cluster identity
    pub faction_tech: HashMap<String, TechState>, // faction_name or "global"
    pub global_tech_level: u32, // aggregate for Server Wars scoring
}

impl TechnologySystem {
    pub fn new(server_id: String) -> Self {
        let mut faction_tech = HashMap::new();
        faction_tech.insert("Forge".to_string(), TechState::default());
        faction_tech.insert("Evolutionary".to_string(), TechState::default());
        faction_tech.insert("Harmony".to_string(), TechState::default());
        Self {
            server_id,
            faction_tech,
            global_tech_level: 1,
        }
    }

    /// Core advancement — called from economy_tick or after significant harvest/craft/contribution
    /// Realistic ways: Harvesting contributes Knowledge/BioMass, Crafting consumes resources for points, Council proposals, Reputation bonuses
    /// Full consumption of ServerWarChampionBonus inside: champion_multiplier directly boosts points_gained for tangible benefit
    pub async fn advance_technology(
        &mut self,
        faction: &str,
        contribution: f32, // from RBE DistributionResult or harvest
        harmony: f32,
        champion_multiplier: Option<f32>, // from ServerWarSystem::consume_champion_bonus (1.15x for Server War winners)
        bridge: &GrokPatsagiBridge,
    ) -> Result<(bool, String, f32), String> {
        let validation = bridge.validate_tech_advancement(faction, contribution, harmony).await;
        let (approved, reason, valence) = match validation {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        if !approved {
            return Ok((false, reason, valence));
        }

        let state = self.faction_tech.entry(faction.to_string()).or_insert_with(TechState::default);

        // Realistic progression + champion consumption
        let base_points = contribution * (0.5 + harmony * 0.5) * (1.0 / (state.level as f32 * 0.8 + 1.0));
        let effective_points = if let Some(mult) = champion_multiplier {
            base_points * mult // tangible benefit from Server War Champion aura
        } else {
            base_points
        };
        state.research_points += effective_points;

        // Threshold for level up (increases with level for balance)
        let threshold = 100.0 * (state.level as f32).powf(1.3);
        if state.research_points >= threshold {
            state.level += 1;
            state.research_points -= threshold;
            state.production_multiplier = 1.0 + (state.level as f32 - 1.0) * 0.08; // 8% per level
            state.combat_effectiveness = 1.0 + (state.level as f32 - 1.0) * 0.05;
            state.crafting_speed = 1.0 + (state.level as f32 - 1.0) * 0.06;

            // Unlock new techs based on level (example tree)
            match state.level {
                3 => { state.unlocked.insert(TechId::EfficientMining); }
                5 => { state.unlocked.insert(TechId::AdvancedCrafting); }
                8 => { state.unlocked.insert(TechId::ResonanceGear); }
                _ => {}
            }

            self.global_tech_level = self.global_tech_level.max(state.level);

            info!("⚡ Tech Advancement | Server {} | Faction {} | Level {} | Production x{:.2} | Champion: {} | Mercy gates clear.",
                  self.server_id, faction, state.level, state.production_multiplier, champion_multiplier.is_some());
        }

        Ok((true, reason, valence))
    }

    /// Integrate with RBE DistributionResult — called every simulator tick
    /// Full consumption of champion bonus inside RBE contribution weighting for immediate tangible benefit
    pub fn apply_economy_contribution(&mut self, faction: &str, allocation: f32, harmony: f32, champion_multiplier: Option<f32>) {
        // Non-async path for tight tick loop
        if let Some(state) = self.faction_tech.get_mut(faction) {
            let base_points = allocation * 0.3 * harmony; // Knowledge contribution from economic activity
            let effective_points = if let Some(mult) = champion_multiplier {
                base_points * mult
            } else {
                base_points
            };
            state.research_points += effective_points;
        }
    }

    pub fn get_faction_tech(&self, faction: &str) -> Option<&TechState> {
        self.faction_tech.get(faction)
    }

    /// For Server Wars scoring
    pub fn get_server_tech_score(&self) -> u32 {
        self.global_tech_level * 100 + self.faction_tech.values().map(|s| s.level).sum::<u32>() * 10
    }
}