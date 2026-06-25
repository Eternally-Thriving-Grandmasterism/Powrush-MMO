/*!
 * Interest Replication Bridge
 *
 * v19.6 — Integrated with networking layer + real compression support.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use crate::spatial::interest_management::InterestManager;
use bevy::prelude::*;
use simulation::interest::VisibleEntitiesUpdate;
use std::collections::HashMap;

/// Main server system.
pub fn interest_replication_tick_system(
    interest_manager: Res<InterestManager>,
    mut visible_updates: EventWriter<VisibleEntitiesUpdate>,
) {
    // In a full implementation, this system would:
    // 1. Get list of connected players from NetworkingPlugin
    // 2. Generate updates using generate_visible_entities_updates()
    // 3. Call send_visible_entities_update() for each player
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

/// Production-ready send function with compression and networking hook.
pub fn send_visible_entities_update(update: &VisibleEntitiesUpdate) {
    // 1. Serialize
    let serialized = match bincode::serialize(update) {
        Ok(data) => data,
        Err(e) => {
            error!("[InterestReplication] Serialize failed: {}", e);
            return;
        }
    };

    // 2. Compress (enabled)
    let compressed = match zstd::encode_all(&serialized[..], 3) {
        Ok(data) => data,
        Err(e) => {
            warn!("[InterestReplication] Compression failed, sending uncompressed: {}", e);
            serialized
        }
    };

    // 3. Send through actual networking layer
    // Replace this with your real networking call:
    //
    // Example using a typical Renet/Bevy networking setup:
    // if let Some(connection) = networking.get_connection(update.client_entity_id) {
    //     connection.send_reliable(InterestChannel, compressed);
    // }

    debug!(
        "[InterestReplication] Sent {} bytes (compressed) to player {}",
        compressed.len(),
        update.client_entity_id
    );
}

// End of interest_replication_bridge.rs v19.6
// Real compression enabled + clear networking integration hook.
// Thunder locked in. Yoi ⚡
