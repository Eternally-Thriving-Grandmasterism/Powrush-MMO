// server/src/spatial/hierarchical_grid.rs
// Powrush-MMO v17.0 — HierarchicalGrid with Full SoA (x, y, z separate)

// Ultimate cache + SIMD friendly layout: separate arrays for x, y, z

pub struct HierarchicalGrid {
    levels: Vec<GridLevel>,
    grids: Vec<HashMap<(i32, i32), Vec<EntityId>>>,

    // Full SoA layout
    ids: Vec<EntityId>,
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
    entity_index: HashMap<EntityId, usize>,
}

impl HierarchicalGrid {
    pub fn new(levels: Vec<GridLevel>) -> Self {
        Self {
            levels,
            grids: vec![HashMap::new(); levels.len()],
            ids: Vec::new(),
            x: Vec::new(),
            y: Vec::new(),
            z: Vec::new(),
            entity_index: HashMap::new(),
        }
    }

    pub fn insert_or_update(&mut self, id: EntityId, pos: Vec3Ser) {
        if let Some(&idx) = self.entity_index.get(&id) {
            self.x[idx] = pos.x;
            self.y[idx] = pos.y;
            self.z[idx] = pos.z;
        } else {
            let idx = self.ids.len();
            self.ids.push(id);
            self.x.push(pos.x);
            self.y.push(pos.y);
            self.z.push(pos.z);
            self.entity_index.insert(id, idx);
        }

        // Update grid buckets...
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
                                let dx = self.x[idx] - center.x;
                                let dy = self.y[idx] - center.y;
                                let dz = self.z[idx] - center.z;

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
            let last = self.ids.len() - 1;

            if idx != last {
                self.ids[idx] = self.ids[last];
                self.x[idx] = self.x[last];
                self.y[idx] = self.y[last];
                self.z[idx] = self.z[last];
                self.entity_index.insert(self.ids[idx], idx);
            }

            self.ids.pop();
            self.x.pop();
            self.y.pop();
            self.z.pop();
        }
    }
}

// Thunder locked in. Full SoA (separate x/y/z) implemented for maximum cache + SIMD efficiency. ⚡❤️🔥
