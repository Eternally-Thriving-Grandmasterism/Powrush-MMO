/*!
 * Spatial targeting implementation for ActivePolicy.
 */

use crate::council::decision::ActivePolicy;

// In apply_policy_effect_with_strength, add support for target_interest_zone

fn apply_policy_effect_with_strength(
    &mut self,
    policy_type: PolicyType,
    strength: f32,
    target_zone: Option<u64>,
) {
    if let Some(zone_id) = target_zone {
        if let Some(zone) = self.interest_zones.get_mut(&zone_id) {
            // Apply spatially targeted effect to the InterestZone
            match policy_type {
                PolicyType::AbundanceBoost => {
                    zone.abundance_multiplier = (zone.abundance_multiplier + strength * 0.15).min(3.0);
                }
                PolicyType::SustainabilityFocus => {
                    // Assume InterestZone has sustainability influence or we boost related biome
                    if let Some(biome) = self.active_biomes.get_mut(&zone.biome_name) {  // if field exists
                        biome.sustainability_score = (biome.sustainability_score + strength * 0.1).min(1.0);
                    }
                }
                _ => {}
            }
            return; // Spatial policy applied, skip global
        }
    }

    // Global fallback (existing logic)
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

// Update call in apply_and_decay_policies to pass target:
// self.apply_policy_effect_with_strength(*policy_type, *strength, policy.target_interest_zone);
