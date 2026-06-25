/*!
 * Simulation Integration for Powrush-MMO
 *
 * v19.8 — Extended visibility culling pattern (Step C).
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashSet;

use simulation::interest::VisibleEntitiesUpdate;

pub use simulation::interest::VisibleEntitiesUpdate as InterestNetworkMessage;

#[derive(Event, Clone, Debug)]
pub struct InterestUpdateEvent {
    pub visible_entities: Vec<u64>,
    pub server_tick: u64,
}

#[derive(Resource, Default)]
pub struct ClientInterestState {
    pub visible_entities: HashSet<u64>,
    pub last_update_tick: u64,
}

impl ClientInterestState {
    pub fn is_visible(&self, entity_id: u64) -> bool {
        self.visible_entities.contains(&entity_id)
    }

    pub fn has_no_data(&self) -> bool {
        self.visible_entities.is_empty() && self.last_update_tick == 0
    }

    pub fn update_visible_entities(&mut self, entities: Vec<u64>, current_tick: u64) {
        self.visible_entities.clear();
        self.visible_entities.extend(entities);
        self.last_update_tick = current_tick;
    }

    pub fn visible_count(&self) -> usize {
        self.visible_entities.len()
    }
}

#[derive(Component, Clone, Debug)]
pub struct HighSalienceAudio {
    pub priority: u8,
    pub gain_boost: f32,
}

impl Default for HighSalienceAudio {
    fn default() -> Self {
        Self { priority: 1, gain_boost: 0.2 }
    }
}

pub fn receive_interest_update(
    mut visible_updates: EventReader<VisibleEntitiesUpdate>,
    mut interest_update_events: EventWriter<InterestUpdateEvent>,
) {
    for update in visible_updates.read() {
        interest_update_events.send(InterestUpdateEvent {
            visible_entities: update.visible_entity_ids.clone(),
            server_tick: update.server_tick,
        });
    }
}

/// Example of how rendering / visibility culling systems should use ClientInterestState.
/// Other systems (rendering, LOD, UI, etc.) can follow this pattern.
pub fn example_rendering_culling_system(
    interest: Res<ClientInterestState>,
    // query: Query<(Entity, &Transform), With<SomeRenderComponent>>,
) {
    // for (entity, _) in query.iter() {
    //     if interest.is_visible(entity.index()) {  // or use a proper entity_id mapping
    //         // Render this entity
    //     } else {
    //         // Skip rendering (cull)
    //     }
    // }
}

// End of simulation_integration.rs v19.8
// Steps A+B+C of replication bridge + visibility culling extension complete.
// Thunder locked in. Yoi ⚡
