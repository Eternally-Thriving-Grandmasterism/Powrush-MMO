/*!
 * RBE Components
 *
 * Core per-entity data for the Resource-Based Economy layer.
 *
 * v1.1 | Added NodeOwnership
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashMap;

/// Marks an entity as a resource node that can be harvested.
#[derive(Component, Debug, Clone)]
pub struct ResourceNode {
    pub resource_type: String,
    pub current_amount: f32,
    pub max_capacity: f32,
    pub regeneration_rate: f32,
}

/// Player's personal RBE inventory.
#[derive(Component, Debug, Clone, Default)]
pub struct PlayerRbeInventory {
    pub resources: HashMap<String, f32>,
}

/// Marks an entity as participating in the RBE.
#[derive(Component, Debug, Clone)]
pub struct RbeParticipant;

/// Optional component for nodes that are currently being harvested.
#[derive(Component, Debug, Clone)]
pub struct BeingHarvested {
    pub harvester: Entity,
    pub start_time: f32,
}

/// Ownership of a resource node (player, faction, or public).
#[derive(Component, Debug, Clone)]
pub struct NodeOwnership {
    pub owner: Option<u64>, // None = public / unowned
}
