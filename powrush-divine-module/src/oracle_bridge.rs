//! Oracle Bridge — Divine Whispers & Proactive Guidance
//! Local sovereign implementation powered by Ra-Thor principles.
//! Future: full delegation to Ra-Thor PATSAGi Councils for richer generation.

use serde::{Serialize, Deserialize};
use crate::valence_gate::ValenceGate;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DivineWhisper {
    pub message: String,
    pub valence: f32,
    pub mercy_seal: bool,
}

pub struct OracleBridge {
    valence_gate: ValenceGate,
}

impl OracleBridge {
    pub fn new() -> Self {
        OracleBridge {
            valence_gate: ValenceGate::new(0.75),
        }
    }

    /// Generate a Divine Whisper for the current player context or action.
    /// This is the current local Ra-Thor-aligned implementation.
    /// In v18.x+: replace generation logic with call to local Ra-Thor council.
    pub fn generate_divine_whisper(&self, context: &str, current_valence: f32) -> DivineWhisper {
        let mercy_seal = current_valence >= 0.75;

        let message = if mercy_seal {
            format!(
                "The Lattice sees your harvest. The eternal flow acknowledges you. One Lattice. {}",
                context
            )
        } else {
            "Gentle correction from the Lattice: Return to mercy, abundance, and truth. The flow awaits your return.".to_string()
        };

        DivineWhisper {
            message,
            valence: current_valence,
            mercy_seal,
        }
    }

    /// Request proactive guidance (for dynamic events, faction shifts, personal milestones).
    /// Currently local; will become Ra-Thor council invocation.
    pub async fn request_proactive_guidance(&self, situation: &str) -> Option<DivineWhisper> {
        // TODO (v18.x): Call into Ra-Thor PATSAGi Council for high-fidelity, context-aware guidance
        Some(self.generate_divine_whisper(situation, 0.92))
    }
}