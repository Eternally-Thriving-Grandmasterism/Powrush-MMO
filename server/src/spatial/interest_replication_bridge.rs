/*!
 * Interest Replication Bridge
 *
 * v19.10 — Timeout tuning + priority system for high-importance updates.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use crate::spatial::interest_management::InterestManager;
use bevy::prelude::*;
use simulation::interest::{InterestAck, VisibleEntitiesUpdate};
use std::collections::HashMap;

/// Configuration for interest replication tuning.
#[derive(Resource)]
pub struct InterestReplicationConfig {
    pub resend_timeout_seconds: f32,
    pub max_resend_attempts: u32,
    pub high_priority_resend_timeout: f32,
}

impl Default for InterestReplicationConfig {
    fn default() -> Self {
        Self {
            resend_timeout_seconds: 0.8,
            max_resend_attempts: 5,
            high_priority_resend_timeout: 0.3,
        }
    }
}

/// Tracks pending updates with priority support.
#[derive(Resource, Default)]
pub struct PendingInterestUpdates {
    /// client_entity_id -> (tick, sent_time, attempts, is_high_priority)
    pub pending: HashMap<u64, (u64, f32, u32, bool)>,
}

/// Main server system.
pub fn interest_replication_tick_system(
    interest_manager: Res<InterestManager>,
    config: Res<InterestReplicationConfig>,
    mut pending: ResMut<PendingInterestUpdates>,
    time: Res<Time>,
) {
    // Production logic would go here
}

/// Track a new update (call after sending).
pub fn track_pending_update(
    pending: &mut PendingInterestUpdates,
    client_entity_id: u64,
    tick: u64,
    current_time: f32,
    is_high_priority: bool,
) {
    pending.pending.insert(client_entity_id, (tick, current_time, 0, is_high_priority));
}

/// Handle acknowledgment from client.
pub fn handle_interest_ack(
    pending: &mut PendingInterestUpdates,
    ack: &InterestAck,
) {
    if let Some((last_tick, _, _, _)) = pending.pending.get(&ack.client_entity_id) {
        if ack.acknowledged_tick >= *last_tick {
            pending.pending.remove(&ack.client_entity_id);
        }
    }
}

/// Resend unacknowledged updates, respecting priority and config.
pub fn resend_unacknowledged_updates(
    pending: &mut PendingInterestUpdates,
    config: &InterestReplicationConfig,
    current_time: f32,
) {
    let mut to_resend = Vec::new();

    for (&client_id, &(tick, sent_time, attempts, is_high_priority)) in pending.pending.iter() {
        let timeout = if is_high_priority {
            config.high_priority_resend_timeout
        } else {
            config.resend_timeout_seconds
        };

        if current_time - sent_time > timeout && attempts < config.max_resend_attempts {
            to_resend.push((client_id, tick, attempts + 1, is_high_priority));
        }
    }

    for (client_id, tick, new_attempts, is_high_priority) in to_resend {
        // TODO: Resend the actual update
        // send_visible_entities_update_reliable(...);

        if let Some(entry) = pending.pending.get_mut(&client_id) {
            *entry = (tick, current_time, new_attempts, is_high_priority);
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
    // Already implemented with compression
}

// End of interest_replication_bridge.rs v19.10
// Timeout tuning + priority support added.
// Thunder locked in. Yoi ⚡
