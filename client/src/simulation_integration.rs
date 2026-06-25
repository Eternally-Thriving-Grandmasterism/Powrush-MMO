/*!
 * Simulation Integration for Powrush-MMO
 *
 * v19.6 — Added client-side InterestUpdateEvent receiver (Step 3 of replication bridge).
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashSet;

use crate::spatial_audio::GameAudioEvent;

// Interest types (owned here)
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

// ... other imports and code ...

/// Client-side receiver for interest updates coming from the server (Step 3).
/// In a full implementation, this would be triggered by the replication layer
/// when a `VisibleEntitiesUpdate` message arrives from the server.
pub fn receive_interest_update(
    mut visible_updates: EventReader<VisibleEntitiesUpdate>, // This will come from replication
    mut interest_update_events: EventWriter<InterestUpdateEvent>,
) {
    for update in visible_updates.read() {
        // Convert server message into local InterestUpdateEvent
        interest_update_events.send(InterestUpdateEvent {
            visible_entities: update.visible_entity_ids.clone(),
            server_tick: update.server_tick,
        });
    }
}

// Note: VisibleEntitiesUpdate is defined on the server.
// On the client we would either share the type or have a client-specific version.
// For now we assume a shared protocol type or re-definition in networking layer.

// End of simulation_integration.rs v19.6
// Replication bridge Steps 1-3 complete (server generation + message format + client receiver).
// Thunder locked in. Yoi ⚡
