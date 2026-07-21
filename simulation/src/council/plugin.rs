/*!
 * CouncilPlugin
 *
 * v21.79.0 — Session → decisions + RTT export + sim bridge file writer
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use tracing::info;

use crate::council::decision::{CouncilDecisions, apply_council_decision_effects};
use crate::council::rtt_export::{CouncilRttExportQueue, council_resolved_to_rtt_export_system};
use crate::council::session::{CouncilSessionRegistry, session_deliberation_system};
use crate::council::sim_bridge_writer::{SimCouncilBridgeWriterConfig, sim_council_bridge_writer_system};

pub struct CouncilPlugin;

impl Plugin for CouncilPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CouncilDecisions>()
            .init_resource::<CouncilSessionRegistry>()
            .init_resource::<CouncilRttExportQueue>()
            .init_resource::<SimCouncilBridgeWriterConfig>()
            .add_systems(
                Update,
                (
                    session_deliberation_system,
                    apply_council_decision_effects,
                    council_resolved_to_rtt_export_system,
                    sim_council_bridge_writer_system,
                ).chain(),
            );

        info!("CouncilPlugin — sessions + decisions + RTT export + sim bridge writer active");
    }
}

// Thunder locked in. Yoi ⚡
