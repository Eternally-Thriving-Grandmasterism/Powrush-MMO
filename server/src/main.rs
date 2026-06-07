// server/src/main.rs
// Powrush-MMO Server v16.7.5 — Production Grade

mod network;
mod interest_management;
mod harvesting_system;
mod trade_system;
mod technology_system;
mod server_war_system;
mod grok_patsagi_bridge;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc;
use tracing::{info, warn};

use shared::protocol::*;
use crate::harvesting_system::{HarvestingSystem, ServerInventoryComponent};
use crate::trade_system::TradeSystem;
use crate::technology_system::TechnologySystem;
use crate::server_war_system::ServerWarSystem;
use crate::grok_patsagi_bridge::GrokPatsagiBridge;
use crate::interest_management::InterestManager;

const TICK_RATE_MS: u64 = 50;
const SERVER_VERSION: &str = "16.7.5";

pub struct PowrushServer {
    harvesting_system: HarvestingSystem,
    trade_system: TradeSystem,
    technology_system: TechnologySystem,
    server_war_system: ServerWarSystem,
    bridge: Arc<GrokPatsagiBridge>,
    interest_manager: InterestManager,
    players: HashMap<u64, PlayerState>,
}

#[derive(Clone, Debug)]
struct PlayerState {
    name: String,
    position: Vec3Ser,
    health: HealthComponent,
    inventory: ServerInventoryComponent,
}

impl PowrushServer {
    pub async fn new(cluster_id: String) -> Self {
        let mut server_war_system = ServerWarSystem::new();
        server_war_system.seed_infrastructure();

        Self {
            harvesting_system: HarvestingSystem::new(),
            trade_system: TradeSystem::new().await,
            technology_system: TechnologySystem::new(cluster_id),
            server_war_system,
            bridge: Arc::new(GrokPatsagiBridge::new()),
            interest_manager: InterestManager::new(120.0),
            players: HashMap::new(),
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("⚡ Powrush-MMO Server {} — Eternal Flow Activated", SERVER_VERSION);

        let (mut transport, mut event_rx, command_tx) =
            network::TokioTransport::new("0.0.0.0:9001").await?;

        tokio::spawn(async move { transport.run().await; });

        let mut tick = tokio::time::interval(Duration::from_millis(TICK_RATE_MS));

        info!("Server listening on ws://0.0.0.0:9001");

        loop {
            tokio::select! {
                biased;

                Some(event) = event_rx.recv() => {
                    match event {
                        network::TransportEvent::ClientConnected { info } => {
                            self.handle_client_connected(info, &command_tx).await;
                        }
                        network::TransportEvent::ClientDisconnected { player_id } => {
                            self.handle_client_disconnected(player_id, &command_tx).await;
                        }
                        network::TransportEvent::MessageReceived { player_id, message } => {
                            self.handle_client_message(player_id, message, &command_tx).await;
                        }
                    }
                }

                _ = tick.tick() => {
                    let current_time = std::time::SystemTime::now()
                        .duration_since(std::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;

                    self.tick(current_time, &command_tx).await;
                }
            }
        }
    }

    async fn handle_client_disconnected(
        &mut self,
        player_id: u64,
        command_tx: &mpsc::Sender<network::TransportCommand>,
    ) {
        info!("Player {} disconnected", player_id);

        if self.players.remove(&player_id).is_some() {
            let returned = self.trade_system
                .return_escrowed_resources_on_disconnect(player_id)
                .await;

            for (target_id, resources) in returned {
                if let Some(state) = self.players.get_mut(&target_id) {
                    for (res_type, amount) in resources {
                        state.inventory.add_resource(&res_type, amount);
                    }
                }
            }
        }

        self.interest_manager.remove_player(player_id);
    }

    // Additional methods (handle_client_connected, handle_client_message, tick, etc.) would be here in full version
}