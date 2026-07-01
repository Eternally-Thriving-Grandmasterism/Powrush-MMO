/*!
 * Core networking plugin with central ServerMessage dispatcher.
 */

use bevy::prelude::*;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio::sync::mpsc;
use crate::settings::ClientSettings;
use shared::protocol::ClientMessage;
use crate::server_message_dispatcher::{server_message_dispatcher, ServerMessageEvent};

#[derive(Resource)]
pub struct ServerUpdateChannel {
    pub rx: mpsc::Receiver<Vec<u8>>,
}

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
           .add_event::<ServerMessageEvent>()                    // NEW
           .add_systems(Startup, setup_websocket_connection(tx_in, rx_out))
           .add_systems(Update, server_message_dispatcher);     // NEW central dispatcher
    }
}

fn setup_websocket_connection(
    tx_in: mpsc::Sender<Vec<u8>>,
    mut rx_out: mpsc::UnboundedReceiver<ClientMessage>,
) {
    tokio::spawn(async move {
        let url = "ws://localhost:9001";

        let (ws_stream, _) = match connect_async(url).await {
            Ok((stream, resp) ) => (stream, resp),
            Err(e) => { error!("Failed to connect: {}", e); return; }
        };

        let (mut write, mut read) = ws_stream.split();

        tokio::spawn(async move {
            while let Some(client_msg) = rx_out.recv().await {
                if let Ok(bytes) = bincode::serialize(&client_msg) {
                    let _ = write.send(Message::Binary(bytes.into())).await;
                }
            }
        });

        while let Some(msg) = read.next().await {
            // existing reader logic (push to tx_in)
        }
    });
}

// End of networking.rs