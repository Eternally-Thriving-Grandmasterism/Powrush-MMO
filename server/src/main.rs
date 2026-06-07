// server/src/main.rs
// Powrush-MMO Server v15.8 — Projectile Travel Time + Basic Effects Scaffolding
// Builds on v15.7 LagComp + HitDetection tightening
// Simple authoritative projectile simulation with travel delay + impact events
// Ready for full projectile system + visual effects in next iteration

mod network;
mod interest_management;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{info, warn};
use shared::protocol::*;
use crate::interest_management::InterestManager;
use game::lag_compensation::LagCompensation;
use game::hit_detection::{HitDetection, HitRequest};

// ... (previous MercyCore, WorldServer, GrokPatsagiBridge, etc. unchanged from v15.7) ...

/// Simple projectile for travel time simulation (v15.8 scaffolding)
#[derive(Clone, Debug)]
struct ActiveProjectile {
    id: u64,
    shooter_id: u64,
    target_id: Option<u64>,
    start_pos: Vec3Ser,
    target_pos: Vec3Ser,
    start_time_ms: u64,
    travel_time_ms: u64, // e.g. 300-800ms depending on distance/speed
    damage: f32,
    is_critical: bool,
    ability_id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("powrush_server=info,tokio_tungstenite=warn")
        .init();

    info!("⚡ Powrush-MMO Server v15.8 — Projectile Travel Time + Basic Effects Scaffolding ACTIVATED");
    info!("Lag-compensated combat + projectile travel simulation + PATSAGi validation. Mercy gates online.");

    // ... (initialization same as v15.7) ...
    let mercy_core = Arc::new(MercyCore::new());
    let world_server = Arc::new(Mutex::new(WorldServer::new()));
    let bridge = Arc::new(GrokPatsagiBridge::new());

    let (mut transport, mut event_rx, command_tx) =
        network::TokioTransport::new("0.0.0.0:9001").await?;

    tokio::spawn(async move { transport.run().await; });

    let mut players: HashMap<u64, (String, Vec3Ser, HealthComponent)> = HashMap::new();
    let mut interest_manager = InterestManager::new(100.0);
    let mut cooldowns: CooldownTracker = HashMap::new();
    let mut lag_comp = LagCompensation::new(game::lag_compensation::LagCompensationConfig::default());
    let mut hit_detection = HitDetection::new(lag_comp.clone());

    // === Active Projectiles (v15.8) ===
    let mut active_projectiles: Vec<ActiveProjectile> = Vec::new();
    let mut next_projectile_id: u64 = 1;

    let mut tick = tokio::time::interval(Duration::from_millis(50)); // 20 TPS

    info!("Server listening on ws://0.0.0.0:9001 — Projectile travel time simulation ready");

    loop {
        tokio::select! {
            biased;

            Some(event) = event_rx.recv() => {
                match event {
                    network::TransportEvent::ClientConnected { info } => { /* same as v15.7 */ }
                    network::TransportEvent::ClientDisconnected { player_id } => { /* same */ }
                    network::TransportEvent::MessageReceived { player_id, message } => {
                        if mercy_core.gate_server_message(&message).is_err() { continue; }

                        match message {
                            ClientMessage::Move { delta } => { /* same */ }
                            ClientMessage::Jump => { /* same */ }
                            ClientMessage::AbilityCast { ability_id, target_id, position: _ } => {
                                // PATSAGi validation + cooldown check (same as v15.7)
                                let validation = bridge.validate_ability_cast(player_id, ability_id, target_id).await;
                                // ... (validation and cooldown logic same as v15.7) ...

                                if let Ok((approved, reason, valence_impact)) = validation {
                                    if !approved { /* block */ continue; }

                                    // Cooldown check (same)
                                    // ...

                                    let is_melee = ability_id % 2 == 0;
                                    let damage = if is_melee { 35.0 } else { 22.5 };
                                    let is_critical = rand::random::<f32>() > 0.85;
                                    let final_damage = if is_critical { damage * 1.8 } else { damage };

                                    let now = std::time::SystemTime::now().duration_since(std::UNIX_EPOCH).unwrap().as_millis() as u64;

                                    if is_melee {
                                        // Melee: immediate hitscan via HitDetection (v15.7 path)
                                        // ... (existing immediate melee logic) ...
                                    } else {
                                        // === PROJECTILE with Travel Time (v15.8 new) ===
                                        if let Some(target) = target_id {
                                            if let Some((_, shooter_pos, _)) = players.get(&player_id) {
                                                if let Some((_, target_pos, _)) = players.get(&target) {
                                                    let distance = {
                                                        let dx = shooter_pos.x - target_pos.x;
                                                        let dy = shooter_pos.y - target_pos.y;
                                                        let dz = shooter_pos.z - target_pos.z;
                                                        (dx*dx + dy*dy + dz*dz).sqrt()
                                                    };
                                                    let travel_time_ms = (distance * 8.0) as u64 + 150; // ~ speed scaling + base

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

                                                    // Launch event
                                                    let _ = command_tx.send(network::TransportCommand::Send {
                                                        player_id,
                                                        message: ServerMessage::CombatEvent {
                                                            event_type: "projectile_launched".to_string(),
                                                            data: format!("Projectile {} fired | Travel time: {}ms | PATSAGi: {}", ability_id, travel_time_ms, reason),
                                                        },
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            // ... other messages same ...
                            _ => {}
                        }
                    }
                }
            }

            _ = tick.tick() => {
                // === Update Active Projectiles (v15.8) ===
                let current_time = std::time::SystemTime::now().duration_since(std::UNIX_EPOCH).unwrap().as_millis() as u64;

                let mut i = 0;
                while i < active_projectiles.len() {
                    let proj = &active_projectiles[i];
                    if current_time >= proj.start_time_ms + proj.travel_time_ms {
                        // Impact!
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
                                    data: format!("Projectile {} impacted target {} | Damage: {:.1}{} | Travel time complete", proj.ability_id, target, proj.damage, if proj.is_critical { " (CRIT)" } else { "" }),
                                },
                            });
                        }
                        active_projectiles.remove(i);
                    } else {
                        i += 1;
                    }
                }

                // Build WorldUpdate with Health (same as v15.7)
                // Record lag comp snapshots (same)
                // Health regen (same)

                // ... rest of tick same as v15.7 ...
            }
        }
    }
}
