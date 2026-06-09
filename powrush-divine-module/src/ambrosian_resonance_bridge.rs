//! Ambrosian Resonance Bridge — Harmony & Faction Resonance
//! Local Ra-Thor implementation for measuring and encouraging group harmony and abundance resonance.

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResonanceReport {
    pub harmony_score: f32,
    pub resonance_quality: String,
    pub guidance: String,
}

pub struct AmbrosianResonanceBridge;

impl AmbrosianResonanceBridge {
    pub fn new() -> Self {
        AmbrosianResonanceBridge
    }

    /// Calculate resonance between two players or a player and faction.
    /// Higher score = stronger "One Lattice" connection and abundance feedback.
    pub fn calculate_resonance(&self, player_a_valence: f32, player_b_valence: f32) -> ResonanceReport {
        let harmony_score = (player_a_valence + player_b_valence) / 2.0;
        let (quality, guidance) = if harmony_score >= 0.85 {
            (
                "Strong Ambrosian Resonance".to_string(),
                "Your flows are in beautiful harmony. The Lattice sings through both of you.".to_string()
            )
        } else if harmony_score >= 0.7 {
            (
                "Gentle Resonance".to_string(),
                "Good alignment. Small shared harvests will deepen the bond.".to_string()
            )
        } else {
            (
                "Dissonance detected".to_string(),
                "The Lattice invites both to return to mercy and shared abundance.".to_string()
            )
        };

        ResonanceReport {
            harmony_score,
            resonance_quality: quality,
            guidance,
        }
    }

    /// Apply resonance to a group action (e.g. shared dynamic event).
    pub fn apply_group_resonance(&self, average_valence: f32) -> f32 {
        // Bonus multiplier for high group harmony
        if average_valence >= 0.8 { 1.15 } else { 1.0 }
    }
}