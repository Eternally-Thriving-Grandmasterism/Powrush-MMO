// AG-SML v1.0 | Autonomicity Games Sovereign Mercy License
// Powrush-MMO simulation/src/diplomacy.rs
// Phase G Step 2: Player-initiated treaty proposals + pending state (full propose → pending → accept flow)
// Derived cleanly from Ra-Thor powrush-mmo-simulator v15.28
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

/// DiplomacyManager — living cross-race diplomatic layer with proposal system
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiplomacyManager {
    pub relations: HashMap<(Race, Race), DiplomacyRelation>,
    /// Pending treaty proposals waiting for sufficient trust to auto-accept
    pub pending_proposals: HashMap<(Race, Race), Vec<TreatyType>>,
}

impl DiplomacyManager {
    pub fn new() -> Self {
        Self {
            relations: HashMap::new(),
            pending_proposals: HashMap::new(),
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

    /// Player-initiated treaty proposal (Step 2)
    /// Returns true if proposal was recorded (requires baseline trust >= 0.55)
    pub fn propose_treaty(&mut self, r1: Race, r2: Race, treaty: TreatyType) -> bool {
        if r1 == r2 {
            return false;
        }
        let key = if r1 < r2 { (r1, r2) } else { (r2, r1) };
        let current_trust = self.get_trust(r1, r2);

        if current_trust < 0.55 {
            return false; // Not enough trust to even propose
        }

        let proposals = self.pending_proposals.entry(key).or_default();
        if !proposals.contains(&treaty) {
            proposals.push(treaty);
        }
        true
    }

    pub fn has_pending_proposal(&self, r1: Race, r2: Race, treaty: TreatyType) -> bool {
        if r1 == r2 {
            return false;
        }
        let key = if r1 < r2 { (r1, r2) } else { (r2, r1) };
        self.pending_proposals
            .get(&key)
            .map(|list| list.contains(&treaty))
            .unwrap_or(false)
    }

    /// Accept a pending proposal when trust is high enough (>= 0.65)
    /// Moves proposal into active_treaties (expiration foundation left for Step 3)
    pub fn accept_pending_treaty(&mut self, r1: Race, r2: Race, treaty: TreatyType, current_tick: u64) -> bool {
        if r1 == r2 {
            return false;
        }
        let key = if r1 < r2 { (r1, r2) } else { (r2, r1) };

        let trust = self.get_trust(r1, r2);
        if trust < 0.65 {
            return false;
        }

        // Remove from pending
        if let Some(proposals) = self.pending_proposals.get_mut(&key) {
            proposals.retain(|t| *t != treaty);
            if proposals.is_empty() {
                self.pending_proposals.remove(&key);
            }
        }

        // Add to active (simple version without expiration duration yet)
        let entry = self.relations.entry(key).or_default();
        // Avoid duplicate active treaties of same type
        if !entry.active_treaties.iter().any(|t| t.treaty_type == treaty) {
            entry.active_treaties.push(ActiveTreaty {
                treaty_type: treaty,
                expires_at_tick: current_tick + 5000, // placeholder duration
            });
        }

        // Signing bonus
        entry.trust = (entry.trust + 0.05).clamp(0.0, 1.0);
        true
    }

    /// Apply passive diplomacy effects to simulation state (harmony, volatility, strength)
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
    fn test_diplomacy_propose_and_accept() {
        let mut mgr = DiplomacyManager::new();
        mgr.improve_relation(Race::Terran, Race::Harmonic, 0.6);

        assert!(mgr.propose_treaty(Race::Terran, Race::Harmonic, TreatyType::HarmonyAccord));
        assert!(mgr.has_pending_proposal(Race::Terran, Race::Harmonic, TreatyType::HarmonyAccord));

        // Trust is still 0.6, not enough to auto-accept yet
        assert!(!mgr.accept_pending_treaty(Race::Terran, Race::Harmonic, TreatyType::HarmonyAccord, 100));

        mgr.improve_relation(Race::Terran, Race::Harmonic, 0.1); // now 0.7
        assert!(mgr.accept_pending_treaty(Race::Terran, Race::Harmonic, TreatyType::HarmonyAccord, 100));
        assert!(!mgr.has_pending_proposal(Race::Terran, Race::Harmonic, TreatyType::HarmonyAccord));
    }
}
