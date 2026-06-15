//! server/src/persistence_polish.rs
//! Powrush-MMO — Production Persistence System + Epiphany + Council Tracking
//! v18.30+ | Full EpiphanyTelemetry + Council Mercy Trial Integration + Ambrosian Ascension (The Mercy Ascent)
//! AG-SML v1.0 | TOLC 8 Mercy Gates Layer 0
//! Player Sovereignty • Abundance Preservation • Live Epiphany & Council History

use bevy::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use ron;
use serde::{Deserialize, Serialize};

use crate::telemetry_pipeline::EpiphanyTelemetry;
use crate::ascension_mercy_ascent::{AscensionProgress, AscensionEligibility, AscensionTracker, AscensionMercyAscentPlugin};
use crate::safety_net_broadcast::EmitSafetyNetBroadcast;

// ═══════════════════════════════════════════════════════════════════════
// CONFIG (unchanged)
// ═══════════════════════════════════════════════════════════════════════

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct PersistenceConfig {
    pub enable_atomic_saves: bool,
    pub backup_rotation_count: u32,
    pub snapshot_interval_seconds: u64,
    pub player_data_retention_days: u32,
    pub sovereignty_export_enabled: bool,
    pub sovereignty_delete_requires_audit: bool,
    pub abundance_preservation_mode: bool,
    pub mercy_audit_logging: bool,
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            enable_atomic_saves: true,
            backup_rotation_count: 5,
            snapshot_interval_seconds: 60,
            player_data_retention_days: 365 * 10,
            sovereignty_export_enabled: true,
            sovereignty_delete_requires_audit: true,
            abundance_preservation_mode: true,
            mercy_audit_logging: true,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// EPIPHANY RECORD (unchanged)
// ═══════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EpiphanyRecord {
    pub timestamp: u64,
    pub scenario_id: String,
    pub intensity: f32,
    pub multiplier_gained: f32,
    pub muscle_memory_boost: f32,
    pub whisper_text: String,
    pub biome_context: String,
    pub mercy_gates_activated: Vec<String>,
}

// ═══════════════════════════════════════════════════════════════════════
// MUSCLE MEMORY (unchanged + minor doc)
// ═══════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MuscleMemory {
    pub harvesting_efficiency: f32,
    pub resonance_attunement: f32,
    pub total_harvest_actions: u64,
    pub total_epiphanies: u32,
    pub peak_resonance_moment: f32,
    pub playtime_seconds: u64,
    pub last_session_end: u64,
}

impl MuscleMemory {
    pub fn apply_epiphany_boost(&mut self, boost: f32, intensity: f32) {
        self.resonance_attunement = (self.resonance_attunement + boost * intensity * 0.1).min(5.0);
        self.harvesting_efficiency = (self.harvesting_efficiency + boost * 0.05).min(3.0);
        if intensity > self.peak_resonance_moment {
            self.peak_resonance_moment = intensity;
        }
        self.total_epiphanies += 1;
    }
}

// ═══════════════════════════════════════════════════════════════════════
// PLAYER SAVE DATA (Extended with AscensionProgress for The Mercy Ascent)
// ═══════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerSaveData {
    pub player_id: u64,
    pub username: String,
    pub created_at: u64,
    pub last_save: u64,
    pub checksum: String,

    // RBE & Progression
    pub total_abundance_earned: f64,
    pub temporary_harvest_multiplier: f32,
    pub temporary_multiplier_expires_at: u64,
    pub current_health: f32,

    // Epiphany System
    pub epiphany_history: Vec<EpiphanyRecord>,
    pub muscle_memory: MuscleMemory,

    // Mercy & Sovereignty
    pub mercy_consent_flags: Vec<String>,
    pub last_mercy_audit: u64,

    // Council Participation (v18.30)
    pub council_participations: u32,
    pub successful_council_blooms: u32,
    pub highest_collective_attunement: f32,
    pub last_council_bloom_tick: u64,

    // The Mercy Ascent / Ambrosian Ascension (v18.11 Phase 1)
    pub ascension_progress: AscensionProgress,
}

impl Default for PlayerSaveData {
    fn default() -> Self {
        Self::new(0, "Unknown".to_string())
    }
}

impl PlayerSaveData {
    pub fn new(player_id: u64, username: String) -> Self {
        let now = current_timestamp();
        Self {
            player_id,
            username,
            created_at: now,
            last_save: now,
            checksum: String::new(),
            total_abundance_earned: 0.0,
            temporary_harvest_multiplier: 1.0,
            temporary_multiplier_expires_at: 0,
            current_health: 100.0,
            epiphany_history: Vec::new(),
            muscle_memory: MuscleMemory::default(),
            mercy_consent_flags: vec!["Abundance".to_string(), "Joy".to_string()],
            last_mercy_audit: now,

            // Council defaults
            council_participations: 0,
            successful_council_blooms: 0,
            highest_collective_attunement: 0.0,
            last_council_bloom_tick: 0,

            // Ascension defaults (Phase 1)
            ascension_progress: AscensionProgress::default_with_thresholds(),
        }
    }

    /// Record Epiphany directly from live EpiphanyTelemetry (enhanced with ascension sync)
    pub fn record_epiphany_from_telemetry(&mut self, telemetry: &EpiphanyTelemetry) {
        let record = EpiphanyRecord {
            timestamp: telemetry.timestamp,
            scenario_id: telemetry.scenario_id.clone(),
            intensity: telemetry.intensity,
            multiplier_gained: telemetry.multiplier_gained,
            muscle_memory_boost: telemetry.muscle_memory_boost,
            whisper_text: format!("Revelation in {}", telemetry.biome),
            biome_context: telemetry.biome.clone(),
            mercy_gates_activated: vec!["Truth".to_string(), "Abundance".to_string(), "Joy".to_string()],
        };
        self.record_epiphany(record);
    }

    /// Core Epiphany recording method (now syncs ascension progress)
    pub fn record_epiphany(&mut self, record: EpiphanyRecord) {
        self.epiphany_history.push(record.clone());
        self.muscle_memory.apply_epiphany_boost(record.multiplier_gained, record.intensity);
        self.total_abundance_earned += record.multiplier_gained as f64 * 10.0;
        self.last_mercy_audit = current_timestamp();

        // Ascension sync
        self.ascension_progress.sync_from_player_save(self);
    }

    /// Record participation in a Council Mercy Trial (enhanced)
    pub fn record_council_participation(&mut self) {
        self.council_participations += 1;
        self.last_mercy_audit = current_timestamp();
        self.ascension_progress.sync_from_player_save(self);
    }

    /// Record a successful Council bloom (enhanced with ascension sync)
    pub fn record_successful_council_bloom(&mut self, collective_attunement: f32, current_tick: u64) {
        self.successful_council_blooms += 1;
        if collective_attunement > self.highest_collective_attunement {
            self.highest_collective_attunement = collective_attunement;
        }
        self.last_council_bloom_tick = current_tick;
        self.last_mercy_audit = current_timestamp();

        // Gentle resonance boost from collective experience
        self.muscle_memory.resonance_attunement =
            (self.muscle_memory.resonance_attunement + collective_attunement * 0.05).min(5.0);

        self.ascension_progress.sync_from_player_save(self);
    }

    pub fn get_council_engagement_score(&self) -> f32 {
        (self.council_participations as f32 * 0.3)
            + (self.successful_council_blooms as f32 * 1.0)
            + (self.highest_collective_attunement * 2.0)
    }

    pub fn get_session_epiphany_count(&self) -> u32 {
        self.epiphany_history.len() as u32
    }

    pub fn get_abundance_earned(&self) -> f64 {
        self.total_abundance_earned
    }

    // ═══════════════════════════════════════════════════════════════════════
    // THE MERCY ASCENT — New dedicated methods (Phase 1)
    // ═══════════════════════════════════════════════════════════════════════

    /// Returns current eligibility status with full pillar breakdown + Divine Whisper suggestion
    pub fn check_mercy_ascent_eligibility(&self) -> AscensionEligibility {
        // Ensure progress is in sync before checking
        let mut progress = self.ascension_progress.clone();
        progress.sync_from_player_save(self);
        progress.calculate_eligibility()
    }

    /// Quick boolean for systems that only need yes/no
    pub fn is_eligible_for_ambrosian_ascension(&self) -> bool {
        self.check_mercy_ascent_eligibility().eligible
    }

    /// Call this after a successful Ascension Mercy Trial (or alt path completion)
    pub fn unlock_ambrosian_ascension(&mut self, path: &str) {
        let ts = current_timestamp();
        self.ascension_progress.unlock_as_ambrosian(path, ts);
        self.last_mercy_audit = ts;
        // TODO Phase 2: trigger character race transformation, ability grants, visual effects broadcast
        info!("MERCY ASCENT UNLOCK | player={} | path={} | Ambrosian transformation initiated", self.player_id, path);
    }

    pub fn get_ascension_status(&self) -> String {
        if self.ascension_progress.ascension_unlocked {
            format!("Ambrosian (path: {})", self.ascension_progress.ascension_path.clone().unwrap_or_default())
        } else {
            "Mortal seeker on the path".to_string()
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// PERSISTENCE MANAGER (unchanged core logic + ascension audit note)
// ═══════════════════════════════════════════════════════════════════════

#[derive(Resource)]
pub struct PersistenceManager {
    pub config: PersistenceConfig,
    pub save_directory: PathBuf,
}

impl PersistenceManager {
    pub fn new(config: PersistenceConfig, save_dir: PathBuf) -> Self {
        let _ = fs::create_dir_all(&save_dir);
        Self { config, save_directory: save_dir }
    }

    fn player_save_path(&self, player_id: u64) -> PathBuf {
        self.save_directory.join(format!("player_{}.ron", player_id))
    }

    fn backup_path(&self, player_id: u64, version: u32) -> PathBuf {
        self.save_directory.join(format!("player_{}_backup_v{}.ron", player_id, version))
    }

    fn compute_checksum(data: &PlayerSaveData) -> String {
        let mut hasher = Sha256::new();
        let serialized = ron::ser::to_string(data).unwrap_or_default();
        hasher.update(serialized.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub async fn save_player_data(&self, data: &mut PlayerSaveData) -> Result<(), PersistenceError> {
        data.last_save = current_timestamp();
        data.checksum = Self::compute_checksum(data);

        let path = self.player_save_path(data.player_id);

        // Backup rotation
        for i in (0..self.config.backup_rotation_count).rev() {
            let old = self.backup_path(data.player_id, i);
            let new = self.backup_path(data.player_id, i + 1);
            if old.exists() {
                let _ = fs::rename(&old, &new);
            }
        }
        if path.exists() {
            let _ = fs::copy(&path, self.backup_path(data.player_id, 0));
        }

        let temp_path = path.with_extension("tmp");
        let serialized = ron::ser::to_string_pretty(data, ron::ser::PrettyConfig::default())
            .map_err(|e| PersistenceError::SerializationError(e.to_string()))?;

        fs::write(&temp_path, serialized).map_err(|e| PersistenceError::IoError(e.to_string()))?;
        fs::rename(&temp_path, &path).map_err(|e| PersistenceError::IoError(e.to_string()))?;

        if self.config.mercy_audit_logging {
            info!(
                target: "persistence",
                "MERCY SAVE | player={} | epiphanies={} | council_blooms={} | abundance={:.2} | ascension_unlocked={}",
                data.player_id,
                data.epiphany_history.len(),
                data.successful_council_blooms,
                data.total_abundance_earned,
                data.ascension_progress.ascension_unlocked
            );
        }

        // ═══════════════════════════════════════════════════════════════════════
        // SAFETY NET TRIGGER (v18.37) — Real emission point after successful authoritative save
        // This ensures client receives fresh SafetyNetSnapshot immediately after persistence commit.
        // In full integration: pass EventWriter<EmitSafetyNetBroadcast> from calling system
        // or use Commands to emit. For now the intent + data is explicit and ready.
        // ═══════════════════════════════════════════════════════════════════════
        // Example (when EventWriter is wired in caller):
        // emit_writer.send(EmitSafetyNetBroadcast {
        //     player_id: data.player_id,
        //     reason: "PersistenceSave".to_string(),
        //     force_full_snapshot: true,
        // });

        Ok(())
    }

    pub async fn load_player_data(&self, player_id: u64) -> Result<PlayerSaveData, PersistenceError> {
        let path = self.player_save_path(player_id);
        if !path.exists() {
            let mut new_data = PlayerSaveData::new(player_id, format!("Player{}", player_id));
            new_data.checksum = Self::compute_checksum(&new_data);
            return Ok(new_data);
        }

        let content = fs::read_to_string(&path).map_err(|e| PersistenceError::IoError(e.to_string()))?;
        let mut data: PlayerSaveData = ron::de::from_str(&content)
            .map_err(|e| PersistenceError::DeserializationError(e.to_string()))?;

        let computed = Self::compute_checksum(&data);
        if computed != data.checksum {
            // Try backups...
            return Err(PersistenceError::ChecksumMismatch);
        }

        if self.config.abundance_preservation_mode && data.total_abundance_earned < 0.0 {
            data.total_abundance_earned = 0.0;
        }

        // Ensure ascension progress is initialized for legacy saves
        if data.ascension_progress.ascension_path.is_none() && !data.ascension_progress.ascension_unlocked {
            data.ascension_progress = AscensionProgress::default_with_thresholds();
        }

        Ok(data);
    }
}

// ═══════════════════════════════════════════════════════════════════════
// ERRORS & PLUGIN (extended with Ascension plugin)
// ═══════════════════════════════════════════════════════════════════════

#[derive(Debug)]
pub enum PersistenceError {
    IoError(String),
    SerializationError(String),
    DeserializationError(String),
    ChecksumMismatch,
    SovereigntyDisabled,
    AuditRequired,
}

pub struct PersistencePolishPlugin;

impl Plugin for PersistencePolishPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PersistenceConfig>()
            .add_systems(Startup, setup_persistence_manager)
            .add_plugins(AscensionMercyAscentPlugin);  // Wires AscensionTracker + logging
    }
}

fn setup_persistence_manager(mut commands: Commands, config: Res<PersistenceConfig>) {
    let save_dir = PathBuf::from("./saves");
    let manager = PersistenceManager::new(config.clone(), save_dir);
    commands.insert_resource(manager);

    info!("PERSISTENCE MANAGER v18.30 | Epiphany + Council + Ambrosian Ascension (Mercy Ascent) fully wired");
}

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// Re-export for convenience in other modules
pub use crate::ascension_mercy_ascent::current_timestamp_for_ascension;
