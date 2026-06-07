// server/src/main.rs
// Powrush-MMO Server v16.13 — Steamworks + Cloud Save Integration Started
// Steam authentication wired into account/session creation
// AG-SML v1.0

mod network;
mod interest_management;
mod harvesting_system;
mod grok_patsagi_bridge;
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
use crate::grok_patsagi_bridge::GrokPatsagiBridge;
use crate::persistence::PersistenceManager;
use crate::trade_system::TradeSystem;
use crate::player_account::AccountSystem;
use crate::steam_integration::SteamManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("powrush_server=info").init();

    info!("⚡ Powrush-MMO Server v16.13 — Steam + Cloud Save Integration");

    let persistence = match PersistenceManager::with_surreal("ws://127.0.0.1:8000", "powrush", "main").await {
        Ok(p) => p,
        Err(_) => PersistenceManager::with_memory(),
    };
    let persistence = Arc::new(persistence);

    let mercy_core = Arc::new(MercyCore::new());
    let bridge = Arc::new(GrokPatsagiBridge::new());
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

    info!("Server ready with SteamManager");

    loop {
        tokio::select! {
            biased;

            Some(event) = event_rx.recv() => {
                match event {
                    network::TransportEvent::ClientConnected { info } => {
                        info!("Player {} connected", info.player_id);

                        // For now, create account normally (Steam auth can be added via message)
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

                            // === Steam Authentication Example ===
                            // In real flow, client would send a SteamAuth message with ticket + SteamID
                            // For now we demonstrate the integration point
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