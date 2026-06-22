/*!
 * Light expansion of policy effects to ability progression and flow state.
 */

impl SovereignWorldState {
    pub fn apply_policy_epigenetic_effects(&mut self, agent_id: AgentId, position: Vec3) {
        if let Some(profile) = self.evolutionary_profiles.get_mut(&agent_id) {
            let mut significant_change = false;

            for (zone_id, zone) in &self.interest_zones {
                // ... existing zone check ...

                if dist_sq <= zone.radius * zone.radius {
                    let policies = self.get_active_policies_for_zone(*zone_id);

                    for policy in policies {
                        // ... existing epigenetic logic ...

                        // Light expansion to ability synergy
                        if let Some(tree) = self.ability_trees.get_mut(&agent_id) {
                            if policy.policy_type == PolicyType::HarmonyStabilization {
                                // Harmony policies slightly accelerate ability synergy
                                // (placeholder - can be expanded with real synergy logic)
                            }
                        }

                        // Light flow state influence
                        if policy.policy_type == PolicyType::AbundanceBoost {
                            self.flow_metrics.current_challenge_level = 
                                (self.flow_metrics.current_challenge_level - policy.strength * 0.02).max(0.05);
                        }
                    }
                }
            }
        }
    }
}
