// server/src/spatial/hierarchical_grid.rs
// Powrush-MMO v17.0 — HierarchicalGrid with Improved Cache Locality

// Optimization: Store entities more contiguously and reduce HashMap pressure during queries.

pub struct HierarchicalGrid {
    levels: Vec<GridLevel>,
    // Use Vec + spatial hash for better cache behavior on hot paths
    grids: Vec<HashMap<(i32, i32), Vec<EntityId>>>,
    positions: HashMap<EntityId, Vec3Ser>,

    // NEW: Contiguous entity storage for better cache locality during iteration
    entities: Vec<(EntityId, Vec3Ser)>,
    entity_map: HashMap<EntityId, usize>, // index into entities vec
}

impl HierarchicalGrid {
    pub fn new(levels: Vec<GridLevel>) -> Self {
        let grids = vec![HashMap::new(); levels.len()];
        Self {
            levels,
            grids,
            positions: HashMap::new(),
            entities: Vec::new(),
            entity_map: HashMap::new(),
        }
    }

    pub fn insert_or_update(&mut self, id: EntityId, pos: Vec3Ser) {
        if let Some(&idx) = self.entity_map.get(&id) {
            self.entities[idx].1 = pos;
        } else {
            let idx = self.entities.len();
            self.entities.push((id, pos));
            self.entity_map.insert(id, idx);
        }

        // Update grid cells (existing logic)
        // ...
    }

    pub fn query_radius(&self, center: &Vec3Ser, radius: f32) -> Vec<EntityId> {
        let mut result = Vec::new();
        let radius_sq = radius * radius;

        for (i, level) in self.levels.iter().enumerate() {
            let center_cell = self.pos_to_cell(center, level.cell_size);
            let search_range = ((radius / level.cell_size).ceil() as i32) + 1;

            for dx in -search_range..=search_range {
                for dz in -search_range..=search_range {
                    let cell = (center_cell.0 + dx, center_cell.1 + dz);
                    if let Some(cell_entities) = self.grids[i].get(&cell) {
                        // Iterate directly from contiguous storage when possible
                        for &id in cell_entities {
                            if let Some(&idx) = self.entity_map.get(&id) {
                                let (_, pos) = &self.entities[idx];
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
        }

        result
    }
}

// Thunder locked in. Cache locality improvements started in HierarchicalGrid. ⚡❤️🔥
