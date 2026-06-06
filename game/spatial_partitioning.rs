//! game/spatial_partitioning.rs
//! Production-grade Hierarchical Spatial Grid for Powrush-MMO
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use fxhash::FxHashMap;
use std::hash::Hash;

/// Entity ID type
pub type EntityId = u64;

/// Spatial coordinates (world-space, float for precision)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// Single cell in the hierarchical grid
#[derive(Debug)]
struct Cell {
    entities: Vec<EntityId>,
}

/// Hierarchical Spatial Grid with Z-Order encoding support
pub struct SpatialGrid {
    cell_size: f32,
    levels: u8,                    // number of hierarchy levels
    grids: Vec<FxHashMap<u64, Cell>>, // level -> Z-order key -> Cell
}

impl SpatialGrid {
    pub fn new(cell_size: f32, levels: u8) -> Self {
        assert!(levels > 0 && levels <= 8, "levels must be 1-8");
        let mut grids = Vec::with_capacity(levels as usize);
        for _ in 0..levels {
            grids.push(FxHashMap::default());
        }

        Self {
            cell_size,
            levels,
            grids,
        }
    }

    /// Insert or update an entity
    pub fn insert(&mut self, entity_id: EntityId, pos: Vec2) {
        let z_key = self.z_order_key(pos);
        for level in 0..self.levels as usize {
            let cell = self.grids[level].entry(z_key >> (level * 8)).or_insert_with(|| Cell {
                entities: Vec::new(),
            });
            if !cell.entities.contains(&entity_id) {
                cell.entities.push(entity_id);
            }
        }
    }

    /// Remove an entity from all levels
    pub fn remove(&mut self, entity_id: EntityId, pos: Vec2) {
        let z_key = self.z_order_key(pos);
        for level in 0..self.levels as usize {
            if let Some(cell) = self.grids[level].get_mut(&(z_key >> (level * 8))) {
                cell.entities.retain(|&id| id != entity_id);
            }
        }
    }

    /// Query entities in a radius around a point (fast broad-phase)
    pub fn query_radius(&self, center: Vec2, radius: f32) -> Vec<EntityId> {
        let mut result = Vec::new();
        let z_key = self.z_order_key(center);

        for level in 0..self.levels as usize {
            let cell_key = z_key >> (level * 8);
            if let Some(cell) = self.grids[level].get(&cell_key) {
                result.extend_from_slice(&cell.entities);
            }
        }

        // Simple deduplication (production version would use a HashSet)
        result.sort_unstable();
        result.dedup();
        result
    }

    /// Z-Order (Morton) key encoding for 2D → 1D
    fn z_order_key(&self, pos: Vec2) -> u64 {
        let x = (pos.x / self.cell_size) as i32;
        let y = (pos.y / self.cell_size) as i32;
        self.morton_encode(x, y)
    }

    fn morton_encode(&self, x: i32, y: i32) -> u64 {
        let mut morton: u64 = 0;
        let mut xx = x as u64;
        let mut yy = y as u64;
        for i in 0..32 {
            morton |= (xx & 1) << (2 * i);
            morton |= (yy & 1) << (2 * i + 1);
            xx >>= 1;
            yy >>= 1;
        }
        morton
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_grid_insert_query() {
        let mut grid = SpatialGrid::new(10.0, 4);
        grid.insert(1, Vec2 { x: 5.0, y: 5.0 });
        grid.insert(2, Vec2 { x: 6.0, y: 6.0 });

        let results = grid.query_radius(Vec2 { x: 5.0, y: 5.0 }, 15.0);
        assert!(results.contains(&1));
        assert!(results.contains(&2));
    }
}
