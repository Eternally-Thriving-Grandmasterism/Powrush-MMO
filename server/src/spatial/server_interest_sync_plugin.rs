/*!
 * Server Interest Sync Plugin
 *
 * Central plugin for server-side interest synchronization in Powrush-MMO.
 *
 * Responsibilities:
 * - Manage VisibleEntitiesUpdate generation and delivery
 * - Handle InterestAck from clients with event-driven processing
 * - Track pending updates with priority and exponential backoff + jitter
 * - Handle client disconnects and reconnections gracefully
 * - Provide metrics for observability
 *
 * Integration Points:
 * - Networking layer should emit ClientDisconnected and ClientReconnected events
 * - Networking layer should send InterestAck events when received from clients
 * - Metrics can be read from InterestReplicationMetrics resource for telemetry
 *
 * v19.5 | Final Polish Pass
 * PATSAGi + Ra-Thor
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
    track_pending_update,
    InterestPriority,
    InterestReplicationConfig,
    InterestReplicationMetrics,
    PendingInterestUpdates,
};
use simulation::interest::{InterestAck, VisibleEntitiesUpdate};

/// Event emitted by networking layer when a client disconnects.
#[derive(Event, Clone, Debug)]
pub struct ClientDisconnected {
    pub client_entity_id: u64,
}

/// Event emitted by networking layer when a client successfully reconnects.
#[derive(Event, Clone, Debug)]
pub struct ClientReconnected {
    pub client_entity_id: u64,
}

/// Main plugin for server-side interest synchronization.
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

/// Processes InterestAck events coming from clients.
fn handle_interest_ack_system(
    mut acks: EventReader<InterestAck>,
    mut pending: ResMut<PendingInterestUpdates>,
    mut metrics: ResMut<InterestReplicationMetrics>,
) {
    for ack in acks.read() {
        handle_interest_ack(&mut pending, &mut metrics, ack);
    }
}

/// Cleans up pending interest state when a client disconnects.
fn handle_client_disconnect_system(
    mut disconnects: EventReader<ClientDisconnected>,
    mut pending: ResMut<PendingInterestUpdates>,
    mut metrics: ResMut<InterestReplicationMetrics>,
) {
    for disconnect in disconnects.read() {
        pending.remove_client(disconnect.client_entity_id);
        metrics.update_pending_counts(&pending);

        info!(
            "[InterestSync] Cleaned up pending state for disconnected client {}",
            disconnect.client_entity_id
        );
    }
}

/// On reconnection, immediately sends a fresh visibility snapshot with High priority.
fn handle_client_reconnect_system(
    mut reconnects: EventReader<ClientReconnected>,
    interest_manager: Res<InterestManager>,
    mut pending: ResMut<PendingInterestUpdates>,
    mut metrics: ResMut<InterestReplicationMetrics>,
    time: Res<Time>,
) {
    for reconnect in reconnects.read() {
        let client_entity_id = reconnect.client_entity_id;

        let visible_entities = interest_manager.get_visible_entities(client_entity_id);

        let current_time = time.elapsed_seconds();
        let server_tick = 0; // TODO: Replace with real server tick when available

        let update = VisibleEntitiesUpdate {
            client_entity_id,
            visible_entity_ids: visible_entities.clone(),
            server_tick,
        };

        send_visible_entities_update_reliable(&update);

        // Track with High priority so reconnecting clients recover faster
        track_pending_update(
            &mut pending,
            &mut metrics,
            client_entity_id,
            server_tick,
            current_time,
            InterestPriority::High,
        );

        info!(
            "[InterestSync] Sent high-priority fresh snapshot to reconnected client {}",
            client_entity_id
        );
    }
}

// ============================================================================
// Metrics Export Hook
// ============================================================================
// The InterestReplicationMetrics resource can be read by any system for:
// - OpenTelemetry / custom telemetry export
// - Admin/debug UI panels
// - Periodic logging or alerting
//
// Example:
// fn export_interest_metrics(metrics: Res<InterestReplicationMetrics>) {
//     // Send metrics.total_resends, metrics.clients_with_pending, etc.
// }

// End of server_interest_sync_plugin.rs v19.5
// Final polish: Improved documentation + metrics export hook.
// Thunder locked in. Yoi ⚡
