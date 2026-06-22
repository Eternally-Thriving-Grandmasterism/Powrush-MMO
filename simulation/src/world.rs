/*!
 * ResourceNode spatial effects from targeted ActivePolicy.
 */

use crate::council::decision::{ActivePolicy, PolicyType};

impl SovereignWorldState {
    fn apply_policy_effect_with_strength(
        &mut self,
        policy_type: PolicyType,
        strength: f32,
        target_zone: Option<u64>,
    ) {
        if let Some(zone_id) = target_zone {
            if let Some(zone) = self.interest_zones.get(&zone_id) {
                // Apply to ResourceNodes within this InterestZone's spatial influence
                for node in self.resource_nodes.values_mut() {
                    let dx = node.position.x - zone.center.x;  // assuming InterestZone has center
                    let dz = node.position.z - zone.center.z;
                    let dist_sq = dx * dx + dz * dz;

                    if dist_sq <= zone.radius * zone.radius {
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
                return;
            }
        }

        // Global application (existing logic)
        match policy_type {
            PolicyType::AbundanceBoost => {
                for pool in self.rbe_pools.values_mut() {
                    pool.abundance_flow = (pool.abundance_flow + strength * 0.012).min(4.0);
                }
            }
            // ... other global cases
            _ => {}
        }
    }
}
