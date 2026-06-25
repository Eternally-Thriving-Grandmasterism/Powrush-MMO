/*!
 * Interest Replication Bridge
 *
 * v19.1 — Added server tick system (Step 1) + message format (Step 2).
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use crate::spatial::interest_management::InterestManager;
use bevy::prelude::*;
use std::collections::HashMap;

/// Data structure sent to a specific client containing their current visible entities.
/// This is the network message format (Step 2).
#[derive(Debug, Clone)]
pub struct VisibleEntitiesUpdate {
    pub client_entity_id: u64,
    pub visible_entity_ids: Vec<u64>,
    pub server_tick: u64,
}

/// System that runs every tick and generates visibility updates for all connected players.
/// This is Step 1 of the replication bridge.
pub fn interest_replication_tick_system(
    interest_manager: Res<InterestManager>,
    // TODO: Replace with actual connected players resource from networking layer
    // For now we use a placeholder
    mut visible_updates: EventWriter<VisibleEntitiesUpdate>,
) {
    // In a real implementation, we would get the list of connected players
    // from the networking/replication layer.
    // For now this is a scaffold.

    // Placeholder: In production this would iterate over actual connected clients
    // and call interest_manager.get_visible_entities(player_entity)

    // Example of what the real loop would look like:
    // for (player_id, player_entity) in connected_players.iter() {
    //     let visible = interest_manager.get_visible_entities(*player_entity);
    //     visible_updates.send(VisibleEntitiesUpdate {
    //         client_entity_id: *player_entity,
    //         visible_entity_ids: visible,
    //         server_tick: current_tick,
    //     });
    // }
}

/// Helper to generate updates (can be called from the system above)
pub fn generate_visible_entities_updates(
    interest_manager: &InterestManager,
    connected_players: &HashMap<u64, u64>,
    current_tick: u64,
) -> Vec<VisibleEntitiesUpdate> {
    let mut updates = Vec::new();

    for (&player_entity, _) in connected_players.iter() {
        let visible = interest_manager.get_visible_entities(player_entity);

        updates.push(VisibleEntitiesUpdate {
            client_entity_id: player_entity,
            visible_entity_ids: visible,
            server_tick: current_tick,
        });
    }

    updates
}

// End of interest_replication_bridge.rs v19.1
// Step 1 (server tick system) and Step 2 (message format) complete.
// Ready for client-side receiver (Step 3).
// Thunder locked in. Yoi ⚡
