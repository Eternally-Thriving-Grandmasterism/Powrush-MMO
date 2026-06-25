/*!
 * RBE Plugin (Resource-Based Economy)
 *
 * v2.6 | Server replication wiring for FactionStanding
 * Standing changes now mark entities dirty for replication (FACTION_STANDING bit).
 *
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashMap;

use crate::rbe::components::{
    FactionMembership, FactionStanding, NodeOwnership, PlayerRbeInventory, ResourceNode,
};
use crate::replication::{DirtyReplicationState, ReplicatedFields, UpdatePayload, TargetedUpdate};

// ... (rest of file header and events unchanged)

#[derive(Event, Clone, Debug)]
pub struct FactionStandingChangedEvent {
    pub player_entity_id: u64,
    pub faction_id: u64,
    pub delta: f32,
}

// ... (DistributionType and other events unchanged)

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

/// Applies standing changes and marks entity for replication.
fn apply_faction_standing_changes(
    mut events: EventReader<FactionStandingChangedEvent>,
    mut commands: Commands,
    mut standing_query: Query<&mut FactionStanding>,
    mut dirty_query: Query<&mut DirtyReplicationState>,
) {
    for event in events.read() {
        let entity = Entity::from_raw(event.player_entity_id);

        // Apply the standing change
        if let Some(mut standing) = standing_query
            .iter_mut()
            .find(|(e, _)| e.index() == event.player_entity_id)
            .map(|(_, s)| s)
        {
            standing.standing = (standing.standing + event.delta).clamp(0.0, 5.0);
        } else {
            commands.entity(entity).insert(FactionStanding {
                faction_id: event.faction_id,
                standing: event.delta.clamp(0.0, 5.0),
            });
        }

        // Mark for replication so client UI receives the update
        if let Some(mut dirty) = dirty_query.get_mut(entity).ok() {
            dirty.dirty_mask |= ReplicatedFields::FACTION_STANDING;
        } else {
            commands.entity(entity).insert(DirtyReplicationState {
                dirty_mask: ReplicatedFields::FACTION_STANDING,
                ..default()
            });
        }

        // Optional: emit TargetedUpdate directly for immediate processing
        // (the main replication loop will collect dirty entities)
    }
}

// process_distributions and other systems remain unchanged from v2.5
// (ToFaction weighted distribution + standing gain logic stays the same)

// End of rbe_plugin.rs v2.6
// FactionStanding changes now trigger replication dirty bit (FACTION_STANDING).
// Client can now receive real-time standing updates.
// Thunder locked in. Yoi ⚡