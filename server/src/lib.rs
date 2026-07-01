/*!
 * server/src/lib.rs
 * Complete mpsc to Bevy Event bridge for TransportEvent.
 */

use bevy::prelude::*;
use tokio::sync::mpsc;
use crate::inventory_replication::handle_inventory_action;
use crate::persistence_polish::PersistenceManager;
use server::network::tokio_transport::{TransportEvent, TransportCommand};

/// Resource holding the receiver from TokioTransport
#[derive(Resource)]
pub struct TransportEventReceiver {
    pub rx: mpsc::UnboundedReceiver<TransportEvent>,
}

pub struct ServerCorePlugin;

impl Plugin for ServerCorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<TransportEvent>()
            .init_resource::<Option<TransportEventReceiver>>() // Will be replaced at startup
            .add_systems(Startup, setup_transport_bridge)
            .add_systems(Update, (
                transport_event_bridge,
                process_inventory_messages,
            ));
    }
}

/// Startup system that receives the channel from main.rs
fn setup_transport_bridge(
    mut commands: Commands,
    // In real usage, this would be passed from main.rs after creating TokioTransport
) {
    // Placeholder - in production main.rs passes the receiver here
    // commands.insert_resource(TransportEventReceiver { rx: ... });
}

/// Bridge system: Drain mpsc and emit Bevy events every frame
fn transport_event_bridge(
    mut receiver: ResMut<TransportEventReceiver>,
    mut event_writer: EventWriter<TransportEvent>,
) {
    while let Ok(event) = receiver.rx.try_recv() {
        event_writer.send(event);
    }
}

/// Process inventory messages from TransportEvent
fn process_inventory_messages(
    mut transport_events: EventReader<TransportEvent>,
    mut persistence: ResMut<PersistenceManager>,
) {
    for event in transport_events.read() {
        if let TransportEvent::MessageReceived { player_id, message } = event {
            if let Some(_reply) = handle_inventory_action(*player_id, message, &mut persistence) {
                // TODO: Send reply using TransportCommand channel
                debug!("[Server] Processed inventory action for player {}", player_id);
            }
        }
    }
}

// End of server/src/lib.rs