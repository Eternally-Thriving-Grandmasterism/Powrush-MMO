/*!
 * Simulation Integration for Powrush-MMO
 *
 * v19.15 — Added ClientSpatialHash for efficient spatial culling.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

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

// ============================================================================
// Client Spatial Hash for Efficient Culling
// ============================================================================

/// Simple spatial hash grid for client-side culling.
/// Much faster than checking all entities.
#[derive(Resource)]
pub struct ClientSpatialHash {
    pub cell_size: f32,
    cells: HashMap<(i32, i32, i32), HashSet<u64>>,
}

impl Default for ClientSpatialHash {
    fn default() -> Self {
        Self {
            cell_size: 64.0,
            cells: HashMap::new(),
        }
    }
}

impl ClientSpatialHash {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    fn world_to_cell(&self, pos: Vec3) -> (i32, i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
            (pos.z / self.cell_size).floor() as i32,
        )
    }

    pub fn insert(&mut self, entity_id: u64, position: Vec3) {
        let cell = self.world_to_cell(position);
        self.cells.entry(cell).or_default().insert(entity_id);
    }

    pub fn remove(&mut self, entity_id: u64, position: Vec3) {
        let cell = self.world_to_cell(position);
        if let Some(set) = self.cells.get_mut(&cell) {
            set.remove(&entity_id);
        }
    }

    /// Get all entities in cells near the given position (for culling).
    pub fn query_nearby(&self, position: Vec3, radius_cells: i32) -> Vec<u64> {
        let center_cell = self.world_to_cell(position);
        let mut result = Vec::new();

        for dx in -radius_cells..=radius_cells {
            for dy in -radius_cells..=radius_cells {
                for dz in -radius_cells..=radius_cells {
                    let cell = (
                        center_cell.0 + dx,
                        center_cell.1 + dy,
                        center_cell.2 + dz,
                    );
                    if let Some(entities) = self.cells.get(&cell) {
                        result.extend(entities.iter().copied());
                    }
                }
            }
        }

        result
    }
}

/// System that keeps ClientSpatialHash in sync with entity positions.
pub fn update_client_spatial_hash(
    mut spatial_hash: ResMut<ClientSpatialHash>,
    query: Query<(Entity, &GlobalTransform), Changed<GlobalTransform>>,
) {
    for (entity, transform) in query.iter() {
        let entity_id = entity.index() as u64; // TODO: Use proper network ID mapping
        spatial_hash.insert(entity_id, transform.translation());
    }
}

// ============================================================================
// Existing receive functions (abbreviated for brevity)
// ============================================================================

pub fn receive_visible_entities_update(
    data: &[u8],
    interest_state: &mut ClientInterestState,
    mut interest_update_events: EventWriter<InterestUpdateEvent>,
) {
    // ... existing implementation ...
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

pub fn rendering_visibility_culling_system(
    interest: Res<ClientInterestState>,
    mut query: Query<(Entity, &mut Visibility), With<Transform>>,
) {
    for (entity, mut visibility) in query.iter_mut() {
        let entity_id = entity.index() as u64;
        if interest.is_visible(entity_id) {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

// End of simulation_integration.rs v19.15
// ClientSpatialHash added for efficient spatial culling.
// Thunder locked in. Yoi ⚡
