/*!
 * server/src/persistence_polish.rs
 *
 * v19.3.39 — Added faction standing support to PlayerSaveData.
 * This is the first step toward unifying faction persistence with the main player save system.
 *
 * AG-SML v1.0 | TOLC 8
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

// ... (existing code) ...

/// Core player save data with epiphany, council, ascension, abundance, language, and **faction standings**.
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

    // === NEW: Faction Standing Integration ===
    /// faction_id -> standing value
    pub faction_standings: HashMap<u64, f32>,
}

impl PlayerSaveData {
    pub fn new(player_id: u64) -> Self {
        Self {
            player_id,
            abundance: 100.0,
            health: 100.0,
            preferred_language: "en".to_string(),
            faction_standings: HashMap::new(),
            ..Default::default()
        }
    }

    // === NEW: Faction Standing Helpers ===
    pub fn set_faction_standing(&mut self, faction_id: u64, standing: f32) {
        self.faction_standings.insert(faction_id, standing.clamp(0.0, 5.0));
        self.recompute_checksum();
    }

    pub fn get_faction_standing(&self, faction_id: u64) -> f32 {
        self.faction_standings.get(&faction_id).copied().unwrap_or(1.0) // default neutral standing
    }

    pub fn remove_faction(&mut self, faction_id: u64) {
        self.faction_standings.remove(&faction_id);
        self.recompute_checksum();
    }

    // ... (existing record_* methods remain unchanged) ...

    fn recompute_checksum(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(self.player_id.to_le_bytes());
        hasher.update(self.abundance.to_le_bytes());
        // ... other fields ...
        // Include faction standings in checksum
        for (fid, standing) in &self.faction_standings {
            hasher.update(fid.to_le_bytes());
            hasher.update(standing.to_le_bytes());
        }
        self.checksum = format!("{:x}", hasher.finalize());
    }

    // ... rest of PlayerSaveData ...
}

// ... (PersistenceManager and tests remain mostly unchanged) ...
