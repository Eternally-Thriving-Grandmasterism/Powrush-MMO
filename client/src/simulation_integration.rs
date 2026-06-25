/*!
 * Simulation Integration for Powrush-MMO
 *
 * v19.9 — Actual network receive implementation for interest updates.
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

/// Actual network receive function.
/// Called when raw bytes arrive from the server for a VisibleEntitiesUpdate.
pub fn receive_visible_entities_update(
    data: &[u8],
    mut interest_update_events: EventWriter<InterestUpdateEvent>,
) {
    match bincode::deserialize::<VisibleEntitiesUpdate>(data) {
        Ok(update) => {
            interest_update_events.send(InterestUpdateEvent {
                visible_entities: update.visible_entity_ids,
                server_tick: update.server_tick,
            });
        }
        Err(e) => {
            error!("Failed to deserialize VisibleEntitiesUpdate: {}", e);
        }
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

// End of simulation_integration.rs v19.9
// Actual network receive (deserialization) implemented.
// Thunder locked in. Yoi ⚡
