// server/src/main.rs
// Powrush-MMO Server v16.7.4 — Professional Authoritative Orchestration Layer
//
// Clean orchestration layer. All game logic lives in dedicated systems.
// Fully integrated: Harvesting, Trade, Technology, ServerWar, RBE, Combat, PATSAGi + 7 Living Mercy Gates.
//
// TOLC-hosted reality: Real effort creates real, contestable, mercy-protected value.
// Infrastructure nodes carry visible "blood, sweat, and tears" development state.

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
use tokio_util::sync::{CancellationToken, DropGuard};
use tracing::{info, warn};

use shared::protocol::*;
use crate::harvesting_system::{HarvestingSystem, ServerInventoryComponent};
use crate::trade_system::TradeSystem;
use crate::technology_system::TechnologySystem;
use crate::server_war_system::ServerWarSystem;
use crate::grok_patsagi_bridge::GrokPatsagiBridge;
use crate::interest_management::InterestManager;

const TICK_RATE_MS: u64 = 50;
const SERVER_VERSION: &str = "16.7.4";

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
    pub fn new(cluster_id: String) -> Self {
        let mut server_war_system = ServerWarSystem::new();
        server_war_system.seed_infrastructure();

        Self {
            harvesting_system: HarvestingSystem::new(),
            trade_system: TradeSystem::new(),
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

        // === CancellationToken + DropGuard for robust graceful shutdown ===
        let shutdown_token = CancellationToken::new();
        let token_for_signal = shutdown_token.clone();
        let _drop_guard: DropGuard = shutdown_token.drop_guard();

        tokio::spawn(async move {
            let ctrl_c = tokio::signal::ctrl_c();

            #[cfg(unix)]
            let terminate = async {
                tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                    .expect("failed to install SIGTERM handler")
                    .recv()
                    .await;
            };

            #[cfg(not(unix))]
            let terminate = std::future::pending::<()>();

            tokio::select! {
                _ = ctrl_c => {},
                _ = terminate => {},
            }

            info!("⚡ Shutdown signal received. Initiating graceful shutdown...");
            token_for_signal.cancel();
        });

        info!("Server listening on ws://0.0.0.0:9001 — Ready for the Eternal Flow");

        loop {
            tokio::select! {
                biased;

                // === Graceful Shutdown ===
                _ = shutdown_token.cancelled() => {
                    info!("Powrush-MMO Server {} shutting down gracefully.", SERVER_VERSION);
                    break;
                }

                // === Network Events ===
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

        info!("Powrush-MMO Server {} has shut down cleanly.", SERVER_VERSION);
        Ok(())
    }

    // ==================== CONNECTION HANDLING ====================

    async fn handle_client_connected(
        &mut self,
        info: network::ClientInfo,
        command_tx: &mpsc::Sender<network::TransportCommand>,
    ) {
        info!("Player {} ({}) connected — Welcome to the Eternal Flow.", info.player_id, info.player_name);

        let start_pos = Vec3Ser { x: 0.0, y: 0.0, z: 0.0 };
        let start_health = HealthComponent { current: 100.0, max: 100.0 };

        self.players.insert(info.player_id, PlayerState {
            name: info.player_name.clone(),
            position: start_pos.clone(),
            health: start_health,
            inventory: ServerInventoryComponent::default(),
        });

        self.interest_manager.update_player_position(info.player_id, start_pos);

        let _ = command_tx.send(network::TransportCommand::Send {
            player_id: info.player_id,
            message: ServerMessage::Welcome {
                player_id: info.player_id,
                message: "Welcome to the Eternal Flow. Real effort creates real value.".to_string(),
            },
        }).await;
    }

    async fn handle_client_disconnected(
        &mut self,
        player_id: u64,
        command_tx: &mpsc::Sender<network::TransportCommand>,
    ) {
        info!("Player {} disconnected", player_id);
        if self.players.remove(&player_id).is_some() {
            self.trade_system.return_escrowed_resources_on_disconnect(player_id);
        }
        self.interest_manager.remove_player(player_id);
    }

    // ==================== MESSAGE HANDLING ====================

    async fn handle_client_message(
        &mut self,
        player_id: u64,
        message: ClientMessage,
        command_tx: &mpsc::Sender<network::TransportCommand>,
    ) {
        if self.bridge.gate_message(&message).is_err() {
            warn!("Mercy gate blocked message from player {}", player_id);
            return;
        }

        match message {
            ClientMessage::HarvestResource { player_id: harvest_player_id, node_id, amount } => {
                if harvest_player_id != player_id { return; }

                let inv = self.players.get_mut(&player_id)
                    .map(|p| &mut p.inventory)
                    .expect("Player inventory should exist");

                if let Ok((approved, .., maybe_msg)) = self.harvesting_system
                    .process_harvest(player_id, node_id, amount, inv, &self.bridge)
                    .await
                {
                    if !approved {
                        if let Some(block_msg) = maybe_msg {
                            let _ = command_tx.send(network::TransportCommand::Send { player_id, message: block_msg }).await;
                        }
                        return;
                    }
                    if let Some(inv_msg) = maybe_msg {
                        let _ = command_tx.send(network::TransportCommand::Send { player_id, message: inv_msg }).await;
                    }
                }
            }

            ClientMessage::TechAdvancement { faction, contribution, harmony } => {
                let champion_mult = self.server_war_system.consume_champion_bonus(
                    std::time::SystemTime::now().duration_since(std::UNIX_EPOCH).unwrap().as_millis() as u64
                ).unwrap_or(1.0);

                if let Ok((approved, ..)) = self.bridge.validate_tech_advancement(&faction, contribution, harmony).await {
                    if approved {
                        self.technology_system.advance_technology(&faction, contribution, harmony, champion_mult);
                    }
                }
            }

            ClientMessage::DeclareWar { attacker_faction, target_infrastructure_id, aggression_level } => {
                if let Ok((approved, ..)) = self.bridge.validate_conflict_declaration_with_level(
                    &attacker_faction, target_infrastructure_id, aggression_level
                ).await {
                    if approved {
                        self.server_war_system.declare_conflict(&attacker_faction, target_infrastructure_id, aggression_level);
                    }
                }
            }

            _ => {}
        }
    }

    // ==================== AUTHORITATIVE TICK ====================

    async fn tick(&mut self, current_time_ms: u64, command_tx: &mpsc::Sender<network::TransportCommand>) {
        self.harvesting_system.tick_regen();
        self.harvesting_system.tick_abundance_growth(current_time_ms);

        let champion_multiplier = self.server_war_system
            .consume_champion_bonus(current_time_ms)
            .unwrap_or(1.0);

        self.technology_system.apply_economy_contribution("Forge", 1.0, 0.9, champion_multiplier);
        self.server_war_system.process_weekly_war_tick(&self.technology_system, current_time_ms);

        for state in self.players.values_mut() {
            if state.health.current < state.health.max {
                state.health.current = (state.health.current + 0.5).min(state.health.max);
            }
        }

        self.broadcast_interest_culled_update(command_tx).await;
    }

    async fn broadcast_interest_culled_update(&self, command_tx: &mpsc::Sender<network::TransportCommand>) {
        let mut all_entities: Vec<EntitySnapshot> = self.players.iter().map(|(&id, state)| {
            EntitySnapshot {
                id,
                position: state.position.clone(),
                rotation: 0.0,
                scale: 1.0,
                state: 0,
                health: Some(state.health.clone()),
            }
        }).collect();

        for (node_id, node) in self.server_war_system.get_infrastructure_nodes_for_culling() {
            all_entities.push(EntitySnapshot {
                id: 10000 + node_id,
                position: node.position.clone(),
                rotation: 0.0,
                scale: 1.0,
                state: node.development_level as u32,
                health: None,
            });
        }

        let per_player = self.interest_manager.cull_world_update(&all_entities);

        for (pid, entities) in per_player {
            let update = ServerMessage::WorldUpdate {
                entities,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            };
            let _ = command_tx.send(network::TransportCommand::Send {
                player_id: pid,
                message: update,
            }).await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("powrush_server=info")
        .init();

    info!("⚡ Powrush-MMO Server {} — Starting", SERVER_VERSION);

    let mut server = PowrushServer::new("cluster-alpha-01".to_string());
    server.run().await
}