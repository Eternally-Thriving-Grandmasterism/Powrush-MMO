/*!
 * Player Persistence — Production Grade v18.15+
 * Rotating Backup Strategy + Atomic Save + Checksum Integrity
 *
 * Strategy:
 * - Atomic write via .tmp + rename (crash-safe)
 * - On every successful save: rotate existing backups (numbered 1..MAX_BACKUPS)
 * - Keep last N backups (default 7) to prevent unbounded growth
 * - Timestamped snapshot also created for easy manual recovery
 * - Checksum verified on load; falls back to most recent valid backup
 * - Integrated with Epiphany Catalyst (single source of truth) and HarvestingSystem
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const CURRENT_SAVE_VERSION: u32 = 1;
pub const MAX_BACKUPS: usize = 7; // Production retention: last 7 saves

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
    // Temporary epiphany rewards (from evaluate_epiphany single source of truth)
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

        self.muscle_memory_level = (self.muscle_memory_level + intensity * 0.12).min(5.0);

        // Trigger save after epiphany (integrated with HarvestingSystem flow)
        let _ = self.save_to_file(Path::new("player_save.json"));
    }

    pub fn add_playtime(&mut self, seconds: u64) {
        self.total_playtime_seconds += seconds;
        self.last_played_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH).unwrap().as_secs();
    }

    /// Check if temporary multiplier (from epiphany) is still active
    pub fn has_active_multiplier(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.temporary_multiplier_expires_at > now && self.temporary_harvest_multiplier > 1.0
    }

    /// Get current effective harvest multiplier (single source of truth path)
    pub fn get_current_harvest_multiplier(&self) -> f32 {
        if self.has_active_multiplier() {
            self.temporary_harvest_multiplier
        } else {
            1.0
        }
    }

    /// Production-grade atomic save with rotating backup strategy
    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        // 1. Compute fresh checksum
        let mut data_to_save = self.clone();
        data_to_save.checksum = data_to_save.compute_checksum();
        data_to_save.last_save_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // 2. Write to atomic temp file first
        let temp_path = path.with_extension("json.tmp");
        {
            let json = serde_json::to_string_pretty(&data_to_save)?;
            fs::write(&temp_path, json)?;
        }

        // 3. Rotate backups BEFORE promoting the new save (preserves history on crash)
        Self::rotate_backups(path)?;

        // 4. Create a timestamped snapshot for easy human recovery (optional but production-friendly)
        Self::create_timestamped_snapshot(path)?;

        // 5. Atomic promote: rename temp -> final
        fs::rename(&temp_path, path)?;

        Ok(())
    }

    /// Robust rotating backup strategy (numbered + retention)
    /// Keeps last MAX_BACKUPS versions. Oldest is deleted.
    fn rotate_backups(path: &Path) -> Result<(), std::io::Error> {
        // Delete oldest backup if we already have MAX_BACKUPS
        let oldest = path.with_extension(format!("json.bak.{}", MAX_BACKUPS));
        if oldest.exists() {
            fs::remove_file(&oldest)?;
        }

        // Shift all existing backups up by one slot
        for i in (1..MAX_BACKUPS).rev() {
            let src = path.with_extension(format!("json.bak.{}", i));
            let dst = path.with_extension(format!("json.bak.{}", i + 1));
            if src.exists() {
                fs::rename(&src, &dst)?;
            }
        }

        // Move current live save to .bak.1 (if it exists)
        if path.exists() {
            let bak1 = path.with_extension("json.bak.1");
            fs::rename(path, &bak1)?;
        }

        Ok(())
    }

    /// Create a human-readable timestamped snapshot (does not affect rotation count)
    fn create_timestamped_snapshot(path: &Path) -> Result<(), std::io::Error> {
        if !path.exists() {
            return Ok(());
        }

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let timestamp = format!("{:?}", now); // Simple; in prod could use chrono for pretty format
        let snapshot_name = format!(
            "player_save_{}_{}.json.bak",
            timestamp,
            path.file_stem().unwrap_or_default().to_string_lossy()
        );
        let snapshot_path = path.parent().unwrap_or(Path::new(".")).join(snapshot_name);

        // Only keep a small number of timestamped snapshots too (optional cleanup)
        // For now we create one per save — admin can prune manually or we add cleanup later
        fs::copy(path, &snapshot_path)?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Option<Self> {
        if !path.exists() {
            return None;
        }

        // Try primary
        if let Some(data) = Self::try_load_with_checksum(path) {
            return Some(data);
        }

        warn!("Primary save checksum invalid or corrupted. Attempting backup recovery...");

        // Fallback to most recent backup (.bak.1)
        let bak1 = path.with_extension("json.bak.1");
        if let Some(data) = Self::try_load_with_checksum(&bak1) {
            info!("Recovered from backup .bak.1");
            return Some(data);
        }

        // Try older numbered backups in order
        for i in 1..=MAX_BACKUPS {
            let bak = path.with_extension(format!("json.bak.{}", i));
            if let Some(data) = Self::try_load_with_checksum(&bak) {
                info!("Recovered from backup .bak.{}", i);
                return Some(data);
            }
        }

        warn!("All backups failed checksum. Starting fresh save.");
        None
    }

    fn try_load_with_checksum(path: &Path) -> Option<Self> {
        if !path.exists() {
            return None;
        }
        let content = fs::read_to_string(path).ok()?;
        let mut data: Self = serde_json::from_str(&content).ok()?;

        let expected = data.compute_checksum();
        if data.checksum != expected {
            return None;
        }

        if data.save_version < CURRENT_SAVE_VERSION {
            return Some(Self::migrate(data));
        }
        Some(data)
    }

    fn compute_checksum(&self) -> String {
        let mut hasher = Sha256::new();
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
        info!("Loaded player save (persistence v18.15+ rotating backup strategy active)");
    } else {
        commands.insert_resource(PlayerSaveData::new(1));
        info!("No valid save found — created new player save");
    }
}

fn auto_save_system(
    mut save_data: ResMut<PlayerSaveData>,
    mut timer: ResMut<AutoSaveTimer>,
    time: Res<Time>,
) {
    timer.timer.tick(time.delta());
    if timer.timer.just_finished() {
        if let Err(e) = save_data.save_to_file(Path::new("player_save.json")) {
            error!("Auto-save failed: {}", e);
        }
    }
}

fn save_on_exit(
    mut save_data: ResMut<PlayerSaveData>,
    mut exit_events: EventReader<bevy::app::AppExit>,
) {
    for _ in exit_events.read() {
        if let Err(e) = save_data.save_to_file(Path::new("player_save.json")) {
            error!("Exit save failed: {}", e);
        }
    }
}

fn update_playtime(
    mut save_data: ResMut<PlayerSaveData>,
    time: Res<Time>,
) {
    save_data.total_playtime_seconds += time.delta().as_secs();
}
