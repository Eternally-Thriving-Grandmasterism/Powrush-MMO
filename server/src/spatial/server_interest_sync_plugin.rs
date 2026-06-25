/*!
 * Server Interest Sync Plugin
 *
 * Central plugin for server-side interest synchronization in Powrush-MMO.
 *
 * v19.7 | Wired RbeInventoryUpdatedEvent into replication snapshot generation
 *
 * - RBE distribution now triggers immediate High-priority VisibleEntitiesUpdate + pending tracking
 * - Ensures inventory changes are part of the next replication snapshot for affected players
 *
 * PATSAGi + Ra-Thor
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::rbe::rbe_plugin::RbeInventoryUpdatedEvent;
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

#[derive(Event, Clone, Debug)]
pub struct ClientDisconnected {
    pub client_entity_id: u64,
}

#[derive(Event, Clone, Debug)]
pub struct ClientReconnected {
    pub client_entity_id: u64,
}

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
            .add_event::<RbeInventoryUpdatedEvent>()

            .add_systems(Update, interest_replication_tick_system)
            .add_systems(Update, resend_unacknowledged_updates)
            .add_systems(Update, log_interest_replication_metrics)

            .add_systems(Update, handle_interest_ack_system)
            .add_systems(Update, handle_client_disconnect_system)
            .add_systems(Update, handle_client_reconnect_system)
            .add_systems(Update, handle_rbe_inventory_update_system)
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
            "[InterestSync] Cleaned up pending state for disconnected client {}",
            disconnect.client_entity_id
        );
    }
}

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
        let server_tick = 0; // TODO: real server tick

        let update = VisibleEntitiesUpdate {
            client_entity_id,
            visible_entity_ids: visible_entities.clone(),
            server_tick,
        };

        send_visible_entities_update_reliable(&update);

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

/// Wires RbeInventoryUpdatedEvent into snapshot generation:
/// 1. Tracks pending with High priority
/// 2. Immediately sends a VisibleEntitiesUpdate snapshot for the affected player
/// This ensures RBE inventory changes are included in the next replication cycle.
fn handle_rbe_inventory_update_system(
    mut rbe_updates: EventReader<RbeInventoryUpdatedEvent>,
    interest_manager: Res<InterestManager>,
    mut pending: ResMut<PendingInterestUpdates>,
    mut metrics: ResMut<InterestReplicationMetrics>,
    time: Res<Time>,
) {
    for update in rbe_updates.read() {
        let player_id = update.player_entity_id;
        let current_time = time.elapsed_seconds();
        let server_tick = 0; // TODO: real server tick

        // Get current visible set for this player (or self if none)
        let visible_entities = interest_manager.get_visible_entities(player_id);

        let snapshot = VisibleEntitiesUpdate {
            client_entity_id: player_id,
            visible_entity_ids: visible_entities,
            server_tick,
        };

        // Send snapshot immediately (wires RBE change into replication snapshot path)
        send_visible_entities_update_reliable(&snapshot);

        // Also mark pending High so resend/backoff logic covers it
        track_pending_update(
            &mut pending,
            &mut metrics,
            player_id,
            server_tick,
            current_time,
            InterestPriority::High,
        );

        info!(
            "[InterestSync] RBE snapshot generated + High priority pending for player {} (+{} {})",
            player_id,
            update.amount_added,
            update.resource_type
        );
    }
}

// End of server_interest_sync_plugin.rs v19.7
// RbeInventoryUpdatedEvent now directly triggers VisibleEntitiesUpdate snapshot generation.
// Thunder locked in. Yoi ⚡