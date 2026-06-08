// server/src/spatial/hierarchical_grid.rs
// Powrush-MMO v17.0 — Hierarchical Spatial Grid (Refined Production Version)

use std::collections::HashMap;
use shared::protocol::Vec3Ser;

pub type EntityId = u64;

/// Configuration for a single grid level
#[derive(Clone, Debug)]
pub struct GridLevel {
    pub cell_size: f32,
    pub name: String,
}

/// A flexible multi-level Hierarchical Grid
pub struct HierarchicalGrid {
    levels: Vec<GridLevel>,
    grids: Vec<HashMap<(i32, i32), Vec<EntityId>>>,
    positions: HashMap<EntityId, Vec3Ser>,
}

impl HierarchicalGrid {
    /// Create a new hierarchical grid with custom levels.
    /// Example levels: coarse (128.0), medium (64.0), fine (32.0)
    pub fn new(levels: Vec<GridLevel>) -> Self {
        let grids = vec![HashMap::new(); levels.len()];
        Self {
            levels,
            grids,
            positions: HashMap::new(),
        }
    }

    /// Convenience constructor with sensible defaults for MMO worlds
    pub fn with_default_levels() -> Self {
        Self::new(vec![
            GridLevel { cell_size: 128.0, name: "Coarse".to_string() },
            GridLevel { cell_size: 64.0,  name: "Medium".to_string() },
            GridLevel { cell_size: 32.0,  name: "Fine".to_string() },
        ])
    }

    fn pos_to_cell(&self, pos: &Vec3Ser, cell_size: f32) -> (i32, i32) {
        (
            (pos.x / cell_size).floor() as i32,
            (pos.z / cell_size).floor() as i32,
        )
    }

    pub fn insert_or_update(&mut self, id: EntityId, pos: Vec3Ser) {
        if let Some(old_pos) = self.positions.get(&id) {
            self.remove_from_all_grids(id, old_pos);
        }

        for (i, level) in self.levels.iter().enumerate() {
            let cell = self.pos_to_cell(&pos, level.cell_size);
            self.grids[i].entry(cell).or_default().push(id);
        }

        self.positions.insert(id, pos);
    }

    fn remove_from_all_grids(&mut self, id: EntityId, pos: &Vec3Ser) {
        for (i, level) in self.levels.iter().enumerate() {
            let cell = self.pos_to_cell(pos, level.cell_size);
            if let Some(cell_vec) = self.grids[i].get_mut(&cell) {
                cell_vec.retain(|&x| x != id);
            }
        }
    }

    pub fn remove(&mut self, id: EntityId) {
        if let Some(pos) = self.positions.remove(&id) {
            self.remove_from_all_grids(id, &pos);
        }
    }

    /// Query entities within radius using all grid levels
    pub fn query_radius(&self, center: &Vec3Ser, radius: f32) -> Vec<EntityId> {
        let mut result = Vec::new();
        let radius_sq = radius * radius;

        for (i, level) in self.levels.iter().enumerate() {
            let center_cell = self.pos_to_cell(center, level.cell_size);
            let search_range = ((radius / level.cell_size).ceil() as i32) + 1;

            for dx in -search_range..=search_range {
                for dz in -search_range..=search_range {
                    let cell = (center_cell.0 + dx, center_cell.1 + dz);
                    if let Some(entities) = self.grids[i].get(&cell) {
                        for &id in entities {
                            if let Some(pos) = self.positions.get(&id) {
                                let dx = pos.x - center.x;
                                let dy = pos.y - center.y;
                                let dz = pos.z - center.z;
                                if (dx*dx + dy*dy + dz*dz) <= radius_sq {
                                    if !result.contains(&id) {
                                        result.push(id);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        result
    }

    pub fn len(&self) -> usize {
        self.positions.len()
    }
}

// Thunder locked in. Refined HierarchicalGrid ready for integration. ⚡❤️🔥
