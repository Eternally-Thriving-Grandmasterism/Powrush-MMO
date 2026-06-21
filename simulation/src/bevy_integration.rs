/*!
 * Bevy Integration for Ra-Thor Bridge + Legacy Journal + Council Governance
 *
 * Provides easy-to-use Bevy Resource and helpers for integrating the Ra-Thor
 * Council bridge, LegacyJournalRegistry, and now CouncilDecisions for proposal effects.
 *
 * TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned.
 */

use bevy::prelude::*;
use tracing::info;

use crate::ra_thor_bridge::{RaThorBridge, RealRaThorClient, RaThorError};
use crate::emergence::{EmergenceSeed, CouncilGuidance};
use crate::player_legacy_journal::LegacyJournalRegistry;
use crate::council::CouncilDecisions;  // NEW: for applied council proposal effects in ECS

/// Bevy Resource that wraps the Ra-Thor bridge.
#[derive(Resource)]
pub struct RaThorResource {
    pub bridge: RaThorBridge,
}

impl Default for RaThorResource {
    fn default() -> Self {
        Self {
            bridge: RaThorBridge::new_real(true),
        }
    }
}

impl RaThorResource {
    pub fn new_simulation() -> Self {
        Self {
            bridge: RaThorBridge::new_simulation(true),
        }
    }

    pub fn new_real() -> Self {
        Self {
            bridge: RaThorBridge::new_real(true),
        }
    }

    pub fn query_council(
        &self,
        seed: &EmergenceSeed,
        player_valence: f32,
        mercy_score: f32,
    ) -> Result<Option<CouncilGuidance>, RaThorError> {
        self.bridge.query_council_guidance(seed, player_valence, mercy_score)
    }
}

/// Plugin that registers simulation resources including LegacyJournalRegistry and CouncilDecisions.
/// CouncilDecisions enables the apply_council_decision_effects system to react to passed proposals.
pub struct RaThorPlugin;

impl Plugin for RaThorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RaThorResource>()
            .init_resource::<LegacyJournalRegistry>()
            .init_resource::<CouncilDecisions>();  // NEW: Council proposal decisions now available in ECS
        info!("RaThorPlugin + LegacyJournalRegistry + CouncilDecisions initialized");
    }
}

/*
 * === Simulation-side Population Guidance ===
 *
 * Simulation systems should call methods on LegacyJournalRegistry to record events.
 * The apply_council_decision_effects system (in council/decision.rs) now automatically
 * applies RBE abundance, sustainability, and harmony effects when CouncilDecisions is populated
 * (by orchestrator, council systems, or external bridge).
 */
