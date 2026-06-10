/*!
 * Player Persistence v18.10
 *
 * Includes save versioning for future-proofing.
 */

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Current save format version.
/// Increment this when changing the structure of PlayerSaveData.
pub const CURRENT_SAVE_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EpiphanyRecord {
    pub scenario_id: String,
    pub timestamp: u64,
    pub intensity: f32,
    pub biome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSaveData {
    pub save_version: u32,
    pub player_id: u64,
    pub total_harvests: u32,
    pub sustainable_harvests: u32,
    pub epiphanies: Vec<EpiphanyRecord>,
    pub muscle_memory_level: f32,
    pub last_save_timestamp: u64,
}

impl Default for PlayerSaveData {
    fn default() -> Self {
        Self {
            save_version: CURRENT_SAVE_VERSION,
            player_id: 0,
            total_harvests: 0,
            sustainable_harvests: 0,
            epiphanies: Vec::new(),
            muscle_memory_level: 1.0,
            last_save_timestamp: 0,
        }
    }
}

impl PlayerSaveData {
    pub fn new(player_id: u64) -> Self {
        Self {
            save_version: CURRENT_SAVE_VERSION,
            player_id,
            total_harvests: 0,
            sustainable_harvests: 0,
            epiphanies: Vec::new(),
            muscle_memory_level: 1.0,
            last_save_timestamp: 0,
        }
    }

    /// Record a new epiphany and auto-save
    pub fn record_epiphany(&mut self, scenario_id: &str, intensity: f32, biome: &str) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.epiphanies.push(EpiphanyRecord {
            scenario_id: scenario_id.to_string(),
            timestamp,
            intensity,
            biome: biome.to_string(),
        });

        self.muscle_memory_level = (self.muscle_memory_level + intensity * 0.12).min(5.0);

        // Auto-save after important progress
        let _ = self.save_to_file(Path::new("player_save.json"));
    }

    /// Load from file with version migration support
    pub fn load_from_file(path: &Path) -> Option<Self> {
        if !path.exists() {
            return None;
        }

        let content = fs::read_to_string(path).ok()?;
        let mut data: PlayerSaveData = serde_json::from_str(&content).ok()?;

        // Migrate if save is from an older version
        if data.save_version < CURRENT_SAVE_VERSION {
            data = Self::migrate(data);
        }

        Some(data)
    }

    /// Basic migration logic (extend this when save format changes)
    fn migrate(mut old_data: PlayerSaveData) -> PlayerSaveData {
        // Example for future versions:
        // if old_data.save_version == 1 {
        //     // migrate from v1 to v2
        //     old_data.save_version = 2;
        // }

        old_data.save_version = CURRENT_SAVE_VERSION;
        old_data
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)
    }
}
