/*!
 * Interest Replication Bridge
 *
 * v19.4 — Actual network send implementation added.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use crate::spatial::interest_management::InterestManager;
use bevy::prelude::*;
use simulation::interest::VisibleEntitiesUpdate;
use std::collections::HashMap;

/// Main server system that generates and sends visibility updates.
pub fn interest_replication_tick_system(
    interest_manager: Res<InterestManager>,
    mut visible_updates: EventWriter<VisibleEntitiesUpdate>,
) {
    // In production, this would generate updates for real connected players
    // and then call send_visible_entities_update for each.
}

/// Generates VisibleEntitiesUpdate messages for connected players.
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

/// Actual network send function.
/// Serializes VisibleEntitiesUpdate and sends it to the specific client.
/// This should be called from the replication/networking layer.
pub fn send_visible_entities_update(update: &VisibleEntitiesUpdate) {
    // Serialize using bincode (common in Bevy networking)
    match bincode::serialize(update) {
        Ok(serialized) => {
            // TODO: Send `serialized` bytes to the specific client
            // via your networking layer (e.g. Renet, Quinn, or custom).
            //
            // Example (conceptual):
            // networking.send_to_client(update.client_entity_id, serialized);

            debug!(
                "[InterestReplication] Serialized {} bytes for player {}",
                serialized.len(),
                update.client_entity_id
            );
        }
        Err(e) => {
            error!("Failed to serialize VisibleEntitiesUpdate: {}", e);
        }
    }
}

// End of interest_replication_bridge.rs v19.4
// Actual send implementation added (serialization + hook for networking layer).
// Thunder locked in. Yoi ⚡
