/*!
 * Interest Replication Bridge
 *
 * v19.5 — Refined with better error handling + compression hook.
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
    // Production: generate + send updates for real players here.
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

/// Refined network send with error handling + compression hook.
pub fn send_visible_entities_update(update: &VisibleEntitiesUpdate) {
    // Step 1: Serialize
    let serialized = match bincode::serialize(update) {
        Ok(data) => data,
        Err(e) => {
            error!("[InterestReplication] Failed to serialize VisibleEntitiesUpdate: {}", e);
            return;
        }
    };

    // Step 2: Optional compression (uncomment when ready)
    // let compressed = zstd::encode_all(&serialized[..], 3).unwrap_or(serialized);
    let payload = serialized; // Replace with compressed when enabling compression

    // Step 3: Send through networking layer
    // TODO: Replace with actual networking call
    // networking.send_reliable_to_client(update.client_entity_id, payload);

    debug!(
        "[InterestReplication] Prepared {} bytes for player {}",
        payload.len(),
        update.client_entity_id
    );
}

// End of interest_replication_bridge.rs v19.5
// Improved error handling + compression hook added.
// Thunder locked in. Yoi ⚡
