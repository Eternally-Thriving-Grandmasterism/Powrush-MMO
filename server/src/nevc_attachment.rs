//! server/src/nevc_attachment.rs
//! Finish Pass A + B — NEVC attachment via shared + durability hooks
//!
//! AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai
//! Thunder locked in. Yoi ⚡

use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use shared::contribution_ledger::ContributionLedger;
use shared::nevc_adapter::ContributionClass;
use shared::nevc_game_loop::{apply_harvest_class, apply_harvest_to_ledger, HarvestNevcInput};
use shared::nevc_persistence::{NevcPersistenceStore, NevcPlayerRecord};

/// Minimum interval between opportunistic disk flushes during ticks.
const TICK_PERSIST_INTERVAL: Duration = Duration::from_secs(30);

struct ServerNevcState {
    ledger: ContributionLedger,
    store: NevcPersistenceStore,
    last_persist: Instant,
    dirty: bool,
}

impl ServerNevcState {
    fn new() -> Self {
        let path = default_persist_path();
        let store = NevcPersistenceStore::load_from_file(&path).unwrap_or_default();
        Self {
            ledger: ContributionLedger::new(),
            store,
            last_persist: Instant::now(),
            dirty: false,
        }
    }

    fn persist_force(&mut self) -> Result<(), String> {
        self.store.save_to_file(&default_persist_path())?;
        self.last_persist = Instant::now();
        self.dirty = false;
        Ok(())
    }

    fn persist_if_due(&mut self) {
        if self.dirty && self.last_persist.elapsed() >= TICK_PERSIST_INTERVAL {
            let _ = self.persist_force();
        }
    }
}

fn default_persist_path() -> PathBuf {
    PathBuf::from("data/nevc_players.json")
}

fn global_state() -> &'static Mutex<ServerNevcState> {
    static STATE: OnceLock<Mutex<ServerNevcState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(ServerNevcState::new()))
}

/// Public hook for harvest handlers.
pub fn on_harvest(
    player_id: u64,
    success: bool,
    was_sustainable: bool,
    regen_participation: bool,
) -> ContributionClass {
    let input =
        HarvestNevcInput::from_harvest(player_id, success, was_sustainable, regen_participation);
    if let Ok(mut state) = global_state().lock() {
        let result = apply_harvest_to_ledger(&mut state.ledger, &input);
        state.store.absorb(player_id, result.score);
        state.dirty = true;
        // Best-effort immediate persist after significant events (Pass B still allows tick flush)
        let _ = state.persist_force();
        result.class
    } else {
        let mut tmp = ContributionLedger::new();
        apply_harvest_class(&mut tmp, &input)
    }
}

pub fn player_contribution_class(player_id: u64) -> ContributionClass {
    if let Ok(state) = global_state().lock() {
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

/// Force durable flush (shutdown / explicit save).
pub fn persist_now() -> Result<(), String> {
    let mut state = global_state().lock().map_err(|e| e.to_string())?;
    state.persist_force()
}

/// Call from server main loop / periodic tick (throttled).
pub fn tick_persist() {
    if let Ok(mut state) = global_state().lock() {
        state.persist_if_due();
    }
}

pub fn reload_from_disk() {
    if let Ok(mut state) = global_state().lock() {
        if let Ok(loaded) = NevcPersistenceStore::load_from_file(&default_persist_path()) {
            state.store = loaded;
            state.dirty = false;
        }
    }
}

pub fn snapshot_records() -> Vec<NevcPlayerRecord> {
    global_state()
        .lock()
        .map(|s| s.store.records.values().cloned().collect())
        .unwrap_or_default()
}

/// Durable record for a single player (for PlayerState embedding).
pub fn record_for(player_id: u64) -> Option<NevcPlayerRecord> {
    global_state()
        .lock()
        .ok()
        .and_then(|s| s.store.get(player_id).cloned())
}

/// Restore / merge a PlayerState-style record into the live store.
pub fn restore_record(record: NevcPlayerRecord) {
    if let Ok(mut state) = global_state().lock() {
        state.store.records.insert(record.player_id, record);
        state.dirty = true;
    }
}

pub fn absorb_score(player_id: u64, score: f64) -> ContributionClass {
    if let Ok(mut state) = global_state().lock() {
        let rec = state.store.absorb(player_id, score);
        state.dirty = true;
        let _ = state.persist_force();
        rec.class
    } else {
        ContributionClass::from_score(score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sustainable_harvest_is_contributor() {
        let pid = 9_101_u64;
        let c = on_harvest(pid, true, true, true);
        assert!(c.is_contributor());
        assert!(is_contributor(pid));
    }

    #[test]
    fn unsustainable_harvest_zombie() {
        let pid = 9_102_u64;
        let c = on_harvest(pid, true, false, false);
        assert!(!c.is_contributor());
    }

    #[test]
    fn record_for_after_harvest() {
        let pid = 9_103_u64;
        let _ = on_harvest(pid, true, true, true);
        let rec = record_for(pid);
        assert!(rec.is_some());
        assert!(rec.unwrap().is_contributor());
    }
}
