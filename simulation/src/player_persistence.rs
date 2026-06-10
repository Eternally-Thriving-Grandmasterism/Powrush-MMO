/*!
 * Player Persistence v18.10
 *
 * Includes atomic save logic for crash safety.
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
    pub player_id: u64,
    pub total_harvests: u32,
    pub sustainable_harvests: u32,
    pub total_playtime_seconds: u64,
    pub last_played_timestamp: u64,
    pub epiphanies: Vec<EpiphanyRecord>,
    pub achievements: Vec<String>,
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
            total_playtime_seconds: 0,
            last_played_timestamp: 0,
            epiphanies: Vec::new(),
            achievements: Vec::new(),
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
            total_playtime_seconds: 0,
            last_played_timestamp: 0,
            epiphanies: Vec::new(),
            achievements: Vec::new(),
            muscle_memory_level: 1.0,
            last_save_timestamp: 0,
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

    /// Atomic save: writes to a temp file then renames (crash-safe)
    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let temp_path = path.with_extension("json.tmp");

        // Write to temporary file first
        {
            let json = serde_json::to_string_pretty(self)?;
            fs::write(&temp_path, json)?;
        }

        // Create backup of existing file (if any)
        if path.exists() {
            let backup_path = path.with_extension("json.bak");
            let _ = fs::copy(path, &backup_path);
        }

        // Atomic rename (this is the key for crash safety)
        fs::rename(&temp_path, path)
    }

    pub fn load_from_file(path: &Path) -> Option<Self> {
        if !path.exists() {
            return None;
        }
        let content = fs::read_to_string(path).ok()?;
        let mut data: Self = serde_json::from_str(&content).ok()?;

        if data.save_version < CURRENT_SAVE_VERSION {
            data = Self::migrate(data);
        }
        Some(data)
    }

    fn migrate(mut old_data: Self) -> Self {
        old_data.save_version = CURRENT_SAVE_VERSION;
        old_data
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
        info!("Loaded player save with {} epiphanies", loaded.epiphanies.len());
    } else {
        let new_save = PlayerSaveData::new(1);
        commands.insert_resource(new_save);
        info!("Created new player save");
    }
}

fn auto_save_system(
    mut save_data: ResMut<PlayerSaveData>,
    mut auto_save_timer: ResMut<AutoSaveTimer>,
    time: Res<Time>,
) {
    auto_save_timer.timer.tick(time.delta());

    if auto_save_timer.timer.just_finished() {
        let save_path = Path::new("player_save.json");
        if let Err(e) = save_data.save_to_file(save_path) {
            warn!("Failed to auto-save: {}", e);
        }
    }
}

fn save_on_exit(
    mut save_data: ResMut<PlayerSaveData>,
    mut exit_events: EventReader<bevy::app::AppExit>,
) {
    for _ in exit_events.read() {
        let save_path = Path::new("player_save.json");
        if let Err(e) = save_data.save_to_file(save_path) {
            error!("Failed to save on exit: {}", e);
        } else {
            info!("Saved player progress on exit");
        }
    }
}

fn update_playtime(
    mut save_data: ResMut<PlayerSaveData>,
    time: Res<Time>,
) {
    save_data.total_playtime_seconds += time.delta().as_secs();
}
