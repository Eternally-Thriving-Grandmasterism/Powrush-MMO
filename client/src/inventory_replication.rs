/*!
 * client/src/inventory_replication.rs
 * Rollback logic for conflicting slots during server reconciliation.
 */

use bevy::prelude::*;
use shared::protocol::HotbarSlot;

// ... existing resources (ClientHotbar, ClientInventory, HotbarSyncFlash) ...

/// Server reconciliation with rollback for conflicting slots.
/// When server state differs from local prediction, we rollback to authoritative value.
pub fn receive_inventory_update(
    mut events: EventReader<crate::server_message_dispatcher::ServerMessageEvent>,
    mut client_hotbar: ResMut<ClientHotbar>,
    mut client_inventory: ResMut<ClientInventory>,
    mut flash: ResMut<HotbarSyncFlash>,
) {
    for crate::server_message_dispatcher::ServerMessageEvent(msg) in events.read() {
        if let crate::server_message_dispatcher::ServerMessage::InventoryUpdate { hotbar, inventory, abundance_score, .. } = msg {
            let mut rolled_back = false;

            // Check hotbar for conflicts
            for (i, server_slot) in hotbar.iter().enumerate() {
                if i < client_hotbar.slots.len() {
                    let local = &client_hotbar.slots[i];
                    if local != server_slot {
                        // Conflict detected → rollback to server authority
                        warn!(
                            "[Rollback] Hotbar slot {} conflicted. Local: item_id={}, count={}. Server: item_id={}, count={}",
                            i, local.item_id, local.count, server_slot.item_id, server_slot.count
                        );
                        client_hotbar.slots[i] = server_slot.clone();
                        rolled_back = true;
                    }
                }
            }

            // Check general inventory for conflicts
            for (i, server_slot) in inventory.iter().enumerate() {
                if i < client_inventory.slots.len() {
                    let local = &client_inventory.slots[i];
                    if local != server_slot {
                        warn!(
                            "[Rollback] Inventory slot {} conflicted. Rolling back to server state.",
                            i
                        );
                        client_inventory.slots[i] = server_slot.clone();
                        rolled_back = true;
                    }
                }
            }

            if rolled_back {
                flash.timer.reset();
                flash.timer.unpause();
                debug!("[InventoryReplication] Rollback applied | abundance={:.1}", abundance_score);
            }
        }
    }
}

// End of inventory_replication.rs