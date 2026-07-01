/*!
 * client/src/inventory_replication.rs
 * Now stores full HotbarSlot data from server for UI rendering.
 */

use bevy::prelude::*;
use crate::rbe_client_sync::GpuSimulationState;
use crate::server_message_dispatcher::ServerMessageEvent;
use shared::protocol::{ServerMessage, HotbarSlot};

/// Client-side storage for the latest authoritative hotbar from server.
#[derive(Resource, Default)]
pub struct ClientHotbar {
    pub slots: [HotbarSlot; 8],
}

pub fn receive_inventory_update(
    mut events: EventReader<ServerMessageEvent>,
    mut gpu_state: ResMut<GpuSimulationState>,
    mut client_hotbar: ResMut<ClientHotbar>,
) {
    for ServerMessageEvent(msg) in events.read() {
        if let ServerMessage::InventoryUpdate { hotbar, abundance_score, .. } = msg {
            // Store full rich data for UI
            client_hotbar.slots = hotbar.clone();

            // Keep minimal GPU state in sync for existing hotbar rendering
            for (i, slot) in hotbar.iter().enumerate() {
                if i < gpu_state.hotbar.len() {
                    gpu_state.hotbar[i].count = slot.count;
                    gpu_state.hotbar[i].cooldown_remaining = slot.cooldown_remaining;
                }
            }

            info!("[InventoryReplication] Full HotbarSlot data received and stored | abundance={:.1}", abundance_score);
        }
    }
}

// End of inventory_replication.rs