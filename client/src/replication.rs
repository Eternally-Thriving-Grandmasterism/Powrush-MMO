/*!
 * Replication Module — Council Bloom Types + Re-exports
 *
 * Restored to clean, functional state after rapid iteration refactor.
 * Acts as the replication boundary for Council Bloom events/payloads.
 * Full implementation lives in council_bloom_feedback.rs (particles, UI, systems).
 *
 * v19.3 — Recovery Pass
 * - Restored core CouncilBloomPayload and CouncilBloomReceived definitions
 * - Clean, accurate re-exports matching the restored council_bloom_feedback.rs
 * - Preserved separation of concerns
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Council Bloom Replication Types (restored)
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize, Event)]
pub struct CouncilBloomReceived {
    pub payload: CouncilBloomPayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilBloomPayload {
    pub bloom_activated: bool,
    pub collective_attunement_score: f32,
    pub bloom_amplification_multiplier: f32,
    pub session_id: u64,
    // Add more fields as needed from original design
}

// ============================================================================
// Re-exports for convenience (updated to match restored feedback module)
// ============================================================================

pub use crate::council_bloom_feedback::{
    BloomSeverity,
    CouncilBloomFeedbackPlugin,
    // Note: Particle spawn now uses unified optimized path in particles.rs
};

// Re-export the main event so other modules can listen cleanly
pub use crate::replication::CouncilBloomReceived;

// Note:
// Full Council Bloom particle feedback, history panel, toasts, and optimized spawn
// now live in client/src/council_bloom_feedback.rs + particles.rs.
// This module provides the replication event/payload boundary.
// Add CouncilBloomFeedbackPlugin to your client app for rich bloom experience.
