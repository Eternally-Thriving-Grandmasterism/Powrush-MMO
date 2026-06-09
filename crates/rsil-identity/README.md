# rsil-identity

**Ra-Thor Sovereign Identity Lattice (RSIL)**

Post-quantum self-sovereign identity primitives for Powrush-MMO.

## Status

This is an initial skeleton. Production implementation will replace the placeholder ed25519 with actual ML-DSA (Dilithium) from a post-quantum cryptography crate.

## Goals

- Sovereign key generation (player owns their keys)
- DID creation (`did:powrush:...`)
- Challenge-response signing for login
- Foundation for Verifiable Credentials

## Future

- Integrate real ML-DSA implementation
- Add Verifiable Credential issuance helpers
- Cross-server gossip support via Lattice Conductor
