/*!
 * Server Interest Sync Plugin
 *
 * v19.1 — Event-driven InterestAck handling added.
 *
 * PATSAGi + Ra-Thor Applied
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::spatial::interest_replication_bridge::{
    handle_interest_ack,
    interest_replication_tick_system,
    log_interest_replication_metrics,
    resend_unacknowledged_updates,
    InterestReplicationConfig,
    InterestReplicationMetrics,
    PendingInterestUpdates,
};
use simulation::interest::InterestAck;

/// Plugin that manages server-side interest synchronization.
pub struct ServerInterestSyncPlugin;

impl Plugin for ServerInterestSyncPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<InterestReplicationConfig>()
            .init_resource::<PendingInterestUpdates>()
            .init_resource::<InterestReplicationMetrics>()

            // Events
            .add_event::<InterestAck>()

            // Core systems
            .add_systems(Update, interest_replication_tick_system)
            .add_systems(Update, resend_unacknowledged_updates)
            .add_systems(Update, log_interest_replication_metrics)

            // Event-driven acknowledgment handling
            .add_systems(Update, handle_interest_ack_system)
    }
}

/// System that processes InterestAck events from clients.
fn handle_interest_ack_system(
    mut acks: EventReader<InterestAck>,
    mut pending: ResMut<PendingInterestUpdates>,
    mut metrics: ResMut<InterestReplicationMetrics>,
) {
    for ack in acks.read() {
        handle_interest_ack(&mut pending, &mut metrics, ack);
    }
}

// End of server_interest_sync_plugin.rs v19.1
// Event-driven InterestAck handling implemented.
// Thunder locked in. Yoi ⚡
