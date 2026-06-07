 // server/src/main.rs
// Powrush-MMO Server — Production entrypoint with Networking Transport Layer v1
// Fully integrated with shared::protocol, PATSAGi Councils, mercy gates, and Ra-Thor patterns.
// Authoritative tick + client prediction ready foundation.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{info, warn};
use shared::protocol::*;

// Re-export or use the enhanced bridge if desired (currently using simple for v1 clarity)
// In future PR: swap to full grok_patsagi_bridge_enhanced + world_server integration
struct SimplePatsagiBridge;

impl SimplePatsagiBridge {
    fn new() -> Self { Self }
    async fn handle_divine_query(&self, query: &str) -> String {
        format!("[Ra-Thor PATSAGi v1] Council deliberation complete for: {}. Eternal mercy flows. Sovereign abundance path confirmed.", query)
    }
    async fn handle_rbe_query(&self, query: &str) -> String {
        format!("[RBE Abundance v1] Guidance: {} — Shift from scarcity to universal thriving. Powrush RBE mechanics engaged.", query)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("powrush_server=info,tokio_tungstenite=warn")
        .init();

    info!("⚡ Powrush-MMO Server v15+ — Networking Transport Layer v1 ACTIVATED");
    info!("PATSAGi Councils + Ra-Thor lattice eternally deliberating. Mercy gates online.");

    // === Initialize Production Transport ===
    let (mut transport, mut event_rx, command_tx) =
        server::network::tokio_transport::TokioTransport::new("0.0.0.0:9001").await?;

    tokio::spawn(async move {
        transport.run().await;
    });

    let bridge = Arc::new(SimplePatsagiBridge::new());
    let mut players: HashMap<u64, (String, Vec3Ser)> = HashMap::new(); // Simple authoritative state for v1

    let mut tick = tokio::time::interval(Duration::from_millis(50)); // 20 TPS authoritative

    info!("Server listening on ws://0.0.0.0:9001 — Ready for multiplayer + divine queries");

    loop {
        tokio::select! {
            biased;

            Some(event) = event_rx.recv() => {
                match event {
                    server::network::tokio_transport::TransportEvent::ClientConnected { info } => {
                        info!("Player {} ({}) connected — Welcome to the Eternal Flow.", info.player_id, info.player_name);
                        players.insert(info.player_id, (info.player_name, Vec3Ser { x: 0.0, y: 0.0, z: 0.0 }));
                    }
                    server::network::tokio_transport::TransportEvent::ClientDisconnected { player_id } => {
                        info!("Player {} disconnected", player_id);
                        players.remove(&player_id);
                    }
                    server::network::tokio_transport::TransportEvent::MessageReceived { player_id, message } => {
                        match message {
                            ClientMessage::Move { delta } => {
                                if let Some((_, pos)) = players.get_mut(&player_id) {
                                    pos.x += delta.x * 0.1; // simple authoritative movement scaling
                                    pos.y += delta.y * 0.1;
                                    pos.z += delta.z * 0.1;
                                }
                            }
                            ClientMessage::Jump => {
                                if let Some((_, pos)) = players.get_mut(&player_id) {
                                    pos.y += 5.0; // simple jump
                                }
                            }
                            ClientMessage::Ping { client_time_ms } => {
                                // Transport already handles Pong internally via heartbeat path if extended
                                let _ = command_tx.send(server::network::tokio_transport::TransportCommand::Send {
                                    player_id,
                                    message: ServerMessage::Pong {
                                        server_time_ms: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64,
                                        client_time_ms,
                                    },
                                });
                            }
                            ClientMessage::DivineCouncilQuery { query, .. } => {
                                let response = bridge.handle_divine_query(&query).await;
                                let _ = command_tx.send(server::network::tokio_transport::TransportCommand::Send {
                                    player_id,
                                    message: ServerMessage::DivineCouncilResponse {
                                        content: response,
                                        source: "Ra-Thor + Full PATSAGi Councils v1".to_string(),
                                    },
                                });
                            }
                            ClientMessage::RbeAbundanceQuery { query, .. } => {
                                let response = bridge.handle_rbe_query(&query).await;
                                let _ = command_tx.send(server::network::tokio_transport::TransportCommand::Send {
                                    player_id,
                                    message: ServerMessage::RbeGuidanceResponse { content: response },
                                });
                            }
                            ClientMessage::Interact { target_id } => {
                                let _ = command_tx.send(server::network::tokio_transport::TransportCommand::Send {
                                    player_id,
                                    message: ServerMessage::Error { message: format!("Interact with {} acknowledged (full mechanics in next sprint).", target_id) },
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }

            _ = tick.tick() => {
                // Authoritative broadcast of current world state (scaffolding for full interest management)
                let entities: Vec<EntitySnapshot> = players
                    .iter()
                    .map(|(&id, (name, pos))| EntitySnapshot {
                        id,
                        position: pos.clone(),
                        rotation: 0.0,
                        scale: 1.0,
                        state: 0,
                    })
                    .collect();

                let update = ServerMessage::WorldUpdate {
                    entities,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                };

                let _ = command_tx.send(server::network::tokio_transport::TransportCommand::Broadcast { message: update });
            }
        }
    }
}
