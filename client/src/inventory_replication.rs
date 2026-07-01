/*!
 * client/src/inventory_replication.rs
 * Full wiring of ClientInventory from server InventoryUpdate.
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

pub fn receive_inventory_update(
    mut events: EventReader<crate::server_message_dispatcher::ServerMessageEvent>,
    mut client_hotbar: ResMut<ClientHotbar>,
    mut client_inventory: ResMut<ClientInventory>,
    mut flash: ResMut<HotbarSyncFlash>,
) {
    for crate::server_message_dispatcher::ServerMessageEvent(msg) in events.read() {
        if let ServerMessage::InventoryUpdate { hotbar, inventory, abundance_score, .. } = msg {
            client_hotbar.slots = hotbar.clone();
            client_inventory.slots = inventory.clone();

            // Trigger hotbar flash
            flash.timer.reset();
            flash.timer.unpause();

            info!("[InventoryReplication] Full inventory + hotbar synced | abundance={:.1}", abundance_score);
        }
    }
}

// End of inventory_replication.rs