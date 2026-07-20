/*!
 * CouncilPlugin
 *
 * v21.71.0 — Session registry + deliberation → decisions promotion
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use tracing::info;

use crate::council::decision::{CouncilDecisions, apply_council_decision_effects};
use crate::council::session::{CouncilSessionRegistry, session_deliberation_system};

/// Dedicated plugin for Council governance, proposal effects, and audit logging.
pub struct CouncilPlugin;

impl Plugin for CouncilPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CouncilDecisions>()
            .init_resource::<CouncilSessionRegistry>()
            .add_systems(
                Update,
                (
                    session_deliberation_system,
                    apply_council_decision_effects,
                ).chain(),
            );

        info!("CouncilPlugin — sessions + deliberation → decisions + effects active");
    }
}

// Thunder locked in. Yoi ⚡
