//! Powrush MMO Server — Mercy-Gated WebSocket Broadcast Core with Heartbeat
//! Integrates Ra-Thor divine module, AOI replication, async queuing, compression, heartbeat
//! MIT + mercy eternal — Eternally-Thriving-Grandmasterism

use anyhow::{Context, Result};
use futures_util::{SinkExt, StreamExt};
use powrush_divine_module::MercyCore;
use shared::protocol::{ClientMessage, ServerMessage};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tracing::{error, info, warn};
use std::panic;
use std::time::{Duration, Instant};
use snappy::compress;

mod world_server;
use world_server::WorldServer;

const QUEUE_CAPACITY: usize = 100;
const COMPRESSION_THRESHOLD_BYTES: usize = 1024;
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
const MAX_MISSED_HEARTBEATS: u32 = 3;

# async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Powrush MMO Server booting — mercy thunder awakening ⚡️");

    panic::set_hook(Box::new(|info| {
        error!("SERVER PANIC: {}", info);
    }));

    let mercy_core = Arc::new(Mutex::new(MercyCore::new()));
    let world_server = Arc::new(Mutex::new(WorldServer::new(mercy_core.clone())));

    let ws_listener = TcpListener::bind("0.0.0.0:9001")
        .await
        .context("Failed to bind WebSocket port")?;
    info!("WebSocket listening on 0.0.0.0:9001");

    tokio::spawn(async move {
        while let Ok((stream, addr)) = ws_listener.accept().await {
            info!("New WebSocket connection from {}", addr);
            let ws_world = world_server.clone();
            let ws_mercy = mercy_core.clone();
            tokio::spawn(handle_websocket(stream, ws_world, ws_mercy));
        }
    });

    let world_clone = world_server.clone();
    tokio::spawn(async move {
        loop {
            let mut world = world_clone.lock().await;
            if let Err(e) = world.tick().await {
                error!("World tick error (recoverable): {}", e);
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    });

    std::future::pending::<()>().await;
    Ok(())
}

async fn handle_websocket(stream: TcpStream, world_server: Arc<Mutex<WorldServer>>, mercy_core: Arc<Mutex<MercyCore>>) {
    let ws_stream = accept_async(stream).await.expect("Failed to accept WebSocket");
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let client_id = rand::random::<u64>();

    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(QUEUE_CAPACITY);

    // Register client
    {
        let mut world = world_server.lock().await;
        world.add_client(client_id, Vec3::ZERO, 0.8, tx.clone()).await.unwrap();
    }

    // Heartbeat state per client
    let mut last_received_heartbeat = Instant::now();
    let mut missed_heartbeats = 0u32;

    // Spawn send loop with compression
    let mut ws_sender = ws_sender;
    tokio::spawn(async move {
        while let Some(payload) = rx.recv().await {
            let final_msg = if payload.len() > COMPRESSION_THRESHOLD_BYTES {
                match compress(&payload) {
                    Ok(compressed) => {
                        let mut msg = vec![1];
                        msg.extend_from_slice(&compressed);
                        msg
                    }
                    Err(e) => {
                        warn!("Compression failed (sending raw): {}", e);
                        let mut msg = vec![0];
                        msg.extend_from_slice(&payload);
                        msg
                    }
                }
            } else {
                let mut msg = vec![0];
                msg.extend_from_slice(&payload);
                msg
            };

            if let Err(e) = ws_sender.send(Message::Binary(final_msg)).await {
                warn!("Send error to client: {}", client_id, e);
                break;
            }
        }
        info!("Send queue closed for client {}", client_id);
    });

    // Handshake
    let handshake = bincode::serialize(&ServerMessage::HandshakeResponse {
        accepted: true,
        reason: None,
        player_id: client_id,
        server_time: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
    }).unwrap();
    let _ = tx.send(handshake).await;

    // Client receive loop with heartbeat handling
    while let Some(msg) = ws_receiver.next().await {
        match msg {
            Ok(Message::Binary(data)) => {
                let mercy_core = mercy_core.lock().await;

                let decompressed = if data.len() > 1 && data[0] == 1 {
                    match decompress(&data[1..]) {
                        Ok(d) => d,
                        Err(e) => {
                            warn!("Decompression failed for client: {}", client_id, e);
                            continue;
                        }
                    }
                } else {
                    data
                };

                if mercy_core.gate_server_message(&decompressed).await.is_ok() {
                    if let Ok(client_msg) = bincode::deserialize::<ClientMessage>(&decompressed) {
                        match client_msg {
                            ClientMessage::Ping { client_time_ms } => {
                                let pong = ServerMessage::Pong {
                                    server_time_ms: std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH)
                                        .unwrap()
                                        .as_millis() as u64,
                                    client_time_ms,
                                };
                                let pong_serialized = bincode::serialize(&pong).unwrap();
                                let _ = tx.send(pong_serialized).await;
                                last_received_heartbeat = Instant::now();
                                missed_heartbeats = 0;
                                info!("Heartbeat received from client {}", client_id);
                            }
                            _ => info!("Received valid message from client {}", client_id),
                        }
                    }
                } else {
                    warn!("Mercy gate blocked message from client {}", client_id);
                    let err = bincode::serialize(&ServerMessage::MercyGateBlocked {
                        reason: "Low valence on receive".to_string(),
                        valence: 0.0,
                    }).unwrap();
                    let _ = tx.send(err).await;
                }
            }
            Ok(Message::Close(_)) => {
                info!("Client disconnected gracefully", client_id);
                break;
            }
            Err(e) => {
                warn!("WebSocket error from client: {}", client_id, e);
                break;
            }
            _ =>
        }

        if Instant::now().duration_since(last_received_heartbeat) > HEARTBEAT_INTERVAL * (MAX_MISSED_HEARTBEATS as u32 + 1) {
            missed_heartbeats += 1;
            if missed_heartbeats >= MAX_MISSED_HEARTBEATS {
                warn!("Client missed heartbeats — disconnecting", client_id, missed_heartbeats);
                break;
            }
        }
    }

    // Cleanup
    {
        let mut world = world_server.lock().await;
        world.clients.remove(&client_id);
    }
    }
