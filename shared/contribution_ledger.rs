// shared/contribution_ledger.rs
// Phase 2 + Finish Pass B — NEVC Contribution Ledger with sample window compaction
//
// AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai
// Thunder locked in. Yoi ⚡

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::nevc_adapter::{
    ContributionClass, NevcConfig, NevcResult, NevcSample, compute_nevc, sample_from_rbe_action,
};

/// Default max samples retained per player (Finish Pass B).
pub const DEFAULT_MAX_SAMPLES: usize = 256;

/// Running contribution state for a single agent / player.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerContribution {
    pub player_id: u64,
    pub samples: Vec<NevcSample>,
    pub last_result: Option<NevcResult>,
}

impl PlayerContribution {
    pub fn new(player_id: u64) -> Self {
        Self {
            player_id,
            samples: Vec::new(),
            last_result: None,
        }
    }

    pub fn class(&self) -> ContributionClass {
        self.last_result
            .as_ref()
            .map(|r| r.class)
            .unwrap_or(ContributionClass::ZombiePartition)
    }

    pub fn is_contributor(&self) -> bool {
        self.class().is_contributor()
    }

    /// Keep only the most recent `max` samples (window compaction).
    pub fn compact(&mut self, max: usize) {
        if max == 0 {
            self.samples.clear();
            return;
        }
        if self.samples.len() > max {
            let skip = self.samples.len() - max;
            self.samples.drain(0..skip);
        }
    }
}

/// In-memory contribution ledger with optional sample window.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContributionLedger {
    players: HashMap<u64, PlayerContribution>,
    config: NevcConfig,
    next_t: u64,
    /// Maximum samples retained per player (0 = unlimited).
    pub max_samples: usize,
}

impl Default for ContributionLedger {
    fn default() -> Self {
        Self::new()
    }
}

impl ContributionLedger {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            config: NevcConfig::default(),
            next_t: 0,
            max_samples: DEFAULT_MAX_SAMPLES,
        }
    }

    pub fn with_config(config: NevcConfig) -> Self {
        Self {
            players: HashMap::new(),
            config,
            next_t: 0,
            max_samples: DEFAULT_MAX_SAMPLES,
        }
    }

    pub fn with_max_samples(mut self, max: usize) -> Self {
        self.max_samples = max;
        self
    }

    /// Record a raw sample for a player and recompute their NEVC result.
    pub fn record_sample(&mut self, player_id: u64, sample: NevcSample) -> NevcResult {
        let max = self.max_samples;
        let entry = self
            .players
            .entry(player_id)
            .or_insert_with(|| PlayerContribution::new(player_id));

        entry.samples.push(sample);
        entry.compact(max);
        let result = compute_nevc(&entry.samples, &self.config);
        entry.last_result = Some(result.clone());
        result
    }

    /// Convenience: record an RBE-style abundance / waste action.
    pub fn record_rbe_action(
        &mut self,
        player_id: u64,
        abundance_alignment: f64,
        waste_or_harm: f64,
    ) -> NevcResult {
        let t = self.next_t;
        self.next_t = self.next_t.saturating_add(1);
        let sample = sample_from_rbe_action(abundance_alignment, waste_or_harm, t);
        self.record_sample(player_id, sample)
    }

    pub fn class_of(&self, player_id: u64) -> ContributionClass {
        self.players
            .get(&player_id)
            .map(|p| p.class())
            .unwrap_or(ContributionClass::ZombiePartition)
    }

    pub fn is_contributor(&self, player_id: u64) -> bool {
        self.class_of(player_id).is_contributor()
    }

    pub fn last_result(&self, player_id: u64) -> Option<&NevcResult> {
        self.players.get(&player_id).and_then(|p| p.last_result.as_ref())
    }

    pub fn sample_count(&self, player_id: u64) -> usize {
        self.players
            .get(&player_id)
            .map(|p| p.samples.len())
            .unwrap_or(0)
    }

    pub fn snapshot(&self) -> Vec<PlayerContribution> {
        self.players.values().cloned().collect()
    }

    /// Compact all players to `max_samples`.
    pub fn compact_all(&mut self) {
        let max = self.max_samples;
        for p in self.players.values_mut() {
            p.compact(max);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pure_abundance_action_makes_contributor() {
        let mut ledger = ContributionLedger::new();
        let r = ledger.record_rbe_action(42, 1.0, 0.0);
        assert!(r.is_contributor());
        assert!(ledger.is_contributor(42));
        assert_eq!(ledger.sample_count(42), 1);
    }

    #[test]
    fn high_waste_action_stays_zombie() {
        let mut ledger = ContributionLedger::new();
        let r = ledger.record_rbe_action(7, 0.0, 3.0);
        assert!(!r.is_contributor());
        assert_eq!(ledger.class_of(7), ContributionClass::ZombiePartition);
    }

    #[test]
    fn unknown_player_defaults_to_zombie() {
        let ledger = ContributionLedger::new();
        assert_eq!(ledger.class_of(999), ContributionClass::ZombiePartition);
        assert!(!ledger.is_contributor(999));
    }

    #[test]
    fn multiple_positive_actions_remain_contributor() {
        let mut ledger = ContributionLedger::new();
        ledger.record_rbe_action(1, 1.0, 0.0);
        ledger.record_rbe_action(1, 0.95, 0.01);
        ledger.record_rbe_action(1, 1.0, 0.0);
        assert!(ledger.is_contributor(1));
        assert_eq!(ledger.sample_count(1), 3);
    }

    #[test]
    fn sample_window_compacts() {
        let mut ledger = ContributionLedger::new().with_max_samples(3);
        for _ in 0..10 {
            ledger.record_rbe_action(5, 1.0, 0.0);
        }
        assert_eq!(ledger.sample_count(5), 3);
        assert!(ledger.is_contributor(5));
    }
}
