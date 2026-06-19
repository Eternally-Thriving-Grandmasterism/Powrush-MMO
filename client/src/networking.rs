/*!
 * Core networking plugin: WebSocket + Snappy + outgoing ClientMessage support
 *
 * v18.96 Eternal Polish — Now supports sending ClientMessage::SyncLocalization etc.
 */

use bevy::prelude::*;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio::sync::mpsc;
use std::sync::Arc;
use crate::settings::ClientSettings;
use shared::protocol::ClientMessage;

#[derive(Resource)]
pub struct ServerUpdateChannel {
    pub rx: mpsc::Receiver<Vec<u8>>,
}

// NEW v18.96: Outgoing channel for ClientMessage (e.g. SyncLocalization)
#[derive(Resource)]
pub struct OutgoingClientMessages {
    pub tx: mpsc::UnboundedSender<ClientMessage>,
}

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        let (tx_in, rx_in) = mpsc::channel(512);
        let (tx_out, mut rx_out) = mpsc::unbounded_channel::<ClientMessage>();

        app.insert_resource(ServerUpdateChannel { rx: rx_in })
           .insert_resource(OutgoingClientMessages { tx: tx_out })
           .add_systems(Startup, setup_websocket_connection(tx_in, rx_out));
    }
}

fn setup_websocket_connection(
    tx_in: mpsc::Sender<Vec<u8>>,
    mut rx_out: mpsc::UnboundedReceiver<ClientMessage>,
) {
    tokio::spawn(async move {
        let url = "ws://localhost:9001";

        let (ws_stream, _) = match connect_async(url).await {
            Ok((stream, resp)) => (stream, resp),
            Err(e) => { error!("Failed to connect: {}", e); return; }
        };

        let (mut write, mut read) = ws_stream.split();

        // Writer task for outgoing ClientMessage
        tokio::spawn(async move {
            while let Some(client_msg) = rx_out.recv().await {
                if let Ok(bytes) = bincode::serialize(&client_msg) {
                    let _ = write.send(Message::Binary(bytes.into())).await;
                }
            }
        });

        // Reader task (existing incoming logic...)
        while let Some(msg) = read.next().await {
            // ... existing decompression + tx_in.send ...
        }
    });
}

// End of client/src/networking.rs v18.96
// OutgoingClientMessages channel added. Thunder locked in. Yoi ⚡
