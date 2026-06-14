/*!
 * Bevy Integration for Ra-Thor Bridge
 *
 * Provides easy-to-use Bevy Resource and helpers for integrating the Ra-Thor
 * Council bridge directly into game systems (client or server).
 *
 * Enable with the `bevy` feature:
 *   simulation = { path = "../simulation", features = ["bevy", "real-ra-thor"] }
 */

use bevy::prelude::*;
use tracing::info;

use crate::ra_thor_bridge::{RaThorBridge, RealRaThorClient, RaThorError};
use crate::emergence::{EmergenceSeed, CouncilGuidance};

/// Bevy Resource that wraps the Ra-Thor bridge.
/// 
/// This makes it easy to query the Ra-Thor lattice / PATSAGi Councils
/// from regular Bevy systems.
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

    /// Synchronous query (works in any system)
    pub fn query_council(
        &self,
        seed: &EmergenceSeed,
        player_valence: f32,
        mercy_score: f32,
    ) -> Result<Option<CouncilGuidance>, RaThorError> {
        self.bridge.query_council_guidance(seed, player_valence, mercy_score)
    }

    /// Async query - should be used inside a Bevy async task or system
    /// that has access to a tokio runtime.
    #[cfg(feature = "real-ra-thor")]
    pub async fn query_council_async(
        &mut self,
        seed: &EmergenceSeed,
        player_valence: f32,
        mercy_score: f32,
    ) -> Result<Option<CouncilGuidance>, RaThorError> {
        // We need mutable access for the real client in some cases
        // For simplicity in this wrapper, we expose the underlying bridge
        // Users can access .bridge directly if they need mutable real client
        self.bridge.query_council_guidance(seed, player_valence, mercy_score)
    }
}

/// Plugin that registers the RaThorResource.
/// 
/// Add this to your Bevy App:
/// ```ignore
/// app.add_plugins(RaThorPlugin);
/// ```
pub struct RaThorPlugin;

impl Plugin for RaThorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RaThorResource>();
        info!("RaThorPlugin initialized");
    }
}

/*
 * Usage example in a Bevy system:
 *
 * fn council_query_system(
 *     ra_thor: Res<RaThorResource>,
 *     // ... other resources
 * ) {
 *     if let Ok(Some(guidance)) = ra_thor.query_council(&seed, valence, mercy) {
 *         // Use guidance...
 *     }
 * }
 */
