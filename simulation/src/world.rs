/*!
 * Zone-Node reverse cache for fast spatial policy queries.
 */

use std::collections::HashMap;

impl SovereignWorldState {
    /// Rebuilds the zone -> node cache.
    /// Call this when interest_zones or resource_nodes change significantly.
    pub fn rebuild_zone_node_cache(&mut self) {
        self.zone_node_cache.clear();

        for (&zone_id, zone) in &self.interest_zones {
            let mut nodes_in_zone = Vec::new();
            let radius_sq = zone.radius * zone.radius;

            for (&node_id, node) in &self.resource_nodes {
                let dx = node.position.x - zone.center.x;
                let dz = node.position.z - zone.center.z;
                if (dx * dx + dz * dz) <= radius_sq {
                    nodes_in_zone.push(node_id);
                }
            }

            self.zone_node_cache.insert(zone_id, nodes_in_zone);
        }
    }

    /// Fast lookup using the cache.
    pub fn get_resource_nodes_in_zone(&self, zone_id: u64) -> Vec<&ResourceNode> {
        if let Some(node_ids) = self.zone_node_cache.get(&zone_id) {
            node_ids
                .iter()
                .filter_map(|id| self.resource_nodes.get(id))
                .collect()
        } else {
            // Fallback to on-the-fly if cache is empty
            self.resource_nodes
                .values()
                .filter(|node| {
                    if let Some(zone) = self.interest_zones.get(&zone_id) {
                        let dx = node.position.x - zone.center.x;
                        let dz = node.position.z - zone.center.z;
                        (dx * dx + dz * dz) <= zone.radius * zone.radius
                    } else {
                        false
                    }
                })
                .collect()
        }
    }

    // Also add a mutable version for policy application
    pub fn get_mut_resource_nodes_in_zone(&mut self, zone_id: u64) -> Vec<&mut ResourceNode> {
        if let Some(node_ids) = self.zone_node_cache.get(&zone_id).cloned() {
            node_ids
                .into_iter()
                .filter_map(|id| self.resource_nodes.get_mut(&id))
                .collect()
        } else {
            vec![]
        }
    }
}
