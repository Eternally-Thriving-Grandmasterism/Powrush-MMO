// game/src/network/client_transport.rs
// Powrush-MMO — Client Networking Transport Layer v2.3
// try_recv for poll loops + protocol-aligned HandshakeRequest
// Dual-target: native + WASM (web-sys)
// AG-SML v1.0 | TOLC 8 | Permanent PATSAGi | Contact: info@Rathor.ai

use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tracing::{info, warn, error};

use shared::protocol::*;

#[cfg(not(target_arch = "wasm32"))]
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WsMessage};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;
#[cfg(target_arch = "wasm32")]
use web_sys::{WebSocket, MessageEvent, BinaryType};
#[cfg(target_arch = "wasm32")]
use js_sys::Uint8Array;

pub struct ClientWsTransport {
    pub player_id: Option<u64>,
    tx_out: mpsc::UnboundedSender<ClientMessage>,
    rx_in: mpsc::UnboundedReceiver<ServerMessage>,
    shutdown: Arc<tokio::sync::Notify>,
    #[cfg(target_arch = "wasm32")]
    ws: Option<WebSocket>,
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

impl ClientWsTransport {
    pub async fn connect(url: &str, player_name: &str) -> Result<(Self, u64), String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let (ws_stream, _) = connect_async(url)
                .await
                .map_err(|e| format!("WebSocket connect failed: {}", e))?;
            let (mut write, mut read) = ws_stream.split();
            let (tx_out, mut rx_out) = mpsc::unbounded_channel::<ClientMessage>();
            let (tx_in, rx_in) = mpsc::unbounded_channel::<ServerMessage>();
            let shutdown = Arc::new(tokio::sync::Notify::new());

            let handshake = ClientMessage::HandshakeRequest {
                version: PROTOCOL_VERSION,
                player_name: player_name.to_string(),
                client_time_ms: now_ms(),
            };
            let bytes = bincode::serialize(&handshake)
                .map_err(|e| format!("Handshake serialize failed: {}", e))?;
            write
                .send(WsMessage::Binary(bytes.into()))
                .await
                .map_err(|e| format!("Handshake send failed: {}", e))?;

            // Writer task
            tokio::spawn(async move {
                while let Some(msg) = rx_out.recv().await {
                    if !apply_mercy_gate(&msg, 0.8) {
                        continue;
                    }
                    if let Ok(bytes) = bincode::serialize(&msg) {
                        if write.send(WsMessage::Binary(bytes.into())).await.is_err() {
                            break;
                        }
                    }
                }
            });

            // Reader task
            let tx_in_reader = tx_in.clone();
            tokio::spawn(async move {
                while let Some(msg_result) = read.next().await {
                    match msg_result {
                        Ok(WsMessage::Binary(bytes)) => {
                            if let Ok(server_msg) = bincode::deserialize::<ServerMessage>(&bytes) {
                                let _ = tx_in_reader.send(server_msg);
                            }
                        }
                        Ok(WsMessage::Close(_)) | Err(_) => break,
                        _ => {}
                    }
                }
            });

            // Heartbeat
            let tx_hb = tx_out.clone();
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(10)).await;
                    let _ = tx_hb.send(ClientMessage::Ping {
                        client_time_ms: now_ms(),
                    });
                }
            });

            info!("[ClientTransport v2.3 Native] Connected to {}", url);
            Ok((
                Self {
                    player_id: None,
                    tx_out,
                    rx_in,
                    shutdown,
                },
                0,
            ))
        }

        #[cfg(target_arch = "wasm32")]
        {
            let ws = WebSocket::new(url)
                .map_err(|e| format!("web-sys WebSocket creation failed: {:?}", e))?;
            ws.set_binary_type(BinaryType::Arraybuffer);

            let (tx_out, mut rx_out) = mpsc::unbounded_channel::<ClientMessage>();
            let (tx_in, rx_in) = mpsc::unbounded_channel::<ServerMessage>();
            let shutdown = Arc::new(tokio::sync::Notify::new());

            let handshake = ClientMessage::HandshakeRequest {
                version: PROTOCOL_VERSION,
                player_name: player_name.to_string(),
                client_time_ms: now_ms(),
            };
            let bytes = bincode::serialize(&handshake)
                .map_err(|e| format!("Handshake serialize failed: {}", e))?;
            let array = Uint8Array::from(&bytes[..]);
            ws.send_with_u8_array(&array)
                .map_err(|e| format!("Handshake send failed: {:?}", e))?;

            use std::cell::RefCell;
            use std::rc::Rc;
            let player_id_cell: Rc<RefCell<Option<u64>>> = Rc::new(RefCell::new(None));
            let player_id_cell_clone = player_id_cell.clone();
            let tx_in_clone = tx_in.clone();

            let onmessage_callback = Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
                if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                    let array = Uint8Array::new(&abuf);
                    let mut data = vec![0; array.length() as usize];
                    array.copy_to(&mut data);
                    if let Ok(server_msg) = bincode::deserialize::<ServerMessage>(&data) {
                        if let ServerMessage::HandshakeResponse { player_id, accepted, .. } =
                            &server_msg
                        {
                            if *accepted {
                                *player_id_cell_clone.borrow_mut() = Some(*player_id);
                                info!(
                                    "[WASM] Handshake complete. player_id = {}",
                                    player_id
                                );
                            }
                        }
                        let _ = tx_in_clone.send(server_msg);
                    }
                }
            });
            ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
            onmessage_callback.forget();

            let onerror_callback = Closure::<dyn FnMut(_)>::new(|e: web_sys::ErrorEvent| {
                error!("[ClientTransport WASM] WebSocket error: {:?}", e);
            });
            ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
            onerror_callback.forget();

            let onclose_callback = Closure::<dyn FnMut(_)>::new(|_| {
                warn!("[ClientTransport WASM] WebSocket closed");
            });
            ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
            onclose_callback.forget();

            // Capture ws for send loop
            let ws_send = ws.clone();
            spawn_local(async move {
                while let Some(msg) = rx_out.recv().await {
                    if !apply_mercy_gate(&msg, 0.8) {
                        continue;
                    }
                    if let Ok(bytes) = bincode::serialize(&msg) {
                        let array = Uint8Array::from(&bytes[..]);
                        if ws_send.send_with_u8_array(&array).is_err() {
                            break;
                        }
                    }
                }
            });

            let tx_out_hb = tx_out.clone();
            spawn_local(async move {
                loop {
                    gloo_timers::future::TimeoutFuture::new(10_000).await;
                    let _ = tx_out_hb.send(ClientMessage::Ping {
                        client_time_ms: now_ms(),
                    });
                }
            });

            let transport = Self {
                player_id: None,
                tx_out,
                rx_in,
                shutdown,
                ws: Some(ws),
            };

            info!("[ClientTransport v2.3 WASM] Connected. Awaiting HandshakeResponse.");
            Ok((transport, 0))
        }
    }

    pub fn send(&self, msg: ClientMessage) -> Result<(), String> {
        self.tx_out
            .send(msg)
            .map_err(|e| format!("Send failed: {}", e))
    }

    /// Non-blocking poll for server messages (call from game / WASM frame loop)
    pub fn try_recv(&mut self) -> Option<ServerMessage> {
        match self.rx_in.try_recv() {
            Ok(msg) => {
                if let ServerMessage::HandshakeResponse {
                    player_id,
                    accepted,
                    ..
                } = &msg
                {
                    if *accepted {
                        self.player_id = Some(*player_id);
                    }
                }
                Some(msg)
            }
            Err(_) => None,
        }
    }

    pub async fn recv(&mut self) -> Option<ServerMessage> {
        let msg = self.rx_in.recv().await?;
        if let ServerMessage::HandshakeResponse {
            player_id,
            accepted,
            ..
        } = &msg
        {
            if *accepted {
                self.player_id = Some(*player_id);
            }
        }
        Some(msg)
    }

    pub fn shutdown(&self) {
        self.shutdown.notify_one();
    }

    pub fn get_player_id(&self) -> Option<u64> {
        self.player_id
    }
}

// Thunder locked in. Yoi ⚡
