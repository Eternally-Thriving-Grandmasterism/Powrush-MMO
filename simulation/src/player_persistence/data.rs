/*!
 * Persistence Data Layer (Final Polish for Item 1)
 * Structs, Event, mutation helpers + clean integration with EpiphanyOutcome.
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Re-export for convenience in other modules
pub use crate::epiphany_catalyst::EpiphanyOutcome;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EpiphanyRecord {
    pub scenario_id: String,
    pub timestamp: u64,
    pub intensity: f32,
    pub biome: String,
}

/// Emitted on meaningful persistence changes for UI reactivity.
#[derive(Event, Debug, Clone)]
pub struct PersistenceUpdated {
    pub reason: String,
}

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

    #[serde(skip)]
    pub dirty: bool,
}

impl Default for PlayerSaveData {
    fn default() -> Self {
        Self {
            save_version: 1,
            checksum: String::new(),
            player_id: 0,
            total_harvests: 0,
            sustainable_harvests: 0,
            total_playtime_seconds: 0,
            last_played_timestamp: 0,
            epiphanies: Vec::new(),
            achievements: Vec::new(),
            muscle_memory_level: 1.0,
            last_save_timestamp: 0,
            total_epiphanies: 0,
            council_sessions_participated: 0,
            resonance_score: 0.5,
            faction_standings: HashMap::new(),
            biome_affinity: HashMap::new(),
            last_epiphany_timestamp: 0,
            temporary_harvest_multiplier: 1.0,
            temporary_multiplier_expires_at: 0,
            dirty: false,
        }
    }
}

impl PlayerSaveData {
    pub fn new(player_id: u64) -> Self {
        let mut data = Self::default();
        data.player_id = player_id;
        data
    }

    /// Preferred integration point from Epiphany Catalyst (single source of truth)
    pub fn apply_epiphany_outcome(&mut self, outcome: &EpiphanyOutcome, biome: &str) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.epiphanies.push(EpiphanyRecord {
            scenario_id: outcome.scenario_id.clone(),
            timestamp,
            intensity: outcome.intensity,
            biome: biome.to_string(),
        });

        self.total_epiphanies += 1;
        self.last_epiphany_timestamp = timestamp;

        // Apply muscle memory consolidation from catalyst
        let muscle_gain = outcome.muscle_memory_consolidation_boost * 0.12;
        self.muscle_memory_level = (self.muscle_memory_level + muscle_gain).min(5.0);

        // Apply temporary multiplier from epiphany
        if outcome.epiphany_multiplier > 1.0 {
            self.temporary_harvest_multiplier = outcome.epiphany_multiplier;
            self.temporary_multiplier_expires_at = timestamp + 300; // 5 minutes
        }

        // Resonance & biome affinity
        self.resonance_score = (self.resonance_score + outcome.intensity * 0.04).min(1.0);
        let affinity = self.biome_affinity.entry(biome.to_string()).or_insert(0.5);
        *affinity = (*affinity + outcome.intensity * 0.1).min(2.0);

        self.dirty = true;
    }

    pub fn record_epiphany(&mut self, scenario_id: &str, intensity: f32, biome: &str) -> f32 {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.epiphanies.push(EpiphanyRecord {
            scenario_id: scenario_id.to_string(),
            timestamp,
            intensity,
            biome: biome.to_string(),
        });

        self.total_epiphanies += 1;
        self.last_epiphany_timestamp = timestamp;

        let muscle_gain = intensity * 0.15 * (1.0 + self.muscle_memory_level * 0.1);
        self.muscle_memory_level = (self.muscle_memory_level + muscle_gain).min(5.0);

        self.resonance_score = (self.resonance_score + intensity * 0.03).min(1.0);

        let affinity = self.biome_affinity.entry(biome.to_string()).or_insert(0.5);
        *affinity = (*affinity + intensity * 0.08).min(2.0);

        self.dirty = true;
        self.muscle_memory_level
    }

    pub fn record_harvest(&mut self, sustainable: bool) {
        self.total_harvests += 1;
        if sustainable { self.sustainable_harvests += 1; }
        self.dirty = true;
    }

    pub fn record_council_participation(&mut self) {
        self.council_sessions_participated += 1;
        self.resonance_score = (self.resonance_score + 0.05).min(1.0);
        self.dirty = true;
    }

    pub fn add_playtime(&mut self, seconds: u64) {
        self.total_playtime_seconds += seconds;
        self.last_played_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.dirty = true;
    }

    pub fn get_muscle_memory_harvest_bonus(&self) -> f32 {
        1.0 + (self.muscle_memory_level - 1.0) * 0.08
    }

    pub fn get_muscle_memory_epiphany_threshold_modifier(&self) -> f32 {
        (5.0 - self.muscle_memory_level) / 4.0
    }

    pub fn has_active_multiplier(&self) -> bool {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.temporary_multiplier_expires_at > now && self.temporary_harvest_multiplier > 1.0
    }

    pub fn get_current_harvest_multiplier(&self) -> f32 {
        let base = if self.has_active_multiplier() {
            self.temporary_harvest_multiplier
        } else { 1.0 };
        base * self.get_muscle_memory_harvest_bonus()
    }
}
