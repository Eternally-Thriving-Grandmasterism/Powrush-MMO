/*!
 * CouncilPlugin
 *
 * Dedicated Bevy plugin for the full Council Proposal System.
 * Bundles resource initialization, effects system, and audit logging.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use tracing::info;

use crate::council::decision::{CouncilDecisions, apply_council_decision_effects};

/// Dedicated plugin for Council governance, proposal effects, and audit logging.
///
/// This plugin:
/// - Initializes the CouncilDecisions resource (pending proposals/effects)
/// - Schedules the apply_council_decision_effects system
/// - Enables full audit log + persistence via SovereignWorldState
pub struct CouncilPlugin;

impl Plugin for CouncilPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CouncilDecisions>()
            .add_systems(Update, apply_council_decision_effects);

        info!("CouncilPlugin initialized — Council Proposal System + Audit Logs active");
    }
}
