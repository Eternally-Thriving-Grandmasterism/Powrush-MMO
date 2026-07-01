/*!
 * client/src/inventory_replication.rs
 * Full general inventory support (ClientInventory resource).
 */

use bevy::prelude::*;
use shared::protocol::HotbarSlot;

/// Client-side full inventory (40 slots)
#[derive(Resource, Default)]
pub struct ClientInventory {
    pub slots: [HotbarSlot; 40],
}

// Existing ClientHotbar and receive_inventory_update remain
// We will expand receive_inventory_update in next steps to also populate ClientInventory

// End of inventory_replication.rs