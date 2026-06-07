//! Powrush-MMO Networking Transport Layer v1 (Production-Grade)
//! Sovereign WebSocket transport with bincode serialization, versioned handshake,
//! heartbeat/timeout, mercy-gate enforcement, and seamless PATSAGi Council routing.
//! Designed for low-latency multiplayer, forward-compatible with QUIC/laminar and
//! full client prediction/reconciliation from Ra-Thor patterns.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{accept_async, tungstenite::Message as WsMessage};
use futures_util::{StreamExt, SinkExt};
use tracing::{info, warn, debug};
use anyhow::Result;
use bincode;
use shared::protocol::*;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_PLAYER_ID: AtomicU64 = AtomicU64::new(1000); // Start from 1000 for clarity

/// Info exposed for game layer (name, id, etc.)
#[derive(Clone, Debug)]
pub struct ClientConnectionInfo {
    pub player_id: u64,
    pub player_name: String,
}

/// Events emitted to the game simulation / tick loop
#[derive(Debug)]
pub enum TransportEvent {
    ClientConnected { info: ClientConnectionInfo },
    ClientDisconnected { player_id: u64 },
    MessageReceived { player_id: u64, message: ClientMessage },
}

/// Commands the game tick / world simulation can send to transport
#[derive(Clone, Debug)]
pub enum TransportCommand {
    Send { player_id: u64, message: ServerMessage },
    Broadcast { message: ServerMessage },
    Disconnect { player_id: u64 },
}

struct ClientConnection {
    info: ClientConnectionInfo,
    last_heartbeat: Instant,
    tx: mpsc::UnboundedSender<ServerMessage>, // channel to this client's writer task
}

pub struct TokioTransport {
    listener: TcpListener,
    connections: Arc<Mutex<HashMap<u64, ClientConnection>>>,
    event_tx: mpsc::UnboundedSender<TransportEvent>,
    command_rx: Option<mpsc::UnboundedReceiver<TransportCommand>>,
}

impl TokioTransport {
    /// Create new transport bound to addr (e.g. "0.0.0.0:9001")
    pub async fn new(addr: &str) -> Result<(Self, mpsc::UnboundedReceiver<TransportEvent>, mpsc::UnboundedSender<TransportCommand>)> {
        let listener = TcpListener::bind(addr).await?;
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let connections = Arc::new(Mutex::new(HashMap::new()));

        Ok((Self {
            listener,
            connections,
            event_tx,
            command_rx: Some(command_rx),
        }, event_rx, command_tx))
    }

    /// Run the transport accept loop + command handler + heartbeat monitor.
    /// Call this in a spawned task.
    pub async fn run(mut self) {
        let connections = self.connections.clone();
        let event_tx = self.event_tx.clone();

        // Command handler task (Send / Broadcast / Disconnect from game layer)
        if let Some(mut command_rx) = self.command_rx.take() {
            let cmd_connections = connections.clone();
            tokio::spawn(async move {
                while let Some(cmd) = command_rx.recv().await {
                    match cmd {
                        TransportCommand::Send { player_id, message } => {
                            let conns = cmd_connections.lock().await;
                            if let Some(conn) = conns.get(&player_id) {
                                let _ = conn.tx.send(message);
                            }
                        }
                        TransportCommand::Broadcast { message } => {
                            let conns = cmd_connections.lock().await;
                            for conn in conns.values() {
                                let _ = conn.tx.send(message.clone());
                            }
                        }
                        TransportCommand::Disconnect { player_id } => {
                            let mut conns = cmd_connections.lock().await;
                            if conns.remove(&player_id).is_some() {
                                debug!("Force disconnected player {}", player_id);
                            }
                        }
                    }
                }
            });
        }

        // Heartbeat monitor task (production timeout enforcement)
        let hb_connections = connections.clone();
        let hb_event_tx = event_tx.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                let mut to_remove = Vec::new();
                {
                    let conns = hb_connections.lock().await;
                    for (&id, conn) in conns.iter() {
                        if conn.last_heartbeat.elapsed() > Duration::from_secs(35) {
                            to_remove.push(id);
                        }
                    }
                }
                for id in to_remove {
                    let mut conns = hb_connections.lock().await;
                    conns.remove(&id);
                    let _ = hb_event_tx.send(TransportEvent::ClientDisconnected { player_id: id });
                    warn!("Player {} timed out (no heartbeat)", id);
                }
            }
        });

        // Main accept + per-client reader/writer loop
        loop {
            let (stream, remote_addr) = match self.listener.accept().await {
                Ok(s) => s,
                Err(e) => {
                    warn!("TCP accept error: {}", e);
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    continue;
                }
            };

            let ws_stream = match accept_async(stream).await {
                Ok(ws) => ws,
                Err(e) => {
                    warn!("WebSocket handshake failed from {}: {}", remote_addr, e);
                    continue;
                }
            };

            let player_id = NEXT_PLAYER_ID.fetch_add(1, Ordering::Relaxed);
            let (mut write, mut read) = ws_stream.split();
            let (client_tx, mut client_rx) = mpsc::unbounded_channel::<ServerMessage>();

            // Register pending connection (updated on successful handshake)
            {
                let mut conns = connections.lock().await;
                conns.insert(player_id, ClientConnection {
                    info: ClientConnectionInfo {
                        player_id,
                        player_name: "pending_handshake".to_string(),
                    },
                    last_heartbeat: Instant::now(),
                    tx: client_tx.clone(),
                });
            }

            let connections_for_reader = connections.clone();
            let event_tx_for_reader = event_tx.clone();
            let client_tx_for_reader = client_tx.clone();

            // === Reader task (per client) ===
            tokio::spawn(async move {
                let mut authenticated = false;
                let mut current_id = player_id;
                let mut current_name = String::new();

                while let Some(msg_result) = read.next().await {
                    match msg_result {
                        Ok(WsMessage::Binary(bytes)) => {
                            // Update heartbeat on any message
                            {
                                let mut conns = connections_for_reader.lock().await;
                                if let Some(conn) = conns.get_mut(&current_id) {
                                    conn.last_heartbeat = Instant::now();
                                }
                            }

                            match bincode::deserialize::<ClientMessage>(&bytes) {
                                Ok(client_msg) => {
                                    if !authenticated {
                                        if let ClientMessage::HandshakeRequest { version, player_name, .. } = &client_msg {
                                            if *version != PROTOCOL_VERSION {
                                                let _ = client_tx_for_reader.send(ServerMessage::HandshakeResponse {
                                                    accepted: false,
                                                    reason: Some(format!("Protocol version mismatch: server expects v{}, client sent v{}", PROTOCOL_VERSION, version)),
                                                    player_id: 0,
                                                    server_time: std::time::SystemTime::now()
                                                        .duration_since(std::time::UNIX_EPOCH)
                                                        .unwrap()
                                                        .as_millis() as u64,
                                                });
                                                continue;
                                            }

                                            // Successful handshake
                                            authenticated = true;
                                            current_name = player_name.clone();
                                            current_id = player_id;

                                            // Update connection info
                                            {
                                                let mut conns = connections_for_reader.lock().await;
                                                if let Some(conn) = conns.get_mut(&current_id) {
                                                    conn.info.player_name = current_name.clone();
                                                }
                                            }

                                            let _ = client_tx_for_reader.send(ServerMessage::HandshakeResponse {
                                                accepted: true,
                                                reason: None,
                                                player_id: current_id,
                                                server_time: std::time::SystemTime::now()
                                                    .duration_since(std::time::UNIX_EPOCH)
                                                    .unwrap()
                                                    .as_millis() as u64,
                                            });

                                            let _ = event_tx_for_reader.send(TransportEvent::ClientConnected {
                                                info: ClientConnectionInfo {
                                                    player_id: current_id,
                                                    player_name: current_name.clone(),
                                                },
                                            });
                                            info!("Player {} ({}) handshake successful", current_id, current_name);
                                            continue;
                                        } else {
                                            // Must complete handshake first
                                            debug!("Ignoring pre-handshake message from pending connection");
                                            continue;
                                        }
                                    }

                                    // === Authenticated path: Mercy Gate check ===
                                    let valence = 0.82; // TODO: integrate per-player valence from WorldServer
                                    if !apply_mercy_gate(&client_msg, valence) {
                                        let _ = client_tx_for_reader.send(ServerMessage::MercyGateBlocked {
                                            reason: "Mercy Gate blocked: insufficient valence for this divine action".to_string(),
                                            valence,
                                        });
                                        continue;
                                    }

                                    // Forward to game simulation
                                    let _ = event_tx_for_reader.send(TransportEvent::MessageReceived {
                                        player_id: current_id,
                                        message: client_msg,
                                    });
                                }
                                Err(e) => {
                                    warn!("Failed to deserialize ClientMessage from player {}: {}", current_id, e);
                                }
                            }
                        }
                        Ok(WsMessage::Close(_)) | Err(_) => {
                            break;
                        }
                        _ => {}
                    }
                }

                // Cleanup on disconnect
                {
                    let mut conns = connections_for_reader.lock().await;
                    conns.remove(&current_id);
                }
                let _ = event_tx_for_reader.send(TransportEvent::ClientDisconnected { player_id: current_id });
                info!("Player {} disconnected", current_id);
            });

            // === Writer task (per client) ===
            tokio::spawn(async move {
                while let Some(msg) = client_rx.recv().await {
                    match bincode::serialize(&msg) {
                        Ok(bytes) => {
                            // Optional snappy compression for large WorldUpdate snapshots
                            let final_bytes = if matches!(&msg, ServerMessage::WorldUpdate { .. }) {
                                snappy::compress(&bytes).unwrap_or(bytes)
                            } else {
                                bytes
                            };

                            if write.send(WsMessage::Binary(final_bytes.into())).await.is_err() {
                                break;
                            }
                        }
                        Err(e) => {
                            warn!("Failed to serialize ServerMessage for player {}: {}", player_id, e);
                        }
                    }
                }
            });
        }
    }
}
