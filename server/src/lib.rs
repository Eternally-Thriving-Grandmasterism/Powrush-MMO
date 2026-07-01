/*!
 * server/src/lib.rs
 * Wired inventory replication handler into ServerCorePlugin.
 */

use bevy::prelude::*;
use crate::inventory_replication::{handle_inventory_action, ClientHotbar}; // example

// ... existing code ...

pub struct ServerCorePlugin;

impl Plugin for ServerCorePlugin {
    fn build(&self, app: &mut App) {
        // ... existing resources and plugins ...

        // Inventory replication message handler
        app.add_systems(Update, process_inventory_messages);
    }
}

/// Processes incoming inventory-related ClientMessages using our authoritative handler.
/// TODO: Connect this to real TransportEvent::MessageReceived when available.
fn process_inventory_messages(
    // mut transport_events: EventReader<TransportEvent>,
    // mut transport_commands: ResMut<TransportCommandSender>,
) {
    // Example integration:
    // for event in transport_events.read() {
    //     if let TransportEvent::MessageReceived { player_id, message } = event {
    //         if let Some(reply) = handle_inventory_action(*player_id, message, &mut persistence) {
    //             // Send reply via transport command
    //         }
    //     }
    // }

    debug!("[Server] Inventory message handler active (waiting for TransportEvent wiring)");
}

// End of server/src/lib.rs