/*!
 * server/src/persistence_polish.rs
 * v19.3.41 — Hotbar now uses full HotbarSlot (item_id, durability, rarity, valence)
 */

use bevy::prelude::*;
use shared::protocol::HotbarSlot;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use ron;
use serde::{Deserialize, Serialize};

// ... existing code ...

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerSaveData {
    pub player_id: u64,
    pub abundance: f64,
    pub health: f32,
    // ... other fields ...

    /// Full hotbar with item identity (replaces simple counts)
    pub hotbar: [HotbarSlot; 8],
}

impl PlayerSaveData {
    pub fn new(player_id: u64) -> Self {
        Self {
            player_id,
            abundance: 100.0,
            health: 100.0,
            hotbar: [HotbarSlot::empty(); 8],
            // ...
            ..Default::default()
        }
    }

    pub fn swap_hotbar_slots(&mut self, from: usize, to: usize) {
        if from < 8 && to < 8 {
            let tmp = self.hotbar[from].clone();
            self.hotbar[from] = self.hotbar[to].clone();
            self.hotbar[to] = tmp;
            self.recompute_checksum();
            self.last_save_timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH).unwrap().as_secs();
        }
    }

    // ... existing methods ...
}

// PersistenceManager continues to work unchanged (serializes the new struct automatically)

// End of persistence_polish.rs v19.3.41