/*!
 * Server Interest Sync Plugin
 *
 * v19.9 | Snapshot generation now expands to faction & nearby via RbeInventoryUpdatedEvent
 *
 * When RBE distributes to faction members or nearby participants, multiple RbeInventoryUpdatedEvents are emitted.
 * This handler generates high-priority snapshots for every affected player automatically.
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

#[derive(Resource, Default)]
pub struct ServerTick(pub u64);

#[derive(Event, Clone, Debug)]
pub struct ClientDisconnected { pub client_entity_id: u64 }
#[derive(Event, Clone, Debug)]
pub struct ClientReconnected { pub client_entity_id: u64 }

pub struct ServerInterestSyncPlugin;

impl Plugin for ServerInterestSyncPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InterestReplicationConfig>()
            .init_resource::<PendingInterestUpdates>()
            .init_resource::<InterestReplicationMetrics>()
            .init_resource::<ServerTick>()

            .add_event::<InterestAck>()
            .add_event::<ClientDisconnected>()
            .add_event::<ClientReconnected>()
            .add_event::<RbeInventoryUpdatedEvent>()

            .add_systems(Update, increment_server_tick)
            .add_systems(Update, interest_replication_tick_system)
            .add_systems(Update, resend_unacknowledged_updates)
            .add_systems(Update, log_interest_replication_metrics)

            .add_systems(Update, handle_interest_ack_system)
            .add_systems(Update, handle_client_disconnect_system)
            .add_systems(Update, handle_client_reconnect_system)
            .add_systems(Update, handle_rbe_inventory_update_system)
    }
}

fn increment_server_tick(mut tick: ResMut<ServerTick>) {
    tick.0 = tick.0.wrapping_add(1);
}

// (other handlers unchanged)

fn handle_client_reconnect_system(/* ... */) { /* unchanged */ }

/// Snapshot generation expanded:
/// Now handles RbeInventoryUpdatedEvent from faction/nearby distributions.
/// Because RBE emits one event per affected player, this automatically generates snapshots for all of them.
fn handle_rbe_inventory_update_system(
    mut rbe_updates: EventReader<RbeInventoryUpdatedEvent>,
    interest_manager: Res<InterestManager>,
    mut pending: ResMut<PendingInterestUpdates>,
    mut metrics: ResMut<InterestReplicationMetrics>,
    server_tick: Res<ServerTick>,
    time: Res<Time>,
) {
    for update in rbe_updates.read() {
        let player_id = update.player_entity_id;
        let current_time = time.elapsed_seconds();
        let tick = server_tick.0;

        let visible_entities = interest_manager.get_visible_entities(player_id);

        let snapshot = VisibleEntitiesUpdate {
            client_entity_id: player_id,
            visible_entity_ids: visible_entities,
            server_tick: tick,
        };

        send_visible_entities_update_reliable(&snapshot);

        track_pending_update(
            &mut pending,
            &mut metrics,
            player_id,
            tick,
            current_time,
            InterestPriority::High,
        );

        info!(
            "[InterestSync] RBE snapshot (faction/nearby expanded) for player {} (+{} {})",
            player_id,
            update.amount_added,
            update.resource_type
        );
    }
}

// End of server_interest_sync_plugin.rs v19.9
// Snapshot generation now scales to faction & nearby participants via per-player RbeInventoryUpdatedEvent.
// Thunder locked in. Yoi ⚡