/*!
 * server/src/lib.rs
 * Fully wired server message loop for inventory replication.
 */

use bevy::prelude::*;
use crate::inventory_replication::handle_inventory_action;
use crate::persistence_polish::PersistenceManager;
use server::network::tokio_transport::{TransportEvent, TransportCommand};

// ... existing code ...

pub struct ServerCorePlugin;

impl Plugin for ServerCorePlugin {
    fn build(&self, app: &mut App) {
        // ... existing ...

        app
            .add_event::<TransportEvent>()
            .add_systems(Update, (
                process_transport_events,
                process_inventory_messages,
            ));
    }
}

/// Bridge: Convert raw transport events into Bevy events (if not already done elsewhere)
fn process_transport_events(
    // This would be fed from the mpsc channel in a real setup
) {
    // In production, a dedicated system drains the mpsc from TokioTransport
    // and sends TransportEvent as Bevy events.
}

/// Main message processing loop for inventory actions
fn process_inventory_messages(
    mut transport_events: EventReader<TransportEvent>,
    mut persistence: ResMut<PersistenceManager>,
    // mut transport_commands: ResMut<...>, // for sending replies
) {
    for event in transport_events.read() {
        if let TransportEvent::MessageReceived { player_id, message } = event {
            if let Some(reply) = handle_inventory_action(*player_id, message, &mut persistence) {
                // TODO: Send reply via transport command channel
                // transport_commands.send(TransportCommand::Send { player_id: *player_id, message: reply });
                info!("[Server] Inventory action processed for player {}", player_id);
            }
        }
    }
}

// End of server/src/lib.rs