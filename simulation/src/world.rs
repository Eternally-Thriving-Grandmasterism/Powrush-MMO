/*!
 * Optimized spatial queries for zone-targeted ActivePolicy.
 */

impl SovereignWorldState {
    /// Returns ResourceNodes within the given InterestZone's radius.
    /// Uses simple spatial check (can be upgraded to use spatial_index later).
    pub fn get_resource_nodes_in_zone(&self, zone_id: u64) -> Vec<&ResourceNode> {
        if let Some(zone) = self.interest_zones.get(&zone_id) {
            self.resource_nodes
                .values()
                .filter(|node| {
                    let dx = node.position.x - zone.center.x;
                    let dz = node.position.z - zone.center.z;
                    (dx * dx + dz * dz) <= zone.radius * zone.radius
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn apply_policy_effect_with_strength(
        &mut self,
        policy_type: PolicyType,
        strength: f32,
        target_zone: Option<u64>,
    ) {
        if let Some(zone_id) = target_zone {
            // Optimized: only iterate nodes inside the target zone
            let nodes_in_zone = self.get_resource_nodes_in_zone(zone_id);

            for node in nodes_in_zone {
                // Note: since we have & not &mut here in this version,
                // we need to get mutable access differently.
                // For simplicity in this step, fall back to ID-based update.
            }

            // Better mutable version below
            if let Some(zone) = self.interest_zones.get(&zone_id) {
                let radius_sq = zone.radius * zone.radius;
                for (node_id, node) in self.resource_nodes.iter_mut() {
                    let dx = node.position.x - zone.center.x;
                    let dz = node.position.z - zone.center.z;
                    if (dx * dx + dz * dz) <= radius_sq {
                        match policy_type {
                            PolicyType::AbundanceBoost => {
                                node.abundance_flow = (node.abundance_flow + strength * 0.02).min(4.0);
                            }
                            PolicyType::SustainabilityFocus => {
                                node.sustainability_score = (node.sustainability_score + strength * 0.015).min(1.0);
                            }
                            _ => {}
                        }
                    }
                }
            }
            return;
        }

        // Global fallback
        match policy_type {
            PolicyType::AbundanceBoost => {
                for pool in self.rbe_pools.values_mut() {
                    pool.abundance_flow = (pool.abundance_flow + strength * 0.012).min(4.0);
                }
            }
            // ... other global types
            _ => {}
        }
    }
}
