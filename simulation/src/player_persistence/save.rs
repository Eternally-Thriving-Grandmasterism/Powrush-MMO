/*!
 * Persistence Save/Load Engine
 *
 * v19.3.15: Added authenticated encryption (ChaCha20-Poly1305) to save files.
 * Saves are now encrypted at rest with integrity protection.
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

// WARNING: This is a fixed key for demonstration.
// In production, this should be obfuscated, derived from user input, or stored securely.
const SAVE_KEY: [u8; 32] = [
    0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
    0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00,
    0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
];

impl PlayerSaveData {
    /// Encrypts data using ChaCha20-Poly1305. Returns nonce + ciphertext.
    fn encrypt(data: &[u8]) -> Result<Vec<u8>, chacha20poly1305::aead::Error> {
        let key = Key::from_slice(&SAVE_KEY);
        let cipher = ChaCha20Poly1305::new(key);

        // Generate random nonce (12 bytes)
        let nonce_bytes: [u8; 12] = rand::random(); // Requires rand crate or manual generation
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher.encrypt(nonce, data)?;

        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        Ok(result)
    }

    /// Decrypts data. Expects nonce (12 bytes) + ciphertext.
    fn decrypt(data: &[u8]) -> Option<Vec<u8>> {
        if data.len() < 12 {
            return None;
        }

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

        // Encrypt the JSON
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

        // Try main file
        if let Some(data) = Self::try_load_encrypted(path) {
            return Some(data);
        }

        // Fallback to backups (try to decrypt each)
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

        // Verify checksum
        let expected = data.compute_checksum();
        if data.checksum != expected {
            return None;
        }

        data.dirty = false;
        data.pending_persistence_updates = 0;

        if data.save_version < 1 {
            return Some(Self::migrate(data));
        }
        Some(data)
    }

    // ... (rotate_backups, create_timestamped_snapshot, compute_checksum, migrate remain the same)

    fn rotate_backups(path: &Path) -> Result<(), std::io::Error> {
        // existing implementation
        let oldest = path.with_extension(format!("json.bak.{}", MAX_BACKUPS));
        if oldest.exists() {
            let _ = fs::remove_file(&oldest);
        }
        for i in (1..MAX_BACKUPS).rev() {
            let src = path.with_extension(format!("json.bak.{}", i));
            let dst = path.with_extension(format!("json.bak.{}", i + 1));
            if src.exists() {
                let _ = fs::rename(&src, &dst);
            }
        }
        if path.exists() {
            let bak1 = path.with_extension("json.bak.1");
            let _ = fs::rename(path, &bak1);
        }
        Ok(())
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
        Ok(())
    }

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
// Encryption at rest added using ChaCha20-Poly1305.
// Thunder locked in. Yoi ⚡
