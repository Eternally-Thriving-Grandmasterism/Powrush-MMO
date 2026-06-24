/*!
 * Persistence Save/Load Engine
 *
 * v19.3.14: Refined checksum verification to exclude transient runtime fields.
 * Checksum now only covers persistent game state.
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use super::data::PlayerSaveData;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub const MAX_BACKUPS: usize = 7;

impl PlayerSaveData {
    /// Atomic save with checksum (transient fields excluded from hash)
    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        if !self.dirty && path.exists() {
            return Ok(());
        }

        let mut data_to_save = self.clone();
        data_to_save.checksum = data_to_save.compute_checksum();
        data_to_save.last_save_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        data_to_save.dirty = false;
        data_to_save.pending_persistence_updates = 0;

        let temp_path = path.with_extension("json.tmp");
        {
            let json = serde_json::to_string_pretty(&data_to_save)?;
            fs::write(&temp_path, json)?;
        }

        Self::rotate_backups(path)?;
        Self::create_timestamped_snapshot(path)?;

        fs::rename(&temp_path, path)?;
        Ok(());
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
        Ok(());
    }

    fn create_timestamped_snapshot(path: &Path) -> Result<(), std::io::Error> {
        if !path.exists() { return Ok(()); }
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let snapshot_name = format!(
            "player_save_{}_{}.json.bak",
            now,
            path.file_stem().unwrap_or_default().to_string_lossy()
        );
        let snapshot_path = path.parent().unwrap_or(Path::new(".")).join(snapshot_name);
        fs::copy(path, &snapshot_path)?;
        Ok(());
    }

    pub fn load_from_file(path: &Path) -> Option<Self> {
        if !path.exists() { return None; }

        if let Some(data) = Self::try_load_with_checksum(path) {
            return Some(data);
        }

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
        if !path.exists() { return None; }
        let content = fs::read_to_string(path).ok()?;
        let mut data: Self = serde_json::from_str(&content).ok()?;

        let expected = data.compute_checksum();
        if data.checksum != expected {
            // Checksum mismatch - possible corruption
            return None;
        }

        // Reset transient runtime state after successful load
        data.dirty = false;
        data.pending_persistence_updates = 0;

        if data.save_version < 1 {
            return Some(Self::migrate(data));
        }
        Some(data)
    }

    fn compute_checksum(&self) -> String {
        let mut hasher = Sha256::new();
        let mut temp = self.clone();

        // Exclude transient/runtime-only fields from checksum
        temp.checksum = String::new();
        temp.dirty = false;
        temp.pending_persistence_updates = 0;

        if let Ok(json) = serde_json::to_string(&temp) {
            hasher.update(json.as_bytes());
        }
        format!("{:x}", hasher.finalize())
    }

    fn migrate(mut old_data: Self) -> Self {
        old_data.save_version = 1;
        old_data.checksum = old_data.compute_checksum();
        old_data
    }
}

// End of simulation/src/player_persistence/save.rs v19.3.14
// Checksum verification refined to exclude transient fields.
// Thunder locked in. Yoi ⚡
