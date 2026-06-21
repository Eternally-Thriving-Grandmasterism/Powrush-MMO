/*!
 * Bevy Integration for Ra-Thor Bridge + Legacy Journal + Council Governance
 *
 * Provides easy-to-use Bevy Resource and helpers for integrating the Ra-Thor
 * Council bridge, LegacyJournalRegistry, and now CouncilDecisions + the effects system.
 *
 * TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned.
 */

use bevy::prelude::*;
use tracing::info;

use crate::ra_thor_bridge::{RaThorBridge, RealRaThorClient, RaThorError};
use crate::emergence::{EmergenceSeed, CouncilGuidance};
use crate::player_legacy_journal::LegacyJournalRegistry;
use crate::council::CouncilDecisions;  // Council proposal decisions resource

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

/// Plugin that registers simulation resources and the Council effects system.
/// CouncilDecisions + apply_council_decision_effects provide the full ECS path for
/// passed proposals affecting RBE, abundance, sustainability, and harmony.
pub struct RaThorPlugin;

impl Plugin for RaThorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RaThorResource>()
            .init_resource::<LegacyJournalRegistry>()
            .init_resource::<CouncilDecisions>()
            .add_systems(Update, crate::council::decision::apply_council_decision_effects);  // Wire effects system into Bevy schedule
        info!("RaThorPlugin + LegacyJournalRegistry + CouncilDecisions + effects system initialized");
    }
}

/*
 * === Simulation-side Population Guidance ===
 *
 * The apply_council_decision_effects system now runs every Update.
 * It reacts to CouncilDecisions resource being populated (by orchestrator tick,
 * council systems, or external input) and applies real RBE/harmony effects to SovereignWorldState.
 * Orchestrator continues direct application for its manual tick path.
 * Both paths are consistent.
 */
