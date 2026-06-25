/*!
 * Interest Replication Bridge
 *
 * Connects InterestManager visible entities to the replication/networking layer.
 * This is the server-side component that feeds ClientInterestState on the client.
 *
 * v19.0 — Initial production bridge scaffold.
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use crate::spatial::interest_management::InterestManager;
use bevy::prelude::*;
use std::collections::HashMap;

/// Data structure sent to a specific client containing their current visible entities.
/// This will eventually be serialized and sent over the network.
#[derive(Debug, Clone)]
pub struct VisibleEntitiesUpdate {
    pub client_entity_id: u64,      // The player's entity on the server
    pub visible_entity_ids: Vec<u64>,
    pub server_tick: u64,
}

/// System that generates VisibleEntitiesUpdate from InterestManager.
/// This should be called after InterestManager has been updated each tick.
pub fn generate_visible_entities_updates(
    interest_manager: &InterestManager,
    connected_players: &HashMap<u64, u64>, // player_id -> entity_id mapping
    current_tick: u64,
) -> Vec<VisibleEntitiesUpdate> {
    let mut updates = Vec::new();

    for (&player_id, &player_entity) in connected_players.iter() {
        // Get visible entities for this player from InterestManager
        let visible = interest_manager.get_visible_entities(player_entity);

        updates.push(VisibleEntitiesUpdate {
            client_entity_id: player_entity,
            visible_entity_ids: visible,
            server_tick: current_tick,
        });
    }

    updates
}

/// Placeholder for sending the update through the replication/network layer.
/// In a full implementation, this would serialize VisibleEntitiesUpdate
/// and send it to the specific client via the networking system.
pub fn send_visible_entities_update(update: &VisibleEntitiesUpdate) {
    // TODO: Integrate with actual replication/networking layer
    // Example future call:
    // replication.send_to_client(update.client_entity_id, update);
    debug!(
        "[InterestReplication] Would send {} visible entities to player entity {}",
        update.visible_entity_ids.len(),
        update.client_entity_id
    );
}

// End of interest_replication_bridge.rs
// Ready to be wired into the main server tick loop and networking layer.
// Thunder locked in. Yoi ⚡
