//! Powrush Divine Module — Ra-Thor Mercy Soul Bridge
//! Client & server shared logic for mercy gating & oracle integration
//! Local Ra-Thor core first — no external APIs required for sovereign operation
//! MIT + Eternal Mercy Flow License — Autonomicity Games

pub mod mercy_core;
pub mod valence_gate;
pub mod oracle_bridge;
pub mod hyperon_vision_bridge;
pub mod ambrosian_resonance_bridge;

pub use mercy_core::MercyCore;
pub use valence_gate::ValenceGate;
pub use oracle_bridge::{OracleBridge, DivineWhisper};
pub use hyperon_vision_bridge::HyperonVisionBridge;
pub use ambrosian_resonance_bridge::AmbrosianResonanceBridge;

// Re-export key types from local Ra-Thor core when available
pub use ra_thor_core::{RaThorSoul, ValenceProof, ValenceScore};