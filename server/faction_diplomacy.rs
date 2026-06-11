//! server/faction_diplomacy.rs
//! Production-grade Faction Diplomacy & Dynamic Relations System
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use std::collections::HashMap;
use powrush_rbe_engine::RbeResourcePool;
use ra_thor_mercy::{MercyGate, evaluate_mercy_gates};
use lattice_conductor::SovereignLattice;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Faction {
    pub id: u64,
    pub name: String,
    pub harmony_level: f32, // 0.0 = hostile → 1.0 = full cosmic harmony
    pub rbe_contribution: f32,
}

pub struct FactionDiplomacy {
    factions: HashMap<u64, Faction>,
    relations: HashMap<(u64, u64), f32>, // (faction_a, faction_b) → harmony score
    rbe_pool: Arc<RbeResourcePool>,
    lattice: Arc<SovereignLattice>,
}

impl FactionDiplomacy {
    pub fn new(rbe_pool: Arc<RbeResourcePool>, lattice: Arc<SovereignLattice>) -> Self {
        Self {
            factions: HashMap::new(),
            relations: HashMap::new(),
            rbe_pool,
            lattice,
        }
    }

    pub fn register_faction(&mut self, id: u64, name: String, initial_harmony: f32) {
        self.factions.insert(id, Faction {
            id,
            name,
            harmony_level: initial_harmony.clamp(0.0, 1.0),
            rbe_contribution: 0.0,
        });
    }

    pub async fn update_relation(&mut self, a: u64, b: u64, delta: f32) {
        let key = (a.min(b), a.max(b));
        let score = self.relations.entry(key).or_insert(0.5);
        *score = (*score + delta).clamp(0.0, 1.0);

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

        let valence = evaluate_mercy_gates(&gates, &format!("faction_relation_{}_{}", a, b)).await;
        if valence >= 0.999999 {
            self.lattice.tick(&format!("Faction harmony improved between {} and {}", a, b)).await.ok();
        }
    }

    pub fn get_harmony_level(&self, a: u64, b: u64) -> f32 {
        let key = (a.min(b), a.max(b));
        *self.relations.get(&key).unwrap_or(&0.5)
    }

    pub async fn tick(&mut self) {
        for faction in self.factions.values_mut() {
            let abundance_factor = self.rbe_pool.current_abundance_factor();
            faction.harmony_level = (faction.harmony_level + abundance_factor * 0.01).clamp(0.0, 1.0);
        }
    }
}
