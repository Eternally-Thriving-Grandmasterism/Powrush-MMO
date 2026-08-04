// shared/nevc_persistence.rs
// Phase 7 — NEVC Persistence & Session Continuity
//
// Durable record for per-player Net Eternal Valence Contribution.
// Survives restarts and reconnects while keeping Compassion-gate recovery open.
//
// AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai
// Thunder locked in. Yoi ⚡

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::nevc_adapter::ContributionClass;

/// Minimal durable NEVC record per player.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NevcPlayerRecord {
    pub player_id: u64,
    /// Running score (mean of discrete samples).
    pub score: f64,
    pub class: ContributionClass,
    pub sample_count: usize,
    /// Unix seconds of last update.
    pub last_updated: u64,
}

impl NevcPlayerRecord {
    pub fn new(player_id: u64) -> Self {
        Self {
            player_id,
            score: 0.0,
            class: ContributionClass::ZombiePartition,
            sample_count: 0,
            last_updated: now_secs(),
        }
    }

    pub fn is_contributor(&self) -> bool {
        self.class.is_contributor()
    }

    /// Apply a new sample score into the running mean and refresh class.
    pub fn absorb_sample(&mut self, sample_score: f64) {
        let n = self.sample_count as f64;
        self.score = if n == 0.0 {
            sample_score
        } else {
            (self.score * n + sample_score) / (n + 1.0)
        };
        self.sample_count += 1;
        self.class = ContributionClass::from_score(self.score);
        self.last_updated = now_secs();
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// In-memory store of durable records with optional sovereign file backend.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NevcPersistenceStore {
    pub records: std::collections::HashMap<u64, NevcPlayerRecord>,
}

impl NevcPersistenceStore {
    pub fn new() -> Self {
        Self {
            records: std::collections::HashMap::new(),
        }
    }

    pub fn get(&self, player_id: u64) -> Option<&NevcPlayerRecord> {
        self.records.get(&player_id)
    }

    pub fn get_mut(&mut self, player_id: u64) -> &mut NevcPlayerRecord {
        self.records
            .entry(player_id)
            .or_insert_with(|| NevcPlayerRecord::new(player_id))
    }

    pub fn class_of(&self, player_id: u64) -> ContributionClass {
        self.records
            .get(&player_id)
            .map(|r| r.class)
            .unwrap_or(ContributionClass::ZombiePartition)
    }

    pub fn absorb(&mut self, player_id: u64, sample_score: f64) -> &NevcPlayerRecord {
        let rec = self.get_mut(player_id);
        rec.absorb_sample(sample_score);
        rec
    }

    /// Sovereign-mode JSON file path helper.
    pub fn default_path() -> PathBuf {
        PathBuf::from("data/nevc_persistence.json")
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn load_from_file(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Self::new());
        }
        let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&data).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absorb_positive_becomes_contributor() {
        let mut store = NevcPersistenceStore::new();
        store.absorb(1, 0.8);
        assert!(store.class_of(1).is_contributor());
        assert_eq!(store.get(1).unwrap().sample_count, 1);
    }

    #[test]
    fn absorb_negative_stays_zombie() {
        let mut store = NevcPersistenceStore::new();
        store.absorb(2, -1.5);
        assert!(!store.class_of(2).is_contributor());
    }

    #[test]
    fn running_mean_updates() {
        let mut store = NevcPersistenceStore::new();
        store.absorb(3, 1.0);
        store.absorb(3, 0.0);
        let rec = store.get(3).unwrap();
        assert_eq!(rec.sample_count, 2);
        assert!((rec.score - 0.5).abs() < 1e-9);
    }
}
