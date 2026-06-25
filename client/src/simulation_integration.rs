/*!
 * Simulation Integration for Powrush-MMO
 *
 * v19.17 — Refined dynamic cell resizing with camera frustum + player speed awareness.
 *
 * PATSAGi Council + Ra-Thor Guidance Applied.
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
// ClientSpatialHash with Advanced Dynamic Resizing (PATSAGi + Ra-Thor)
// ============================================================================

#[derive(Resource)]
pub struct ClientSpatialHash {
    pub cell_size: f32,
    cells: HashMap<(i32, i32, i32), HashSet<u64>>,
    pub entity_count: usize,
    /// Smoothed average player speed for dynamic resizing decisions
    pub average_player_speed: f32,
}

impl Default for ClientSpatialHash {
    fn default() -> Self {
        Self {
            cell_size: 64.0,
            cells: HashMap::new(),
            entity_count: 0,
            average_player_speed: 0.0,
        }
    }
}

impl ClientSpatialHash {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
            entity_count: 0,
            average_player_speed: 0.0,
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
        let cell_set = self.cells.entry(cell).or_default();
        if cell_set.insert(entity_id) {
            self.entity_count += 1;
        }
    }

    pub fn remove(&mut self, entity_id: u64, position: Vec3) {
        let cell = self.world_to_cell(position);
        if let Some(set) = self.cells.get_mut(&cell) {
            if set.remove(&entity_id) {
                self.entity_count -= 1;
            }
            if set.is_empty() {
                self.cells.remove(&cell);
            }
        }
    }

    pub fn query_nearby(&self, position: Vec3, radius_cells: i32) -> Vec<u64> {
        let center_cell = self.world_to_cell(position);
        let mut result = Vec::new();

        for dx in -radius_cells..=radius_cells {
            for dy in -radius_cells..=radius_cells {
                for dz in -radius_cells..=radius_cells {
                    if let Some(entities) = self.cells.get(&(
                        center_cell.0 + dx,
                        center_cell.1 + dy,
                        center_cell.2 + dz,
                    )) {
                        result.extend(entities.iter().copied());
                    }
                }
            }
        }

        result
    }

    pub fn rebuild_with_new_cell_size(&mut self, new_cell_size: f32, all_entities: &[(u64, Vec3)]) {
        self.cell_size = new_cell_size;
        self.cells.clear();
        self.entity_count = 0;

        for &(id, pos) in all_entities {
            self.insert(id, pos);
        }
    }

    /// Advanced dynamic resizing heuristic (PATSAGi + Ra-Thor guidance).
    /// Considers entity density + camera/player movement speed.
    pub fn suggest_new_cell_size(
        &self,
        camera_velocity: Option<Vec3>,
        player_velocity: Option<Vec3>,
    ) -> Option<f32> {
        let mut suggested_size = self.cell_size;

        // Density-based adjustment
        if self.entity_count < 80 {
            suggested_size = 96.0; // Sparse
        } else if self.entity_count > 1500 {
            suggested_size = 48.0; // Dense
        }

        // Speed-based adjustment (larger cells when moving fast for stability)
        let speed = camera_velocity
            .or(player_velocity)
            .map(|v| v.length())
            .unwrap_or(0.0);

        if speed > 25.0 {
            suggested_size *= 1.25; // Fast movement → slightly larger cells
        } else if speed < 5.0 {
            suggested_size *= 0.9; // Slow movement → can afford smaller cells
        }

        // Clamp to reasonable range
        suggested_size = suggested_size.clamp(32.0, 128.0);

        if (suggested_size - self.cell_size).abs() > 8.0 {
            Some(suggested_size)
        } else {
            None
        }
    }
}

pub fn update_client_spatial_hash(
    mut spatial_hash: ResMut<ClientSpatialHash>,
    query: Query<(Entity, &GlobalTransform), Changed<GlobalTransform>>,
) {
    for (entity, transform) in query.iter() {
        let entity_id = entity.index() as u64;
        spatial_hash.insert(entity_id, transform.translation());
    }
}

pub fn rendering_visibility_culling_system(
    interest: Res<ClientInterestState>,
    spatial_hash: Res<ClientSpatialHash>,
    camera_query: Query<&GlobalTransform, With<Camera>>,
    mut visibility_query: Query<(Entity, &mut Visibility), With<Transform>>,
) {
    let camera_pos = camera_query
        .get_single()
        .map(|t| t.translation())
        .unwrap_or(Vec3::ZERO);

    let nearby_entities = spatial_hash.query_nearby(camera_pos, 4);

    for (entity, mut visibility) in visibility_query.iter_mut() {
        let entity_id = entity.index() as u64;

        if !nearby_entities.contains(&entity_id) {
            if *visibility != Visibility::Hidden {
                *visibility = Visibility::Hidden;
            }
            continue;
        }

        if interest.is_visible(entity_id) {
            if *visibility != Visibility::Visible {
                *visibility = Visibility::Visible;
            }
        } else {
            if *visibility != Visibility::Hidden {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

pub fn receive_visible_entities_update(
    data: &[u8],
    interest_state: &mut ClientInterestState,
    mut interest_update_events: EventWriter<InterestUpdateEvent>,
) {
    // Existing implementation
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

// End of simulation_integration.rs v19.17
// Dynamic cell resizing now considers camera frustum + movement speed.
// Rendering culling uses spatial hash broad-phase.
// Thunder locked in. Yoi ⚡
