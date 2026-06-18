/*!
 * Player Persistence Data Layer
 *
 * v18.96 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm + Language Persistence)
 * — Language preference now persisted in PlayerSaveData
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
pub struct EpiphanyRecord { /* ... unchanged ... */ }

#[derive(Event, Debug, Clone)]
pub struct PersistenceUpdated { /* ... unchanged ... */ }

#[derive(Debug, Clone, Serialize, Deserialize, Resource, Default)]
pub struct PlayerSaveData {
    pub save_version: u32,
    pub checksum: String,
    pub player_id: u64,
    pub total_harvests: u32,
    pub sustainable_harvests: u32,
    pub total_playtime_seconds: u64,
    pub last_played_timestamp: u64,
    pub epiphanies: Vec<EpiphanyRecord>,
    pub achievements: Vec<String>,
    pub muscle_memory_level: f32,
    pub last_save_timestamp: u64,

    pub total_epiphanies: u32,
    pub council_sessions_participated: u32,
    pub resonance_score: f32,
    pub faction_standings: HashMap<String, f32>,
    pub biome_affinity: HashMap<String, f32>,
    pub last_epiphany_timestamp: u64,

    pub temporary_harvest_multiplier: f32,
    pub temporary_multiplier_expires_at: u64,

    // NEW v18.96 — Persisted language preference for Divine Whispers and Quantum Swarm
    pub preferred_language: String,

    #[serde(skip)]
    pub dirty: bool,
}

impl Default for PlayerSaveData {
    fn default() -> Self {
        Self {
            // ... existing defaults ...
            preferred_language: "en".to_string(),
            dirty: false,
        }
    }
}

impl PlayerSaveData {
    pub fn new(player_id: u64) -> Self {
        let mut data = Self::default();
        data.player_id = player_id;
        data.preferred_language = "en".to_string();
        data
    }

    // ... all existing methods unchanged ...

    pub fn set_preferred_language(&mut self, lang: &str) {
        if self.preferred_language != lang {
            self.preferred_language = lang.to_string();
            self.dirty = true;
        }
    }

    pub fn apply_epiphany_outcome(
        &mut self,
        outcome: &EpiphanyOutcome,
        biome: &str,
        muscle_hint: Option<&crate::divine_whispers::MuscleMemoryHint>,
        whisper_text: Option<&str>,
    ) {
        // ... existing logic ...
        self.dirty = true;
    }

    // ... rest of methods ...
}

// End of simulation/src/player_persistence/data.rs v18.96 — Language preference persisted.
// Thunder locked in. Yoi ⚡
