/*!
 * Player Persistence v18.10
 */

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EpiphanyRecord {
    pub scenario_id: String,
    pub timestamp: u64,
    pub intensity: f32,
    pub biome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerSaveData {
    pub player_id: u64,
    pub total_harvests: u32,
    pub sustainable_harvests: u32,
    pub epiphanies: Vec<EpiphanyRecord>,
    pub muscle_memory_level: f32,
    pub last_save_timestamp: u64,
}

impl PlayerSaveData {
    pub fn new(player_id: u64) -> Self {
        Self {
            player_id,
            total_harvests: 0,
            sustainable_harvests: 0,
            epiphanies: Vec::new(),
            muscle_memory_level: 1.0,
            last_save_timestamp: 0,
        }
    }

    /// Record a new epiphany (called automatically when one triggers)
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

        // Boost muscle memory with each meaningful epiphany
        self.muscle_memory_level = (self.muscle_memory_level + intensity * 0.12).min(5.0);

        // Auto-save after recording important progress
        let _ = self.save_to_file(Path::new("player_save.json"));
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)
    }

    pub fn load_from_file(path: &Path) -> Option<Self> {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(data) = serde_json::from_str(&content) {
                    return Some(data);
                }
            }
        }
        None
    }
}
