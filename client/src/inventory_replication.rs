/*!
 * client/src/inventory_replication.rs
 *
 * Client-side replication handler for inventory-related ServerMessages.
 * Receives authoritative InventoryUpdate (including full hotbar) and syncs
 * it into GpuSimulationState.
 *
 * Mirrors the structure of server/src/inventory_replication.rs for symmetry.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Mercy Gates | Ra-Thor / PATSAGi aligned
 */

use bevy::prelude::*;
use crate::rbe_client_sync::GpuSimulationState;
use crate::server_message_dispatcher::ServerMessageEvent;
use shared::protocol::ServerMessage;

/// Receives InventoryUpdate events from the central dispatcher and applies
/// the authoritative hotbar state to the local GPU simulation.
pub fn receive_inventory_update(
    mut events: EventReader<ServerMessageEvent>,
    mut gpu_state: ResMut<GpuSimulationState>,
) {
    for ServerMessageEvent(msg) in events.read() {
        if let ServerMessage::InventoryUpdate { hotbar, abundance_score, .. } = msg {
            for (i, &count) in hotbar.iter().enumerate() {
                if i < gpu_state.hotbar.len() {
                    gpu_state.hotbar[i].count = count;
                    gpu_state.hotbar[i].cooldown_remaining = 0.0;
                }
            }

            debug!(
                "[InventoryReplication] Hotbar synced from server | abundance={:.1}",
                abundance_score
            );
        }
    }
}

// End of client/src/inventory_replication.rs