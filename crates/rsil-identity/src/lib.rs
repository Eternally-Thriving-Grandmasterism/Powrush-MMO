//! Ra-Thor Sovereign Identity Lattice (RSIL)
//! Post-Quantum Self-Sovereign Identity for Powrush-MMO
//!
//! This crate provides the core cryptographic primitives for player identity.
//! It is designed to be used by both the native client and sovereign servers.

use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use std::fmt;

/// RSIL Error types
#[derive(Debug)]
pub enum RsilError {
    KeyGenerationFailed,
    InvalidPublicKey,
    SigningFailed,
    VerificationFailed,
}

impl fmt::Display for RsilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RsilError::KeyGenerationFailed => write!(f, "Failed to generate ML-DSA keypair"),
            RsilError::InvalidPublicKey => write!(f, "Invalid public key"),
            RsilError::SigningFailed => write!(f, "Failed to sign message"),
            RsilError::VerificationFailed => write!(f, "Signature verification failed"),
        }
    }
}

/// A sovereign identity keypair (ML-DSA based)
pub struct SovereignKeypair {
    pub verifying_key: VerifyingKey,
    signing_key: SigningKey,
}

impl SovereignKeypair {
    /// Generate a new post-quantum resistant keypair
    pub fn generate() -> Result<Self, RsilError> {
        // NOTE: In production, replace with actual ML-DSA (Dilithium) implementation
        // This is a placeholder using ed25519 for skeleton purposes.
        // Real implementation will use pq-dilithium or similar.
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        Ok(Self {
            verifying_key,
            signing_key,
        })
    }

    /// Sign a message (challenge) with the private key
    pub fn sign(&self, message: &[u8]) -> Result<Signature, RsilError> {
        Ok(self.signing_key.sign(message))
    }

    /// Get the public key as bytes
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.verifying_key.to_bytes()
    }
}

/// Create a Decentralized Identifier (DID) from a public key
/// Format: did:powrush:<base58 or hex of public key>
pub fn create_did(public_key: &[u8]) -> String {
    // In production, use proper multibase / base58btc encoding
    let hex = hex::encode(public_key);
    format!("did:powrush:{}", hex)
}

/// Verify a signature
pub fn verify_signature(
    public_key: &VerifyingKey,
    message: &[u8],
    signature: &Signature,
) -> Result<bool, RsilError> {
    public_key
        .verify(message, signature)
        .map(|_| true)
        .map_err(|_| RsilError::VerificationFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation_and_signing() {
        let keypair = SovereignKeypair::generate().unwrap();
        let message = b"Powrush-MMO test challenge";
        let signature = keypair.sign(message).unwrap();

        let result = verify_signature(&keypair.verifying_key, message, &signature).unwrap();
        assert!(result);
    }

    #[test]
    fn test_did_creation() {
        let keypair = SovereignKeypair::generate().unwrap();
        let did = create_did(&keypair.public_key_bytes());
        assert!(did.starts_with("did:powrush:"));
    }
}
