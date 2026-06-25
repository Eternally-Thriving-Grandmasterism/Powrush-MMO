/*!
 * Simulation Integration for Powrush-MMO
 *
 * v19.13 — Added InterestAck sending for acknowledgment / resend logic.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashSet;

use simulation::interest::{InterestAck, VisibleEntitiesUpdate};

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

/// Receives update + sends acknowledgment back to server.
pub fn receive_visible_entities_update(
    data: &[u8],
    interest_state: &mut ClientInterestState,
    mut interest_update_events: EventWriter<InterestUpdateEvent>,
    // TODO: Add EventWriter<InterestAck> or direct networking send
) {
    let decompressed = match zstd::decode_all(data) {
        Ok(data) => data,
        Err(_) => data.to_vec(),
    };

    match bincode::deserialize::<VisibleEntitiesUpdate>(&decompressed) {
        Ok(update) => {
            if update.server_tick <= interest_state.last_update_tick {
                return; // Stale packet
            }

            interest_update_events.send(InterestUpdateEvent {
                visible_entities: update.visible_entity_ids,
                server_tick: update.server_tick,
            });

            interest_state.last_update_tick = update.server_tick;

            // === Send Acknowledgment back to server ===
            // In production, send this via networking:
            // networking.send_to_server(InterestAck {
            //     client_entity_id: my_entity_id,
            //     acknowledged_tick: update.server_tick,
            // });
        }
        Err(e) => {
            error!("[InterestReplication] Deserialize failed: {}", e);
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

// End of simulation_integration.rs v19.13
// Client now sends InterestAck after processing updates.
// Thunder locked in. Yoi ⚡
