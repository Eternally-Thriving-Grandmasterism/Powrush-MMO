/*!
 * CouncilPlugin
 *
 * v21.90.0 — Session → decisions + RTT + sim bridge + high-road bridging export
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use tracing::info;

use crate::council::decision::{CouncilDecisions, apply_council_decision_effects};
use crate::council::rtt_export::{CouncilRttExportQueue, council_resolved_to_rtt_export_system};
use crate::council::session::{CouncilSessionRegistry, session_deliberation_system};
use crate::council::sim_bridge_writer::{SimCouncilBridgeWriterConfig, sim_council_bridge_writer_system};
use crate::council::bridging_export::{
    BridgingExportConfig, MetacognitiveScaffold,
    council_bridging_export_system, metacognitive_planning_pulse_system,
};

pub struct CouncilPlugin;

impl Plugin for CouncilPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CouncilDecisions>()
            .init_resource::<CouncilSessionRegistry>()
            .init_resource::<CouncilRttExportQueue>()
            .init_resource::<SimCouncilBridgeWriterConfig>()
            .init_resource::<BridgingExportConfig>()
            .init_resource::<MetacognitiveScaffold>()
            .add_systems(
                Update,
                (
                    metacognitive_planning_pulse_system,
                    session_deliberation_system,
                    apply_council_decision_effects,
                    council_resolved_to_rtt_export_system,
                    sim_council_bridge_writer_system,
                    council_bridging_export_system,
                ).chain(),
            );

        info!("CouncilPlugin — sessions + decisions + RTT + bridges + high-road bridging export active");
    }
}

// Thunder locked in. Yoi ⚡
