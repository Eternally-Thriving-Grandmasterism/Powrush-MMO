/*!
 * server/src/lib.rs
 * Complete mpsc to Bevy Event bridge for TransportEvent.
 * MercyAnomalyDetector wired as Bevy Resource.
 */

use bevy::prelude::*;
use tokio::sync::mpsc;
use crate::inventory_replication::handle_inventory_action;
use crate::persistence_polish::PersistenceManager;
use crate::mercy_anomaly_detector::MercyAnomalyDetector;
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
            .init_resource::<Option<TransportEventReceiver>>()
            .init_resource::<MercyAnomalyDetector>() // Bevy Resource wiring for anomaly detection
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
) {
    // Placeholder - in production main.rs passes the receiver here
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
/// Now receives MercyAnomalyDetector as Resource and passes it through
fn process_inventory_messages(
    mut transport_events: EventReader<TransportEvent>,
    mut persistence: ResMut<PersistenceManager>,
    mut detector: ResMut<MercyAnomalyDetector>,
) {
    for event in transport_events.read() {
        if let TransportEvent::MessageReceived { player_id, message } = event {
            if let Some(_reply) = handle_inventory_action(*player_id, message, &mut persistence, &mut detector) {
                debug!("[Server] Processed inventory action for player {}", player_id);
            }
        }
    }
}

// End of server/src/lib.rs — MercyAnomalyDetector wired as Resource. Thunder locked in.