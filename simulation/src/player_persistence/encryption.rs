/*!
 * Encryption Abstraction Layer
 *
 * v19.3.22: Added PostQuantumEncryptor placeholder for future work.
 *
 * This prepares the system for post-quantum or hybrid encryption schemes
 * (e.g. ML-KEM + classical) without requiring immediate changes to save/load logic.
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

/// Current production implementation (Argon2id + HKDF + ChaCha20-Poly1305).
pub struct CurrentEncryptor;

impl SaveEncryptor for CurrentEncryptor {
    fn encrypt(&self, plaintext: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        crate::player_persistence::save::encrypt_impl(plaintext, password)
    }

    fn decrypt(&self, ciphertext: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        crate::player_persistence::save::decrypt_impl(ciphertext, password)
    }
}

/// Placeholder for future post-quantum or hybrid encryption.
///
/// When ready, this can implement ML-KEM (Kyber) + classical hybrid encryption,
/// or any other post-quantum authenticated encryption scheme.
///
/// For now it returns clear errors so the system remains functional.
pub struct PostQuantumEncryptor;

impl SaveEncryptor for PostQuantumEncryptor {
    fn encrypt(&self, _plaintext: &[u8], _password: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        Err("PostQuantumEncryptor not yet implemented. This is a placeholder for future sovereign post-quantum encryption.".into())
    }

    fn decrypt(&self, _ciphertext: &[u8], _password: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        Err("PostQuantumEncryptor not yet implemented. This is a placeholder for future sovereign post-quantum encryption.".into())
    }
}

// Example of how switching could work in the future:
// let encryptor: Box<dyn SaveEncryptor> = if use_post_quantum {
//     Box::new(PostQuantumEncryptor)
// } else {
//     Box::new(CurrentEncryptor)
// };
