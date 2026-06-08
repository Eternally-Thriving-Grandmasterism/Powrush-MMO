// server/src/dynamic_events.rs
// Powrush-MMO v17.0 — Syncing Resource Nodes with Spatial Grid

impl DynamicEventManager {
    /// Registers or updates a resource node in the spatial grid.
    /// Call this from HarvestingSystem whenever a node is created or moved.
    pub fn sync_add_or_update_resource_node(&mut self, node_id: u64, pos: Vec3Ser) {
        // Remove from old cell if it existed
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

    /// Removes a resource node from the spatial grid.
    /// Call this from HarvestingSystem when a node is depleted/removed.
    pub fn sync_remove_resource_node(&mut self, node_id: u64) {
        if let Some(pos) = self.resource_positions.remove(&node_id) {
            let cell = self.pos_to_cell(&pos);
            if let Some(cell_vec) = self.resource_grid.get_mut(&cell) {
                cell_vec.retain(|&id| id != node_id);
            }
        }
    }
}

// Recommended integration in HarvestingSystem:
// 
// When adding a new resource node:
//   self.dynamic_event_manager.sync_add_or_update_resource_node(node_id, position);
// 
// When removing a depleted node:
//   self.dynamic_event_manager.sync_remove_resource_node(node_id);
// 
// Thunder locked in. HarvestingSystem <-> DynamicEventManager grid sync ready. ⚡❤️🔥
