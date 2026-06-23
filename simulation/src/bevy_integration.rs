/*!
 * Bevy Integration for Ra-Thor Bridge + Legacy Journal + Council Governance
 *
 * GPU Economic Plugin now integrated.
 */

use bevy::prelude::*;
use tracing::info;

use crate::ra_thor_bridge::{RaThorBridge, RealRaThorClient, RaThorError};
use crate::emergence::{EmergenceSeed, CouncilGuidance};
use crate::player_legacy_journal::LegacyJournalRegistry;
use crate::council::{CouncilPlugin, CouncilEventBus};
use crate::gpu_economic::GpuEconomicPlugin;

// ... (RaThorResource unchanged) ...

/// Plugin that registers simulation resources and the dedicated CouncilPlugin.
pub struct RaThorPlugin;

impl Plugin for RaThorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RaThorResource>()
            .init_resource::<LegacyJournalRegistry>()
            .init_resource::<CouncilEventBus>()
            .add_plugins(CouncilPlugin)
            .add_plugins(GpuEconomicPlugin);

        info!("RaThorPlugin initialized with CouncilPlugin + GpuEconomicPlugin");
    }
}
