/*!
 * Bevy Integration for Ra-Thor Bridge + Legacy Journal
 *
 * Provides easy-to-use Bevy Resource and helpers for integrating the Ra-Thor
 * Council bridge and LegacyJournalRegistry directly into game systems.
 *
 * TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned.
 */

use bevy::prelude::*;
use tracing::info;

use crate::ra_thor_bridge::{RaThorBridge, RealRaThorClient, RaThorError};
use crate::emergence::{EmergenceSeed, CouncilGuidance};
use crate::player_legacy_journal::LegacyJournalRegistry;  // NEW: for simulation-side population

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

/// Plugin that registers simulation resources including LegacyJournalRegistry.
/// 
/// This ensures LegacyJournalRegistry is available for simulation systems
/// to populate with harvest, epiphany, war victory, and proactive joy events.
pub struct RaThorPlugin;

impl Plugin for RaThorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RaThorResource>()
            .init_resource::<LegacyJournalRegistry>();  // Simulation-side initialization
        info!("RaThorPlugin + LegacyJournalRegistry initialized");
    }
}

/*
 * === Simulation-side Population Guidance ===
 *
 * Simulation systems should call methods on LegacyJournalRegistry to record events:
 *
 * Example in a war resolution system:
 *   fn on_merciful_war_victory(
 *       mut legacy: ResMut<LegacyJournalRegistry>,
 *       // ... other params
 *   ) {
 *       legacy.record_war_victory_legacy_export(
 *           player_id,
 *           server_name,
 *           true,           // merciful
 *           abundance_bonus,
 *           "Key Contributor".to_string(),
 *           current_tick,
 *           server_id,
 *           current_mercy,
 *           valence,
 *       );
 *   }
 *
 * Example after high-yield sustainable harvest or council bloom:
 *   legacy.generate_proactive_joy_redemption_thread(
 *       player_id,
 *       "Sustainable harvest abundance celebration".to_string(),
 *       joy_amount,
 *       valence_boost,
 *       current_tick,
 *       server_id,
 *   );
 *
 * This keeps the My Mercy Journey panel in sync with live simulation events.
 */
