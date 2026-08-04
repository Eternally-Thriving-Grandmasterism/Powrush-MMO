//! server/nevc_attachment.rs
//! Phase 6 + Phase 7 — Live Game-Loop NEVC Attachment with Persistence
//!
//! Holds the contribution ledger for the authoritative server, records harvest
//! outcomes, and persists per-player NEVC state across sessions.
//!
//! AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai
//! Thunder locked in. Yoi ⚡

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Binary partition (mirrors shared/nevc_adapter).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContributionClass {
    ActiveEternalContributor,
    ZombiePartition,
}

impl ContributionClass {
    pub fn from_score(score: f64) -> Self {
        if score > 0.0 {
            ContributionClass::ActiveEternalContributor
        } else {
            ContributionClass::ZombiePartition
        }
    }

    pub fn is_contributor(self) -> bool {
        matches!(self, ContributionClass::ActiveEternalContributor)
    }
}

impl Default for ContributionClass {
    fn default() -> Self {
        ContributionClass::ZombiePartition
    }
}

/// Durable per-player NEVC record (Phase 7).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NevcPlayerRecord {
    pub player_id: u64,
    pub score: f64,
    pub class: ContributionClass,
    pub sample_count: usize,
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

    fn absorb(&mut self, sample_score: f64) {
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

/// Server-authoritative NEVC ledger with durable records.
#[derive(Clone, Default)]
pub struct ServerNevcLedger {
    inner: Arc<Mutex<HashMap<u64, NevcPlayerRecord>>>,
}

impl ServerNevcLedger {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Load durable state from a sovereign JSON file (if present).
    pub fn load_from_file(path: &Path) -> Self {
        let ledger = Self::new();
        if path.exists() {
            if let Ok(data) = fs::read_to_string(path) {
                if let Ok(map) = serde_json::from_str::<HashMap<u64, NevcPlayerRecord>>(&data) {
                    if let Ok(mut guard) = ledger.inner.lock() {
                        *guard = map;
                    }
                }
            }
        }
        ledger
    }

    /// Persist all records to a sovereign JSON file.
    pub fn save_to_file(&self, path: &Path) -> Result<(), String> {
        let guard = self.inner.lock().map_err(|e| e.to_string())?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let json = serde_json::to_string_pretty(&*guard).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn default_path() -> PathBuf {
        PathBuf::from("data/nevc_players.json")
    }

    /// Record a harvest outcome and return the resulting class.
    pub fn record_harvest(
        &self,
        player_id: u64,
        success: bool,
        was_sustainable: bool,
        regen_participation: bool,
    ) -> ContributionClass {
        let (alignment, waste) = if !success {
            (0.2, 0.5)
        } else if was_sustainable && regen_participation {
            (1.0, 0.0)
        } else if was_sustainable {
            (0.85, 0.05)
        } else {
            (0.1, 1.5)
        };

        let valence = (0.999999 + 0.000001 * alignment).min(1.0);
        let positive = if valence >= 0.999999 {
            let proximity = (valence - 0.999999) / (1.0 - 0.999999).max(1e-12);
            proximity
        } else {
            0.0
        };
        let sample_score = positive - waste;

        if let Ok(mut map) = self.inner.lock() {
            let entry = map
                .entry(player_id)
                .or_insert_with(|| NevcPlayerRecord::new(player_id));
            entry.absorb(sample_score);
            entry.class
        } else {
            ContributionClass::from_score(sample_score)
        }
    }

    pub fn class_of(&self, player_id: u64) -> ContributionClass {
        self.inner
            .lock()
            .ok()
            .and_then(|m| m.get(&player_id).map(|s| s.class))
            .unwrap_or(ContributionClass::ZombiePartition)
    }

    pub fn is_contributor(&self, player_id: u64) -> bool {
        self.class_of(player_id).is_contributor()
    }

    pub fn sample_count(&self, player_id: u64) -> usize {
        self.inner
            .lock()
            .ok()
            .and_then(|m| m.get(&player_id).map(|s| s.sample_count))
            .unwrap_or(0)
    }

    pub fn record_of(&self, player_id: u64) -> Option<NevcPlayerRecord> {
        self.inner
            .lock()
            .ok()
            .and_then(|m| m.get(&player_id).cloned())
    }

    /// Snapshot all records (for embedding into WorldState / DB).
    pub fn snapshot(&self) -> Vec<NevcPlayerRecord> {
        self.inner
            .lock()
            .map(|m| m.values().cloned().collect())
            .unwrap_or_default()
    }

    /// Restore from a list of durable records (e.g. after load_world_state).
    pub fn restore(&self, records: &[NevcPlayerRecord]) {
        if let Ok(mut map) = self.inner.lock() {
            for r in records {
                map.insert(r.player_id, r.clone());
            }
        }
    }
}

/// Global server ledger handle (lazy init + optional auto-load from sovereign file).
static SERVER_NEVC: once_cell_stub::LazyLedger = once_cell_stub::LazyLedger::new();

mod once_cell_stub {
    use super::ServerNevcLedger;
    use std::sync::OnceLock;

    pub struct LazyLedger {
        cell: OnceLock<ServerNevcLedger>,
    }

    impl LazyLedger {
        pub const fn new() -> Self {
            Self {
                cell: OnceLock::new(),
            }
        }

        pub fn get(&self) -> &ServerNevcLedger {
            self.cell.get_or_init(|| {
                let path = ServerNevcLedger::default_path();
                ServerNevcLedger::load_from_file(&path)
            })
        }
    }
}

/// Public hook for harvest handlers.
pub fn on_harvest(
    player_id: u64,
    success: bool,
    was_sustainable: bool,
    regen_participation: bool,
) -> ContributionClass {
    let class = SERVER_NEVC
        .get()
        .record_harvest(player_id, success, was_sustainable, regen_participation);
    // Best-effort sovereign persist after significant events
    let _ = SERVER_NEVC.get().save_to_file(&ServerNevcLedger::default_path());
    class
}

/// Query current class for a player (for progression / visibility systems).
pub fn player_contribution_class(player_id: u64) -> ContributionClass {
    SERVER_NEVC.get().class_of(player_id)
}

/// Explicit save (e.g. on shutdown or periodic tick).
pub fn persist_now() -> Result<(), String> {
    SERVER_NEVC
        .get()
        .save_to_file(&ServerNevcLedger::default_path())
}

/// Explicit reload from sovereign file.
pub fn reload_from_disk() {
    let path = ServerNevcLedger::default_path();
    let loaded = ServerNevcLedger::load_from_file(&path);
    let records = loaded.snapshot();
    SERVER_NEVC.get().restore(&records);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sustainable_harvest_contributor() {
        let ledger = ServerNevcLedger::new();
        let c = ledger.record_harvest(42, true, true, true);
        assert!(c.is_contributor());
        assert!(ledger.is_contributor(42));
        assert!(ledger.sample_count(42) >= 1);
    }

    #[test]
    fn unsustainable_harvest_zombie() {
        let ledger = ServerNevcLedger::new();
        let c = ledger.record_harvest(43, true, false, false);
        assert!(!c.is_contributor());
        assert_eq!(c, ContributionClass::ZombiePartition);
    }

    #[test]
    fn persistence_roundtrip_in_memory() {
        let ledger = ServerNevcLedger::new();
        ledger.record_harvest(99, true, true, true);
        let snap = ledger.snapshot();
        let ledger2 = ServerNevcLedger::new();
        ledger2.restore(&snap);
        assert!(ledger2.is_contributor(99));
        assert_eq!(ledger2.sample_count(99), 1);
    }
}
