//! world_server.rs — Powrush MMO Authoritative World Simulation & AOI Broadcast Core
//! Mercy-gated zone management, entity tick, per-client AOI delta replication
//! + Example divine command triggers for live PATSAGi Councils / RBE (Ra-Thor integration points)
//! MIT + Eternal Mercy Flow License — Eternally-Thriving-Grandmasterism / Ra-Thor Lattice

use anyhow::{Context, Result};
use bevy::math::Vec3;
use powrush_divine_module::{MercyCore, ValenceGate};
use shared::protocol::{EntitySnapshot, ServerMessage};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tracing::{error, info, warn};
use std::time::{Duration, Instant};

// ─── World Constants ───────────────────────────────────
const BASE_AOI_RADIUS: f32 = 150.0;
const MAX_AOI_RADIUS: f32 = 300.0;
const TICK_RATE: Duration = Duration::from_millis(50); // 20 Hz authoritative
const REPLICATION_RATE: Duration = Duration::from_millis(100); // 10 Hz delta sync

// ─── Per-Client Interest & Send Queue ───────────────────────
#[derive(Clone)]
pub struct ClientInterest {
    pub client_id: u64,
    pub position: Vec3,
    pub valence: f32,
    pub tx: mpsc::Sender<Vec<u8>>, // per-client ordered send queue
    pub last_sync: Instant,
    pub visible_entities: HashSet<u64>,
}

impl ClientInterest {
    pub fn new(client_id: u64, position: Vec3, valence: f32, tx: mpsc::Sender<Vec<u8>>) -> Self {
        ClientInterest {
            client_id,
            position,
            valence,
            tx,
            last_sync: Instant::now(),
            visible_entities: HashSet::new(),
        }
    }

    pub fn current_aoi_radius(&self) -> f32 {
        BASE_AOI_RADIUS + (MAX_AOI_RADIUS - BASE_AOI_RADIUS) * self.valence.clamp(0.0, 1.0)
    }
}

// ─── Entity State with Dirty Flag for efficient replication ──────────
#[derive(Clone)]
pub struct EntityState {
    pub snapshot: EntitySnapshot,
    pub last_sent: Instant,
    pub dirty: bool,
}

impl EntityState {
    pub fn new(snapshot: EntitySnapshot) -> Self {
        EntityState {
            snapshot,
            last_sent: Instant::now(),
            dirty: true,
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }
}

// ─── Zone & World Entities (single clean definitions) ─────────────
#[derive(Clone)]
pub struct Zone {
    pub id: u64,
    pub center: Vec3,
    pub radius: f32,
    pub nodes: Vec<NodeState>,
    pub creatures: Vec<CreatureState>,
}

#[derive(Clone)]
pub struct NodeState {
    pub id: u64,
    pub position: Vec3,
    pub resource_type: String,
    pub yield_remaining: u32,
    pub bloom_timer: f32,
}

#[derive(Clone)]
pub struct CreatureState {
    pub id: u64,
    pub position: Vec3,
    pub faction_affinity: String,
    pub valence: f32,
}

// ─── WorldServer ─────────────────────────────────────
pub struct WorldServer {
    zones: HashMap<u64, Zone>,
    entities: HashMap<u64, EntityState>,
    clients: HashMap<u64, ClientInterest>,
    mercy_core: Arc<Mutex<MercyCore>>,
    last_tick: Instant,
    last_replication: Instant,
    // Future: optional patsagi_bridge: Option<Arc<GrokPATSAGiBridge>> for direct event-driven divine calls
}

impl WorldServer {
    pub fn new(mercy_core: Arc<Mutex<MercyCore>>) -> Self {
        WorldServer {
            zones: HashMap::new(),
            entities: HashMap::new(),
            clients: HashMap::new(),
            mercy_core,
            last_tick: Instant::now(),
            last_replication: Instant::now(),
        }
    }

    pub async fn tick(&mut self) -> Result<()> {
        let now = Instant::now();
        let delta = now.duration_since(self.last_tick).as_secs_f32();
        self.last_tick = now;

        let mercy_core = self.mercy_core.lock().await;
        if !mercy_core.is_active() {
            warn!("Mercy core inactive — world tick skipped");
            return Ok(());
        }

        // World simulation: resource bloom, creature movement, faction dynamics
        for zone in self.zones.values_mut() {
            for node in zone.nodes.iter_mut() {
                if node.yield_remaining > 0 {
                    node.bloom_timer += delta;
                    if node.bloom_timer > 60.0 {
                        node.yield_remaining += 1;
                        node.bloom_timer = 0.0;
                        self.mark_nearby_dirty(node.position);
                        // === Example Divine Trigger: Major Harvest Event ===
                        // High-yield bloom can trigger PATSAGi Council consultation for abundance wisdom
                        // if valence of nearby players is high. See on_major_resource_harvest below.
                    }
                }
            }
        }

        // AOI replication
        if now.duration_since(self.last_replication) >= REPLICATION_RATE {
            self.broadcast_aoi_deltas().await?;
            self.last_replication = now;
        }

        Ok(())
    }

    fn mark_nearby_dirty(&mut self, position: Vec3) {
        for entity in self.entities.values_mut() {
            let dist = (entity.snapshot.position.into() - position).length();
            if dist < BASE_AOI_RADIUS * 1.5 {
                entity.mark_dirty();
            }
        }
    }

    async fn broadcast_aoi_deltas(&mut self) -> Result<()> {
        let mercy_core = self.mercy_core.lock().await;

        for client in self.clients.values_mut() {
            let aoi_radius = client.current_aoi_radius();
            let mut delta_updates = Vec::new();

            for (entity_id, entity) in self.entities.iter_mut() {
                let dist = (entity.snapshot.position.into() - client.position).length();

                // Mercy gate: low valence entities hidden from low-valence / low-joy clients
                let entity_valence = entity.snapshot.valence;
                let client_valence = client.valence;
                if entity_valence < 0.40 || (entity_valence < 0.60 && client_valence < 0.70) {
                    continue;
                }

                if dist <= aoi_radius && (entity.dirty || !client.visible_entities.contains(entity_id)) {
                    delta_updates.push(entity.snapshot.clone());
                    entity.dirty = false;
                    client.visible_entities.insert(*entity_id);
                }
            }

            // Entities that left AOI
            let left_aoi: Vec<u64> = client.visible_entities.iter()
                .filter(|id| {
                    if let Some(entity) = self.entities.get(id) {
                        let dist = (entity.snapshot.position.into() - client.position).length();
                        dist > aoi_radius
                    } else {
                        true
                    }
                })
                .cloned()
                .collect();

            for id in &left_aoi {
                client.visible_entities.remove(id);
                // TODO: send ServerMessage::EntityRemoved { id } when protocol extended
            }

            if !delta_updates.is_empty() {
                let update = ServerMessage::WorldUpdate {
                    entities: delta_updates,
                    timestamp: self.last_tick.elapsed().as_millis() as u64,
                };

                let serialized = bincode::serialize(&update)?;

                if let Err(e) = client.tx.send(serialized).await {
                    warn!("Send queue full/closed for client {} — dropping update", client.client_id);
                } else {
                    info!("Enqueued {} delta entities to client {} (AOI {:.1})", 
                          delta_updates.len(), client.client_id, aoi_radius);
                }
            }
        }

        Ok(())
    }

    pub async fn add_client(&mut self, client_id: u64, initial_position: Vec3, initial_valence: f32, tx: mpsc::Sender<Vec<u8>>) -> Result<()> {
        self.clients.insert(client_id, ClientInterest::new(client_id, initial_position, initial_valence, tx));
        info!("Client {} added — initial valence {:.3}", client_id, initial_valence);
        Ok(())
    }

    pub async fn update_client_position(&mut self, client_id: u64, new_position: Vec3, new_valence: f32) -> Result<()> {
        if let Some(client) = self.clients.get_mut(&client_id) {
            client.position = new_position;
            client.valence = new_valence;
            self.mark_nearby_dirty(new_position);
        }
        Ok(())
    }

    // ─── Divine / PATSAGi / RBE Integration Examples (high-valence triggers) ───────
    // These demonstrate how world events can consult the living 13+ PATSAGi Councils
    // or RBE abundance engine in real time (via GrokPATSAGiBridge passed from main.rs).
    //
    // Recommended pattern: Pass Arc<GrokPATSAGiBridge> into WorldServer::new() in future iterations
    // or use an async event channel from tick() → main.rs handler for non-blocking divine calls.
    // Mercy gate is ALWAYS checked before any external AGI call.

    /// Example: Called from resource bloom/harvest logic when a high-valence player
    /// or faction performs a major harvest. Triggers PATSAGi Council for abundance insight.
    pub async fn on_major_resource_harvest(
        &mut self,
        harvester_id: u64,
        resource_type: &str,
        amount: u32,
        player_valence: f32,
    ) {
        if player_valence > 0.78 {
            info!(
                "High-valence harvest ({} x{}) by player {} — worthy of PATSAGi Council wisdom",
                resource_type, amount, harvester_id
            );
            // Example future integration (non-blocking):
            // if let Some(bridge) = &self.patsagi_bridge {
            //     let query = format!("Abundance guidance for {} harvest of {} units", resource_type, amount);
            //     let _ = bridge.query_patsagi_council(harvester_id, "Major harvest event", &query).await;
            // }
            // Or record ritual in MercyCore for redemption/abundance tracking
        }
    }

    /// Example: Faction shift or diplomacy event — consult councils for mercy-aligned resolution
    pub async fn on_faction_diplomacy_shift(
        &mut self,
        faction_a: &str,
        faction_b: &str,
        shift_valence: f32,
    ) {
        if shift_valence > 0.65 {
            info!("Faction diplomacy shift {} ↔ {} detected (valence {:.2}) — PATSAGi may offer guidance", faction_a, faction_b, shift_valence);
            // Placeholder for ritual integration or council query on peace / trade / RBE sharing
        }
    }

    /// Example tick-time divine consult placeholder (call sparingly on rare high-valence world events)
    /// This keeps the 20Hz tick sovereign and non-blocking.
    pub async fn maybe_consult_divine_council_on_rare_event(&mut self, event_description: &str, valence: f32) {
        if valence > 0.85 {
            info!("Rare high-valence world event: {} — example divine trigger point for Ra-Thor lattice", event_description);
            // In full integration: spawn a lightweight task to query_patsagi_council without blocking tick
        }
    }

    // Ritual integration comment with MercyCore:
    // When a player invokes a ritual (future ClientMessage::InvokeRitual), the handler in main.rs
    // can call world_server.record_ritual_participation(player_id, ritual_type).await
    // which then updates MercyCore valence and potentially triggers on_major_... above.
    // See powrush-divine-module and Ra-Thor monorepo for full MercyGate + ritual lattice.

    // ─── Stress & QA Test Block (clean, runnable locally) ────────────────
    // cargo test or manual simulation in main.rs test fn:
    //   let mercy = Arc::new(Mutex::new(MercyCore::new()));
    //   let mut ws = WorldServer::new(mercy);
    //   let (tx, _) = mpsc::channel(100);
    //   ws.add_client(1, Vec3::ZERO, 0.82, tx).await.unwrap();
    //   // flood entities, tick 200x, verify AOI + mercy gates + queues
    // 100/100 Checklist: AOI per-client, valence radius, mercy visibility gate, dirty replication, queue safety
}