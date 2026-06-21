/*!
 * Bevy Integration for Ra-Thor Bridge + Legacy Journal + Council Governance
 *
 * Now uses the dedicated CouncilPlugin for clean modular initialization.
 *
 * TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned.
 */

use bevy::prelude::*;
use tracing::info;

use crate::ra_thor_bridge::{RaThorBridge, RealRaThorClient, RaThorError};
use crate::emergence::{EmergenceSeed, CouncilGuidance};
use crate::player_legacy_journal::LegacyJournalRegistry;
use crate::council::CouncilPlugin;  // NEW: dedicated Council plugin

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

/// Plugin that registers simulation resources and the dedicated CouncilPlugin.
pub struct RaThorPlugin;

impl Plugin for RaThorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RaThorResource>()
            .init_resource::<LegacyJournalRegistry>()
            .add_plugins(CouncilPlugin);  // Clean delegation to dedicated Council plugin

        info!("RaThorPlugin initialized with CouncilPlugin");
    }
}

/*
 * === Simulation-side Population Guidance ===
 *
 * The CouncilPlugin now handles:
 * - CouncilDecisions resource
 * - apply_council_decision_effects system
 * - Full audit logging into SovereignWorldState.council_decision_history
 */
