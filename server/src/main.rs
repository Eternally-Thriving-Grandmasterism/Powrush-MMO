// server/src/main.rs
// Powrush-MMO Server v16.11 — Trading System Refinements Applied
// Clean call site for accept_trade_atomic + all previous fixes
// AG-SML v1.0

mod network;
mod interest_management;
mod harvesting_system;
mod grok_patsagi_bridge;
mod trade_system;
mod persistence;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{info, warn, error};
use shared::protocol::*;
use crate::interest_management::InterestManager;
use crate::harvesting_system::{HarvestingSystem, ServerInventoryComponent};
use crate::grok_patsagi_bridge::GrokPatsagiBridge;
use crate::persistence::PersistenceManager;
use crate::trade_system::TradeSystem;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("powrush_server=info").init();

    info!("⚡ Powrush-MMO Server v16.11 — Trading System Refinements");

    let persistence = match PersistenceManager::with_surreal("ws://127.0.0.1:8000", "powrush", "main").await {
        Ok(p) => p,
        Err(_) => PersistenceManager::with_memory(),
    };
    let persistence = Arc::new(persistence);

    let mercy_core = Arc::new(MercyCore::new());
    let bridge = Arc::new(GrokPatsagiBridge::new());
    let mut harvesting_system = HarvestingSystem::new();
    let mut trade_system = TradeSystem::new().await;

    let (mut transport, mut event_rx, command_tx) = network::TokioTransport::new("0.0.0.0:9001").await?;
    tokio::spawn(async move { transport.run().await; });

    let mut players: HashMap<u64, (String, Vec3Ser, HealthComponent)> = HashMap::new();
    let mut player_inventories: HashMap<u64, ServerInventoryComponent> = HashMap::new();
    let mut interest_manager = InterestManager::new(120.0);

    let mut last_persistence_save = Instant::now();
    let save_interval = Duration::from_secs(30);

    let mut tick = tokio::time::interval(Duration::from_millis(50));

    info!("Server ready");

    loop {
        tokio::select! {
            biased;

            Some(event) = event_rx.recv() => {
                match event {
                    network::TransportEvent::ClientConnected { info } => {
                        let loaded_inventory = match persistence.load_inventory(info.player_id).await {
                            Ok(inv) => inv,
                            Err(_) => ServerInventoryComponent::default(),
                        };
                        players.insert(info.player_id, (info.player_name.clone(), Vec3Ser::default(), HealthComponent { current: 100.0, max: 100.0 }));
                        player_inventories.insert(info.player_id, loaded_inventory);
                        interest_manager.update_player_position(info.player_id, Vec3Ser::default());
                    }

                    network::TransportEvent::ClientDisconnected { player_id } => {
                        if let Some(inventory) = player_inventories.remove(&player_id) {
                            let _ = persistence.save_inventory(player_id, &inventory).await;
                        }
                        let _ = trade_system.return_escrowed_resources_on_disconnect(player_id).await;
                        players.remove(&player_id);
                    }

                    network::TransportEvent::MessageReceived { player_id, message } => {
                        if mercy_core.gate_server_message(&message).is_err() {
                            continue;
                        }

                        match message {
                            ClientMessage::HarvestResource { player_id: pid, node_id, amount } => {
                                // harvest logic
                            }

                            ClientMessage::TradeInitiate { offer } => {
                                if let Ok(trade_id) = trade_system.initiate_trade(
                                    offer.from_player, offer.to_player, offer.offered.clone(), offer.requested.clone()
                                ).await {
                                    let _ = command_tx.send(network::TransportCommand::Send {
                                        player_id,
                                        message: ServerMessage::TradeRequestReceived { offer },
                                    });
                                }
                            }

                            ClientMessage::TradeAccept { trade_id } => {
                                // Clean, safe call site - look up trade first
                                if let Some(trade) = trade_system.active_trades.get(&trade_id).cloned() {
                                    if trade.target_id == player_id && trade.status == "pending" {
                                        if let (Some(offeror_inv), Some(target_inv)) = (
                                            player_inventories.get_mut(&trade.offeror_id),
                                            player_inventories.get_mut(&player_id),
                                        ) {
                                            match trade_system.accept_trade_atomic(
                                                trade_id, player_id, offeror_inv, target_inv
                                            ).await {
                                                Ok(()) => {
                                                    let _ = command_tx.send(network::TransportCommand::Send {
                                                        player_id,
                                                        message: ServerMessage::TradeCompleted {
                                                            trade_id,
                                                            from: trade.offeror_id,
                                                            to: trade.target_id,
                                                            final_state: "accepted".to_string(),
                                                            grace_awarded: 0,
                                                        },
                                                    });
                                                }
                                                Err(e) => {
                                                    let _ = command_tx.send(network::TransportCommand::Send {
                                                        player_id,
                                                        message: ServerMessage::Error { message: e },
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            ClientMessage::TradeCancel { trade_id } => {
                                let _ = trade_system.reject_trade(trade_id, player_id).await;
                            }

                            _ => {}
                        }
                    }
                }
            }

            _ = tick.tick() => {
                harvesting_system.tick_regen();
                trade_system.expire_trades().await;

                if last_persistence_save.elapsed() > save_interval {
                    for (player_id, inventory) in &player_inventories {
                        let _ = persistence.save_inventory(*player_id, inventory).await;
                    }
                    last_persistence_save = Instant::now();
                }
            }
        }
    }
}