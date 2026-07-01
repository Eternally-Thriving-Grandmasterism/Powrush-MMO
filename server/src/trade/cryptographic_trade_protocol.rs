// server/src/trade/cryptographic_trade_protocol.rs
// Cryptographic Trade Protocol v1
// Commitment + Reveal + Post-Quantum (Dilithium) Signatures
// Optional high-security path on top of hardened trade system
// AG-SML v1.0 | PATSAGi + Ra-Thor aligned

use crate::trade_system::Trade;
use pqcrypto_dilithium::dilithium2::{self, PublicKey, SecretKey, DetachedSignature};
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

/// Cryptographic commitment to a trade offer (prevents last-second changes)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeCommitment {
    pub trade_id: u64,
    pub offeror_id: u64,
    pub commitment_hash: Vec<u8>,     // Hash of offered items + nonce
    pub created_at: u64,
}

/// Cryptographically signed and committed trade offer
#[derive(Clone, Debug)]
pub struct CryptographicTradeOffer {
    pub trade: Trade,
    pub commitment: TradeCommitment,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,           // Dilithium signature over commitment
}

/// Main protocol interface
pub trait CryptographicTradeProtocol {
    /// Generate a new post-quantum keypair for a player
    fn generate_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), CryptoTradeError>;

    /// Create a committed + signed trade offer
    fn create_signed_offer(
        &self,
        trade: &Trade,
        secret_key: &[u8],
    ) -> Result<CryptographicTradeOffer, CryptoTradeError>;

    /// Verify a cryptographic trade offer (signature + commitment)
    fn verify_offer(&self, offer: &CryptographicTradeOffer) -> bool;

    /// Reveal and validate the full trade against its commitment
    fn reveal_and_validate(
        &self,
        offer: &CryptographicTradeOffer,
        revealed_trade: &Trade,
    ) -> Result<(), CryptoTradeError>;
}

/// Dilithium2 implementation
pub struct DilithiumTradeProtocol;

impl CryptographicTradeProtocol for DilithiumTradeProtocol {
    fn generate_keypair(&self) -> Result<(Vec<u8>, Vec<u8>), CryptoTradeError> {
        let (pk, sk) = dilithium2::keypair();
        Ok((pk.as_bytes().to_vec(), sk.as_bytes().to_vec()))
    }

    fn create_signed_offer(
        &self,
        trade: &Trade,
        secret_key: &[u8],
    ) -> Result<CryptographicTradeOffer, CryptoTradeError> {
        let sk = SecretKey::from_bytes(secret_key)
            .map_err(|_| CryptoTradeError::SigningFailed)?;

        // Create commitment (hash of offered resources + nonce)
        let commitment_data = format!("{:?}:{:?}:{}", trade.offered, trade.nonce, trade.trade_id);
        let commitment_hash = blake3::hash(commitment_data.as_bytes()).as_bytes().to_vec();

        let commitment = TradeCommitment {
            trade_id: trade.trade_id,
            offeror_id: trade.offeror_id,
            commitment_hash,
            created_at: trade.created_at,
        };

        // Sign the commitment
        let commitment_bytes = bincode::serialize(&commitment)
            .map_err(|_| CryptoTradeError::SigningFailed)?;

        let signature = dilithium2::detached_sign(&commitment_bytes, &sk);

        Ok(CryptographicTradeOffer {
            trade: trade.clone(),
            commitment,
            public_key: vec![], // Caller should provide or store separately
            signature: signature.as_bytes().to_vec(),
        })
    }

    fn verify_offer(&self, offer: &CryptographicTradeOffer) -> bool {
        if let (Ok(pk), Ok(sig)) = (
            PublicKey::from_bytes(&offer.public_key),
            DetachedSignature::from_bytes(&offer.signature),
        ) {
            if let Ok(bytes) = bincode::serialize(&offer.commitment) {
                return dilithium2::verify_detached_signature(&sig, &bytes, &pk).is_ok();
            }
        }
        false
    }

    fn reveal_and_validate(
        &self,
        offer: &CryptographicTradeOffer,
        revealed_trade: &Trade,
    ) -> Result<(), CryptoTradeError> {
        // Recompute commitment from revealed trade
        let commitment_data = format!(
            "{:?}:{:?}:{}",
            revealed_trade.offered, revealed_trade.nonce, revealed_trade.trade_id
        );
        let recomputed_hash = blake3::hash(commitment_data.as_bytes()).as_bytes().to_vec();

        if recomputed_hash != offer.commitment.commitment_hash {
            return Err(CryptoTradeError::CommitmentMismatch);
        }

        // Verify signature on the original commitment
        if !self.verify_offer(offer) {
            return Err(CryptoTradeError::VerificationFailed);
        }

        Ok(())
    }
}