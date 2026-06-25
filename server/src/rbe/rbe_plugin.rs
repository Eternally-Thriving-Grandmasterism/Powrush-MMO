/*!
 * RBE Plugin (Resource-Based Economy)
 *
 * Core plugin for the Resource-Based Economy simulation layer.
 *
 * v1.1 | Core Systems Added (Harvesting + Regeneration)
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::rbe::components::{PlayerRbeInventory, ResourceNode};

// ============================================================================
// Core RBE Resources
// ============================================================================

#[derive(Resource, Default)]
pub struct RbeEconomyState {
    pub total_resources_distributed: u64,
    pub active_nodes: u32,
}

#[derive(Resource, Default)]
pub struct ResourceRegistry {
    // TODO: Define resource types and properties
}

// ============================================================================
// RBE Events
// ============================================================================

#[derive(Event, Clone, Debug)]
pub struct HarvestEvent {
    pub harvester_entity: u64,
    pub node_entity: u64,
    pub resource_type: String,
    pub amount: f32,
}

#[derive(Event, Clone, Debug)]
pub struct ResourceNodeDepletedEvent {
    pub node_entity: u64,
    pub resource_type: String,
}

// ============================================================================
// RBE Plugin
// ============================================================================

pub struct RbePlugin;

impl Plugin for RbePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RbeEconomyState>()
            .init_resource::<ResourceRegistry>()

            .add_event::<HarvestEvent>()
            .add_event::<ResourceNodeDepletedEvent>()

            .add_systems(Update, (
                process_harvest_events,
                regenerate_resource_nodes,
            ))
    }
}

// ============================================================================
// Core RBE Systems
// ============================================================================

/// Processes harvest requests and updates inventories + nodes.
fn process_harvest_events(
    mut harvest_events: EventReader<HarvestEvent>,
    mut node_query: Query<&mut ResourceNode>,
    mut inventory_query: Query<&mut PlayerRbeInventory>,
    mut economy: ResMut<RbeEconomyState>,
    mut depleted_events: EventWriter<ResourceNodeDepletedEvent>,
) {
    for event in harvest_events.read() {
        // Try to get the resource node
        if let Ok(mut node) = node_query.get_mut(Entity::from_raw(event.node_entity)) {
            let harvested = event.amount.min(node.current_amount);

            if harvested > 0.0 {
                node.current_amount -= harvested;

                // Update player inventory
                if let Ok(mut inventory) = inventory_query.get_mut(Entity::from_raw(event.harvester_entity)) {
                    *inventory.resources.entry(event.resource_type.clone()).or_insert(0.0) += harvested;
                }

                economy.total_resources_distributed += harvested as u64;

                if node.current_amount <= 0.0 {
                    depleted_events.send(ResourceNodeDepletedEvent {
                        node_entity: event.node_entity,
                        resource_type: event.resource_type.clone(),
                    });
                }
            }
        }
    }
}

/// Regenerates resource nodes over time.
fn regenerate_resource_nodes(
    mut node_query: Query<&mut ResourceNode>,
    time: Res<Time>,
) {
    for mut node in node_query.iter_mut() {
        if node.current_amount < node.max_capacity {
            node.current_amount = (node.current_amount + node.regeneration_rate * time.delta_seconds())
                .min(node.max_capacity);
        }
    }
}

// End of rbe_plugin.rs v1.1
// Core harvesting and regeneration systems added.
// Thunder locked in. Yoi ⚡
