/*!
 * server/src/lib.rs
 * v21.89.3 — Inventory + AudioMoment + NEVC attachment (Finish Pass A).
 * SafetyNet emission preserved. rathor_integration public for unified cohost.
 * AG-SML v1.0 | TOLC 8 + RBE + PATSAGi | info@Rathor.ai
 */

use bevy::prelude::*;
use tokio::sync::mpsc;
use crate::inventory_replication::handle_inventory_action;
use crate::persistence_polish::PersistenceManager;
use crate::mercy_anomaly_detector::MercyAnomalyDetector;
use crate::safety_net_broadcast::EmitSafetyNetBroadcast;
use crate::network::tokio_transport::{TransportEvent, TransportCommand};
use crate::audio_moment_catalog::{AudioMomentCatalogPlugin, ServerAudioMomentStore};
use crate::audio_moment_net_handler::route_client_audio_message;
use shared::protocol::ClientMessage;

// Public Ra-Thor / PATSAGi / RTT cohost surface
pub mod rathor_integration;

pub mod audio_moment_catalog;
pub mod audio_moment_net_handler;

// Finish Pass A: NEVC live attachment (shared-backed, no algorithm mirror)
pub mod nevc_attachment;

#[derive(Resource)]
pub struct TransportEventReceiver {
    pub rx: mpsc::UnboundedReceiver<TransportEvent>,
}

/// Injected by host bootstrap so simulation can reply to clients
#[derive(Resource)]
pub struct TransportCommandSender {
    pub tx: mpsc::UnboundedSender<TransportCommand>,
}

pub struct ServerCorePlugin;

impl Plugin for ServerCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioMomentCatalogPlugin)
            .add_event::<TransportEvent>()
            .add_event::<EmitSafetyNetBroadcast>()
            .init_resource::<Option<TransportEventReceiver>>()
            .init_resource::<MercyAnomalyDetector>()
            .add_systems(Startup, setup_transport_bridge)
            .add_systems(
                Update,
                (
                    transport_event_bridge,
                    process_inventory_messages,
                    process_audio_moment_messages,
                ),
            );
    }
}

fn setup_transport_bridge(mut commands: Commands) {
    info!("[ServerCore] Transport bridge setup ready (receiver + audio catalog + NEVC attachment)."
    );
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

/// Process inventory actions. Forwards safety_net_writer for severe cases.
fn process_inventory_messages(
    mut transport_events: EventReader<TransportEvent>,
    mut persistence: ResMut<PersistenceManager>,
    mut detector: ResMut<MercyAnomalyDetector>,
    mut safety_net_writer: EventWriter<EmitSafetyNetBroadcast>,
) {
    for event in transport_events.read() {
        if let TransportEvent::MessageReceived { player_id, message } = event {
            if let Some(_reply) = handle_inventory_action(
                *player_id,
                message,
                &mut persistence,
                &mut detector,
                &mut safety_net_writer,
            ) {
                debug!(
                    "[Server] Inventory action processed for player {}",
                    player_id
                );
            }
        }
    }
}

/// Route AudioMomentSave / CatalogRequest / SetFavorite → store + TransportCommand replies
fn process_audio_moment_messages(
    mut transport_events: EventReader<TransportEvent>,
    mut store: ResMut<ServerAudioMomentStore>,
    command_tx: Option<Res<TransportCommandSender>>,
) {
    for event in transport_events.read() {
        let TransportEvent::MessageReceived { player_id, message } = event else {
            continue;
        };

        let mut msg = message.clone();
        if let ClientMessage::AudioMomentSave { ref mut moment } = msg {
            if moment.owner_player_id == 0 {
                moment.owner_player_id = *player_id;
            }
        }

        let replies = route_client_audio_message(&msg, &mut store);
        if replies.is_empty() {
            continue;
        }

        if let Some(sender) = command_tx.as_ref() {
            for reply in replies {
                let _ = sender.tx.send(TransportCommand::Send {
                    player_id: *player_id,
                    message: reply,
                });
            }
            debug!(
                target: "powrush::audio",
                player_id,
                "Audio moment message routed + replies queued"
            );
        } else {
            info!(
                target: "powrush::audio",
                player_id,
                reply_count = replies.len(),
                "Audio moment handled (no TransportCommandSender yet — replies held)"
            );
        }
    }
}

// End of server/src/lib.rs v21.89.3 — NEVC attachment declared. Thunder locked in. Yoi ⚡
