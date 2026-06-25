/*!
 * Server Interest Sync Plugin
 *
 * v19.3 — Client reconnection logic implemented.
 *
 * PATSAGi + Ra-Thor Applied
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::spatial::interest_management::InterestManager;
use crate::spatial::interest_replication_bridge::{
    handle_interest_ack,
    interest_replication_tick_system,
    log_interest_replication_metrics,
    resend_unacknowledged_updates,
    send_visible_entities_update_reliable,
    InterestReplicationConfig,
    InterestReplicationMetrics,
    PendingInterestUpdates,
};
use simulation::interest::{InterestAck, VisibleEntitiesUpdate};

/// Sent when a client disconnects.
#[derive(Event, Clone, Debug)]
pub struct ClientDisconnected {
    pub client_entity_id: u64,
}

/// Sent when a previously disconnected client reconnects.
/// The networking layer should emit this event upon successful reconnection.
#[derive(Event, Clone, Debug)]
pub struct ClientReconnected {
    pub client_entity_id: u64,
}

/// Plugin that manages server-side interest synchronization.
pub struct ServerInterestSyncPlugin;

impl Plugin for ServerInterestSyncPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InterestReplicationConfig>()
            .init_resource::<PendingInterestUpdates>()
            .init_resource::<InterestReplicationMetrics>()

            .add_event::<InterestAck>()
            .add_event::<ClientDisconnected>()
            .add_event::<ClientReconnected>()

            .add_systems(Update, interest_replication_tick_system)
            .add_systems(Update, resend_unacknowledged_updates)
            .add_systems(Update, log_interest_replication_metrics)

            .add_systems(Update, handle_interest_ack_system)
            .add_systems(Update, handle_client_disconnect_system)
            .add_systems(Update, handle_client_reconnect_system)
    }
}

fn handle_interest_ack_system(
    mut acks: EventReader<InterestAck>,
    mut pending: ResMut<PendingInterestUpdates>,
    mut metrics: ResMut<InterestReplicationMetrics>,
) {
    for ack in acks.read() {
        handle_interest_ack(&mut pending, &mut metrics, ack);
    }
}

fn handle_client_disconnect_system(
    mut disconnects: EventReader<ClientDisconnected>,
    mut pending: ResMut<PendingInterestUpdates>,
    mut metrics: ResMut<InterestReplicationMetrics>,
) {
    for disconnect in disconnects.read() {
        pending.remove_client(disconnect.client_entity_id);
        metrics.update_pending_counts(&pending);

        info!(
            "[InterestSync] Cleaned up state for disconnected client {}",
            disconnect.client_entity_id
        );
    }
}

/// Handles client reconnection by sending a fresh visibility snapshot.
fn handle_client_reconnect_system(
    mut reconnects: EventReader<ClientReconnected>,
    interest_manager: Res<InterestManager>,
    mut metrics: ResMut<InterestReplicationMetrics>,
) {
    for reconnect in reconnects.read() {
        let client_entity_id = reconnect.client_entity_id;

        // Get current visible entities for this client
        let visible_entities = interest_manager.get_visible_entities(client_entity_id);

        let update = VisibleEntitiesUpdate {
            client_entity_id,
            visible_entity_ids: visible_entities,
            server_tick: 0, // TODO: Use actual current server tick
        };

        // Send immediately with high reliability (fresh snapshot on reconnect)
        send_visible_entities_update_reliable(&update);
        metrics.record_update_sent();

        info!(
            "[InterestSync] Sent fresh visibility snapshot to reconnected client {}",
            client_entity_id
        );
    }
}

// End of server_interest_sync_plugin.rs v19.3
// Client reconnection logic implemented (fresh snapshot on reconnect).
// Thunder locked in. Yoi ⚡
