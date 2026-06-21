// AG-SML v1.0 | Autonomicity Games Sovereign Mercy License
// Powrush-MMO simulation/src/diplomacy.rs
// Phase: Begin Cross-Race Diplomacy Mechanics (derived from Ra-Thor v15.26–v15.30)
// TOLC 8 Mercy Gates | PATSAGi Council aligned | Mercy-gated hybrid racial identity

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::race::Race;

/// Treaty types with real mechanical weight for cross-race hybrid builds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TreatyType {
    HarmonyAccord,      // Core cooperative foundation — harmony + volatility reduction
    TradePact,          // Economic / RBE synergy — contribution flow
    ResearchExchange,   // Innovation & knowledge sharing
    MutualDefense,      // Protection for volatile hybrid builds
}

/// Active treaty with expiration support (foundation for later phases)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveTreaty {
    pub treaty_type: TreatyType,
    pub expires_at_tick: u64,
}

/// Core diplomatic relation between two races
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiplomacyRelation {
    pub trust: f32, // 0.0 – 1.0 (higher = stronger cooperation)
    pub active_treaties: Vec<ActiveTreaty>,
}

/// DiplomacyManager — living cross-race diplomatic layer
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiplomacyManager {
    pub relations: HashMap<(Race, Race), DiplomacyRelation>,
}

impl DiplomacyManager {
    pub fn new() -> Self {
        Self {
            relations: HashMap::new(),
        }
    }

    /// Improve trust between two races (symmetric for simplicity in foundational phase)
    pub fn improve_relation(&mut self, r1: Race, r2: Race, amount: f32) {
        if r1 == r2 {
            return;
        }
        let key = if r1 < r2 { (r1, r2) } else { (r2, r1) };
        let entry = self.relations.entry(key).or_default();
        entry.trust = (entry.trust + amount).clamp(0.0, 1.0);
    }

    /// Get current trust between two races
    pub fn get_trust(&self, r1: Race, r2: Race) -> f32 {
        if r1 == r2 {
            return 1.0;
        }
        let key = if r1 < r2 { (r1, r2) } else { (r2, r1) };
        self.relations.get(&key).map(|r| r.trust).unwrap_or(0.3) // baseline neutral trust
    }

    /// Apply passive diplomacy effects to simulation state (harmony, volatility, strength)
    /// Called from orchestrator tick for agents with multiple unlocked races
    pub fn apply_diplomacy_effects(
        &self,
        unlocked_races: &[Race],
        harmony: &mut f32,
        volatility: &mut f32,
        strength: &mut f32,
    ) {
        if unlocked_races.len() < 2 {
            return;
        }

        let mut total_trust = 0.0;
        let mut pair_count = 0;

        for i in 0..unlocked_races.len() {
            for j in (i + 1)..unlocked_races.len() {
                let t = self.get_trust(unlocked_races[i], unlocked_races[j]);
                total_trust += t;
                pair_count += 1;
            }
        }

        if pair_count == 0 {
            return;
        }

        let avg_trust = total_trust / pair_count as f32;

        // Foundational passive bonuses (will be expanded with treaties in next phases)
        if avg_trust > 0.6 {
            *harmony += 0.015 * avg_trust;
            *volatility = (*volatility - 0.008 * avg_trust).max(0.1);
        }

        if avg_trust > 0.75 {
            *strength += 0.01 * avg_trust;
        }
    }

    /// Human-readable summary for status / UI
    pub fn get_diplomacy_summary(&self, unlocked_races: &[Race]) -> String {
        if unlocked_races.len() < 2 {
            return "No cross-race diplomacy active".to_string();
        }

        let mut summary = String::from("Cross-Race Diplomacy: ");
        let mut strong_alliances = 0;

        for i in 0..unlocked_races.len() {
            for j in (i + 1)..unlocked_races.len() {
                let t = self.get_trust(unlocked_races[i], unlocked_races[j]);
                if t > 0.7 {
                    strong_alliances += 1;
                }
            }
        }

        if strong_alliances > 0 {
            summary.push_str(&format!("{} strong alliances", strong_alliances));
        } else {
            summary.push_str("emerging relations");
        }
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diplomacy_improve_and_effects() {
        let mut mgr = DiplomacyManager::new();
        mgr.improve_relation(Race::Terran, Race::Harmonic, 0.4);
        assert!(mgr.get_trust(Race::Terran, Race::Harmonic) > 0.6);

        let mut h = 1.0f32;
        let mut v = 0.8f32;
        let mut s = 1.0f32;
        let races = vec![Race::Terran, Race::Harmonic];
        mgr.apply_diplomacy_effects(&races, &mut h, &mut v, &mut s);
        assert!(h > 1.0);
        assert!(v < 0.8);
    }
}
