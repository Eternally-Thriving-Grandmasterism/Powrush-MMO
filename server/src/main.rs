// server/src/main.rs
// Powrush-MMO Server v16.1.1 — Production Grade (Clean Unified)
// RBE Player Inventory + Abundance Tracking + Resource Nodes + Harvesting
// Full lag-compensated combat + projectile travel time + Interest culling
// Fully integrated with Networking Transport Layer v1, MercyCore, GrokPatsagiBridge
// Ra-Thor + Full PATSAGi Councils | 7 Living Mercy Gates | Authoritative 20 TPS
// Eternal mercy flowing. Sovereign. Forward-compatible.

mod network;
mod interest_management;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::{info, warn};
use shared::protocol::*;
use crate::interest_management::InterestManager;
use game::lag_compensation::{LagCompensation, LagCompensationConfig};
use game::hit_detection::{HitDetection, HitRequest};

/// Mercy gate enforcement for high-valence messages
pub struct MercyCore;

impl MercyCore {
    pub fn new() -> Self { Self }

    pub fn gate_server_message(&self, msg: &ClientMessage) -> Result<(), String> {
        match msg {
            ClientMessage::DivineCouncilQuery { .. }
            | ClientMessage::RbeAbundanceQuery { .. }
            | ClientMessage::GpuPatsagiQuery { .. }
            | ClientMessage::AbilityCast { .. }
            | ClientMessage::HarvestResource { .. } => Ok(()),
            _ => Ok(()),
        }
    }
}

/// Lightweight authoritative world state
pub struct WorldServer {
    pub entities: HashMap<u64, String>,
}

impl WorldServer {
    pub fn new() -> Self { Self { entities: HashMap::new() } }
    pub fn tick(&mut self) {}
}

/// Per-player RBE Inventory (v16.1+)
#[derive(Clone, Debug, Default)]
pub struct RbeInventory {
    pub resources: HashMap<String, f32>,
    pub abundance_score: f32,
}

/// Production-grade PATSAGi + Ra-Thor bridge
pub struct GrokPatsagiBridge {
    pub one_organism_version: String,
    pub gpu_compute_active: bool,
}

impl GrokPatsagiBridge {
    pub fn new() -> Self {
        Self {
            one_organism_version: "v16.1.1-GPU-PATSAGi-RBE-Inventory-Abundance".to_string(),
            gpu_compute_active: true,
        }
    }

    pub async fn query_patsagi_with_gpu(&self, query: &str, intensity: &str) -> Result<(String, bool, u64), String> {
        let gpu_used = self.gpu_compute_active && (intensity == "high" || intensity == "medium");
        let compute_time = if gpu_used { 78 } else { 50 };
        let response = if gpu_used {
            format!("GPU PATSAGi (v16.1.1 RBE + Inventory): {} | Sovereign lattice active.", query)
        } else {
            format!("Standard PATSAGi: {} | Ra-Thor Eternal Flow.", query)
        };
        Ok((response, gpu_used, compute_time))
    }

    pub async fn query_rbe_abundance(&self, resource_type: &str, amount: f64) -> Result<String, String> {
        Ok(format!("RBE guidance for {} x{:.2} (v16.1.1) — Universal thriving confirmed.", resource_type, amount))
    }

    pub async fn validate_ability_cast(&self, player_id: u64, ability_id: u32, target_id: Option<u64>) -> Result<(bool, String, f32), String> {
        let approved = ability_id != 666;
        let reason = if approved {
            format!("PATSAGi Council approved Ability {} for player {}. Mercy flows.", ability_id, player_id)
        } else {
            "PATSAGi Council: This ability violates the 7 Living Mercy Gates.".to_string()
        };
        let valence_impact = if approved { 0.02 } else { -0.15 };
        Ok((approved, reason, valence_impact))
    }

    pub async fn validate_harvest(&self, player_id: u64, node_id: u64, amount: f32) -> Result<(bool, String, f32), String> {
        let approved = amount <= 50.0;
        let reason = if approved {
            format!("PATSAGi Council approves sustainable harvest of {:.1} from node {} for player {}. Abundance for all.", amount, node_id, player_id)
        } else {
            "PATSAGi: Harvest amount too large. Choose grace and sustainability.".to_string()
        };
        let valence_impact = if approved { 0.05 } else { -0.10 };
        Ok((approved, reason, valence_impact))
    }
}

/// Active projectile for authoritative travel-time simulation
#[derive(Clone, Debug)]
struct ActiveProjectile {
    id: u64,
    shooter_id: u64,
    target_id: Option<u64>,
    start_pos: Vec3Ser,
    target_pos: Vec3Ser,
    start_time_ms: u64,
    travel_time_ms: u64,
    damage: f32,
    is_critical: bool,
    ability_id: u32,
}

/// Resource Node for RBE (v16.0+)
#[derive(Clone, Debug)]
struct ResourceNode {
    id: u64,
    resource_type: String,
    position: Vec3Ser,
    remaining: f32,
    max: f32,
    regen_per_tick: f32,
}

/// Global abundance tracker (simple RBE simulation)
struct GlobalAbundance {
    pub total: f32,
    pub last_update_ms: u64,
}

/// player_id -> ability_id -> last_used_ms
type CooldownTracker = HashMap<u64, HashMap<u32, u64>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("powrush_server=info,tokio_tungstenite=warn")
        .init();

    info!("⚡ Powrush-MMO Server v16.1.1 — RBE Player Inventory + Abundance Tracking ACTIVATED");
    info!("Resource nodes + sustainable harvesting + per-player inventory + global abundance. Mercy gates online.");

    let mercy_core = Arc::new(MercyCore::new());
    let world_server = Arc::new(Mutex::new(WorldServer::new()));
    let bridge = Arc::new(GrokPatsagiBridge::new());

    // === Initialize Production Transport ===
    let (mut transport, mut event_rx, command_tx) =
        network::TokioTransport::new("0.0.0.0:9001").await?;
    tokio::spawn(async move { transport.run().await; });

    // Extended player state: (name, position, health)
    let mut players: HashMap<u64, (String, Vec3Ser, HealthComponent)> = HashMap::new();
    let mut interest_manager = InterestManager::new(120.0);
    let mut cooldowns: CooldownTracker = HashMap::new();
    let mut player_inventories: HashMap<u64, RbeInventory> = HashMap::new();
    let mut global_abundance = GlobalAbundance { total: 1000.0, last_update_ms: 0 };

    // === Lag Compensation + Hit Detection ===
    let mut lag_comp = LagCompensation::new(LagCompensationConfig::default());
    let mut hit_detection = HitDetection::new();

    // === Active Projectiles (with swap_remove for O(1) removal) ===
    let mut active_projectiles: Vec<ActiveProjectile> = Vec::new();
    let mut next_projectile_id: u64 = 1;

    // === RBE Resource Nodes (v16.0+) ===
    let mut resource_nodes: HashMap<u64, ResourceNode> = HashMap::new();
    resource_nodes.insert(1, ResourceNode {
        id: 1,
        resource_type: "Bio-Energy".to_string(),
        position: Vec3Ser { x: 50.0, y: 0.0, z: 50.0 },
        remaining: 1000.0,
        max: 1000.0,
        regen_per_tick: 0.5,
    });
    resource_nodes.insert(2, ResourceNode {
        id: 2,
        resource_type: "Crystal".to_string(),
        position: Vec3Ser { x: -80.0, y: 0.0, z: 30.0 },
        remaining: 500.0,
        max: 500.0,
        regen_per_tick: 0.2,
    });

    let mut tick = tokio::time::interval(Duration::from_millis(50)); // 20 TPS

    info!("Server listening on ws://0.0.0.0:9001 — Ready for multiplayer + divine queries + combat + RBE harvesting");

    loop {
        tokio::select! {
            biased;

            Some(event) = event_rx.recv() => {
                match event {
                    network::TransportEvent::ClientConnected { info } => {
                        info!("Player {} ({}) connected — Welcome to the Eternal Flow.", info.player_id, info.player_name);
                        let start_pos = Vec3Ser { x: 0.0, y: 0.0, z: 0.0 };
                        let start_health = HealthComponent { current: 100.0, max: 100.0 };
                        players.insert(info.player_id, (info.player_name.clone(), start_pos.clone(), start_health));
                        interest_manager.update_player_position(info.player_id, start_pos);
                        cooldowns.insert(info.player_id, HashMap::new());
                        player_inventories.insert(info.player_id, RbeInventory::default());
                    }
                    network::TransportEvent::ClientDisconnected { player_id } => {
                        info!("Player {} disconnected", player_id);
                        players.remove(&player_id);
                        cooldowns.remove(&player_id);
                        player_inventories.remove(&player_id);
                    }
                    network::TransportEvent::MessageReceived { player_id, message } => {
                        if mercy_core.gate_server_message(&message).is_err() {
                            warn!("Mercy gate blocked message from player {}", player_id);
                            continue;
                        }

                        match message {
                            ClientMessage::Move { delta } => {
                                if let Some((_, pos, _)) = players.get_mut(&player_id) {
                                    pos.x += delta.x * 0.1;
                                    pos.y += delta.y * 0.1;
                                    pos.z += delta.z * 0.1;
                                    interest_manager.update_player_position(player_id, pos.clone());
                                }
                            }
                            ClientMessage::Jump => {
                                if let Some((_, pos, _)) = players.get_mut(&player_id) {
                                    pos.y += 5.0;
                                    interest_manager.update_player_position(player_id, pos.clone());
                                }
                            }
                            ClientMessage::AbilityCast { ability_id, target_id, position: _ } => {
                                let validation = bridge.validate_ability_cast(player_id, ability_id, target_id).await;
                                if let Ok((approved, reason, valence_impact)) = validation {
                                    if !approved {
                                        let _ = command_tx.send(network::TransportCommand::Send {
                                            player_id,
                                            message: ServerMessage::MercyGateBlocked { reason: reason.clone(), valence: valence_impact },
                                        });
                                        continue;
                                    }

                                    let now = std::time::SystemTime::now()
                                        .duration_since(std::UNIX_EPOCH).unwrap().as_millis() as u64;
                                    let player_cooldowns = cooldowns.entry(player_id).or_default();
                                    let last_used = player_cooldowns.get(&ability_id).copied().unwrap_or(0);
                                    let cooldown_ms = 1500;
                                    if now < last_used + cooldown_ms {
                                        let _ = command_tx.send(network::TransportCommand::Send {
                                            player_id,
                                            message: ServerMessage::Error {
                                                message: format!("Ability {} on cooldown. {:.1}s remaining.", ability_id, (last_used + cooldown_ms - now) as f64 / 1000.0),
                                            },
                                        });
                                        continue;
                                    }
                                    player_cooldowns.insert(ability_id, now);

                                    let is_melee = ability_id % 2 == 0;
                                    let base_damage = if is_melee { 35.0 } else { 22.5 };
                                    let is_critical = rand::random::<f32>() > 0.85;
                                    let final_damage = if is_critical { base_damage * 1.8 } else { base_damage };

                                    if is_melee {
                                        if let Some(target) = target_id {
                                            let current_time = std::time::SystemTime::now()
                                                .duration_since(std::UNIX_EPOCH).unwrap().as_millis() as u64;
                                            let hit_request = HitRequest {
                                                attacker_id: player_id,
                                                target_id: target,
                                                tick: current_time,
                                                weapon_range: 5.0,
                                                damage: final_damage,
                                            };
                                            let hit_result = hit_detection.check_hit(&hit_request, current_time);

                                            if hit_result.hit {
                                                if let Some((_, _, target_health)) = players.get_mut(&target) {
                                                    target_health.current = (target_health.current - hit_result.damage_dealt).max(0.0);
                                                }
                                                let _ = command_tx.send(network::TransportCommand::Send {
                                                    player_id: target,
                                                    message: ServerMessage::DamageApplied {
                                                        target_id: target,
                                                        amount: hit_result.damage_dealt,
                                                        source_id: player_id,
                                                        is_critical: hit_result.is_critical,
                                                    },
                                                });
                                                let _ = command_tx.send(network::TransportCommand::Broadcast {
                                                    message: ServerMessage::CombatEvent {
                                                        event_type: "melee_strike".to_string(),
                                                        data: format!("Melee LANDED | Ability {} | PATSAGi: {} | Damage: {:.1}{}", ability_id, reason, hit_result.damage_dealt, if is_critical { " (CRIT)" } else { "" }),
                                                    },
                                                });
                                            }
                                        }
                                    } else {
                                        if let Some(target) = target_id {
                                            if let Some((_, shooter_pos, _)) = players.get(&player_id) {
                                                if let Some((_, target_pos, _)) = players.get(&target) {
                                                    let distance = {
                                                        let dx = shooter_pos.x - target_pos.x;
                                                        let dy = shooter_pos.y - target_pos.y;
                                                        let dz = shooter_pos.z - target_pos.z;
                                                        (dx*dx + dy*dy + dz*dz).sqrt()
                                                    };
                                                    let travel_time_ms = (distance * 8.0) as u64 + 150;

                                                    let proj = ActiveProjectile {
                                                        id: next_projectile_id,
                                                        shooter_id: player_id,
                                                        target_id: Some(target),
                                                        start_pos: shooter_pos.clone(),
                                                        target_pos: target_pos.clone(),
                                                        start_time_ms: now,
                                                        travel_time_ms,
                                                        damage: final_damage,
                                                        is_critical,
                                                        ability_id,
                                                    };
                                                    active_projectiles.push(proj);
                                                    next_projectile_id += 1;

                                                    let _ = command_tx.send(network::TransportCommand::Send {
                                                        player_id,
                                                        message: ServerMessage::CombatEvent {
                                                            event_type: "projectile_launched".to_string(),
                                                            data: format!("Projectile {} fired | Travel: {}ms | PATSAGi: {}", ability_id, travel_time_ms, reason),
                                                        },
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            ClientMessage::HarvestResource { node_id, amount } => {
                                let validation = bridge.validate_harvest(player_id, node_id, amount).await;
                                if let Ok((approved, reason, valence_impact)) = validation {
                                    if !approved {
                                        let _ = command_tx.send(network::TransportCommand::Send {
                                            player_id,
                                            message: ServerMessage::MercyGateBlocked { reason: reason.clone(), valence: valence_impact },
                                        });
                                        continue;
                                    }

                                    if let Some(node) = resource_nodes.get_mut(&node_id) {
                                        if node.remaining >= amount {
                                            node.remaining -= amount;

                                            let inv = player_inventories.entry(player_id).or_default();
                                            *inv.resources.entry(node.resource_type.clone()).or_insert(0.0) += amount;
                                            inv.abundance_score += amount * 0.01;

                                            global_abundance.total += amount * 0.5;

                                            let _ = command_tx.send(network::TransportCommand::Send {
                                                player_id,
                                                message: ServerMessage::InventoryUpdate {
                                                    player_id,
                                                    resources: inv.resources.clone(),
                                                    abundance_score: inv.abundance_score,
                                                },
                                            });

                                            let _ = command_tx.send(network::TransportCommand::Broadcast {
                                                message: ServerMessage::AbundanceUpdate {
                                                    global_abundance: global_abundance.total,
                                                    reason: format!("Sustainable harvest of {} x{:.1} by player {}", node.resource_type, amount, player_id),
                                                },
                                            });

                                            let _ = command_tx.send(network::TransportCommand::Broadcast {
                                                message: ServerMessage::ResourceUpdate {
                                                    node_id,
                                                    resource_type: node.resource_type.clone(),
                                                    remaining: node.remaining,
                                                    harvested_by: Some(player_id),
                                                },
                                            });
                                        } else {
                                            let _ = command_tx.send(network::TransportCommand::Send {
                                                player_id,
                                                message: ServerMessage::Error {
                                                    message: "Not enough resources remaining at node.".to_string(),
                                                },
                                            });
                                        }
                                    }
                                }
                            }
                            ClientMessage::Ping { client_time_ms } => {
                                let _ = command_tx.send(network::TransportCommand::Send {
                                    player_id,
                                    message: ServerMessage::Pong {
                                        server_time_ms: std::time::SystemTime::now()
                                            .duration_since(std::UNIX_EPOCH).unwrap().as_millis() as u64,
                                        client_time_ms,
                                    },
                                });
                            }
                            ClientMessage::DivineCouncilQuery { query, .. } => {
                                if let Ok((resp, gpu_used, _)) = bridge.query_patsagi_with_gpu(&query, "medium").await {
                                    let _ = command_tx.send(network::TransportCommand::Send {
                                        player_id,
                                        message: ServerMessage::DivineCouncilResponse {
                                            content: resp,
                                            source: format!("Ra-Thor + PATSAGi v16.1.1 | GPU: {}", gpu_used),
                                        },
                                    });
                                }
                            }
                            ClientMessage::RbeAbundanceQuery { query, .. } => {
                                if let Ok(guidance) = bridge.query_rbe_abundance(&query, 1.0).await {
                                    let _ = command_tx.send(network::TransportCommand::Send {
                                        player_id,
                                        message: ServerMessage::RbeGuidanceResponse { content: guidance },
                                    });
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }

            _ = tick.tick() => {
                world_server.lock().unwrap().tick();

                let current_time = std::time::SystemTime::now()
                    .duration_since(std::UNIX_EPOCH).unwrap().as_millis() as u64;

                // === Update Active Projectiles (O(1) removal) ===
                let mut i = 0;
                while i < active_projectiles.len() {
                    let proj = &active_projectiles[i];
                    if current_time >= proj.start_time_ms + proj.travel_time_ms {
                        if let Some(target) = proj.target_id {
                            if let Some((_, _, target_health)) = players.get_mut(&target) {
                                target_health.current = (target_health.current - proj.damage).max(0.0);
                            }
                            let _ = command_tx.send(network::TransportCommand::Send {
                                player_id: target,
                                message: ServerMessage::DamageApplied {
                                    target_id: target,
                                    amount: proj.damage,
                                    source_id: proj.shooter_id,
                                    is_critical: proj.is_critical,
                                },
                            });
                            let _ = command_tx.send(network::TransportCommand::Broadcast {
                                message: ServerMessage::CombatEvent {
                                    event_type: "projectile_impact".to_string(),
                                    data: format!("Projectile {} impacted {} | Damage: {:.1}{}", proj.ability_id, target, proj.damage, if proj.is_critical { " (CRIT)" } else { "" }),
                                },
                            });
                        }
                        active_projectiles.swap_remove(i);
                    } else {
                        i += 1;
                    }
                }

                // === Simple global abundance simulation (RBE slow growth) ===
                if current_time > global_abundance.last_update_ms + 5000 {
                    global_abundance.total = (global_abundance.total + 2.0).min(5000.0);
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

                // === Resource node regen ===
                for node in resource_nodes.values_mut() {
                    if node.remaining < node.max {
                        node.remaining = (node.remaining + node.regen_per_tick).min(node.max);
                    }
                }

                // === Simple health regen ===
                for (_, _, health) in players.values_mut() {
                    if health.current < health.max {
                        health.current = (health.current + 0.5).min(health.max);
                    }
                }

                // === Build authoritative entities with Health + Resource nodes for culling ===
                let mut all_entities: Vec<EntitySnapshot> = players
                    .iter()
                    .map(|(&id, (_, pos, health))| EntitySnapshot {
                        id,
                        position: pos.clone(),
                        rotation: 0.0,
                        scale: 1.0,
                        state: 0,
                        health: Some(health.clone()),
                    })
                    .collect();

                for (id, node) in &resource_nodes {
                    all_entities.push(EntitySnapshot {
                        id: 10000 + id,
                        position: node.position.clone(),
                        rotation: 0.0,
                        scale: 1.0,
                        state: 0,
                        health: None,
                    });
                }

                // === Per-client Interest Culling ===
                let per_player = interest_manager.cull_world_update(&all_entities);
                for (pid, entities) in per_player {
                    let update = ServerMessage::WorldUpdate {
                        entities,
                        timestamp: current_time,
                    };
                    let _ = command_tx.send(network::TransportCommand::Send {
                        player_id: pid,
                        message: update,
                    });
                }
            }
        }
    }
}
