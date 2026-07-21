/*!
 * CouncilPlugin
 *
 * v21.78.0 — Session → decisions + RTT export queue for host bridge
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use tracing::info;

use crate::council::decision::{CouncilDecisions, apply_council_decision_effects};
use crate::council::rtt_export::{CouncilRttExportQueue, council_resolved_to_rtt_export_system};
use crate::council::session::{CouncilSessionRegistry, session_deliberation_system};

/// Dedicated plugin for Council governance, proposal effects, and audit logging.
pub struct CouncilPlugin;

impl Plugin for CouncilPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CouncilDecisions>()
            .init_resource::<CouncilSessionRegistry>()
            .init_resource::<CouncilRttExportQueue>()
            .add_systems(
                Update,
                (
                    session_deliberation_system,
                    apply_council_decision_effects,
                    council_resolved_to_rtt_export_system,
                ).chain(),
            );

        info!("CouncilPlugin — sessions + decisions + RTT export queue active");
    }
}

// Thunder locked in. Yoi ⚡
