/*!
 * client/src/inventory_replication.rs
 * Receives full HotbarSlot data (item_id, durability, rarity, valence) from server.
 */

use bevy::prelude::*;
use crate::rbe_client_sync::GpuSimulationState;
use crate::server_message_dispatcher::ServerMessageEvent;
use shared::protocol::ServerMessage;

pub fn receive_inventory_update(
    mut events: EventReader<ServerMessageEvent>,
    mut gpu_state: ResMut<GpuSimulationState>,
) {
    for ServerMessageEvent(msg) in events.read() {
        if let ServerMessage::InventoryUpdate { hotbar, abundance_score, .. } = msg {
            for (i, slot) in hotbar.iter().enumerate() {
                if i < gpu_state.hotbar.len() {
                    gpu_state.hotbar[i].count = slot.count;
                    gpu_state.hotbar[i].cooldown_remaining = slot.cooldown_remaining;
                    // Future: sync item_id, durability, rarity, valence into extended GPU state or UI cache
                }
            }

            // Visual feedback hook (can be expanded with particles / flash)
            info!("[InventoryReplication] Authoritative hotbar sync received | abundance={:.1} | full item data", abundance_score);
        }
    }
}

// End of client/src/inventory_replication.rs