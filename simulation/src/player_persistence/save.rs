/*!
 * Persistence Save/Load Engine
 *
 * v19.3.15: Added authenticated encryption (ChaCha20-Poly1305) to save files.
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use super::data::PlayerSaveData;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, NewAead};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub const MAX_BACKUPS: usize = 7;

// WARNING: Fixed key for demonstration. Should be properly protected in production.
const SAVE_KEY: [u8; 32] = [0u8; 32]; // TODO: Replace with secure key

impl PlayerSaveData {
    fn encrypt(data: &[u8]) -> Result<Vec<u8>, chacha20poly1305::aead::Error> {
        let key = Key::from_slice(&SAVE_KEY);
        let cipher = ChaCha20Poly1305::new(key);

        // Simple nonce from timestamp (for demo; use random in production)
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let nonce_bytes: [u8; 12] = now.to_le_bytes()[..12].try_into().unwrap_or([0u8; 12]);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher.encrypt(nonce, data)?;

        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        Ok(result)
    }

    fn decrypt(data: &[u8]) -> Option<Vec<u8>> {
        if data.len() < 12 { return None; }
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let key = Key::from_slice(&SAVE_KEY);
        let cipher = ChaCha20Poly1305::new(key);
        let nonce = Nonce::from_slice(nonce_bytes);

        cipher.decrypt(nonce, ciphertext).ok()
    }

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

        let json = serde_json::to_string_pretty(&data_to_save)?;
        let encrypted = Self::encrypt(json.as_bytes())
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Encryption failed"))?;

        let temp_path = path.with_extension("json.tmp");
        fs::write(&temp_path, encrypted)?;

        Self::rotate_backups(path)?;
        Self::create_timestamped_snapshot(path)?;

        fs::rename(&temp_path, path)?;
        Ok(());
    }

    pub fn load_from_file(path: &Path) -> Option<Self> {
        if !path.exists() { return None; }

        if let Some(data) = Self::try_load_encrypted(path) {
            return Some(data);
        }

        let bak1 = path.with_extension("json.bak.1");
        if let Some(data) = Self::try_load_encrypted(&bak1) {
            return Some(data);
        }

        for i in 1..=MAX_BACKUPS {
            let bak = path.with_extension(format!("json.bak.{}", i));
            if let Some(data) = Self::try_load_encrypted(&bak) {
                return Some(data);
            }
        }
        None
    }

    fn try_load_encrypted(path: &Path) -> Option<Self> {
        if !path.exists() { return None; }
        let encrypted = fs::read(path).ok()?;
        let decrypted = Self::decrypt(&encrypted)?;
        let json_str = String::from_utf8(decrypted).ok()?;
        let mut data: Self = serde_json::from_str(&json_str).ok()?;

        let expected = data.compute_checksum();
        if data.checksum != expected { return None; }

        data.dirty = false;
        data.pending_persistence_updates = 0;

        if data.save_version < 1 {
            return Some(Self::migrate(data));
        }
        Some(data)
    }

    // Backup and other helper methods (abbreviated for clarity)
    fn rotate_backups(_path: &Path) -> Result<(), std::io::Error> { Ok(()) }
    fn create_timestamped_snapshot(_path: &Path) -> Result<(), std::io::Error> { Ok(()) }

    fn compute_checksum(&self) -> String {
        let mut hasher = Sha256::new();
        let mut temp = self.clone();
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

// End of simulation/src/player_persistence/save.rs v19.3.15
// Encryption at rest with ChaCha20-Poly1305 added.
// Thunder locked in. Yoi ⚡
