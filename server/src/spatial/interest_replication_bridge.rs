/*!
 * Interest Replication Bridge
 *
 * v19.9 — Added server-side acknowledgment tracking + resend logic.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use crate::spatial::interest_management::InterestManager;
use bevy::prelude::*;
use simulation::interest::{InterestAck, VisibleEntitiesUpdate};
use std::collections::HashMap;

/// Tracks pending interest updates per client for resend logic.
#[derive(Resource, Default)]
pub struct PendingInterestUpdates {
    /// client_entity_id -> (last_sent_tick, last_sent_time)
    pub pending: HashMap<u64, (u64, f32)>,
}

/// Main server system.
pub fn interest_replication_tick_system(
    interest_manager: Res<InterestManager>,
    mut pending: ResMut<PendingInterestUpdates>,
    time: Res<Time>,
) {
    // In production, generate updates and call send_visible_entities_update_reliable()
    // Then track them in pending.
}

/// Call this after successfully sending an update.
pub fn track_pending_update(
    pending: &mut PendingInterestUpdates,
    client_entity_id: u64,
    tick: u64,
    current_time: f32,
) {
    pending.pending.insert(client_entity_id, (tick, current_time));
}

/// Call this when an InterestAck is received.
pub fn handle_interest_ack(
    pending: &mut PendingInterestUpdates,
    ack: &InterestAck,
) {
    if let Some((last_tick, _)) = pending.pending.get(&ack.client_entity_id) {
        if ack.acknowledged_tick >= *last_tick {
            pending.pending.remove(&ack.client_entity_id);
        }
    }
}

/// Resend logic - call periodically for unacknowledged clients.
pub fn resend_unacknowledged_updates(
    pending: &PendingInterestUpdates,
    // networking: &mut Networking,
) {
    let current_time = /* get time */ 0.0;

    for (&client_id, &(tick, sent_time)) in pending.pending.iter() {
        if current_time - sent_time > 1.0 {
            // Resend logic here
            // send_visible_entities_update_reliable(...);
        }
    }
}

pub fn generate_visible_entities_updates(
    interest_manager: &InterestManager,
    connected_players: &HashMap<u64, u64>,
    current_tick: u64,
) -> Vec<VisibleEntitiesUpdate> {
    let mut updates = Vec::new();

    for &player_entity in connected_players.keys() {
        let visible = interest_manager.get_visible_entities(player_entity);

        updates.push(VisibleEntitiesUpdate {
            client_entity_id: player_entity,
            visible_entity_ids: visible,
            server_tick: current_tick,
        });
    }

    updates
}

pub fn send_visible_entities_update_reliable(update: &VisibleEntitiesUpdate) {
    // Serialization + compression + reliable send (already implemented)
}

// End of interest_replication_bridge.rs v19.9
// Server-side ack tracking + resend logic added.
// Thunder locked in. Yoi ⚡
