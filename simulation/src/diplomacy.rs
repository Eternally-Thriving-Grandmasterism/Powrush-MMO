// AG-SML v1.0 | Autonomicity Games Sovereign Mercy License
// Powrush-MMO simulation/src/diplomacy.rs
// Phase G Step 3: Treaty expiration + renewal mechanics
// Derived cleanly from Ra-Thor powrush-mmo-simulator v15.29–v15.30
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

/// Active treaty with expiration support
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

/// DiplomacyManager — living cross-race diplomatic layer with full proposal + expiration + renewal
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiplomacyManager {
    pub relations: HashMap<(Race, Race), DiplomacyRelation>,
    pub pending_proposals: HashMap<(Race, Race), Vec<TreatyType>>,
}

impl DiplomacyManager {
    pub fn new() -> Self {
        Self {
            relations: HashMap::new(),
            pending_proposals: HashMap::new(),
        }
    }

    pub fn improve_relation(&mut self, r1: Race, r2: Race, amount: f32) {
        if r1 == r2 { return; }
        let key = if r1 < r2 { (r1, r2) } else { (r2, r1) };
        let entry = self.relations.entry(key).or_default();
        entry.trust = (entry.trust + amount).clamp(0.0, 1.0);
    }

    pub fn get_trust(&self, r1: Race, r2: Race) -> f32 {
        if r1 == r2 { return 1.0; }
        let key = if r1 < r2 { (r1, r2) } else { (r2, r1) };
        self.relations.get(&key).map(|r| r.trust).unwrap_or(0.3)
    }

    /// Returns true if the pair currently has an active (non-expired) treaty of this type
    pub fn has_active_treaty(&self, r1: Race, r2: Race, treaty: TreatyType, current_tick: u64) -> bool {
        if r1 == r2 { return false; }
        let key = if r1 < r2 { (r1, r2) } else { (r2, r1) };
        if let Some(rel) = self.relations.get(&key) {
            rel.active_treaties.iter().any(|t| {
                t.treaty_type == treaty && t.expires_at_tick > current_tick
            })
        } else {
            false
        }
    }

    pub fn propose_treaty(&mut self, r1: Race, r2: Race, treaty: TreatyType) -> bool {
        if r1 == r2 { return false; }
        let key = if r1 < r2 { (r1, r2) } else { (r2, r1) };
        if self.get_trust(r1, r2) < 0.55 { return false; }
        let proposals = self.pending_proposals.entry(key).or_default();
        if !proposals.contains(&treaty) { proposals.push(treaty); }
        true
    }

    pub fn has_pending_proposal(&self, r1: Race, r2: Race, treaty: TreatyType) -> bool {
        if r1 == r2 { return false; }
        let key = if r1 < r2 { (r1, r2) } else { (r2, r1) };
        self.pending_proposals.get(&key).map(|list| list.contains(&treaty)).unwrap_or(false)
    }

    pub fn accept_pending_treaty(&mut self, r1: Race, r2: Race, treaty: TreatyType, current_tick: u64) -> bool {
        if r1 == r2 { return false; }
        let key = if r1 < r2 { (r1, r2) } else { (r2, r1) };
        if self.get_trust(r1, r2) < 0.65 { return false; }

        if let Some(proposals) = self.pending_proposals.get_mut(&key) {
            proposals.retain(|t| *t != treaty);
            if proposals.is_empty() { self.pending_proposals.remove(&key); }
        }

        let entry = self.relations.entry(key).or_default();
        if !entry.active_treaties.iter().any(|t| t.treaty_type == treaty) {
            entry.active_treaties.push(ActiveTreaty {
                treaty_type: treaty,
                expires_at_tick: current_tick + Self::default_duration_for_treaty(treaty),
            });
        }
        entry.trust = (entry.trust + 0.05).clamp(0.0, 1.0);
        true
    }

    fn default_duration_for_treaty(treaty: TreatyType) -> u64 {
        match treaty {
            TreatyType::HarmonyAccord => 12000,
            TreatyType::TradePact => 8000,
            TreatyType::ResearchExchange => 6000,
            TreatyType::MutualDefense => 4000,
        }
    }

    /// Cleanup expired treaties and apply small trust penalty on lapse
    pub fn cleanup_expired_treaties(&mut self, current_tick: u64) {
        for rel in self.relations.values_mut() {
            let before = rel.active_treaties.len();
            rel.active_treaties.retain(|t| t.expires_at_tick > current_tick);
            if rel.active_treaties.len() < before {
                // Small trust penalty for letting a treaty lapse
                rel.trust = (rel.trust - 0.03).max(0.0);
            }
        }
    }

    /// Player-initiated renewal of an active treaty
    pub fn renew_treaty(&mut self, r1: Race, r2: Race, treaty: TreatyType, current_tick: u64) -> bool {
        if r1 == r2 { return false; }
        let key = if r1 < r2 { (r1, r2) } else { (r2, r1) };

        if self.get_trust(r1, r2) < 0.60 { return false; }

        if let Some(rel) = self.relations.get_mut(&key) {
            // Remove old version if exists
            rel.active_treaties.retain(|t| t.treaty_type != treaty);
            rel.active_treaties.push(ActiveTreaty {
                treaty_type: treaty,
                expires_at_tick: current_tick + Self::default_duration_for_treaty(treaty),
            });
            rel.trust = (rel.trust + 0.04).clamp(0.0, 1.0);
            return true;
        }
        false
    }

    pub fn apply_diplomacy_effects(
        &self,
        unlocked_races: &[Race],
        harmony: &mut f32,
        volatility: &mut f32,
        strength: &mut f32,
    ) {
        if unlocked_races.len() < 2 { return; }
        let mut total_trust = 0.0;
        let mut pair_count = 0;
        for i in 0..unlocked_races.len() {
            for j in (i + 1)..unlocked_races.len() {
                let t = self.get_trust(unlocked_races[i], unlocked_races[j]);
                total_trust += t;
                pair_count += 1;
            }
        }
        if pair_count == 0 { return; }
        let avg_trust = total_trust / pair_count as f32;

        if avg_trust > 0.6 {
            *harmony += 0.015 * avg_trust;
            *volatility = (*volatility - 0.008 * avg_trust).max(0.1);
        }
        if avg_trust > 0.75 {
            *strength += 0.01 * avg_trust;
        }
    }

    pub fn get_diplomacy_summary(&self, unlocked_races: &[Race]) -> String {
        if unlocked_races.len() < 2 {
            return "No cross-race diplomacy active".to_string();
        }
        let mut summary = String::from("Cross-Race Diplomacy: ");
        let mut strong_alliances = 0;
        for i in 0..unlocked_races.len() {
            for j in (i + 1)..unlocked_races.len() {
                let t = self.get_trust(unlocked_races[i], unlocked_races[j]);
                if t > 0.7 { strong_alliances += 1; }
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
    fn test_full_diplomacy_lifecycle() {
        let mut mgr = DiplomacyManager::new();
        mgr.improve_relation(Race::Terran, Race::Harmonic, 0.7);

        assert!(mgr.propose_treaty(Race::Terran, Race::Harmonic, TreatyType::HarmonyAccord));
        assert!(mgr.accept_pending_treaty(Race::Terran, Race::Harmonic, TreatyType::HarmonyAccord, 100));

        assert!(mgr.has_active_treaty(Race::Terran, Race::Harmonic, TreatyType::HarmonyAccord, 100));
        assert!(!mgr.has_active_treaty(Race::Terran, Race::Harmonic, TreatyType::HarmonyAccord, 20000)); // expired

        // Renewal
        assert!(mgr.renew_treaty(Race::Terran, Race::Harmonic, TreatyType::HarmonyAccord, 101));
        assert!(mgr.has_active_treaty(Race::Terran, Race::Harmonic, TreatyType::HarmonyAccord, 5000));
    }
}
