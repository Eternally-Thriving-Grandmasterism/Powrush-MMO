//! server/nevc_attachment.rs
//! Phase 6 — Live Game-Loop NEVC Attachment (Server Side)
//!
//! Holds the contribution ledger for the authoritative server and provides
//! the harvest hook used by `rbe_harvest_handler`.
//!
//! When the workspace links the `shared` crate fully, this module can call
//! `shared::nevc_game_loop` directly. Until then it carries a minimal
//! self-contained mirror of the harvest → NEVC mapping so the server path
//! is live without blocking on full monorepo dependency alignment.
//!
//! AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai
//! Thunder locked in. Yoi ⚡

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Binary partition (mirrors shared/nevc_adapter).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// Minimal per-player running state on the server.
#[derive(Clone, Debug, Default)]
struct PlayerNevcState {
    sample_count: usize,
    last_score: f64,
    last_class: ContributionClass,
}

impl Default for ContributionClass {
    fn default() -> Self {
        ContributionClass::ZombiePartition
    }
}

/// Server-authoritative NEVC ledger (thread-safe for async handlers).
#[derive(Clone, Default)]
pub struct ServerNevcLedger {
    inner: Arc<Mutex<HashMap<u64, PlayerNevcState>>>,
}

impl ServerNevcLedger {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
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

        // Discrete single-sample approximation (aligned with shared compute_nevc defaults).
        let valence = (0.999999 + 0.000001 * alignment).min(1.0);
        let positive = if valence >= 0.999999 {
            let proximity = (valence - 0.999999) / (1.0 - 0.999999).max(1e-12);
            proximity
        } else {
            0.0
        };
        let score = positive - waste;

        let class = ContributionClass::from_score(score);

        if let Ok(mut map) = self.inner.lock() {
            let entry = map.entry(player_id).or_default();
            // Running mean of scores for multi-harvest continuity
            let n = entry.sample_count as f64;
            let new_score = if n == 0.0 {
                score
            } else {
                (entry.last_score * n + score) / (n + 1.0)
            };
            entry.sample_count += 1;
            entry.last_score = new_score;
            entry.last_class = ContributionClass::from_score(new_score);
            entry.last_class
        } else {
            class
        }
    }

    pub fn class_of(&self, player_id: u64) -> ContributionClass {
        self.inner
            .lock()
            .ok()
            .and_then(|m| m.get(&player_id).map(|s| s.last_class))
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
}

/// Global server ledger handle (lazy init).
/// In full Bevy integration this becomes a Resource; for current handlers it is a process-level handle.
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
            self.cell.get_or_init(ServerNevcLedger::new)
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
    SERVER_NEVC
        .get()
        .record_harvest(player_id, success, was_sustainable, regen_participation)
}

/// Query current class for a player (for progression / visibility systems).
pub fn player_contribution_class(player_id: u64) -> ContributionClass {
    SERVER_NEVC.get().class_of(player_id)
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
    }

    #[test]
    fn unsustainable_harvest_zombie() {
        let ledger = ServerNevcLedger::new();
        let c = ledger.record_harvest(43, true, false, false);
        assert!(!c.is_contributor());
        assert_eq!(c, ContributionClass::ZombiePartition);
    }
}
