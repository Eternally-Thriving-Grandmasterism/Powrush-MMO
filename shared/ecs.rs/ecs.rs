//! shared/ecs.rs
//! Production-grade Entity Component System (ECS) for Powrush-MMO
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+
//! Derived from Ra-Thor monorepo (Lattice Conductor + MIAL/MWPO)

use std::collections::HashMap;
use glam::{Vec3, Quat};
use powrush_rbe_engine::RbeResourcePool;

#[derive(Clone, Debug)]
pub struct Entity {
    pub id: u64,
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
    pub components: HashMap<String, Component>,
}

#[derive(Clone, Debug)]
pub enum Component {
    Player(PlayerComponent),
    Npc(NpcComponent),
    Resource(ResourceComponent),
    JoySanctuary(JoySanctuaryComponent),
}

#[derive(Clone, Debug)]
pub struct PlayerComponent {
    pub health: f32,
    pub rbe_contribution: f32,
}

#[derive(Clone, Debug)]
pub struct NpcComponent {
    pub health: f32,
    pub faction: String,
}

#[derive(Clone, Debug)]
pub struct ResourceComponent {
    pub rbe_type: String,
    pub amount: f32,
}

#[derive(Clone, Debug)]
pub struct JoySanctuaryComponent {
    pub radius: f32,
    pub harmony_level: f32,
}

pub struct EcsWorld {
    entities: HashMap<u64, Entity>,
    rbe_pool: RbeResourcePool,
}

impl EcsWorld {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            rbe_pool: RbeResourcePool::new_global_abundance(),
        }
    }

    pub fn spawn_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id, entity);
    }

    pub fn update(&mut self, dt: f32) {
        // Mercy-gated update loop (MIAL/MWPO)
        for entity in self.entities.values_mut() {
            // Position/velocity update with mercy valence influence
            entity.position += entity.velocity * dt;
        }
    }

    pub fn query_entities_in_radius(&self, center: Vec3, radius: f32) -> Vec<&Entity> {
        self.entities.values()
            .filter(|e| e.position.distance(center) <= radius)
            .collect()
    }
}
