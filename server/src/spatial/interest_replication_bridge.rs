/*!
 * Interest Replication Bridge
 *
 * v19.3 — Now uses shared VisibleEntitiesUpdate from simulation crate.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use crate::spatial::interest_management::InterestManager;
use bevy::prelude::*;
use simulation::interest::VisibleEntitiesUpdate;
use std::collections::HashMap;

/// Main server system that generates visibility updates.
pub fn interest_replication_tick_system(
    interest_manager: Res<InterestManager>,
    mut visible_updates: EventWriter<VisibleEntitiesUpdate>,
) {
    // Production implementation would iterate over connected players
    // and emit VisibleEntitiesUpdate for each.
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

// End of interest_replication_bridge.rs v19.3
// Uses shared type from simulation::interest.
// Thunder locked in. Yoi ⚡
