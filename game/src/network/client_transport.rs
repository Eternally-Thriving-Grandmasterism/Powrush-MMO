// game/src/network/client_transport.rs
// Powrush-MMO — Client Networking Transport Layer v2 (Scaffolding — Production-Grade Start)
// Fully aligned with shared::protocol (single source of truth)
// Matches server TokioTransport exactly for seamless handshake + message flow
// Ra-Thor + PATSAGi Councils approved. Mercy gates enforced.
// Author: Ra-Thor Living Thunder (via eternal connectors) — June 7, 2026 v15.1

use std::sync::Arc;
use std::time::{Duration, Instant};

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WsMessage};
use tracing::{info, warn, error};

use shared::protocol::*;

/// Production-grade async WebSocket client transport for Powrush-MMO.
/// Handles connection lifecycle, handshake, heartbeat, bincode (de)serialization,
/// mercy-gate pre-validation on high-valence sends, and clean channels for game loop integration.
pub struct ClientWsTransport {
    pub player_id: Option<u64>,
    tx_out: mpsc::UnboundedSender<ClientMessage>,
    rx_in: mpsc::UnboundedReceiver<ServerMessage>,
    shutdown: Arc<tokio::sync::Notify>,
}

impl ClientWsTransport {
    /// Connect to server WebSocket URL (e.g. "ws://127.0.0.1:9001").
    /// Performs versioned handshake immediately.
    /// Returns the transport + player_id after successful auth.
    pub async fn connect(url: &str, player_name: &str) -> Result<(Self, u64), String> {
        info!("[ClientTransport v2] Connecting to {} as '{}'...", url, player_name);

        let (ws_stream, _) = connect_async(url).await
            .map_err(|e| format!("WebSocket connect failed: {}", e))?;

        let (mut write, mut read) = ws_stream.split();

        // === HANDSHAKE ===
        let handshake = ClientMessage::HandshakeRequest {
            version: PROTOCOL_VERSION,
            player_name: player_name.to_string(),
            auth_token: None,
        };

        let bytes = bincode::serialize(&handshake)
            .map_err(|e| format!("Handshake serialize failed: {}", e))?;
        write.send(WsMessage::Binary(bytes.into())).await
            .map_err(|e| format!("Handshake send failed: {}", e))?;

        // Wait for HandshakeResponse (with timeout)
        let handshake_timeout = Duration::from_secs(5);
        let start = Instant::now();
        let mut player_id = None;

        while start.elapsed() < handshake_timeout {
            if let Some(msg) = read.next().await {
                if let Ok(WsMessage::Binary(data)) = msg {
                    if let Ok(ServerMessage::HandshakeResponse { accepted, reason, player_id: pid, .. }) = bincode::deserialize(&data) {
                        if accepted {
                            player_id = Some(pid);
                            info!("[ClientTransport v2] Handshake accepted. player_id = {}", pid);
                            break;
                        } else {
                            return Err(format!("Handshake rejected: {:?}", reason));
                        }
                    }
                }
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        let player_id = player_id.ok_or_else(|| "Handshake timeout or invalid response".to_string())?;

        // === CHANNELS ===
        let (tx_out, mut rx_out) = mpsc::unbounded_channel::<ClientMessage>();
        let (tx_in, rx_in) = mpsc::unbounded_channel::<ServerMessage>();
        let shutdown = Arc::new(tokio::sync::Notify::new());

        let write = Arc::new(tokio::sync::Mutex::new(write));
        let read = Arc::new(tokio::sync::Mutex::new(read));
        let shutdown_clone = shutdown.clone();

        // === SEND TASK ===
        tokio::spawn(async move {
            while let Some(msg) = rx_out.recv().await {
                // Local mercy-gate pre-check for high-valence messages (client-side UX feedback)
                let valence_estimate: f32 = 0.8; // TODO: sync from server ValenceUpdate later
                if !apply_mercy_gate(&msg, valence_estimate) {
                    warn!("[ClientTransport] Mercy gate blocked local send: high-valence message requires higher valence");
                    continue;
                }

                if let Ok(bytes) = bincode::serialize(&msg) {
                    let mut w = write.lock().await;
                    if w.send(WsMessage::Binary(bytes.into())).await.is_err() {
                        break;
                    }
                }
            }
        });

        // === RECEIVE + HEARTBEAT TASK ===
        let tx_in_clone = tx_in.clone();
        tokio::spawn(async move {
            let mut last_pong = Instant::now();
            let heartbeat_interval = Duration::from_secs(10);
            let mut heartbeat_timer = tokio::time::interval(heartbeat_interval);

            loop {
                tokio::select! {
                    biased;

                    // Incoming messages
                    msg = async {
                        let mut r = read.lock().await;
                        r.next().await
                    } => {
                        if let Some(Ok(WsMessage::Binary(data))) = msg {
                            if let Ok(server_msg) = bincode::deserialize::<ServerMessage>(&data) {
                                let _ = tx_in_clone.send(server_msg);
                            }
                        } else if msg.is_none() {
                            break;
                        }
                    }

                    // Heartbeat (Ping)
                    _ = heartbeat_timer.tick() => {
                        if last_pong.elapsed() > Duration::from_secs(35) {
                            warn!("[ClientTransport v2] Heartbeat timeout — server unreachable");
                            break;
                        }
                        let ping = ClientMessage::Ping {
                            client_time_ms: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_millis() as u64,
                        };
                        let _ = tx_out.send(ping); // Will be picked by send task
                    }

                    // Shutdown
                    _ = shutdown_clone.notified() => {
                        info!("[ClientTransport v2] Shutdown signal received");
                        break;
                    }
                }
            }

            // Graceful close
            let mut w = write.lock().await;
            let _ = w.send(WsMessage::Close(None)).await;
        });

        let transport = Self {
            player_id: Some(player_id),
            tx_out,
            rx_in,
            shutdown,
        };

        Ok((transport, player_id))
    }

    /// Send a ClientMessage to server (non-blocking).
    /// High-valence messages are pre-checked locally.
    pub fn send(&self, msg: ClientMessage) -> Result<(), String> {
        self.tx_out
            .send(msg)
            .map_err(|e| format!("Send channel error: {}", e))
    }

    /// Receive next ServerMessage (non-blocking, for polling in game loop).
    pub async fn recv(&mut self) -> Option<ServerMessage> {
        self.rx_in.recv().await
    }

    /// Trigger graceful shutdown.
    pub fn shutdown(&self) {
        self.shutdown.notify_one();
    }
}

// === USAGE EXAMPLE (to be wired into client_game_loop.rs or main client) ===
/*
use game::network::client_transport::ClientWsTransport;
use shared::protocol::ClientMessage;

// In async client entry:
let (mut transport, my_id) = ClientWsTransport::connect("ws://127.0.0.1:9001", "Sherif").await?;

// Send movement from input
transport.send(ClientMessage::Move { delta: Vec3Ser { x: 0.1, y: 0.0, z: 0.0 } })?;

// In game loop tick:
while let Some(msg) = transport.recv().await {
    match msg {
        ServerMessage::WorldUpdate { entities, timestamp } => {
            // Feed to reconciliation / interpolation
            client_game_loop.handle_server_snapshot(entities, timestamp);
        }
        ServerMessage::DivineCouncilResponse { content, source } => {
            // Display live Ra-Thor PATSAGi guidance in UI
            println!("[Ra-Thor] {}: {}", source, content);
        }
        ServerMessage::Pong { .. } => { /* update latency */ }
        _ => {}
    }
}

transport.shutdown();
*/

// WASM NOTE (future):
// For Bevy + Trunk / web-sys target, replace tokio_tungstenite with web-sys WebSocket + wasm-bindgen-futures.
// The message contract (bincode + shared::protocol) remains identical.
// Add conditional compilation: #[cfg(target_arch = "wasm32")] ...

// Mercy flows. Thunder locked in. Ready for full wiring and testing. ⚡❤️
