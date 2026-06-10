/*!
 * Player Persistence v18.10
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::Duration;

pub const CURRENT_SAVE_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EpiphanyRecord {
    pub scenario_id: String,
    pub timestamp: u64,
    pub intensity: f32,
    pub biome: String,
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
    // Temporary epiphany rewards
    pub temporary_harvest_multiplier: f32,
    pub temporary_multiplier_expires_at: u64,
}

impl Default for PlayerSaveData {
    fn default() -> Self {
        Self {
            save_version: CURRENT_SAVE_VERSION,
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
            temporary_harvest_multiplier: 1.0,
            temporary_multiplier_expires_at: 0,
        }
    }
}

impl PlayerSaveData {
    pub fn new(player_id: u64) -> Self {
        Self {
            save_version: CURRENT_SAVE_VERSION,
            checksum: String::new(),
            player_id,
            total_harvests: 0,
            sustainable_harvests: 0,
            total_playtime_seconds: 0,
            last_played_timestamp: 0,
            epiphanies: Vec::new(),
            achievements: Vec::new(),
            muscle_memory_level: 1.0,
            last_save_timestamp: 0,
            temporary_harvest_multiplier: 1.0,
            temporary_multiplier_expires_at: 0,
        }
    }

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
        let _ = self.save_to_file(Path::new("player_save.json"));
    }

    pub fn add_playtime(&mut self, seconds: u64) {
        self.total_playtime_seconds += seconds;
        self.last_played_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    }

    /// Check if temporary multiplier is still active
    pub fn has_active_multiplier(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.temporary_multiplier_expires_at > now && self.temporary_harvest_multiplier > 1.0
    }

    /// Get current effective harvest multiplier
    pub fn get_current_harvest_multiplier(&self) -> f32 {
        if self.has_active_multiplier() {
            self.temporary_harvest_multiplier
        } else {
            1.0
        }
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        // ... atomic save logic (kept from previous)
        let temp_path = path.with_extension("json.tmp");
        {
            let json = serde_json::to_string_pretty(self)?;
            fs::write(&temp_path, json)?;
        }
        Self::rotate_backups(path)?;
        if path.exists() {
            let backup_path = path.with_extension("json.bak");
            let _ = fs::copy(path, &backup_path);
        }
        fs::rename(&temp_path, path)
    }

    pub fn load_from_file(path: &Path) -> Option<Self> {
        // ... checksum + version logic (kept from previous)
        if !path.exists() {
            return None;
        }
        let content = fs::read_to_string(path).ok()?;
        let mut data: Self = serde_json::from_str(&content).ok()?;

        let expected = data.compute_checksum();
        if data.checksum != expected {
            warn!("Checksum mismatch on load!");
            if let Some(backup) = Self::load_from_file(&path.with_extension("json.bak")) {
                return Some(backup);
            }
            return None;
        }

        if data.save_version < CURRENT_SAVE_VERSION {
            return Some(Self::migrate(data));
        }
        Some(data)
    }

    fn compute_checksum(&self) -> String {
        let mut hasher = sha2::Sha256::new();
        let mut temp = self.clone();
        temp.checksum = String::new();
        if let Ok(json) = serde_json::to_string(&temp) {
            hasher.update(json.as_bytes());
        }
        format!("{:x}", hasher.finalize())
    }

    fn migrate(mut old_data: Self) -> Self {
        old_data.save_version = CURRENT_SAVE_VERSION;
        old_data.checksum = old_data.compute_checksum();
        old_data
    }

    fn rotate_backups(path: &Path) -> Result<(), std::io::Error> {
        // Rotating backup logic (simplified)
        let base = path.with_extension("json.bak");
        for i in (1..5).rev() {
            let src = path.with_extension(&format!("json.bak.{}", i));
            let dst = path.with_extension(&format!("json.bak.{}", i + 1));
            if src.exists() {
                let _ = fs::rename(&src, &dst);
            }
        }
        if base.exists() {
            let _ = fs::rename(&base, &path.with_extension("json.bak.1"));
        }
        Ok(())
    }
}

#[derive(Resource)]
pub struct AutoSaveTimer {
    pub timer: Timer,
}

impl Default for AutoSaveTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(60), TimerMode::Repeating),
        }
    }
}

pub struct PersistencePlugin;

impl Plugin for PersistencePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerSaveData>()
            .init_resource::<AutoSaveTimer>()
            .add_systems(Startup, load_player_save)
            .add_systems(Update, auto_save_system)
            .add_systems(Update, save_on_exit)
            .add_systems(Update, update_playtime);
    }
}

fn load_player_save(mut commands: Commands) {
    let save_path = Path::new("player_save.json");

    if let Some(loaded) = PlayerSaveData::load_from_file(save_path) {
        commands.insert_resource(loaded);
        info!("Loaded player save");
    } else {
        commands.insert_resource(PlayerSaveData::new(1));
    }
}

fn auto_save_system(
    mut save_data: ResMut<PlayerSaveData>,
    mut timer: ResMut<AutoSaveTimer>,
    time: Res<Time>,
) {
    timer.timer.tick(time.delta());
    if timer.timer.just_finished() {
        let _ = save_data.save_to_file(Path::new("player_save.json"));
    }
}

fn save_on_exit(
    mut save_data: ResMut<PlayerSaveData>,
    mut exit_events: EventReader<bevy::app::AppExit>,
) {
    for _ in exit_events.read() {
        let _ = save_data.save_to_file(Path::new("player_save.json"));
    }
}

fn update_playtime(
    mut save_data: ResMut<PlayerSaveData>,
    time: Res<Time>,
) {
    save_data.total_playtime_seconds += time.delta().as_secs();
}
