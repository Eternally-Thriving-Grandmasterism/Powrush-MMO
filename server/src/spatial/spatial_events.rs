//! server/src/spatial/spatial_events.rs
//! Production-grade Spatial Event System for Powrush-MMO
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use crate::spatial::hierarchical_grid::Vec3;
use powrush_rbe_engine::RbeResourcePool;
use ra_thor_mercy::{MercyGate, evaluate_mercy_gates};
use lattice_conductor::SovereignLattice;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum SpatialEvent {
    EntityMoved { entity_id: u64, old_pos: Vec3, new_pos: Vec3 },
    RbeNodeSpawned { node_id: u64, pos: Vec3, resource_type: String },
    JoySanctuaryActivated { sanctuary_id: u64, pos: Vec3, radius: f32 },
    FactionHarmonyShift { faction_a: u64, faction_b: u64, delta: f32 },
    PlayerEnteredAOI { player_id: u64, entity_id: u64 },
}

pub struct SpatialEventBus {
    sender: mpsc::Sender<SpatialEvent>,
    receiver: mpsc::Receiver<SpatialEvent>,
    rbe_pool: Arc<RbeResourcePool>,
    lattice: Arc<SovereignLattice>,
}

impl SpatialEventBus {
    pub fn new(rbe_pool: Arc<RbeResourcePool>, lattice: Arc<SovereignLattice>) -> Self {
        let (sender, receiver) = mpsc::channel(512);
        Self { sender, receiver, rbe_pool, lattice }
    }

    pub async fn send(&self, event: SpatialEvent) {
        let _ = self.sender.send(event).await;
    }

    pub async fn process_events(&mut self) {
        while let Ok(event) = self.receiver.try_recv() {
            // Mercy-gated event processing
            let gates = [
                MercyGate::Truth,
                MercyGate::Order,
                MercyGate::Love,
                MercyGate::Compassion,
                MercyGate::Service,
                MercyGate::Abundance,
                MercyGate::Joy,
                MercyGate::CosmicHarmony,
            ];

            let valence = evaluate_mercy_gates(&gates, &event).await;
            if valence < 0.999999 {
                continue; // refinement required
            }

            match event {
                SpatialEvent::EntityMoved { entity_id, old_pos, new_pos } => {
                    // Update spatial grid via SpatialManager (already integrated)
                    self.lattice.tick(&format!("Entity {} moved", entity_id)).await.ok();
                }
                SpatialEvent::RbeNodeSpawned { node_id, pos, resource_type } => {
                    self.rbe_pool.add_node(node_id, pos, resource_type);
                    self.lattice.tick("RBE node spawned with abundance").await.ok();
                }
                SpatialEvent::JoySanctuaryActivated { sanctuary_id, pos, radius } => {
                    self.lattice.tick(&format!("Joy Sanctuary {} activated", sanctuary_id)).await.ok();
                }
                SpatialEvent::FactionHarmonyShift { faction_a, faction_b, delta } => {
                    self.lattice.tick(&format!("Faction harmony shift {} → {}", faction_a, faction_b)).await.ok();
                }
                SpatialEvent::PlayerEnteredAOI { player_id, entity_id } => {
                    self.lattice.tick(&format!("Player {} entered AOI of {}", player_id, entity_id)).await.ok();
                }
            }
        }
    }
}
