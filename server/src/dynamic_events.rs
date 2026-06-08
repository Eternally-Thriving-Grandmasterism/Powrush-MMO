// server/src/dynamic_events.rs
// Powrush-MMO v17.0 — DynamicEventManager using HierarchicalGrid

use crate::spatial::hierarchical_grid::HierarchicalGrid;

pub struct DynamicEventManager {
    events: HashMap<u64, DynamicEvent>,
    next_id: u64,

    // Unified spatial system
    spatial: HierarchicalGrid,
}

impl DynamicEventManager {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
            next_id: 1,
            spatial: HierarchicalGrid::with_default_levels(),
        }
    }

    pub fn add_or_update_resource_node(&mut self, node_id: u64, pos: Vec3Ser) {
        self.spatial.insert_or_update(node_id, pos);
    }

    pub fn remove_resource_node(&mut self, node_id: u64) {
        self.spatial.remove(node_id);
    }

    /// Refresh affected nodes using the unified HierarchicalGrid
    pub fn refresh_affected_nodes_spatial(&mut self, event: &mut DynamicEvent) {
        event.affected_nodes = self.spatial.query_radius(&event.position, event.radius);
    }

    pub fn refresh_all_surge_nodes(&mut self) {
        for event in self.events.values_mut() {
            if event.event_type == EventType::ResourceSurge && event.is_active() {
                self.refresh_affected_nodes_spatial(event);
            }
        }
    }
}

// Thunder locked in. DynamicEventManager now also uses HierarchicalGrid. ⚡❤️🔥
