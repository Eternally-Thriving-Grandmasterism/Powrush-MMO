/*!
 * Hierarchical Spatial Grid (Shared)
 *
 * Production-grade Z-Order multi-level grid with raycast support.
 * Used for interest management, replication culling, procedural reverb estimation,
 * occlusion, and spatial audio probes.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor Lattice Native
 */

use fxhash::FxHashMap;

pub type EntityId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

struct Cell {
    entities: Vec<EntityId>,
}

/// Multi-level hierarchical grid using Z-order curve for fast spatial queries.
pub struct HierarchicalGrid {
    cell_size: f32,
    levels: u8,
    grids: Vec<FxHashMap<u64, Cell>>,
}

impl HierarchicalGrid {
    pub fn new(cell_size: f32, levels: u8) -> Self {
        assert!(levels > 0 && levels <= 8, "levels must be between 1 and 8");
        let mut grids = Vec::with_capacity(levels as usize);
        for _ in 0..levels {
            grids.push(FxHashMap::default());
        }
        Self { cell_size, levels, grids }
    }

    pub fn levels(&self) -> u8 { self.levels }
    pub fn cell_size(&self) -> f32 { self.cell_size }

    fn world_to_cell(&self, pos: Vec3) -> (i32, i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
            (pos.z / self.cell_size).floor() as i32,
        )
    }

    fn cell_to_zorder(&self, cell: (i32, i32, i32)) -> u64 {
        let mut code: u64 = 0;
        let mut x = cell.0 as u64;
        let mut y = cell.1 as u64;
        let mut z = cell.2 as u64;
        for i in 0..21 {
            code |= ((x & 1) << (3 * i));
            code |= ((y & 1) << (3 * i + 1));
            code |= ((z & 1) << (3 * i + 2));
            x >>= 1;
            y >>= 1;
            z >>= 1;
        }
        code
    }

    pub fn insert(&mut self, entity_id: EntityId, pos: Vec3) {
        let cell = self.world_to_cell(pos);
        for level in 0..self.levels as usize {
            let key = self.cell_to_zorder(cell) >> (level * 8);
            let cell_entry = self.grids[level].entry(key).or_insert_with(|| Cell { entities: Vec::new() });
            if !cell_entry.entities.contains(&entity_id) {
                cell_entry.entities.push(entity_id);
            }
        }
    }

    pub fn remove(&mut self, entity_id: EntityId, pos: Vec3) {
        let cell = self.world_to_cell(pos);
        for level in 0..self.levels as usize {
            let key = self.cell_to_zorder(cell) >> (level * 8);
            if let Some(cell_entry) = self.grids[level].get_mut(&key) {
                cell_entry.entities.retain(|&id| id != entity_id);
            }
        }
    }

    pub fn query_radius(&self, center: Vec3, radius: f32) -> Vec<EntityId> {
        let mut result = Vec::new();
        let cell_radius = (radius / self.cell_size).ceil() as i32 + 1;
        let center_cell = self.world_to_cell(center);

        for dx in -cell_radius..=cell_radius {
            for dy in -cell_radius..=cell_radius {
                for dz in -cell_radius..=cell_radius {
                    let cell = (center_cell.0 + dx, center_cell.1 + dy, center_cell.2 + dz);
                    let key = self.cell_to_zorder(cell);
                    for level in 0..self.levels as usize {
                        if let Some(cell_entry) = self.grids[level].get(&(key >> (level * 8))) {
                            result.extend_from_slice(&cell_entry.entities);
                        }
                    }
                }
            }
        }

        result.sort_unstable();
        result.dedup();
        result
    }

    /// DDA-style raycast using the hierarchical grid for acceleration.
    /// Returns distance to first occupied cell or None.
    pub fn raycast_distance(&self, origin: Vec3, direction: Vec3, max_distance: f32) -> Option<f32> {
        if max_distance <= 0.0 { return None; }

        let dir_len = (direction.x.powi(2) + direction.y.powi(2) + direction.z.powi(2)).sqrt();
        if dir_len < 1e-6 { return None; }

        let dir = Vec3 {
            x: direction.x / dir_len,
            y: direction.y / dir_len,
            z: direction.z / dir_len,
        };

        let mut current_pos = origin;
        let step_size = self.cell_size * 0.5;
        let mut traveled = 0.0;

        while traveled < max_distance {
            let cell = self.world_to_cell(current_pos);
            let key = self.cell_to_zorder(cell);

            for level in 0..self.levels as usize {
                if let Some(cell_entry) = self.grids[level].get(&(key >> (level * 8))) {
                    if !cell_entry.entities.is_empty() {
                        return Some(traveled.max(0.1));
                    }
                }
            }

            current_pos.x += dir.x * step_size;
            current_pos.y += dir.y * step_size;
            current_pos.z += dir.z * step_size;
            traveled += step_size;
        }

        None
    }
}
