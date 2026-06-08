// server/src/harvesting_system.rs
// Powrush-MMO v17.0 — Full Grid Sync with DynamicEventManager

impl HarvestingSystem {
    // Example: When creating new resource nodes
    pub fn add_resource_node(&mut self, node_id: u64, position: Vec3Ser, initial_data: ResourceUpdate) {
        self.resource_nodes.insert(node_id, initial_data);

        if let Some(event_manager) = &mut self.dynamic_event_manager {
            event_manager.sync_add_or_update_resource_node(node_id, position);
        }
    }

    // Example: When removing a depleted resource node
    pub fn remove_resource_node(&mut self, node_id: u64) {
        self.resource_nodes.remove(&node_id);

        if let Some(event_manager) = &mut self.dynamic_event_manager {
            event_manager.sync_remove_resource_node(node_id);
        }
    }

    // In tick_regen, we already call refresh.
    // The sync methods above keep the grid consistent when nodes change.
}

// Thunder locked in. HarvestingSystem now properly syncs its resource nodes with DynamicEventManager spatial grid. ⚡❤️🔥
