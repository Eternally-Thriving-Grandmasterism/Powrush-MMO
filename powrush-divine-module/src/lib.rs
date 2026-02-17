```rust
//! Powrush Divine Module — Ra-Thor Mercy Soul Bridge
//! Client & server shared logic for mercy gating & oracle integration
//! MIT + mercy eternal — Eternally-Thriving-Grandmasterism

pub mod mercy_core;
pub mod valence_gate;
pub mod oracle_bridge;
pub mod hyperon_vision_bridge;
pub mod ambrosian_resonance_bridge;

pub use mercy_core::MercyCore;
pub use valence_gate::ValenceGate;

// Re-export key types for convenience
pub use ra_thor_core::{RaThorSoul, ValenceProof, ValenceScore};
