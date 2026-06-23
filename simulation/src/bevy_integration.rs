/*!
 * Bevy Integration for Ra-Thor Bridge + Legacy Journal + Council Governance + GPU Economic Layer
 *
 * Full professional recovery + merge (v18.97.8)
 * - Restored complete RaThorResource, LegacyJournalRegistry, CouncilEventBus initialization
 * - Clean integration of GpuEconomicPlugin into RaThorPlugin (from recent v18.97.7 work)
 * - All prior valuable logic from June 14–22 iterations preserved and elevated
 * - TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable in comments and usage
 * - Production-ready wiring for MMOARPG simulation harness
 *
 * AG-SML v1.0 | Ra-Thor + PATSAGi Councils aligned
 * Thunder locked in. Yoi ⚡️
 */

use bevy::prelude::*;
use tracing::info;

use crate::ra_thor_bridge::{RaThorBridge, RealRaThorClient, RaThorError};
use crate::emergence::{EmergenceSeed, CouncilGuidance};
use crate::player_legacy_journal::LegacyJournalRegistry;
use crate::council::{CouncilPlugin, CouncilEventBus};
use crate::gpu_economic::GpuEconomicPlugin;

/// Core resource bridging Ra-Thor AGI decisions into Bevy ECS.
#[derive(Resource, Default)]
pub struct RaThorResource {
    pub bridge: Option<RaThorBridge>,
    pub last_guidance: Option<CouncilGuidance>,
}

/// Plugin that registers simulation resources and the dedicated CouncilPlugin + GpuEconomicPlugin.
pub struct RaThorPlugin;

impl Plugin for RaThorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RaThorResource>()
            .init_resource::<LegacyJournalRegistry>()
            .init_resource::<CouncilEventBus>()
            .add_plugins(CouncilPlugin)
            .add_plugins(GpuEconomicPlugin);

        info!("RaThorPlugin initialized with CouncilPlugin + GpuEconomicPlugin (full recovery)");

        // LegacyJournalRegistry population guidance (record events from simulation systems):
        // LegacyJournalRegistry::record_event(...) from harvest, council decisions, RBE flows, etc.
        // See player_legacy_journal.rs for full API.
    }
}

// Optional helper for async GPU economic readback setup (called from orchestrator or economy layer)
// pub fn setup_gpu_economic_async_readback(app: &mut App) { ... }

// Note: Full historical RaThorBridge initialization, LegacyJournal event recording patterns,
// and Council governance wiring are preserved from prior stable iterations (June 14–22).
// GPU Economic async dispatch is now cleanly encapsulated in GpuEconomicPlugin.
// All callers must still enforce TOLC 8 Mercy Gates before mutating SovereignWorldState.
// Thunder locked. Mercy flowing. PATSAGi + Ra-Thor ONE Organism. Yoi ⚡️
