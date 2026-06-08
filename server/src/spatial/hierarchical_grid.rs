// server/src/spatial/hierarchical_grid.rs
// Powrush-MMO v17.0 — Hierarchical Spatial Grid (Production Implementation)
// Two-level hierarchical grid for better handling of varying entity density

use std::collections::HashMap;
use shared::protocol::Vec3Ser;

pub type EntityId = u64;

/// Hierarchical Grid with two resolution levels
pub struct HierarchicalGrid {
    // Level 0: Coarse grid (large cells) - good for sparse areas
    coarse_grid: HashMap<(i32, i32), Vec<EntityId>>,
    coarse_cell_size: f32,

    // Level 1: Fine grid (smaller cells) - good for dense clusters
    fine_grid: HashMap<(i32, i32), Vec<EntityId>>,
    fine_cell_size: f32,

    // Position tracking for fast updates
    positions: HashMap<EntityId, Vec3Ser>,
}

impl HierarchicalGrid {
    pub fn new(coarse_cell_size: f32, fine_cell_size: f32) -> Self {
        Self {
            coarse_grid: HashMap::new(),
            coarse_cell_size,
            fine_grid: HashMap::new(),
            fine_cell_size,
            positions: HashMap::new(),
        }
    }

    fn pos_to_cell(&self, pos: &Vec3Ser, cell_size: f32) -> (i32, i32) {
        (
            (pos.x / cell_size).floor() as i32,
            (pos.z / cell_size).floor() as i32,
        )
    }

    /// Insert or update an entity in both grid levels
    pub fn insert_or_update(&mut self, id: EntityId, pos: Vec3Ser) {
        // Remove from old cells if exists
        if let Some(old_pos) = self.positions.get(&id) {
            self.remove_from_grids(id, old_pos);
        }

        // Insert into coarse grid
        let coarse_cell = self.pos_to_cell(&pos, self.coarse_cell_size);
        self.coarse_grid.entry(coarse_cell).or_default().push(id);

        // Insert into fine grid
        let fine_cell = self.pos_to_cell(&pos, self.fine_cell_size);
        self.fine_grid.entry(fine_cell).or_default().push(id);

        self.positions.insert(id, pos);
    }

    fn remove_from_grids(&mut self, id: EntityId, pos: &Vec3Ser) {
        let coarse_cell = self.pos_to_cell(pos, self.coarse_cell_size);
        if let Some(cell) = self.coarse_grid.get_mut(&coarse_cell) {
            cell.retain(|&x| x != id);
        }

        let fine_cell = self.pos_to_cell(pos, self.fine_cell_size);
        if let Some(cell) = self.fine_grid.get_mut(&fine_cell) {
            cell.retain(|&x| x != id);
        }
    }

    /// Remove an entity completely
    pub fn remove(&mut self, id: EntityId) {
        if let Some(pos) = self.positions.remove(&id) {
            self.remove_from_grids(id, &pos);
        }
    }

    /// Query entities within a radius using hierarchical approach
    pub fn query_radius(&self, center: &Vec3Ser, radius: f32) -> Vec<EntityId> {
        let mut result = Vec::new();
        let radius_sq = radius * radius;

        // Query coarse grid first (larger area)
        let coarse_cell = self.pos_to_cell(center, self.coarse_cell_size);
        let search_range = ((radius / self.coarse_cell_size).ceil() as i32) + 1;

        for dx in -search_range..=search_range {
            for dz in -search_range..=search_range {
                let cell = (coarse_cell.0 + dx, coarse_cell.1 + dz);
                if let Some(entities) = self.coarse_grid.get(&cell) {
                    for &id in entities {
                        if let Some(pos) = self.positions.get(&id) {
                            let dx = pos.x - center.x;
                            let dy = pos.y - center.y;
                            let dz = pos.z - center.z;
                            if (dx*dx + dy*dy + dz*dz) <= radius_sq {
                                result.push(id);
                            }
                        }
                    }
                }
            }
        }

        // Also check fine grid for precision in dense areas
        let fine_cell = self.pos_to_cell(center, self.fine_cell_size);
        let fine_search = ((radius / self.fine_cell_size).ceil() as i32) + 1;

        for dx in -fine_search..=fine_search {
            for dz in -fine_search..=fine_search {
                let cell = (fine_cell.0 + dx, fine_cell.1 + dz);
                if let Some(entities) = self.fine_grid.get(&cell) {
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

        result
    }

    pub fn len(&self) -> usize {
        self.positions.len()
    }
}

// Thunder locked in. Hierarchical Grid implemented for production use. ⚡❤️🔥
