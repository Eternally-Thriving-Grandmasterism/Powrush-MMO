/*!
 * client/src/inventory_replication.rs
 * Server confirmation logic: Always treat incoming InventoryUpdate as authoritative truth.
 */

use bevy::prelude::*;
use shared::protocol::{ServerMessage, HotbarSlot};

#[derive(Resource, Default)]
pub struct ClientHotbar {
    pub slots: [HotbarSlot; 8],
}

#[derive(Resource, Default)]
pub struct ClientInventory {
    pub slots: [HotbarSlot; 40],
}

#[derive(Resource)]
pub struct HotbarSyncFlash {
    pub timer: Timer,
}

impl Default for HotbarSyncFlash {
    fn default() -> Self {
        Self { timer: Timer::from_seconds(0.6, TimerMode::Once) }
    }
}

/// Server confirmation logic.
/// Every InventoryUpdate from the server is treated as the single source of truth.
/// Any optimistic local changes are overwritten by the authoritative server state.
pub fn receive_inventory_update(
    mut events: EventReader<crate::server_message_dispatcher::ServerMessageEvent>,
    mut client_hotbar: ResMut<ClientHotbar>,
    mut client_inventory: ResMut<ClientInventory>,
    mut flash: ResMut<HotbarSyncFlash>,
) {
    for crate::server_message_dispatcher::ServerMessageEvent(msg) in events.read() {
        if let ServerMessage::InventoryUpdate { hotbar, inventory, abundance_score, .. } = msg {
            // === SERVER CONFIRMATION ===
            // Always replace local state with server's authoritative version.
            // This reconciles any optimistic predictions made during drag & drop.
            client_hotbar.slots = hotbar.clone();
            client_inventory.slots = inventory.clone();

            // Trigger visual confirmation flash
            flash.timer.reset();
            flash.timer.unpause();

            debug!("[InventoryReplication] Server confirmation received | abundance={:.1}", abundance_score);
        }
    }
}

// End of inventory_replication.rs