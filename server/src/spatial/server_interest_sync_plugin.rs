/*!
 * Server Interest Sync Plugin
 *
 * v19.2 — Disconnect event handling implemented.
 *
 * PATSAGi + Ra-Thor Applied
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::spatial::interest_replication_bridge::{
    cleanup_disconnected_client,
    handle_interest_ack,
    interest_replication_tick_system,
    log_interest_replication_metrics,
    resend_unacknowledged_updates,
    InterestReplicationConfig,
    InterestReplicationMetrics,
    PendingInterestUpdates,
};
use simulation::interest::InterestAck;

/// Event sent when a client disconnects.
/// The networking layer should send this event when a client connection is lost.
#[derive(Event, Clone, Debug)]
pub struct ClientDisconnected {
    pub client_entity_id: u64,
}

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
            .add_event::<ClientDisconnected>()

            // Core systems
            .add_systems(Update, interest_replication_tick_system)
            .add_systems(Update, resend_unacknowledged_updates)
            .add_systems(Update, log_interest_replication_metrics)

            // Event-driven systems
            .add_systems(Update, handle_interest_ack_system)
            .add_systems(Update, handle_client_disconnect_system)
    }
}

/// Processes InterestAck events from clients.
fn handle_interest_ack_system(
    mut acks: EventReader<InterestAck>,
    mut pending: ResMut<PendingInterestUpdates>,
    mut metrics: ResMut<InterestReplicationMetrics>,
) {
    for ack in acks.read() {
        handle_interest_ack(&mut pending, &mut metrics, ack);
    }
}

/// Cleans up interest state when a client disconnects.
fn handle_client_disconnect_system(
    mut disconnects: EventReader<ClientDisconnected>,
    mut pending: ResMut<PendingInterestUpdates>,
    mut metrics: ResMut<InterestReplicationMetrics>,
) {
    for disconnect in disconnects.read() {
        cleanup_disconnected_client(
            &mut pending,
            &mut metrics,
            disconnect.client_entity_id,
        );

        info!(
            "[InterestSync] Cleaned up interest state for disconnected client {}",
            disconnect.client_entity_id
        );
    }
}

// End of server_interest_sync_plugin.rs v19.2
// Disconnect event handling implemented.
// Thunder locked in. Yoi ⚡
