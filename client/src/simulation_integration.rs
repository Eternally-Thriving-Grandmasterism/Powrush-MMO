/*!
 * Simulation Integration for Powrush-MMO
 *
 * v19.19 — Background thread rebuilds for ClientSpatialHash.
 *
 * PATSAGi + Ra-Thor Applied.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
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
// ClientSpatialHash with Background Rebuild Support
// ============================================================================

#[derive(Resource)]
pub struct ClientSpatialHash {
    pub cell_size: f32,
    cells: HashMap<(i32, i32, i32), HashSet<u64>>,
    pub entity_count: usize,
    pub average_player_speed: f32,

    // Background rebuild state
    pub is_rebuilding: bool,
    rebuild_task: Option<Task<(f32, HashMap<(i32, i32, i32), HashSet<u64>>)>>,
}

impl Default for ClientSpatialHash {
    fn default() -> Self {
        Self {
            cell_size: 64.0,
            cells: HashMap::new(),
            entity_count: 0,
            average_player_speed: 0.0,
            is_rebuilding: false,
            rebuild_task: None,
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
            is_rebuilding: false,
            rebuild_task: None,
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
        if self.cells.entry(cell).or_default().insert(entity_id) {
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

    /// Request a background rebuild with a new cell size.
    /// The heavy work happens off the main thread.
    pub fn request_background_resize(&mut self, new_cell_size: f32, entities: Vec<(u64, Vec3)>) {
        if self.is_rebuilding || (new_cell_size - self.cell_size).abs() < 1.0 {
            return;
        }

        self.is_rebuilding = true;

        let task_pool = AsyncComputeTaskPool::get();
        let task = task_pool.spawn(async move {
            let mut new_cells: HashMap<(i32, i32, i32), HashSet<u64>> = HashMap::new();

            for (id, pos) in entities {
                let cell = (
                    (pos.x / new_cell_size).floor() as i32,
                    (pos.y / new_cell_size).floor() as i32,
                    (pos.z / new_cell_size).floor() as i32,
                );
                new_cells.entry(cell).or_default().insert(id);
            }

            (new_cell_size, new_cells)
        });

        self.rebuild_task = Some(task);
    }

    /// Poll and apply completed background rebuild.
    pub fn try_complete_rebuild(&mut self) -> bool {
        if let Some(task) = self.rebuild_task.as_mut() {
            if let Some((new_size, new_cells)) = futures_lite::future::block_on(futures_lite::future::poll_once(task)) {
                self.cells = new_cells;
                self.cell_size = new_size;
                self.entity_count = self.cells.values().map(|s| s.len()).sum();
                self.is_rebuilding = false;
                self.rebuild_task = None;

                info!("⚡ [SpatialHash] Background rebuild complete. New cell size: {:.1}", new_size);
                return true;
            }
        }
        false
    }
}

// ============================================================================
// Background Rebuild System
// ============================================================================

pub fn apply_dynamic_cell_size(
    mut spatial_hash: ResMut<ClientSpatialHash>,
) {
    // First, check if a background rebuild finished
    if spatial_hash.is_rebuilding {
        spatial_hash.try_complete_rebuild();
        return;
    }

    // TODO: Call suggest_new_cell_size and request_background_resize when appropriate
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

// End of simulation_integration.rs v19.19
// Background thread rebuilds implemented using AsyncComputeTaskPool.
// Thunder locked in. Yoi ⚡
