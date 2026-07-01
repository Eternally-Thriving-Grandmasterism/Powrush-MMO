/*!
 * client/src/inventory_replication.rs
 * Triggers visual sync flash when hotbar is updated from server.
 */

use bevy::prelude::*;
use crate::server_message_dispatcher::ServerMessageEvent;
use shared::protocol::ServerMessage;

/// Resource to drive hotbar sync flash effect
#[derive(Resource)]
pub struct HotbarSyncFlash {
    pub timer: Timer,
}

impl Default for HotbarSyncFlash {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.6, TimerMode::Once),
        }
    }
}

pub fn receive_inventory_update(
    mut events: EventReader<ServerMessageEvent>,
    mut client_hotbar: ResMut<ClientHotbar>,
    mut flash: ResMut<HotbarSyncFlash>,
) {
    for ServerMessageEvent(msg) in events.read() {
        if let ServerMessage::InventoryUpdate { hotbar, .. } = msg {
            client_hotbar.slots = hotbar.clone();

            // Trigger visual flash
            flash.timer.reset();
            flash.timer.unpause();

            info!("[InventoryReplication] Hotbar synced from server - flash triggered");
        }
    }
}

// End of inventory_replication.rs