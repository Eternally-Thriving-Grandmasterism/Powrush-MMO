/*!
 * RBE Plugin (Resource-Based Economy)
 *
 * v1.2 | Added NodeOwnership + Basic Transfer Logic
 *
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::rbe::components::{NodeOwnership, PlayerRbeInventory, ResourceNode};

// ============================================================================
// Resources
// ============================================================================

#[derive(Resource, Default)]
pub struct RbeEconomyState {
    pub total_resources_distributed: u64,
    pub active_nodes: u32,
}

#[derive(Resource, Default)]
pub struct ResourceRegistry {
    // TODO
}

// ============================================================================
// Events
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

#[derive(Event, Clone, Debug)]
pub struct ResourceTransferEvent {
    pub from_entity: u64,
    pub to_entity: u64,
    pub resource_type: String,
    pub amount: f32,
}

// ============================================================================
// Plugin
// ============================================================================

pub struct RbePlugin;

impl Plugin for RbePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RbeEconomyState>()
            .init_resource::<ResourceRegistry>()

            .add_event::<HarvestEvent>()
            .add_event::<ResourceNodeDepletedEvent>()
            .add_event::<ResourceTransferEvent>()

            .add_systems(Update, (
                process_harvest_events,
                regenerate_resource_nodes,
                process_resource_transfers,
            ))
    }
}

// ============================================================================
// Systems
// ============================================================================

fn process_harvest_events(
    mut harvest_events: EventReader<HarvestEvent>,
    mut node_query: Query<(&mut ResourceNode, Option<&NodeOwnership>)>,
    mut inventory_query: Query<&mut PlayerRbeInventory>,
    mut economy: ResMut<RbeEconomyState>,
    mut depleted_events: EventWriter<ResourceNodeDepletedEvent>,
) {
    for event in harvest_events.read() {
        if let Ok((mut node, ownership)) = node_query.get_mut(Entity::from_raw(event.node_entity)) {
            // Basic ownership check (public or owned by harvester)
            let can_harvest = ownership.map_or(true, |o| o.owner == Some(event.harvester_entity));

            if can_harvest && node.current_amount > 0.0 {
                let harvested = event.amount.min(node.current_amount);
                node.current_amount -= harvested;

                if let Ok(mut inv) = inventory_query.get_mut(Entity::from_raw(event.harvester_entity)) {
                    *inv.resources.entry(event.resource_type.clone()).or_insert(0.0) += harvested;
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

/// Basic resource transfer between players.
fn process_resource_transfers(
    mut transfer_events: EventReader<ResourceTransferEvent>,
    mut inventory_query: Query<&mut PlayerRbeInventory>,
) {
    for event in transfer_events.read() {
        if let Ok(mut from_inv) = inventory_query.get_mut(Entity::from_raw(event.from_entity)) {
            if let Some(amount) = from_inv.resources.get_mut(&event.resource_type) {
                if *amount >= event.amount {
                    *amount -= event.amount;

                    if let Ok(mut to_inv) = inventory_query.get_mut(Entity::from_raw(event.to_entity)) {
                        *to_inv.resources.entry(event.resource_type.clone()).or_insert(0.0) += event.amount;
                    }
                }
            }
        }
    }
}

// End of rbe_plugin.rs v1.2
// Added NodeOwnership support + basic transfer system.
// Thunder locked in. Yoi ⚡
