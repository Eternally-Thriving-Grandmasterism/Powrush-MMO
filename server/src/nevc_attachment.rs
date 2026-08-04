//! server/src/nevc_attachment.rs
//! Finish Pass A — NEVC live attachment using shared crate (no algorithm mirror)
//!
//! Harvest and other server systems call into `shared::nevc_game_loop` and
//! `shared::nevc_persistence` for a single source of truth.
//!
//! AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai
//! Thunder locked in. Yoi ⚡

use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

use shared::nevc_adapter::ContributionClass;
use shared::nevc_game_loop::{apply_harvest_class, apply_harvest_to_ledger, HarvestNevcInput};
use shared::nevc_persistence::{NevcPersistenceStore, NevcPlayerRecord};
use shared::contribution_ledger::ContributionLedger;
use shared::nevc_adapter::NevcResult;

/// Process-level server NEVC state: live ledger + durable store.
struct ServerNevcState {
    ledger: ContributionLedger,
    store: NevcPersistenceStore,
}

impl ServerNevcState {
    fn new() -> Self {
        let path = default_persist_path();
        let store = NevcPersistenceStore::load_from_file(&path).unwrap_or_default();
        Self {
            ledger: ContributionLedger::new(),
            store,
        }
    }

    fn persist(&self) {
        let _ = self.store.save_to_file(&default_persist_path());
    }
}

fn default_persist_path() -> PathBuf {
    PathBuf::from("data/nevc_players.json")
}

fn global_state() -> &'static Mutex<ServerNevcState> {
    static STATE: OnceLock<Mutex<ServerNevcState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(ServerNevcState::new()))
}

/// Public hook for harvest handlers — uses shared game-loop mapping.
pub fn on_harvest(
    player_id: u64,
    success: bool,
    was_sustainable: bool,
    regen_participation: bool,
) -> ContributionClass {
    let input = HarvestNevcInput::from_harvest(player_id, success, was_sustainable, regen_participation);
    if let Ok(mut state) = global_state().lock() {
        let result = apply_harvest_to_ledger(&mut state.ledger, &input);
        // Mirror running score into durable store (session continuity)
        state.store.absorb(player_id, result.score);
        state.persist();
        result.class
    } else {
        // Lock poisoned: still compute class without durable write
        let mut tmp = ContributionLedger::new();
        apply_harvest_class(&mut tmp, &input)
    }
}

/// Query current contribution class for a player.
pub fn player_contribution_class(player_id: u64) -> ContributionClass {
    if let Ok(state) = global_state().lock() {
        // Prefer live ledger; fall back to durable store
        if state.ledger.sample_count(player_id) > 0 {
            return state.ledger.class_of(player_id);
        }
        return state.store.class_of(player_id);
    }
    ContributionClass::ZombiePartition
}

pub fn is_contributor(player_id: u64) -> bool {
    player_contribution_class(player_id).is_contributor()
}

/// Explicit save (shutdown / periodic tick).
pub fn persist_now() -> Result<(), String> {
    let state = global_state().lock().map_err(|e| e.to_string())?;
    state.store.save_to_file(&default_persist_path())
}

/// Reload durable store from disk into memory.
pub fn reload_from_disk() {
    if let Ok(mut state) = global_state().lock() {
        if let Ok(loaded) = NevcPersistenceStore::load_from_file(&default_persist_path()) {
            state.store = loaded;
        }
    }
}

/// Snapshot durable records (for embedding into world state / DB later).
pub fn snapshot_records() -> Vec<NevcPlayerRecord> {
    global_state()
        .lock()
        .map(|s| s.store.records.values().cloned().collect())
        .unwrap_or_default()
}

/// Record a raw NEVC result score into durable store (advanced callers).
pub fn absorb_score(player_id: u64, score: f64) -> ContributionClass {
    if let Ok(mut state) = global_state().lock() {
        let rec = state.store.absorb(player_id, score);
        state.persist();
        rec.class
    } else {
        ContributionClass::from_score(score)
    }
}

/// Thread-safe handle for injection into larger server structs if needed.
pub type ServerNevcHandle = Arc<Mutex<ServerNevcState>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sustainable_harvest_is_contributor() {
        // Use a high player id to avoid colliding with other tests in same process
        let pid = 9_001_u64;
        let c = on_harvest(pid, true, true, true);
        assert!(c.is_contributor(), "expected contributor, got {c:?}");
        assert!(is_contributor(pid));
    }

    #[test]
    fn unsustainable_harvest_zombie() {
        let pid = 9_002_u64;
        let c = on_harvest(pid, true, false, false);
        assert!(!c.is_contributor());
        assert_eq!(c, ContributionClass::ZombiePartition);
    }
}
