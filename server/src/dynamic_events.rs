// server/src/dynamic_events.rs
// Powrush-MMO v17.0 — Spatial Grid Optimization for Node Refresh

// Add a lightweight spatial grid inside DynamicEventManager for resource nodes

pub struct DynamicEventManager {
    events: HashMap<u64, DynamicEvent>,
    next_id: u64,
    // Spatial grid for fast resource node queries (same cell size as InterestManager)
    resource_grid: HashMap<(i32, i32), Vec<u64>>,
    resource_positions: HashMap<u64, Vec3Ser>,
}

impl DynamicEventManager {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
            next_id: 1,
            resource_grid: HashMap::new(),
            resource_positions: HashMap::new(),
        }
    }

    pub fn add_or_update_resource_node(&mut self, node_id: u64, pos: Vec3Ser) {
        // Remove from old cell
        if let Some(old_pos) = self.resource_positions.get(&node_id) {
            let old_cell = self.pos_to_cell(old_pos);
            if let Some(cell) = self.resource_grid.get_mut(&old_cell) {
                cell.retain(|&id| id != node_id);
            }
        }

        let cell = self.pos_to_cell(&pos);
        self.resource_grid.entry(cell).or_default().push(node_id);
        self.resource_positions.insert(node_id, pos);
    }

    fn pos_to_cell(&self, pos: &Vec3Ser) -> (i32, i32) {
        const CELL_SIZE: f32 = 64.0;
        (
            (pos.x / CELL_SIZE).floor() as i32,
            (pos.z / CELL_SIZE).floor() as i32,
        )
    }

    /// Highly optimized refresh using spatial grid (same as InterestManager)
    pub fn refresh_affected_nodes_spatial(&mut self, event: &mut DynamicEvent) {
        event.affected_nodes.clear();

        let center_cell = self.pos_to_cell(&event.position);
        let radius_sq = event.radius * event.radius;

        for dx in -2..=2_i32 {
            for dz in -2..=2_i32 {
                let cell_key = (center_cell.0 + dx, center_cell.1 + dz);
                if let Some(node_ids) = self.resource_grid.get(&cell_key) {
                    for &node_id in node_ids {
                        if let Some(node_pos) = self.resource_positions.get(&node_id) {
                            let dx = node_pos.x - event.position.x;
                            let dy = node_pos.y - event.position.y;
                            let dz = node_pos.z - event.position.z;

                            if (dx*dx + dy*dy + dz*dz) <= radius_sq {
                                event.affected_nodes.push(node_id);
                            }
                        }
                    }
                }
            }
        }
    }
}

// Thunder locked in. Spatial grid optimization for node refresh implemented. ⚡❤️🔥
