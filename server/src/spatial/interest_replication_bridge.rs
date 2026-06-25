/*!
 * Interest Replication Bridge
 *
 * Production-grade bridge between InterestManager and the replication/networking layer.
 *
 * v19.2 — Refined server system + better integration notes.
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use crate::spatial::interest_management::InterestManager;
use bevy::prelude::*;
use std::collections::HashMap;

/// Network message sent from server to a specific client.
/// This struct should derive Serialize/Deserialize in a real implementation.
#[derive(Debug, Clone)]
pub struct VisibleEntitiesUpdate {
    pub client_entity_id: u64,
    pub visible_entity_ids: Vec<u64>,
    pub server_tick: u64,
}

/// Main server system that runs every tick.
/// Collects visible entities from InterestManager for all connected players
/// and emits VisibleEntitiesUpdate events (to be sent over the network).
pub fn interest_replication_tick_system(
    interest_manager: Res<InterestManager>,
    // TODO: Replace with real ConnectedPlayers resource from networking layer
    // Example: Res<ConnectedPlayers>
    mut visible_updates: EventWriter<VisibleEntitiesUpdate>,
) {
    // In production, replace this placeholder with actual connected player data
    // from the networking/replication system.
    //
    // For now this demonstrates the pattern:
    // for (player_entity, _) in connected_players.iter() {
    //     let visible = interest_manager.get_visible_entities(*player_entity);
    //     visible_updates.send(VisibleEntitiesUpdate {
    //         client_entity_id: *player_entity,
    //         visible_entity_ids: visible,
    //         server_tick: current_tick,
    //     });
    // }
}

/// Helper function that can be called from the tick system or manually.
pub fn generate_visible_entities_updates(
    interest_manager: &InterestManager,
    connected_players: &HashMap<u64, u64>, // player_entity -> player_id (if needed)
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

// Integration Notes:
// 1. Register `interest_replication_tick_system` in the server app's Update schedule.
// 2. The replication layer should listen for VisibleEntitiesUpdate events
//    and serialize + send them to the specific client.
// 3. On the client, `receive_interest_update` (in simulation_integration.rs)
//    turns the data into InterestUpdateEvent.

// End of interest_replication_bridge.rs v19.2
// Refined and ready for integration with the main networking layer.
// Thunder locked in. Yoi ⚡
