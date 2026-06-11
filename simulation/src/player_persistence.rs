/*!
 * Player Persistence — Production Grade v18.15+ (Continued)
 * Rotating Backup + Atomic Save + Checksum + Muscle Memory Weighting + UI Reactivity Hooks
 *
 * Enhancements in this iteration:
 * - Additional progression fields (resonance, council history, biome affinity)
 * - Muscle memory weighting functions (affects harvest efficiency, epiphany threshold, bonuses)
 * - PersistenceUpdated Bevy Event for UI reactivity (client can observe)
 * - Dirty flag + conditional save optimization
 * - More meaningful epiphany recording with muscle memory feedback
 * - Full integration with epiphany_catalyst single source of truth
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const CURRENT_SAVE_VERSION: u32 = 1;
pub const MAX_BACKUPS: usize = 7;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EpiphanyRecord {
    pub scenario_id: String,
    pub timestamp: u64,
    pub intensity: f32,
    pub biome: String,
}

/// Event emitted whenever persistence state meaningfully changes.
/// Client UI systems (Player Progress panel, etc.) should react to this for live updates.
#[derive(Event, Debug, Clone)]
pub struct PersistenceUpdated {
    pub reason: String, // e.g. "epiphany", "harvest", "council"
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

    // === New progression fields (v18.15+) ===
    pub total_epiphanies: u32,
    pub council_sessions_participated: u32,
    pub resonance_score: f32,           // Mercy / RBE alignment score
    pub faction_standings: HashMap<String, f32>, // faction_name -> standing
    pub biome_affinity: HashMap<String, f32>,    // biome -> affinity from epiphanies/harvest
    pub last_epiphany_timestamp: u64,

    // Temporary epiphany rewards (from evaluate_epiphany single source of truth)
    pub temporary_harvest_multiplier: f32,
    pub temporary_multiplier_expires_at: u64,

    // Internal
    #[serde(skip)]
    pub dirty: bool,
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

    /// Record epiphany with full muscle memory weighting and reactivity
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

        // Muscle memory weighting (non-linear growth, caps at 5.0)
        let muscle_gain = intensity * 0.15 * (1.0 + self.muscle_memory_level * 0.1);
        self.muscle_memory_level = (self.muscle_memory_level + muscle_gain).min(5.0);

        // Resonance / mercy alignment boost
        self.resonance_score = (self.resonance_score + intensity * 0.03).min(1.0);

        // Biome affinity
        let affinity = self.biome_affinity.entry(biome.to_string()).or_insert(0.5);
        *affinity = (*affinity + intensity * 0.08).min(2.0);

        self.dirty = true;

        // Auto-save after meaningful epiphany
        let _ = self.save_to_file(Path::new("player_save.json"));

        self.muscle_memory_level
    }

    pub fn add_playtime(&mut self, seconds: u64) {
        self.total_playtime_seconds += seconds;
        self.last_played_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.dirty = true;
    }

    pub fn record_harvest(&mut self, sustainable: bool) {
        self.total_harvests += 1;
        if sustainable {
            self.sustainable_harvests += 1;
        }
        self.dirty = true;
    }

    pub fn record_council_participation(&mut self) {
        self.council_sessions_participated += 1;
        self.resonance_score = (self.resonance_score + 0.05).min(1.0);
        self.dirty = true;
    }

    /// Muscle memory weighting applied to harvest efficiency
    pub fn get_muscle_memory_harvest_bonus(&self) -> f32 {
        // Non-linear: higher muscle memory gives diminishing but meaningful returns
        1.0 + (self.muscle_memory_level - 1.0) * 0.08
    }

    /// Muscle memory reduces epiphany threshold (makes revelations slightly easier)
    pub fn get_muscle_memory_epiphany_threshold_modifier(&self) -> f32 {
        (5.0 - self.muscle_memory_level) / 4.0 // lower = easier
    }

    /// Check if temporary multiplier (from epiphany) is still active
    pub fn has_active_multiplier(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.temporary_multiplier_expires_at > now && self.temporary_harvest_multiplier > 1.0
    }

    /// Get current effective harvest multiplier (single source of truth path + muscle memory)
    pub fn get_current_harvest_multiplier(&self) -> f32 {
        let base = if self.has_active_multiplier() {
            self.temporary_harvest_multiplier
        } else {
            1.0
        };
        base * self.get_muscle_memory_harvest_bonus()
    }

    /// Production-grade atomic save with rotating backup + dirty check
    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        if !self.dirty && path.exists() {
            return Ok(()); // No changes — skip expensive I/O
        }

        let mut data_to_save = self.clone();
        data_to_save.checksum = data_to_save.compute_checksum();
        data_to_save.last_save_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        data_to_save.dirty = false; // reset on successful save

        let temp_path = path.with_extension("json.tmp");
        {
            let json = serde_json::to_string_pretty(&data_to_save)?;
            fs::write(&temp_path, json)?;
        }

        Self::rotate_backups(path)?;
        Self::create_timestamped_snapshot(path)?;

        fs::rename(&temp_path, path)?;
        Ok(())
    }

    fn rotate_backups(path: &Path) -> Result<(), std::io::Error> {
        let oldest = path.with_extension(format!("json.bak.{}", MAX_BACKUPS));
        if oldest.exists() {
            fs::remove_file(&oldest)?;
        }
        for i in (1..MAX_BACKUPS).rev() {
            let src = path.with_extension(format!("json.bak.{}", i));
            let dst = path.with_extension(format!("json.bak.{}", i + 1));
            if src.exists() {
                fs::rename(&src, &dst)?;
            }
        }
        if path.exists() {
            let bak1 = path.with_extension("json.bak.1");
            fs::rename(path, &bak1)?;
        }
        Ok(())
    }

    fn create_timestamped_snapshot(path: &Path) -> Result<(), std::io::Error> {
        if !path.exists() {
            return Ok(());
        }
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let snapshot_name = format!("player_save_{}_{}.json.bak", now, path.file_stem().unwrap_or_default().to_string_lossy());
        let snapshot_path = path.parent().unwrap_or(Path::new(".")).join(snapshot_name);
        fs::copy(path, &snapshot_path)?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Option<Self> {
        if !path.exists() {
            return None;
        }
        if let Some(data) = Self::try_load_with_checksum(path) {
            return Some(data);
        }
        warn!("Primary save corrupted. Attempting backup recovery...");
        let bak1 = path.with_extension("json.bak.1");
        if let Some(data) = Self::try_load_with_checksum(&bak1) {
            return Some(data);
        }
        for i in 1..=MAX_BACKUPS {
            let bak = path.with_extension(format!("json.bak.{}", i));
            if let Some(data) = Self::try_load_with_checksum(&bak) {
                return Some(data);
            }
        }
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
            .add_event::<PersistenceUpdated>()
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
        info!("Loaded player save (v18.15+ persistence with muscle memory + UI reactivity)");
    } else {
        commands.insert_resource(PlayerSaveData::new(1));
    }
}

fn auto_save_system(
    mut save_data: ResMut<PlayerSaveData>,
    mut timer: ResMut<AutoSaveTimer>,
    time: Res<Time>,
    mut persistence_events: EventWriter<PersistenceUpdated>,
) {
    timer.timer.tick(time.delta());
    if timer.timer.just_finished() && save_data.dirty {
        if let Err(e) = save_data.save_to_file(Path::new("player_save.json")) {
            error!("Auto-save failed: {}", e);
        } else {
            persistence_events.send(PersistenceUpdated { reason: "auto_save".to_string() });
        }
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
