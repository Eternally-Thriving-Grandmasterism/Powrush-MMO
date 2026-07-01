/*!
 * client/src/inventory_replication.rs
 * Delta reconciliation logic for InventoryUpdate.
 */

use bevy::prelude::*;
use shared::protocol::{ServerMessage, HotbarSlot};

// ... existing resources ...

/// Server confirmation with delta reconciliation.
/// Compares incoming server state with local state and only applies differences.
pub fn receive_inventory_update(
    mut events: EventReader<crate::server_message_dispatcher::ServerMessageEvent>,
    mut client_hotbar: ResMut<ClientHotbar>,
    mut client_inventory: ResMut<ClientInventory>,
    mut flash: ResMut<HotbarSyncFlash>,
) {
    for crate::server_message_dispatcher::ServerMessageEvent(msg) in events.read() {
        if let ServerMessage::InventoryUpdate { hotbar, inventory, abundance_score, .. } = msg {
            let mut changed = false;

            // Delta reconcile hotbar
            for (i, server_slot) in hotbar.iter().enumerate() {
                if i < client_hotbar.slots.len() && client_hotbar.slots[i] != *server_slot {
                    client_hotbar.slots[i] = server_slot.clone();
                    changed = true;
                }
            }

            // Delta reconcile general inventory
            for (i, server_slot) in inventory.iter().enumerate() {
                if i < client_inventory.slots.len() && client_inventory.slots[i] != *server_slot {
                    client_inventory.slots[i] = server_slot.clone();
                    changed = true;
                }
            }

            if changed {
                flash.timer.reset();
                flash.timer.unpause();
                debug!("[InventoryReplication] Delta reconciliation applied changes | abundance={:.1}", abundance_score);
            } else {
                trace!("[InventoryReplication] InventoryUpdate received with no changes");
            }
        }
    }
}

// End of inventory_replication.rs