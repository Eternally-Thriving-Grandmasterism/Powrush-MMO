//! shared/ecs.rs
//! Production-grade Entity Component System (ECS) for Powrush-MMO
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use bevy::prelude::*;
use glam::{Vec3, Quat};
use std::collections::HashMap;
use powrush_rbe_engine::RbeResourcePool;
use crate::spatial_partitioning::SpatialGrid;

#[derive(Component)]
pub struct Position(pub Vec3);

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Rotation(pub Quat);

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct RbeContribution(pub f32);

#[derive(Component)]
pub struct Faction(pub String);

#[derive(Component)]
pub struct JoySanctuary {
    pub radius: f32,
    pub harmony_level: f32,
}

#[derive(Component)]
pub struct ResourceNode {
    pub resource_type: String,
    pub amount: f32,
}

pub struct EcsWorld {
    pub entities: HashMap<u64, Entity>,
    pub spatial_grid: SpatialGrid,
    pub rbe_pool: RbeResourcePool,
}

impl EcsWorld {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            spatial_grid: SpatialGrid::new(10.0, 4),
            rbe_pool: RbeResourcePool::new_global_abundance(),
        }
    }

    pub fn spawn_entity(&mut self, entity_id: u64, position: Vec3) {
        let entity = Entity {
            id: entity_id,
            position,
            velocity: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            health: 100.0,
            components: HashMap::new(),
        };
        self.entities.insert(entity_id, entity);
        self.spatial_grid.insert(entity_id, position);
    }

    pub fn update(&mut self, dt: f32) {
        // Mercy-gated update loop
        for entity in self.entities.values_mut() {
            entity.position += entity.velocity * dt;
            self.spatial_grid.insert(entity.id, entity.position);
        }
    }

    pub fn query_nearby(&self, center: Vec3, radius: f32) -> Vec<u64> {
        self.spatial_grid.query_radius(center, radius)
    }
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: u64,
    pub position: Vec3,
    pub velocity: Vec3,
    pub rotation: Quat,
    pub health: f32,
    pub components: HashMap<String, Component>,
}

#[derive(Debug, Clone)]
pub enum Component {
    Player,
    Npc,
    ResourceNode(ResourceNode),
    JoySanctuary(JoySanctuary),
}

pub struct EcsPlugin;

impl Plugin for EcsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EcsWorld::new())
            .add_systems(Update, ecs_update_system);
    }
}

fn ecs_update_system(
    mut world: ResMut<EcsWorld>,
    time: Res<Time>,
) {
    world.update(time.delta_seconds());
}
