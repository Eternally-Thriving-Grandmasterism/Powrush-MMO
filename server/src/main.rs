// server/src/main.rs
// Powrush-MMO Server v16.1 — RBE Player Inventory + Abundance Tracking
// Builds on v16.0 Resource Nodes + Harvesting
// Per-player inventory, global abundance simulation, PATSAGi sustainability checks
// Fully integrated with InterestManager, LagComp, Projectile, Combat, Mercy Gates

mod network;
mod interest_management;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::{info, warn};
use shared::protocol::*;
use crate::interest_management::InterestManager;
use game::lag_compensation::LagCompensation;
use game::hit_detection::HitDetection;

// ... (MercyCore, WorldServer, GrokPatsagiBridge, ActiveProjectile, CooldownTracker — unchanged from v16.0) ...

/// Per-player RBE Inventory (v16.1)
#[derive(Clone, Debug, Default)]
pub struct RbeInventory {
    pub resources: HashMap<String, f32>,
    pub abundance_score: f32,
}

/// Global abundance tracker (simple simulation for v16.1)
struct GlobalAbundance {
    pub total: f32,
    pub last_update_ms: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... tracing init, mercy_core, world_server, bridge, transport ...

    let mut players: HashMap<u64, (String, Vec3Ser, HealthComponent)> = HashMap::new();
    let mut player_inventories: HashMap<u64, RbeInventory> = HashMap::new();
    let mut global_abundance = GlobalAbundance { total: 1000.0, last_update_ms: 0 };
    let mut interest_manager = InterestManager::new(120.0);
    let mut cooldowns: CooldownTracker = HashMap::new();
    let mut lag_comp = LagCompensation::new(/* config */);
    let mut hit_detection = HitDetection::new();
    let mut active_projectiles: Vec<ActiveProjectile> = Vec::new();
    let mut next_projectile_id: u64 = 1;
    let mut resource_nodes: HashMap<u64, ResourceNode> = HashMap::new(); // from v16.0

    // Spawn example resource nodes (from v16.0)
    resource_nodes.insert(1, ResourceNode { id: 1, position: Vec3Ser { x: 50.0, y: 0.0, z: 50.0 }, resource_type: "Bio-Energy".to_string(), remaining: 500.0, max: 500.0, regen_per_tick: 0.5 });
    resource_nodes.insert(2, ResourceNode { id: 2, position: Vec3Ser { x: -80.0, y: 0.0, z: 30.0 }, resource_type: "Crystal".to_string(), remaining: 300.0, max: 300.0, regen_per_tick: 0.3 });

    let mut tick = tokio::time::interval(Duration::from_millis(50));

    info!("⚡ Powrush-MMO Server v16.1 — RBE Player Inventory + Abundance Tracking ACTIVATED");

    loop {
        tokio::select! {
            biased;

            Some(event) = event_rx.recv() => {
                match event {
                    network::TransportEvent::ClientConnected { info } => {
                        // ... existing player init ...
                        player_inventories.insert(info.player_id, RbeInventory::default());
                    }
                    network::TransportEvent::ClientDisconnected { player_id } => {
                        player_inventories.remove(&player_id);
                    }
                    network::TransportEvent::MessageReceived { player_id, message } => {
                        if mercy_core.gate_server_message(&message).is_err() { continue; }

                        match message {
                            ClientMessage::HarvestResource { node_id, amount } => {
                                // PATSAGi sustainability validation + mercy gate
                                if let Some(node) = resource_nodes.get_mut(&node_id) {
                                    let sustainable = amount <= node.remaining * 0.3; // example sustainability rule
                                    let validation = bridge.validate_harvest(player_id, node_id, amount, sustainable).await; // extend GrokPatsagiBridge

                                    if let Ok((approved, reason, valence_impact)) = validation {
                                        if !approved || !sustainable {
                                            let _ = command_tx.send(network::TransportCommand::Send {
                                                player_id,
                                                message: ServerMessage::MercyGateBlocked { reason, valence: valence_impact },
                                            });
                                            continue;
                                        }

                                        // Apply harvest
                                        node.remaining = (node.remaining - amount).max(0.0);

                                        // Add to player inventory
                                        let inv = player_inventories.entry(player_id).or_default();
                                        *inv.resources.entry(node.resource_type.clone()).or_insert(0.0) += amount;
                                        inv.abundance_score += amount * 0.01; // simple abundance gain

                                        // Update global abundance
                                        global_abundance.total += amount * 0.5; // RBE philosophy: harvesting increases shared abundance when sustainable

                                        // Send inventory update to player
                                        let _ = command_tx.send(network::TransportCommand::Send {
                                            player_id,
                                            message: ServerMessage::InventoryUpdate {
                                                player_id,
                                                resources: inv.resources.clone(),
                                                abundance_score: inv.abundance_score,
                                            },
                                        });

                                        // Broadcast abundance update
                                        let _ = command_tx.send(network::TransportCommand::Broadcast {
                                            message: ServerMessage::AbundanceUpdate {
                                                global_abundance: global_abundance.total,
                                                reason: format!("Sustainable harvest of {} x{:.1} by player {}", node.resource_type, amount, player_id),
                                            },
                                        });

                                        // ResourceUpdate for all interested
                                        let _ = command_tx.send(network::TransportCommand::Broadcast {
                                            message: ServerMessage::ResourceUpdate {
                                                node_id,
                                                resource_type: node.resource_type.clone(),
                                                remaining: node.remaining,
                                                harvested_by: Some(player_id),
                                            },
                                        });
                                    }
                                }
                            }
                            // ... other messages (Move, Jump, AbilityCast, Divine, RBE queries) unchanged ...
                            _ => {}
                        }
                    }
                }
            }

            _ = tick.tick() => {
                // ... existing projectile update, lag recording, health regen, interest culling ...

                // Simple global abundance simulation (RBE slow growth when sustainable)
                let current_time = std::time::SystemTime::now().duration_since(std::UNIX_EPOCH).unwrap().as_millis() as u64;
                if current_time > global_abundance.last_update_ms + 5000 {
                    global_abundance.total = (global_abundance.total + 2.0).min(5000.0); // slow natural abundance growth
                    global_abundance.last_update_ms = current_time;

                    if global_abundance.total > 2000.0 {
                        let _ = command_tx.send(network::TransportCommand::Broadcast {
                            message: ServerMessage::AbundanceUpdate {
                                global_abundance: global_abundance.total,
                                reason: "Natural abundance flow increasing — RBE thriving".to_string(),
                            },
                        });
                    }
                }

                // Resource node regen (from v16.0)
                for node in resource_nodes.values_mut() {
                    if node.remaining < node.max {
                        node.remaining = (node.remaining + node.regen_per_tick).min(node.max);
                    }
                }

                // ... rest of tick (WorldUpdate per client via InterestManager) ...
            }
        }
    }
}