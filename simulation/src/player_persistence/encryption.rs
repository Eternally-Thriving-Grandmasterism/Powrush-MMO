/*!
 * Encryption Abstraction Layer
 *
 * v19.3.23: Added HybridClassicalMLKEMEncryptor sketch for future post-quantum work.
 *
 * This represents a hybrid classical + post-quantum approach:
 * - X25519 (classical) + ML-KEM (post-quantum) for key encapsulation
 * - HKDF for final key derivation
 * - ChaCha20-Poly1305 or future AEAD for encryption
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use std::error::Error;

/// Trait for pluggable save file encryption.
pub trait SaveEncryptor {
    fn encrypt(&self, plaintext: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn Error>>;
    fn decrypt(&self, ciphertext: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn Error>>;
}

/// Current production implementation using Argon2id + HKDF + ChaCha20-Poly1305.
pub struct CurrentEncryptor;

impl SaveEncryptor for CurrentEncryptor {
    fn encrypt(&self, plaintext: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        crate::player_persistence::save::encrypt_impl(plaintext, password)
    }

    fn decrypt(&self, ciphertext: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        crate::player_persistence::save::decrypt_impl(ciphertext, password)
    }
}

/// Placeholder for future hybrid classical + post-quantum encryption.
///
/// Intended design:
/// - Perform both X25519 and ML-KEM key encapsulation
/// - Combine the two shared secrets
/// - Derive final symmetric key using HKDF
/// - Use authenticated encryption (ChaCha20-Poly1305 or future AEAD)
///
/// This provides security even if one of the algorithms is broken.
pub struct HybridClassicalMLKEMEncryptor;

impl SaveEncryptor for HybridClassicalMLKEMEncryptor {
    fn encrypt(&self, _plaintext: &[u8], _password: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        Err(
            "HybridClassicalMLKEMEncryptor is a placeholder. \
             Future implementation will combine X25519 + ML-KEM for hybrid key establishment."
                .into(),
        )
    }

    fn decrypt(&self, _ciphertext: &[u8], _password: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        Err(
            "HybridClassicalMLKEMEncryptor is a placeholder. \
             Future implementation will combine X25519 + ML-KEM for hybrid key establishment."
                .into(),
        )
    }
}

// Example future usage pattern:
// let encryptor: Box<dyn SaveEncryptor> = if post_quantum_enabled {
//     Box::new(HybridClassicalMLKEMEncryptor)
// } else {
//     Box::new(CurrentEncryptor)
// };
