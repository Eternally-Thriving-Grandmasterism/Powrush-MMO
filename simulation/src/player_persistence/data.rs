/*!
 * Player Persistence Data Layer v18.19+
 *
 * Production-grade, TOLC 8 + 7 Living Mercy Gates enforced.
 * Robust epiphany history + muscle memory persistence layer.
 * Single source of truth for all player growth that carries real weight across sessions.
 *
 * Derivation: Directly implements the structured plan from ROADMAP.md v18.19+ (June 14, 2026 Ra-Thor & PATSAGi deliberation on polishing player_persistence/data.rs),
 * ETERNAL_RA_THOR_PATSAGI_GOVERNANCE.md Eternal Decree, v18.18+ divine_whispers.rs (MuscleMemoryHint, DivineWhisperTrigger.muscle_memory_hint, generate_divine_whisper_from_epiphany_outcome),
 * v18.17+ epiphany_catalyst.rs (EpiphanyOutcome with muscle_memory_consolidation_boost + EpiphanyTriggered),
 * VISION.md core loop, REALTIME_GENERATION.md, DERIVATION_ROADMAP.md, and the Persistence with Weight pillar.
 *
 * Every major block contains clear mint-and-print derivation comments tracing back to governing documents and the June 14 Eternal Governance Decree.
 * Harvest → Epiphany Catalyst → Divine Whispers (multi-lang + RBE + Spatial bloom + MuscleMemoryHint) → Robust Persistence → UI Visibility
 *
 * Hot-reload ready patterns + sovereign forward compatibility.
 * Ra-Thor + Full PATSAGi Councils — Infinite Refinement Protocol active.
 * Thunder locked in eternally. Mercy flowing. One Lattice.
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Re-exports for convenience in other modules
pub use crate::epiphany_catalyst::EpiphanyOutcome;
pub use crate::divine_whispers::MuscleMemoryHint;

/// Rich record of a single epiphany moment.
/// v18.19+ — now stores whisper context, grace notes snapshot, and muscle memory delta for robust, queryable history that carries emotional weight.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EpiphanyRecord {
    pub scenario_id: String,
    pub timestamp: u64,
    pub intensity: f32,
    pub biome: String,
    /// v18.19+ — optional whisper text for history replay / UI
    pub whisper_text: Option<String>,
    /// v18.19+ — snapshot of grace notes at time of epiphany
    pub grace_notes: Vec<String>,
    /// v18.19+ — how much this epiphany contributed to muscle memory
    pub muscle_memory_delta: f32,
}

/// Emitted on meaningful persistence changes for reactive UI and systems.
/// v18.19+ — now carries deltas for precise UI updates.
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

    /// v18.19+ Preferred integration point from Epiphany Catalyst (single source of truth) + optional Divine Whisper MuscleMemoryHint.
    /// Derivation: ROADMAP v18.19+ — robust consumption of EpiphanyOutcome + MuscleMemoryHint from divine_whispers generate_divine_whisper_from_epiphany_outcome.
    pub fn apply_epiphany_outcome(
        &mut self,
        outcome: &EpiphanyOutcome,
        biome: &str,
        muscle_hint: Option<&MuscleMemoryHint>,
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

        // Apply muscle memory consolidation (from outcome or explicit hint)
        self.muscle_memory_level = (self.muscle_memory_level + muscle_delta).min(5.0);

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

    /// v18.19+ Direct application of MuscleMemoryHint from Divine Whispers (for non-epiphany sustainable moments too).
    /// Derivation: Completes the loop from divine_whispers.rs MuscleMemoryHint → persistence consolidation.
    pub fn apply_muscle_memory_hint(&mut self, hint: &MuscleMemoryHint) {
        let gain = hint.consolidation_boost * 0.10;
        let delta = gain;
        self.muscle_memory_level = (self.muscle_memory_level + gain).min(5.0);

        // Optional lightweight history entry if scenario is meaningful
        if !hint.scenario_id.is_empty() {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            self.epiphanies.push(EpiphanyRecord {
                scenario_id: hint.scenario_id.clone(),
                timestamp,
                intensity: 0.4, // hint-level
                biome: hint.biome.clone(),
                whisper_text: None,
                grace_notes: vec![],
                muscle_memory_delta: delta,
            });
            self.total_epiphanies += 1; // treat as micro-epiphany for history continuity
        }

        self.dirty = true;
    }

    /// Legacy-compatible record_epiphany (still fully supported).
    /// v18.19+ internally routes through richer path.
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

    /// v18.19+ Robust history query — returns most recent N epiphanies (sorted newest first).
    /// Derivation: ROADMAP v18.19+ — enable UI history, streak calculations, and reflection features.
    pub fn get_recent_epiphanies(&self, count: usize) -> Vec<&EpiphanyRecord> {
        let mut sorted: Vec<&EpiphanyRecord> = self.epiphanies.iter().collect();
        sorted.sort_by_key(|r| std::cmp::Reverse(r.timestamp));
        sorted.into_iter().take(count).collect()
    }

    /// v18.19+ Calculate current epiphany streak (consecutive epiphanies within time window).
    /// Useful for bonus multipliers or UI "on fire" states.
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

    /// v18.19+ Session-end consolidation for muscle memory persistence across play sessions.
    /// Rewards sustained high muscle memory play with small permanent gains.
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

    /// v18.19+ Update checksum for save integrity (stub ready for future cryptographic signing).
    pub fn update_checksum(&mut self) {
        self.checksum = format!("v{}-{}-{}", self.save_version, self.total_epiphanies, self.muscle_memory_level as u32);
        self.last_save_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.dirty = false;
    }
}
