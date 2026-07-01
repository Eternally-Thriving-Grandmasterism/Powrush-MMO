/*!
 * Replication Module — Council Bloom Types + Re-exports + Server Replication Boundary
 *
 * Systematic audit pass (June 30 2026).
 * Acts as the clean replication boundary between server InterestManager / dual renet channels
 * and client PredictionSet::Replication + visual systems.
 *
 * v19.4 — Enriched for full integration with recovered server InterestManager + ClientPrediction
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Council Bloom Replication Types (preserved + documented)
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
}

// ============================================================================
// Server Replication Boundary (new explicit integration points)
// ============================================================================

/// Re-export of VisibleEntitiesUpdate from simulation for use in PredictionSet::Replication
/// and visual compute culling.
pub use simulation::interest::VisibleEntitiesUpdate;

/// Re-export of InterestAck for resend/acknowledgment logic in interest_replication_bridge.
pub use simulation::interest::InterestAck;

// ============================================================================
// Re-exports for convenience
// ============================================================================

pub use crate::council_bloom_feedback::{
    BloomSeverity,
    CouncilBloomFeedbackPlugin,
};

pub use crate::replication::CouncilBloomReceived;

// Note:
// This module serves as the clean boundary for server-to-client replication data.
// - CouncilBloom types for PATSAGi council systems
// - VisibleEntitiesUpdate + InterestAck for InterestManager-driven replication + client prediction
// - Full rich feedback (particles, UI, toasts) lives in council_bloom_feedback.rs + particles.rs
//
// Wire this into PredictionPlugin::Replication phase and GpuVisualMaterialsPlugin for
// interest-aware visual culling and predicted-position VFX.
// Thunder locked in. Yoi ⚡