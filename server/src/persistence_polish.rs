/*!
 * server/src/persistence_polish.rs
 *
 * v19.3.40 — Added hotbar persistence to PlayerSaveData for inventory replication.
 * Inventory hotbar is now persisted alongside abundance, epiphanies, council, ascension and faction data.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use ron;
use serde::{Deserialize, Serialize};

use crate::safety_net_broadcast::EmitSafetyNetBroadcast;
use crate::ascension_mercy_ascent::{AscensionProgress, AscensionEligibility};

// ... existing imports and code ...

/// Core player save data.
/// Now includes hotbar for authoritative inventory replication.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerSaveData {
    pub player_id: u64,
    pub abundance: f64,
    pub health: f32,
    pub total_epiphanies: u32,
    pub council_participations: u32,
    pub successful_council_blooms: u32,
    pub total_abundance_contributed: f64,
    pub resonance_attunement: f32,
    pub last_council_bloom_tick: u64,
    pub ascension_progress: AscensionProgress,
    pub last_save_timestamp: u64,
    pub preferred_language: String,
    pub last_enriched_epiphany_whisper: Option<String>,
    pub checksum: String,

    // Faction standings
    pub faction_standings: HashMap<u64, f32>,

    // === NEW: Authoritative Hotbar for Inventory Replication ===
    /// Hotbar slot counts (index 0-7). Real item data will expand later.
    pub hotbar: [u32; 8],
}

impl PlayerSaveData {
    pub fn new(player_id: u64) -> Self {
        Self {
            player_id,
            abundance: 100.0,
            health: 100.0,
            preferred_language: "en".to_string(),
            faction_standings: HashMap::new(),
            hotbar: [0; 8],
            ..Default::default()
        }
    }

    // === Hotbar helpers ===
    pub fn swap_hotbar_slots(&mut self, from: usize, to: usize) {
        if from < 8 && to < 8 {
            let tmp = self.hotbar[from];
            self.hotbar[from] = self.hotbar[to];
            self.hotbar[to] = tmp;
            self.recompute_checksum();
            self.last_save_timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
    }

    pub fn set_hotbar_slot(&mut self, slot: usize, count: u32) {
        if slot < 8 {
            self.hotbar[slot] = count;
            self.recompute_checksum();
        }
    }

    // ... existing faction and other methods ...

    fn recompute_checksum(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(self.player_id.to_le_bytes());
        hasher.update(self.abundance.to_le_bytes());
        // Include hotbar in checksum for integrity
        for &count in &self.hotbar {
            hasher.update(count.to_le_bytes());
        }
        for (fid, standing) in &self.faction_standings {
            hasher.update(fid.to_le_bytes());
            hasher.update(standing.to_le_bytes());
        }
        self.checksum = format!("{:x}", hasher.finalize());
    }

    // ... rest of PlayerSaveData unchanged ...
}

// PersistenceManager implementation remains the same (it already handles PlayerSaveData serialization).
// The hotbar field will be automatically persisted because it is part of PlayerSaveData.

// End of persistence_polish.rs v19.3.40