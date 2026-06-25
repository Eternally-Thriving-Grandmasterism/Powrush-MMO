/*!
 * RBE Plugin (Resource-Based Economy)
 *
 * v2.3 | Refactored for modularity - components moved to components.rs
 * FactionMembership + FactionStanding now live in the proper module.
 *
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashMap;

use crate::rbe::components::{
    FactionMembership, FactionStanding, NodeOwnership, PlayerRbeInventory, ResourceNode,
};

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
pub struct HarvestEvent { /* ... */ }

#[derive(Event, Clone, Debug)]
pub struct ResourceNodeDepletedEvent { /* ... */ }

#[derive(Event, Clone, Debug)]
pub struct ResourceTransferEvent { /* ... */ }

#[derive(Event, Clone, Debug)]
pub struct ClaimNodeEvent { /* ... */ }

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

// Systems (harvest, regenerate, transfer, claiming unchanged)

fn process_harvest_events(/* ... */) { /* unchanged */ }
fn regenerate_resource_nodes(/* ... */) { /* unchanged */ }
fn process_resource_transfers(/* ... */) { /* unchanged */ }
fn process_node_claiming(/* ... */) { /* unchanged */ }

/// Distribution logic with Faction Standing System support.
/// ProportionalToStanding scales reward by the source's standing.
fn process_distributions(
    mut dist_events: EventReader<DistributeResourcesEvent>,
    mut inventory_query: Query<&mut PlayerRbeInventory>,
    node_query: Query<(&ResourceNode, &NodeOwnership)>,
    mut faction_query: Query<(Entity, &FactionMembership, &mut PlayerRbeInventory)>,
    mut standing_query: Query<(Entity, &FactionStanding)>,
    mut rbe_updated_events: EventWriter<RbeInventoryUpdatedEvent>,
) {
    for event in dist_events.read() {
        let mut affected_players: EntityHashSet = EntityHashSet::default();

        match event.distribution_type {
            DistributionType::ToOwner => {
                if let Ok((_, ownership)) = node_query.get(Entity::from_raw(event.source_entity)) {
                    if let Some(owner) = ownership.owner {
                        if let Ok(mut inv) = inventory_query.get_mut(Entity::from_raw(owner)) {
                            *inv.resources.entry(event.resource_type.clone()).or_insert(0.0) += event.total_amount;
                            affected_players.insert(Entity::from_raw(owner));
                        }
                    }
                }
            }
            DistributionType::ToFaction => {
                if let Ok((_, ownership)) = node_query.get(Entity::from_raw(event.source_entity)) {
                    if let Some(owner) = ownership.owner {
                        if let Ok(mut inv) = inventory_query.get_mut(Entity::from_raw(owner)) {
                            *inv.resources.entry(event.resource_type.clone()).or_insert(0.0) += event.total_amount;
                            affected_players.insert(Entity::from_raw(owner));
                        }

                        if let Some((_, owner_membership, _)) = faction_query
                            .iter()
                            .find(|(e, _, _)| e.index() == owner)
                        {
                            let owner_faction_id = owner_membership.faction_id;

                            for (entity, membership, mut inv) in faction_query.iter_mut() {
                                if membership.faction_id == owner_faction_id {
                                    *inv.resources.entry(event.resource_type.clone()).or_insert(0.0) += event.total_amount;
                                    affected_players.insert(entity);
                                }
                            }
                        }
                    }
                }
            }
            DistributionType::ToNearbyParticipants => {
                if let Ok(mut inv) = inventory_query.get_mut(Entity::from_raw(event.source_entity)) {
                    *inv.resources.entry(event.resource_type.clone()).or_insert(0.0) += event.total_amount;
                    affected_players.insert(Entity::from_raw(event.source_entity));
                }
            }
            DistributionType::ProportionalToStanding => {
                if let Ok(mut inv) = inventory_query.get_mut(Entity::from_raw(event.source_entity)) {
                    let standing_multiplier = standing_query
                        .iter()
                        .find(|(e, standing)| e.index() == event.source_entity)
                        .map(|(_, s)| s.standing.clamp(0.0, 3.0))
                        .unwrap_or(1.0);

                    let scaled_amount = event.total_amount * standing_multiplier;

                    *inv.resources.entry(event.resource_type.clone()).or_insert(0.0) += scaled_amount;
                    affected_players.insert(Entity::from_raw(event.source_entity));
                }
            }
        }

        for player_entity in affected_players.iter() {
            rbe_updated_events.send(RbeInventoryUpdatedEvent {
                player_entity_id: player_entity.index() as u64,
                resource_type: event.resource_type.clone(),
                amount_added: event.total_amount,
            });
        }
    }
}

// End of rbe_plugin.rs v2.3
// Components refactored into components.rs for modularity.
// Thunder locked in. Yoi ⚡