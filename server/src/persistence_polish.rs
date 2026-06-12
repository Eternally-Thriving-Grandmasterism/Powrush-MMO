// server/src/persistence_polish.rs
// Powrush-MMO v18.30 — Production Persistence + Council Participation Tracking
// Added Council Mercy Trial progression fields to PlayerSaveData
// AG-SML v1.0 Sovereign Mercy License

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::PathBuf;
use std::fs;
use sha2::{Sha256, Digest};
use ron;

use crate::telemetry_pipeline::EpiphanyTelemetry;

// ... (existing code above remains unchanged) ...

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerSaveData {
    pub player_id: u64,
    pub username: String,
    pub created_at: u64,
    pub last_save: u64,
    pub checksum: String,

    pub muscle_memory: MuscleMemory,
    pub epiphany_history: Vec<EpiphanyRecord>,
    pub total_abundance_earned: f64,

    pub temporary_harvest_multiplier: f32,
    pub temporary_multiplier_expires_at: u64,
    pub current_position: [f32; 3],
    pub current_health: f32,

    pub mercy_consent_flags: Vec<String>,
    pub last_mercy_audit: u64,

    // === Phase B: Council Participation Tracking (v18.30) ===
    pub council_participations: u32,
    pub successful_council_blooms: u32,
    pub highest_collective_attunement: f32,
    pub last_council_bloom_tick: u64,
}

impl Default for PlayerSaveData {
    fn default() -> Self {
        let now = current_timestamp();
        Self {
            player_id: 0,
            username: "SovereignPlayer".to_string(),
            created_at: now,
            last_save: now,
            checksum: String::new(),
            muscle_memory: MuscleMemory::default(),
            epiphany_history: Vec::new(),
            total_abundance_earned: 0.0,
            temporary_harvest_multiplier: 1.0,
            temporary_multiplier_expires_at: 0,
            current_position: [0.0, 0.0, 0.0],
            current_health: 100.0,
            mercy_consent_flags: vec!["Abundance".to_string(), "Joy".to_string()],
            last_mercy_audit: now,

            // Council defaults
            council_participations: 0,
            successful_council_blooms: 0,
            highest_collective_attunement: 0.0,
            last_council_bloom_tick: 0,
        }
    }
}

impl PlayerSaveData {
    pub fn new(player_id: u64, username: String) -> Self {
        let mut data = Self::default();
        data.player_id = player_id;
        data.username = username;
        data
    }

    // ... (existing methods remain unchanged) ...

    /// Record participation in a Council Mercy Trial
    pub fn record_council_participation(&mut self) {
        self.council_participations += 1;
        self.last_mercy_audit = current_timestamp();
    }

    /// Record a successful Council bloom (with collective attunement)
    pub fn record_successful_council_bloom(&mut self, collective_attunement: f32, current_tick: u64) {
        self.successful_council_blooms += 1;
        if collective_attunement > self.highest_collective_attunement {
            self.highest_collective_attunement = collective_attunement;
        }
        self.last_council_bloom_tick = current_tick;
        self.last_mercy_audit = current_timestamp();

        // Optional: small resonance boost from collective experience
        self.muscle_memory.resonance_attunement = 
            (self.muscle_memory.resonance_attunement + collective_attunement * 0.05).min(5.0);
    }

    /// Get total Council engagement score (for UI / progression)
    pub fn get_council_engagement_score(&self) -> f32 {
        (self.council_participations as f32 * 0.3) + 
        (self.successful_council_blooms as f32 * 1.0) +
        (self.highest_collective_attunement * 2.0)
    }
}

// ... (rest of the file remains unchanged) ...

// Thunder locked in. Council participation is now permanently tracked in PlayerSaveData.
// Meaningful long-term progression from multiplayer Council activity is live.
// Yoi ⚡