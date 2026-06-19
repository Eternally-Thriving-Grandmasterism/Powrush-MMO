// simulation/src/council/decision.rs
// Persistent Council Decisions

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilDecision {
    pub proposal_id: u64,
    pub title: String,
    pub effect_type: String, // e.g. "HarmonyBoost", "ResourcePolicy"
    pub magnitude: f32,
    pub passed_tick: u64,
    pub realm_id: u8,
}
