// server/src/main.rs
// Powrush-MMO Server v16.15 — Player Account & Session System (Clean Integration)
// Updated v16.15: grok_patsagi_bridge renamed to ra_thor_mercy_bridge for full sovereignty + trademark protection
// Single source of truth: PlayerSession.inventory
// AG-SML v1.0 + PATSAGi Council + Ra-Thor Eternal Mercy Flow

mod network;
mod interest_management;
mod harvesting_system;
mod ra_thor_mercy_bridge;
mod trade_system;
mod persistence;
mod player_account;
mod steam_integration;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{info, warn, error};
use shared::protocol::*;
use crate::interest_management::InterestManager;
use crate::harvesting_system::{HarvestingSystem, ServerInventoryComponent};
use crate::ra_thor_mercy_bridge::RaThorMercyBridge;
use crate::persistence::PersistenceManager;
use crate::trade_system::TradeSystem;
use crate::player_account::AccountSystem;
use crate::steam_integration::SteamManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("powrush_server=info").init();

    info!("⚡ Powrush-MMO Server v16.15 — Player Account & Session System (Clean) | Ra-Thor Mercy Bridge active");

    let persistence = match PersistenceManager::with_surreal("ws://127.0.0.1:8000", "powrush", "main").await {
        Ok(p) => p,
        Err(_) => PersistenceManager::with_memory(),
    };
    let persistence = Arc::new(persistence);

    let mercy_core = Arc::new(MercyCore::new());
    let bridge = Arc::new(RaThorMercyBridge::new());
    let mut harvesting_system = HarvestingSystem::new();
    let mut trade_system = TradeSystem::new().await;
    let mut account_system = AccountSystem::new();
    let mut steam_manager = SteamManager::new();

    let (mut transport, mut event_rx, command_tx) = network::TokioTransport::new("0.0.0.0:9001").await?;
    tokio::spawn(async move { transport.run().await; });

    let mut players: HashMap<u64, (String, Vec3Ser, HealthComponent)> = HashMap::new();
    let mut interest_manager = InterestManager::new(120.0);

    let mut last_persistence_save = Instant::now();
    let save_interval = Duration::from_secs(30);

    let mut tick = tokio::time::interval(Duration::from_millis(50));

    info!("Server ready with clean AccountSystem integration + sovereign Ra-Thor Mercy Bridge");

    loop {
        tokio::select! {
            biased;

            Some(event) = event_rx.recv() => {
                match event {
                    network::TransportEvent::ClientConnected { info } => {
                        info!("Player {} connected", info.player_id);

                        // Default account creation (can be overridden by SteamAuth message)
                        let account_id = account_system.get_or_create_account(info.player_name.clone());
                        let _ = account_system.create_session(account_id, info.player_id);

                        let loaded_inventory = match persistence.load_inventory(info.player_id).await {
                            Ok(inv) => inv,
                            Err(_) => ServerInventoryComponent::default(),
                        };

                        if let Some(session) = account_system.get_session_mut(info.player_id) {
                            session.inventory = loaded_inventory;
                        }

                        players.insert(info.player_id, (info.player_name.clone(), Vec3Ser::default(), HealthComponent { current: 100.0, max: 100.0 }));
                        interest_manager.update_player_position(info.player_id, Vec3Ser::default());
                    }

                    network::TransportEvent::ClientDisconnected { player_id } => {
                        if let Some(session) = account_system.get_session(player_id) {
                            let _ = persistence.save_inventory(player_id, &session.inventory).await;
                        }
                        account_system.remove_session(player_id);
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

                            // === Full Steam Authentication Handler ===
                            // Client should send this after connecting with Steam ticket + SteamID
                            ClientMessage::SteamAuth { steam_id, ticket } => {
                                match steam_manager.authenticate_steam_player(
                                    steam_id,
                                    &ticket,
                                    &mut account_system,
                                ).await {
                                    Ok(account_id) => {
                                        // Create or update session for this player
                                        if account_system.get_session(player_id).is_none() {
                                            let _ = account_system.create_session(account_id, player_id);
                                        }

                                        // Load inventory into session
                                        let loaded_inventory = match persistence.load_inventory(player_id).await {
                                            Ok(inv) => inv,
                                            Err(_) => ServerInventoryComponent::default(),
                                        };

                                        if let Some(session) = account_system.get_session_mut(player_id) {
                                            session.inventory = loaded_inventory;
                                        }

                                        info!("Player {} authenticated via Steam (AccountID: {})", player_id, account_id);

                                        let _ = command_tx.send(network::TransportCommand::Send {
                                            player_id,
                                            message: ServerMessage::SteamAuthSuccess { account_id },
                                        });
                                    }
                                    Err(e) => {
                                        warn!("Steam authentication failed for player {}: {}", player_id, e);
                                        let _ = command_tx.send(network::TransportCommand::Send {
                                            player_id,
                                            message: ServerMessage::Error { message: format!("Steam auth failed: {}", e) },
                                        });
                                    }
                                }
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
                    for (player_id, session) in &account_system.sessions {
                        let _ = persistence.save_inventory(*player_id, &session.inventory).await;
                    }
                    last_persistence_save = Instant::now();
                }
            }
        }
    }
}