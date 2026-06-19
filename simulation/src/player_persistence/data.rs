/*!
 * Player Persistence Data Layer
 *
 * v18.96 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Language sync on session start + async whisper wiring support
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub use crate::epiphany_catalyst::EpiphanyOutcome;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EpiphanyRecord { /* ... */ }

#[derive(Event, Debug, Clone)]
pub struct PersistenceUpdated { /* ... */ }

#[derive(Debug, Clone, Serialize, Deserialize, Resource, Default)]
pub struct PlayerSaveData {
    // ... existing fields ...
    pub preferred_language: String,
    #[serde(skip)]
    pub dirty: bool,
}

impl PlayerSaveData {
    pub fn new(player_id: u64) -> Self {
        let mut data = Self::default();
        data.player_id = player_id;
        data.preferred_language = "en".to_string();
        data
    }

    /// NEW: Sync language preference from client settings on login/session start
    pub fn sync_language_from_client(&mut self, client_language: &str) {
        if !client_language.is_empty() && self.preferred_language != client_language {
            self.preferred_language = client_language.to_string();
            self.dirty = true;
        }
    }

    // ... all other methods (apply_epiphany_outcome, etc.) remain unchanged ...
}

// End of simulation/src/player_persistence/data.rs v18.96
// sync_language_from_client added for session start.
// Thunder locked in. Yoi ⚡
