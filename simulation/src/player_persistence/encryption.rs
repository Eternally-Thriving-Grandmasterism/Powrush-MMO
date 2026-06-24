/*!
 * Encryption Abstraction Layer
 *
 * v19.3.21: Created to allow future algorithm swaps (including post-quantum).
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use std::error::Error;

/// Trait for pluggable save file encryption.
/// This allows us to swap the underlying algorithm (e.g. to post-quantum hybrids)
/// without changing the rest of the persistence layer.
pub trait SaveEncryptor {
    /// Encrypt plaintext using the provided password/secret.
    fn encrypt(&self, plaintext: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn Error>>;

    /// Decrypt ciphertext using the provided password/secret.
    fn decrypt(&self, ciphertext: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn Error>>;
}

/// Current production implementation using Argon2id + HKDF + ChaCha20-Poly1305.
pub struct CurrentEncryptor;

impl SaveEncryptor for CurrentEncryptor {
    fn encrypt(&self, plaintext: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        // Delegate to the existing implementation in save.rs for now.
        // In the future this can be moved here completely.
        crate::player_persistence::save::encrypt_impl(plaintext, password)
    }

    fn decrypt(&self, ciphertext: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        crate::player_persistence::save::decrypt_impl(ciphertext, password)
    }
}

// Future example:
// pub struct PostQuantumEncryptor;
// impl SaveEncryptor for PostQuantumEncryptor { ... }
