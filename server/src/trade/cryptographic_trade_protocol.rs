// server/src/trade/cryptographic_trade_protocol.rs
// Cryptographic Trade Protocol v2.2
// Hybrid Signatures (Ed25519 + Dilithium) + Commitment + Reveal
// Corrected Dilithium public key handling for reliable verification
// AG-SML v1.0 | PATSAGi + Ra-Thor aligned

use crate::trade_system::Trade;
use ed25519_dalek::{Keypair as EdKeypair, Signature as EdSignature, Signer, Verifier};
use pqcrypto_dilithium::dilithium2::{self, PublicKey as DilPublicKey, SecretKey as DilSecretKey, DetachedSignature as DilSignature};
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoTradeError {
    #[error("Key generation failed")]
    KeyGenerationFailed,
    #[error("Signing failed")]
    SigningFailed,
    #[error("Verification failed")]
    VerificationFailed,
    #[error("Commitment mismatch")]
    CommitmentMismatch,
}

/// Cryptographic commitment to a trade offer
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeCommitment {
    pub trade_id: u64,
    pub offeror_id: u64,
    pub commitment_hash: Vec<u8>,
    pub created_at: u64,
}

/// Hybrid signature container (Classical + Post-Quantum)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HybridSignature {
    pub classical_signature: Vec<u8>,   // Ed25519
    pub pq_signature: Vec<u8>,          // Dilithium
}

/// Cryptographically signed and committed trade offer (Hybrid)
#[derive(Clone, Debug)]
pub struct CryptographicTradeOffer {
    pub trade: Trade,
    pub commitment: TradeCommitment,
    pub classical_public_key: Vec<u8>,  // Ed25519 public key
    pub pq_public_key: Vec<u8>,         // Dilithium public key
    pub signature: HybridSignature,
}

/// Main protocol interface
pub trait CryptographicTradeProtocol {
    fn generate_keypair(&self) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>), CryptoTradeError>;

    fn create_signed_offer(
        &self,
        trade: &Trade,
        classical_secret: &[u8],
        classical_public: &[u8],
        pq_secret: &[u8],
        pq_public: &[u8],
    ) -> Result<CryptographicTradeOffer, CryptoTradeError>;

    fn verify_offer(&self, offer: &CryptographicTradeOffer) -> bool;

    fn reveal_and_validate(
        &self,
        offer: &CryptographicTradeOffer,
        revealed_trade: &Trade,
    ) -> Result<(), CryptoTradeError>;
}

/// Hybrid implementation (Ed25519 + Dilithium2)
pub struct HybridTradeProtocol;

impl CryptographicTradeProtocol for HybridTradeProtocol {
    fn generate_keypair(&self) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>), CryptoTradeError> {
        // Classical Ed25519
        let ed_keypair = EdKeypair::generate(&mut rand::rngs::OsRng);
        let classical_pk = ed_keypair.public.to_bytes().to_vec();
        let classical_sk = ed_keypair.secret.to_bytes().to_vec();

        // Post-quantum Dilithium
        let (dil_pk, dil_sk) = dilithium2::keypair();
        let pq_pk = dil_pk.as_bytes().to_vec();
        let pq_sk = dil_sk.as_bytes().to_vec();

        Ok((classical_pk, classical_sk, pq_pk, pq_sk))
    }

    fn create_signed_offer(
        &self,
        trade: &Trade,
        classical_secret: &[u8],
        classical_public: &[u8],
        pq_secret: &[u8],
        pq_public: &[u8],
    ) -> Result<CryptographicTradeOffer, CryptoTradeError> {
        // Reconstruct Ed25519 keypair
        let classical_sk = ed25519_dalek::SecretKey::from_bytes(classical_secret)
            .map_err(|_| CryptoTradeError::SigningFailed)?;
        let classical_pk = ed25519_dalek::PublicKey::from_bytes(classical_public)
            .map_err(|_| CryptoTradeError::SigningFailed)?;
        let ed_keypair = EdKeypair { secret: classical_sk, public: classical_pk };

        // Dilithium keys (public key passed in for correctness)
        let _dil_sk = DilSecretKey::from_bytes(pq_secret)
            .map_err(|_| CryptoTradeError::SigningFailed)?;
        let dil_pk = DilPublicKey::from_bytes(pq_public)
            .map_err(|_| CryptoTradeError::SigningFailed)?;

        // Create commitment
        let commitment_data = format!("{:?}:{:?}:{}", trade.offered, trade.nonce, trade.trade_id);
        let commitment_hash = blake3::hash(commitment_data.as_bytes()).as_bytes().to_vec();

        let commitment = TradeCommitment {
            trade_id: trade.trade_id,
            offeror_id: trade.offeror_id,
            commitment_hash,
            created_at: trade.created_at,
        };

        let commitment_bytes = bincode::serialize(&commitment)
            .map_err(|_| CryptoTradeError::SigningFailed)?;

        // Sign with both schemes (using reconstructed Ed25519 keypair and Dilithium secret)
        let classical_sig = ed_keypair.sign(&commitment_bytes);
        let pq_sig = dilithium2::detached_sign(&commitment_bytes, &DilSecretKey::from_bytes(pq_secret).unwrap());

        let hybrid_sig = HybridSignature {
            classical_signature: classical_sig.to_bytes().to_vec(),
            pq_signature: pq_sig.as_bytes().to_vec(),
        };

        Ok(CryptographicTradeOffer {
            trade: trade.clone(),
            commitment,
            classical_public_key: classical_pk.to_bytes().to_vec(),
            pq_public_key: dil_pk.as_bytes().to_vec(),
            signature: hybrid_sig,
        })
    }

    fn verify_offer(&self, offer: &CryptographicTradeOffer) -> bool {
        // Verify Ed25519 signature
        let classical_pk = match ed25519_dalek::PublicKey::from_bytes(&offer.classical_public_key) {
            Ok(pk) => pk,
            Err(_) => return false,
        };
        let classical_sig = match EdSignature::from_bytes(&offer.signature.classical_signature) {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        let commitment_bytes = match bincode::serialize(&offer.commitment) {
            Ok(b) => b,
            Err(_) => return false,
        };

        if classical_pk.verify(&commitment_bytes, &classical_sig).is_err() {
            return false;
        }

        // Verify Dilithium signature (core logic)
        let pq_pk = match DilPublicKey::from_bytes(&offer.pq_public_key) {
            Ok(pk) => pk,
            Err(_) => return false,
        };
        let pq_sig = match DilSignature::from_bytes(&offer.signature.pq_signature) {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        // This is the key Dilithium verification step
        if dilithium2::verify_detached_signature(&pq_sig, &commitment_bytes, &pq_pk).is_err() {
            return false;
        }

        true
    }

    fn reveal_and_validate(
        &self,
        offer: &CryptographicTradeOffer,
        revealed_trade: &Trade,
    ) -> Result<(), CryptoTradeError> {
        let commitment_data = format!(
            "{:?}:{:?}:{}",
            revealed_trade.offered, revealed_trade.nonce, revealed_trade.trade_id
        );
        let recomputed_hash = blake3::hash(commitment_data.as_bytes()).as_bytes().to_vec();

        if recomputed_hash != offer.commitment.commitment_hash {
            return Err(CryptoTradeError::CommitmentMismatch);
        }

        if !self.verify_offer(offer) {
            return Err(CryptoTradeError::VerificationFailed);
        }

        Ok(())
    }
}