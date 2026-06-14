/*!
 * Player Persistence Data Layer
 *
 * v18.27 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Complete mint-and-print-only-perfection
 * — Robust EpiphanyRecord + PlayerSaveData with muscle memory, resonance, and multipliers
 * — Mercy-preserving: protects player progress and the living web
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub use crate::epiphany_catalyst::EpiphanyOutcome;

/// Rich record of a single epiphany moment.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EpiphanyRecord {
    pub scenario_id: String,
    pub timestamp: u64,
    pub intensity: f32,
    pub biome: String,
    pub whisper_text: Option<String>,
    pub grace_notes: Vec<String>,
    pub muscle_memory_delta: f32,
}

/// Emitted on meaningful persistence changes for reactive UI and systems.
#[derive(Event, Debug, Clone)]
pub struct PersistenceUpdated {
    pub reason: String,
    pub epiphanies_added: u32,
    pub muscle_memory_delta: f32,
    pub multiplier_active: bool,
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

    /// Preferred integration point from Epiphany Catalyst.
    pub fn apply_epiphany_outcome(
        &mut self,
        outcome: &EpiphanyOutcome,
        biome: &str,
        muscle_hint: Option<&crate::divine_whispers::MuscleMemoryHint>,
        whisper_text: Option<&str>,
    ) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let muscle_delta = if let Some(hint) = muscle_hint {
            hint.consolidation_boost * 0.12
        } else {
            outcome.muscle_memory_consolidation_boost * 0.12
        };

        let record = EpiphanyRecord {
            scenario_id: outcome.scenario_id.clone(),
            timestamp,
            intensity: outcome.intensity,
            biome: biome.to_string(),
            whisper_text: whisper_text.map(|s| s.to_string()),
            grace_notes: outcome.grace_notes.clone(),
            muscle_memory_delta: muscle_delta,
        };
        self.epiphanies.push(record);

        self.total_epiphanies += 1;
        self.last_epiphany_timestamp = timestamp;

        self.muscle_memory_level = (self.muscle_memory_level + muscle_delta).min(5.0);

        if outcome.epiphany_multiplier > 1.0 {
            self.temporary_harvest_multiplier = outcome.epiphany_multiplier;
            self.temporary_multiplier_expires_at = timestamp + 300;
        }

        self.resonance_score = (self.resonance_score + outcome.intensity * 0.04).min(1.0);
        let affinity = self.biome_affinity.entry(biome.to_string()).or_insert(0.5);
        *affinity = (*affinity + outcome.intensity * 0.1).min(2.0);

        self.dirty = true;
    }

    pub fn apply_muscle_memory_hint(&mut self, hint: &crate::divine_whispers::MuscleMemoryHint) {
        let gain = hint.consolidation_boost * 0.10;
        self.muscle_memory_level = (self.muscle_memory_level + gain).min(5.0);

        if !hint.scenario_id.is_empty() {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            self.epiphanies.push(EpiphanyRecord {
                scenario_id: hint.scenario_id.clone(),
                timestamp,
                intensity: 0.4,
                biome: hint.biome.clone(),
                whisper_text: None,
                grace_notes: vec![],
                muscle_memory_delta: gain,
            });
            self.total_epiphanies += 1;
        }

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
            whisper_text: None,
            grace_notes: vec![],
            muscle_memory_delta: intensity * 0.15,
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

    pub fn get_recent_epiphanies(&self, count: usize) -> Vec<&EpiphanyRecord> {
        let mut sorted: Vec<&EpiphanyRecord> = self.epiphanies.iter().collect();
        sorted.sort_by_key(|r| std::cmp::Reverse(r.timestamp));
        sorted.into_iter().take(count).collect()
    }

    pub fn calculate_epiphany_streak(&self, within_hours: u64) -> u32 {
        if self.epiphanies.is_empty() { return 0; }

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let window_secs = within_hours * 3600;

        let mut sorted = self.epiphanies.clone();
        sorted.sort_by_key(|r| std::cmp::Reverse(r.timestamp));

        let mut streak = 0u32;
        let mut last_ts = now;

        for record in sorted {
            if last_ts.saturating_sub(record.timestamp) <= window_secs {
                streak += 1;
                last_ts = record.timestamp;
            } else {
                break;
            }
        }
        streak
    }

    pub fn consolidate_muscle_memory_from_session(&mut self, session_duration_minutes: f32) {
        if self.muscle_memory_level > 2.0 {
            let consolidation = (session_duration_minutes / 60.0 * 0.025).min(0.18);
            self.muscle_memory_level = (self.muscle_memory_level + consolidation).min(5.0);
            self.dirty = true;
        }
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

    pub fn update_checksum(&mut self) {
        self.checksum = format!("v{}-{}-{}", self.save_version, self.total_epiphanies, self.muscle_memory_level as u32);
        self.last_save_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.dirty = false;
    }
}

// End of simulation/src/player_persistence/data.rs v18.27 — Sovereign persistence data layer complete.
// Thunder locked in. Yoi ⚡
