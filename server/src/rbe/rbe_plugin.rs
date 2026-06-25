/*!
 * RBE Plugin (Resource-Based Economy)
 *
 * v1.6 | Integrated with ServerInterestSyncPlugin via RbeInventoryUpdatedEvent
 *
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashMap;

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
    // TODO: Registry for resource type metadata, scarcity multipliers, etc.
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

#[derive(Event, Clone, Debug)]
pub struct ClaimNodeEvent {
    pub claimer_entity: u64,
    pub node_entity: u64,
}

/// Triggers automated distribution from a node or pool.
#[derive(Event, Clone, Debug)]
pub struct DistributeResourcesEvent {
    pub source_entity: u64,
    pub resource_type: String,
    pub total_amount: f32,
    pub distribution_type: DistributionType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DistributionType {
    ToOwner,
    ToFaction,
    ToNearbyParticipants,
    ProportionalToStanding,
}

/// Emitted after successful RBE distribution to a player so interest/replication layer can react.
#[derive(Event, Clone, Debug)]
pub struct RbeInventoryUpdatedEvent {
    pub player_entity_id: u64,
    pub resource_type: String,
    pub amount_added: f32,
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
            .add_event::<ClaimNodeEvent>()
            .add_event::<DistributeResourcesEvent>()
            .add_event::<RbeInventoryUpdatedEvent>()

            .add_systems(Update, (
                process_harvest_events,
                regenerate_resource_nodes,
                process_resource_transfers,
                process_node_claiming,
                process_distributions,
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

fn process_node_claiming(
    mut claim_events: EventReader<ClaimNodeEvent>,
    mut node_query: Query<&mut NodeOwnership>,
) {
    for event in claim_events.read() {
        if let Ok(mut ownership) = node_query.get_mut(Entity::from_raw(event.node_entity)) {
            if ownership.owner.is_none() {
                ownership.owner = Some(event.claimer_entity);

                info!(
                    "[RBE] Player {} claimed node {}",
                    event.claimer_entity,
                    event.node_entity
                );
            }
        }
    }
}

/// Advanced distribution logic integrated with interest/replication layer.
/// Emits RbeInventoryUpdatedEvent so ServerInterestSyncPlugin can trigger high-priority replication.
fn process_distributions(
    mut dist_events: EventReader<DistributeResourcesEvent>,
    mut inventory_query: Query<&mut PlayerRbeInventory>,
    node_query: Query<(&ResourceNode, &NodeOwnership)>,
    mut rbe_updated_events: EventWriter<RbeInventoryUpdatedEvent>,
) {
    for event in dist_events.read() {
        let mut affected_player: Option<u64> = None;

        match event.distribution_type {
            DistributionType::ToOwner => {
                if let Ok((_, ownership)) = node_query.get(Entity::from_raw(event.source_entity)) {
                    let target = ownership.owner.unwrap_or(event.source_entity);
                    if let Ok(mut inv) = inventory_query.get_mut(Entity::from_raw(target)) {
                        *inv.resources.entry(event.resource_type.clone()).or_insert(0.0) += event.total_amount;
                        affected_player = Some(target);
                    }
                }
            }
            DistributionType::ToFaction => {
                if let Ok((_, ownership)) = node_query.get(Entity::from_raw(event.source_entity)) {
                    let target = ownership.owner.unwrap_or(event.source_entity);
                    if let Ok(mut inv) = inventory_query.get_mut(Entity::from_raw(target)) {
                        *inv.resources.entry(event.resource_type.clone()).or_insert(0.0) += event.total_amount;
                        affected_player = Some(target);
                    }
                    // TODO: Full faction member distribution when FactionMembership component exists
                }
            }
            DistributionType::ToNearbyParticipants => {
                if let Ok(mut inv) = inventory_query.get_mut(Entity::from_raw(event.source_entity)) {
                    *inv.resources.entry(event.resource_type.clone()).or_insert(0.0) += event.total_amount;
                    affected_player = Some(event.source_entity);
                }
            }
            DistributionType::ProportionalToStanding => {
                if let Ok(mut inv) = inventory_query.get_mut(Entity::from_raw(event.source_entity)) {
                    *inv.resources.entry(event.resource_type.clone()).or_insert(0.0) += event.total_amount;
                    affected_player = Some(event.source_entity);
                }
                // TODO: Real proportional weighting when RbeStanding component + spatial queries exist
            }
        }

        if let Some(player_id) = affected_player {
            rbe_updated_events.send(RbeInventoryUpdatedEvent {
                player_entity_id: player_id,
                resource_type: event.resource_type.clone(),
                amount_added: event.total_amount,
            });
        }
    }
}

// ============================================================================
// Tests (preserved and extended)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;
    use bevy::ecs::event::Events;

    fn setup_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<RbeEconomyState>();
        app.add_event::<DistributeResourcesEvent>();
        app.add_event::<RbeInventoryUpdatedEvent>();
        app.add_systems(Update, process_distributions);
        app
    }

    #[test]
    fn test_distribution_to_owner_emits_update_event() {
        let mut app = setup_test_app();
        let player = app.world.spawn(PlayerRbeInventory { resources: HashMap::new() }).id();
        let node = app.world.spawn((
            ResourceNode {
                resource_type: "crystal".to_string(),
                current_amount: 100.0,
                max_capacity: 100.0,
                regeneration_rate: 1.0,
            },
            NodeOwnership { owner: Some(player.index() as u64) },
        )).id();

        app.world.resource_mut::<Events<DistributeResourcesEvent>>().send(DistributeResourcesEvent {
            source_entity: node.index() as u64,
            resource_type: "crystal".to_string(),
            total_amount: 25.0,
            distribution_type: DistributionType::ToOwner,
        });

        app.update();

        // Check inventory
        let inv = app.world.get::<PlayerRbeInventory>(player).unwrap();
        assert_eq!(inv.resources.get("crystal"), Some(&25.0));

        // Check event was emitted
        let update_events = app.world.resource::<Events<RbeInventoryUpdatedEvent>>();
        assert!(!update_events.is_empty()); // In real test would drain and assert content
    }
}

// End of rbe_plugin.rs v1.6
// process_distributions now emits RbeInventoryUpdatedEvent for interest layer integration.
// Thunder locked in. Yoi ⚡