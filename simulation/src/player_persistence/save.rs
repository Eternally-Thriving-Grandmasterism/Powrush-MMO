/*!
 * Persistence Save/Load Engine
 *
 * v19.3.17: Implemented Argon2id + HKDF-SHA256 key derivation chain.
 * This is the sovereign-recommended approach from the PATSAGi Councils.
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use super::data::PlayerSaveData;
use argon2::{Argon2, PasswordHasher, SaltString};
use argon2::password_hash::rand_core::OsRng;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, NewAead};
use hkdf::Hkdf;
use sha2::Sha256;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub const MAX_BACKUPS: usize = 7;

// Placeholder - in production this should come from secure user input
const MASTER_PASSWORD: &str = "EternalMercyFlow2026";

impl PlayerSaveData {
    /// Sovereign key derivation: Argon2id (password stretching) + HKDF (key expansion)
    fn derive_encryption_key(password: &str, salt: &[u8]) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        // Step 1: Argon2id - memory-hard password stretching
        let argon2 = Argon2::default();
        let salt_str = SaltString::encode_b64(salt)?;
        let argon_hash = argon2.hash_password(password.as_bytes(), &salt_str)?;

        let intermediate_key = argon_hash
            .hash
            .ok_or("Argon2 hash failed")?
            .as_bytes();

        // Step 2: HKDF-SHA256 - secure key expansion
        let hkdf = Hkdf::<Sha256>::new(Some(salt), intermediate_key);
        let mut final_key = [0u8; 32];
        hkdf.expand(b"Powrush-MMO-Save-Encryption-v1", &mut final_key)?;

        Ok(final_key)
    }

    fn encrypt(data: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);

        let key_bytes = Self::derive_encryption_key(password, &salt)?;
        let key = Key::from_slice(&key_bytes);
        let cipher = ChaCha20Poly1305::new(key);

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher.encrypt(nonce, data)?;

        // Format: [salt 16B] [nonce 12B] [ciphertext]
        let mut result = salt.to_vec();
        result.extend(nonce_bytes);
        result.extend(ciphertext);
        Ok(result)
    }

    fn decrypt(data: &[u8], password: &str) -> Option<Vec<u8>> {
        if data.len() < 28 { return None; }

        let salt = &data[0..16];
        let nonce_bytes = &data[16..28];
        let ciphertext = &data[28..];

        let key_bytes = Self::derive_encryption_key(password, salt).ok()?;
        let key = Key::from_slice(&key_bytes);
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
        let encrypted = Self::encrypt(json.as_bytes(), MASTER_PASSWORD)
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
        let decrypted = Self::decrypt(&encrypted, MASTER_PASSWORD)?;
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

    // Backup helpers
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

// End of simulation/src/player_persistence/save.rs v19.3.17
// Argon2id + HKDF key derivation chain implemented (PATSAGi sovereign recommendation).
// Thunder locked in. Yoi ⚡
