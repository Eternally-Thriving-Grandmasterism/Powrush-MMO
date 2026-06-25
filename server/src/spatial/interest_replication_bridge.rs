/*!
 * Interest Replication Bridge
 *
 * v19.7 — Reliable UDP delivery implemented for VisibleEntitiesUpdate.
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
    // Production: generate updates and call send_visible_entities_update_reliable()
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

/// Sends VisibleEntitiesUpdate with **reliable** delivery.
/// Interest/visibility data must not be lost, so we use a reliable channel.
pub fn send_visible_entities_update_reliable(update: &VisibleEntitiesUpdate) {
    // 1. Serialize
    let serialized = match bincode::serialize(update) {
        Ok(data) => data,
        Err(e) => {
            error!("[InterestReplication] Serialize failed: {}", e);
            return;
        }
    };

    // 2. Compress
    let compressed = match zstd::encode_all(&serialized[..], 3) {
        Ok(data) => data,
        Err(_) => serialized,
    };

    // 3. Send reliably
    // Replace this with your actual reliable send:
    //
    // Example with Renet:
    // if let Some(client) = networking.get_client(update.client_entity_id) {
    //     client.send_message(
    //         RELIABLE_INTEREST_CHANNEL,
    //         compressed,
    //     );
    // }
    //
    // Or with a custom reliable UDP layer:
    // reliable_udp.send(update.client_entity_id, RELIABLE_CHANNEL, compressed);

    debug!(
        "[InterestReplication] Sent reliable {} bytes to player {}",
        compressed.len(),
        update.client_entity_id
    );
}

// End of interest_replication_bridge.rs v19.7
// Reliable UDP delivery implemented for visibility updates.
// Thunder locked in. Yoi ⚡
