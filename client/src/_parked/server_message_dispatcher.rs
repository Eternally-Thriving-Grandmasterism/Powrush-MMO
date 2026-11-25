/*!
 * client/src/server_message_dispatcher.rs
 *
 * Central dispatcher for incoming ServerMessage.
 * Deserializes bytes from ServerUpdateChannel and emits ServerMessageEvent.
 * All other systems should listen to this event instead of deserializing themselves.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Mercy Gates
 */

use bevy::prelude::*;
use tokio::sync::mpsc;
use bincode;
use shared::protocol::ServerMessage;
use crate::networking::ServerUpdateChannel;

/// Wrapper event so multiple systems can react to server messages cleanly.
#[derive(Event, Debug, Clone)]
pub struct ServerMessageEvent(pub ServerMessage);

/// System that drains the raw byte channel and emits typed ServerMessageEvent.
pub fn server_message_dispatcher(
    mut update_channel: ResMut<ServerUpdateChannel>,
    mut events: EventWriter<ServerMessageEvent>,
) {
    while let Ok(bytes) = update_channel.rx.try_recv() {
        match bincode::deserialize::<ServerMessage>(&bytes) {
            Ok(msg) => {
                events.send(ServerMessageEvent(msg));
            }
            Err(e) => {
                warn!("[Dispatcher] Failed to deserialize ServerMessage: {}", e);
            }
        }
    }
}

// End of server_message_dispatcher.rs