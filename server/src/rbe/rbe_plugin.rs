/*!
 * RBE Plugin (Resource-Based Economy)
 *
 * v2.5 | ToFaction now distributes proportionally to FactionStanding
 * Higher standing members receive larger shares. Core virtuous cycle complete.
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

#[derive(Event, Clone, Debug)]
pub struct FactionStandingChangedEvent {
    pub player_entity_id: u64,
    pub faction_id: u64,
    pub delta: f32,
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
            .add_event::<FactionStandingChangedEvent>()
            .add_event::<RbeInventoryUpdatedEvent>()

            .add_systems(Update, (
                process_harvest_events,
                regenerate_resource_nodes,
                process_resource_transfers,
                process_node_claiming,
                process_distributions,
                apply_faction_standing_changes,
            ))
    }
}

// Systems (harvest, regenerate, transfer, claiming unchanged)

fn process_harvest_events(/* ... */) { /* unchanged */ }
fn regenerate_resource_nodes(/* ... */) { /* unchanged */ }
fn process_resource_transfers(/* ... */) { /* unchanged */ }
fn process_node_claiming(/* ... */) { /* unchanged */ }

/// Applies standing changes from FactionStandingChangedEvent.
fn apply_faction_standing_changes(
    mut events: EventReader<FactionStandingChangedEvent>,
    mut commands: Commands,
    mut standing_query: Query<&mut FactionStanding>,
) {
    for event in events.read() {
        if let Some(mut standing) = standing_query
            .iter_mut()
            .find(|(e, _)| e.index() == event.player_entity_id)
            .map(|(_, s)| s)
        {
            standing.standing = (standing.standing + event.delta).clamp(0.0, 5.0);
        } else {
            commands.entity(Entity::from_raw(event.player_entity_id)).insert(FactionStanding {
                faction_id: event.faction_id,
                standing: event.delta.clamp(0.0, 5.0),
            });
        }
    }
}

/// Distribution logic with standing-weighted ToFaction.
/// ToFaction now distributes proportionally based on each member's FactionStanding.
fn process_distributions(
    mut dist_events: EventReader<DistributeResourcesEvent>,
    mut inventory_query: Query<&mut PlayerRbeInventory>,
    node_query: Query<(&ResourceNode, &NodeOwnership)>,
    mut faction_query: Query<(Entity, &FactionMembership, &mut PlayerRbeInventory)>,
    mut standing_query: Query<(Entity, &FactionStanding)>,
    mut rbe_updated_events: EventWriter<RbeInventoryUpdatedEvent>,
    mut standing_changed_events: EventWriter<FactionStandingChangedEvent>,
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
                        // Owner still receives full amount (preserved behavior)
                        if let Ok(mut inv) = inventory_query.get_mut(Entity::from_raw(owner)) {
                            *inv.resources.entry(event.resource_type.clone()).or_insert(0.0) += event.total_amount;
                            affected_players.insert(Entity::from_raw(owner));
                        }

                        if let Some((_, owner_membership, _)) = faction_query
                            .iter()
                            .find(|(e, _, _)| e.index() == owner)
                        {
                            let owner_faction_id = owner_membership.faction_id;

                            // === Standing-weighted distribution among faction members ===
                            // Collect members + their standing (default 1.0 if no standing data)
                            let mut faction_members: Vec<(Entity, f32)> = Vec::new();
                            let mut total_standing: f32 = 0.0;

                            for (entity, _membership, _inv) in faction_query.iter() {
                                if _membership.faction_id == owner_faction_id {
                                    let member_standing = standing_query
                                        .iter()
                                        .find(|(e, _)| e.index() == entity.index())
                                        .map(|(_, s)| s.standing)
                                        .unwrap_or(1.0);

                                    faction_members.push((entity, member_standing));
                                    total_standing += member_standing;
                                }
                            }

                            if total_standing > 0.0 {
                                for (entity, member_standing) in faction_members {
                                    let share = event.total_amount * (member_standing / total_standing);

                                    if let Ok(mut inv) = inventory_query.get_mut(entity) {
                                        *inv.resources.entry(event.resource_type.clone()).or_insert(0.0) += share;
                                    }

                                    affected_players.insert(entity);

                                    // Participation still grants standing
                                    standing_changed_events.send(FactionStandingChangedEvent {
                                        player_entity_id: entity.index() as u64,
                                        faction_id: owner_faction_id,
                                        delta: 0.05,
                                    });
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

// End of rbe_plugin.rs v2.5
// ToFaction now distributes proportionally based on FactionStanding.
// Higher standing = larger share. Virtuous cycle complete.
// Thunder locked in. Yoi ⚡