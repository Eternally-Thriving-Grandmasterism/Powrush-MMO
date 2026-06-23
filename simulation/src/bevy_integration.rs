/*!
 * Bevy Integration for Ra-Thor Bridge + Legacy Journal + Council Governance
 *
 * Now includes CouncilEventBus as a proper Bevy Resource.
 * GPU Economic Async Readback setup added (v18.97.5).
 */

use bevy::prelude::*;
use tracing::info;

use crate::ra_thor_bridge::{RaThorBridge, RealRaThorClient, RaThorError};
use crate::emergence::{EmergenceSeed, CouncilGuidance};
use crate::player_legacy_journal::LegacyJournalRegistry;
use crate::council::{CouncilPlugin, CouncilEventBus};
use crate::orchestrator::setup_gpu_economic_async_readback;

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
            .init_resource::<CouncilEventBus>()   // Bevy Resource adapter for Council events
            .add_plugins(CouncilPlugin);

        // Wire production async GPU economic readback (resource + apply system)
        setup_gpu_economic_async_readback(app);

        info!("RaThorPlugin initialized with CouncilEventBus, CouncilPlugin, and GPU Economic Async Readback");
    }
}
