// server/src/spatial/hierarchical_grid.rs
// Powrush-MMO v17.0 — HierarchicalGrid with SoA Layout

// Major cache + future SIMD optimization: Struct of Arrays (SoA) for positions

pub struct HierarchicalGrid {
    levels: Vec<GridLevel>,
    grids: Vec<HashMap<(i32, i32), Vec<EntityId>>>,

    // SoA layout for better cache locality and SIMD potential
    ids: Vec<EntityId>,
    positions: Vec<Vec3Ser>,        // Still grouped as Vec3Ser for now (good balance)
    entity_index: HashMap<EntityId, usize>,
}

impl HierarchicalGrid {
    pub fn new(levels: Vec<GridLevel>) -> Self {
        Self {
            levels,
            grids: vec![HashMap::new(); levels.len()],
            ids: Vec::new(),
            positions: Vec::new(),
            entity_index: HashMap::new(),
        }
    }

    pub fn insert_or_update(&mut self, id: EntityId, pos: Vec3Ser) {
        if let Some(&idx) = self.entity_index.get(&id) {
            self.positions[idx] = pos;
        } else {
            let idx = self.ids.len();
            self.ids.push(id);
            self.positions.push(pos);
            self.entity_index.insert(id, idx);
        }

        // Update grid cells...
    }

    pub fn query_radius(&self, center: &Vec3Ser, radius: f32) -> Vec<EntityId> {
        let mut result = Vec::new();
        let radius_sq = radius * radius;

        for (level_idx, level) in self.levels.iter().enumerate() {
            let center_cell = self.pos_to_cell(center, level.cell_size);
            let search_range = ((radius / level.cell_size).ceil() as i32) + 1;

            for dx in -search_range..=search_range {
                for dz in -search_range..=search_range {
                    let cell = (center_cell.0 + dx, center_cell.1 + dz);

                    if let Some(cell_ids) = self.grids[level_idx].get(&cell) {
                        for &id in cell_ids {
                            if let Some(&idx) = self.entity_index.get(&id) {
                                let pos = &self.positions[idx];

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

    pub fn remove(&mut self, id: EntityId) {
        if let Some(idx) = self.entity_index.remove(&id) {
            // Simple swap-remove for contiguous storage
            let last_idx = self.ids.len() - 1;
            if idx != last_idx {
                self.ids[idx] = self.ids[last_idx];
                self.positions[idx] = self.positions[last_idx];
                self.entity_index.insert(self.ids[idx], idx);
            }
            self.ids.pop();
            self.positions.pop();
        }
    }
}

// Thunder locked in. SoA layout implemented for positions. ⚡❤️🔥
