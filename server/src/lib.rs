/*!
 * server/src/lib.rs
 * v21.80.0 — Complete mpsc-to-Bevy Event bridge + inventory processing + public Ra-Thor integration.
 * SafetyNet emission now correctly passed through to handle_inventory_action for severe cases only.
 * rathor_integration (CohostExportMirror, CouncilRttInbox, transfer session) now public for unified cohost.
 * All prior bridge + detector logic preserved. AG-SML v1.0 | TOLC 8 + RBE + PATSAGi
 */

use bevy::prelude::*;
use tokio::sync::mpsc;
use crate::inventory_replication::handle_inventory_action;
use crate::persistence_polish::PersistenceManager;
use crate::mercy_anomaly_detector::MercyAnomalyDetector;
use crate::safety_net_broadcast::EmitSafetyNetBroadcast;
use crate::network::tokio_transport::{TransportEvent, TransportCommand};

// Public Ra-Thor / PATSAGi / RTT cohost surface
pub mod rathor_integration;

#[derive(Resource)]
pub struct TransportEventReceiver {
    pub rx: mpsc::UnboundedReceiver<TransportEvent>,
}

pub struct ServerCorePlugin;

impl Plugin for ServerCorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<TransportEvent>()
            .add_event::<EmitSafetyNetBroadcast>()
            .init_resource::<Option<TransportEventReceiver>>()
            .init_resource::<MercyAnomalyDetector>()
            .add_systems(Startup, setup_transport_bridge)
            .add_systems(Update, (
                transport_event_bridge,
                process_inventory_messages,
            ));
    }
}

fn setup_transport_bridge(mut commands: Commands) {
    info!("[ServerCore] Transport bridge setup ready (receiver injection point).");
}

fn transport_event_bridge(
    mut receiver: ResMut<Option<TransportEventReceiver>>,
    mut event_writer: EventWriter<TransportEvent>,
) {
    if let Some(recv) = receiver.as_mut() {
        while let Ok(event) = recv.rx.try_recv() {
            event_writer.send(event);
        }
    }
}

/// Process inventory actions. Now correctly forwards safety_net_writer so that
/// severe ModerationAction cases emit SafetyNetBroadcast.
fn process_inventory_messages(
    mut transport_events: EventReader<TransportEvent>,
    mut persistence: ResMut<PersistenceManager>,
    mut detector: ResMut<MercyAnomalyDetector>,
    mut safety_net_writer: EventWriter<EmitSafetyNetBroadcast>,
) {
    for event in transport_events.read() {
        if let TransportEvent::MessageReceived { player_id, message } = event {
            // Pass the safety_net_writer so handle_inventory_action can emit on severe cases
            if let Some(reply) = handle_inventory_action(
                *player_id,
                message,
                &mut persistence,
                &mut detector,
                &mut safety_net_writer,
            ) {
                debug!("[Server] Inventory action processed for player {} -> reply generated", player_id);
            }
        }
    }
}

// End of server/src/lib.rs v21.80.0 — rathor_integration public for cohost. Thunder locked in. Yoi ⚡
