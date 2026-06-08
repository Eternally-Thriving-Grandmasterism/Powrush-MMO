// server/src/dynamic_events.rs
// Powrush-MMO v17.0 — Optimized Node Refresh Performance

impl DynamicEvent {
    pub last_refresh_tick: u64,           // For time-based optimization
    pub needs_refresh: bool,
}

impl DynamicEvent {
    pub fn new(...) -> Self {
        Self {
            // ...
            last_refresh_tick: 0,
            needs_refresh: true,
        }
    }

    /// Optimized refresh with bounding box pre-check + dirty flag
    pub fn refresh_affected_nodes_optimized(
        &mut self,
        nodes: &HashMap<u64, ResourceUpdate>,
        current_tick: u64,
        refresh_interval: u64, // e.g. 5 ticks
    ) {
        if !self.needs_refresh && (current_tick - self.last_refresh_tick) < refresh_interval {
            return;
        }

        self.affected_nodes.clear();
        self.last_refresh_tick = current_tick;
        self.needs_refresh = false;

        let min_x = self.position.x - self.radius;
        let max_x = self.position.x + self.radius;
        let min_z = self.position.z - self.radius;
        let max_z = self.position.z + self.radius;

        for (&node_id, node) in nodes {
            // Fast AABB rejection test first
            if node.position_x < min_x || node.position_x > max_x ||
               node.position_z < min_z || node.position_z > max_z {
                continue;
            }

            let dx = node.position_x - self.position.x;
            let dy = node.position_y - self.position.y;
            let dz = node.position_z - self.position.z;

            if (dx*dx + dy*dy + dz*dz).sqrt() <= self.radius {
                self.affected_nodes.push(node_id);
            }
        }
    }
}

impl DynamicEventManager {
    pub fn refresh_resource_surge_nodes_optimized(
        &mut self,
        nodes: &HashMap<u64, ResourceUpdate>,
        current_tick: u64,
        refresh_interval: u64,
    ) {
        for event in self.events.values_mut() {
            if event.event_type == EventType::ResourceSurge && event.is_active() {
                event.refresh_affected_nodes_optimized(nodes, current_tick, refresh_interval);
            }
        }
    }
}

// In HarvestingSystem tick_regen:
// event_manager.refresh_resource_surge_nodes_optimized(&self.resource_nodes, current_tick, 5);
//
// Thunder locked in. Node refresh performance optimized. ⚡❤️🔥
