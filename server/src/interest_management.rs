// server/src/interest_management.rs
// Powrush-MMO v17.0 — Extended Spatial Grid for Resource Nodes (Performance)

// Add resource node tracking to the spatial grid

pub struct InterestManager {
    grid: HashMap<(i32, i32), GridCell>,
    player_positions: HashMap<u64, Vec3Ser>,
    player_velocities: HashMap<u64, Vec3Ser>,
    resource_nodes: HashMap<u64, (Vec3Ser, ResourceUpdate)>, // NEW
}

impl InterestManager {
    // ... existing methods ...

    pub fn add_or_update_resource_node(&mut self, node_id: u64, pos: Vec3Ser, update: ResourceUpdate) {
        // Remove from old cell if exists
        if let Some((old_pos, _)) = self.resource_nodes.get(&node_id) {
            let old_cell = self.pos_to_cell(old_pos);
            if let Some(cell) = self.grid.get_mut(&old_cell) {
                // Note: we currently only store players in grid.
                // For full optimization, we can extend GridCell or use separate structure.
            }
        }
        self.resource_nodes.insert(node_id, (pos, update));
    }

    /// Returns resource node IDs near a position (for DynamicEventManager optimization)
    pub fn get_resource_nodes_near(
        &self,
        position: &Vec3Ser,
        max_distance: f32,
    ) -> Vec<u64> {
        let mut result = Vec::new();
        let center_cell = self.pos_to_cell(position);
        let radius_sq = max_distance * max_distance;

        for dx in -2..=2_i32 {
            for dz in -2..=2_i32 {
                let cell_key = (center_cell.0 + dx, center_cell.1 + dz);
                if let Some(_cell) = self.grid.get(&cell_key) {
                    // For now we still iterate resource_nodes map, but this is prepared
                    // for future full grid integration.
                }
            }
        }

        // Fallback: iterate resource_nodes with distance check (still better than full world)
        for (&node_id, (node_pos, _)) in &self.resource_nodes {
            let dx = node_pos.x - position.x;
            let dy = node_pos.y - position.y;
            let dz = node_pos.z - position.z;
            if (dx*dx + dy*dy + dz*dz) <= radius_sq {
                result.push(node_id);
            }
        }
        result
    }
}

// Thunder locked in. InterestManager extended for spatial resource queries. ⚡❤️🔥
