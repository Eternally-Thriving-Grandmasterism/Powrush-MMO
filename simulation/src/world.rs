/*!
 * Player-facing policy effects implementation details.
 */

impl SovereignWorldState {
    /// Returns active policies affecting a given InterestZone.
    pub fn get_active_policies_for_zone(&self, zone_id: u64) -> Vec<&ActivePolicy> {
        self.active_policies
            .iter()
            .filter(|p| p.target_interest_zone.map_or(true, |zid| zid == zone_id))
            .collect()
    }

    /// Applies active policy effects to a player/agent at a given position.
    /// This is the main entry point for player-facing effects.
    pub fn apply_policy_effects_to_agent(
        &self,
        agent_id: AgentId,
        position: Vec3,
        current_yield: f32,
    ) -> f32 {
        let mut modified_yield = current_yield;

        // Find zones the agent is in (simplified: check all zones for now)
        for (zone_id, zone) in &self.interest_zones {
            let dx = position.x - zone.center.x;
            let dz = position.z - zone.center.z;
            let dist_sq = dx * dx + dz * dz;

            if dist_sq <= zone.radius * zone.radius {
                let policies = self.get_active_policies_for_zone(*zone_id);

                for policy in policies {
                    match policy.policy_type {
                        PolicyType::AbundanceBoost => {
                            modified_yield *= 1.0 + (policy.strength * 0.15);
                        }
                        PolicyType::SustainabilityFocus => {
                            // Could affect long-term yield stability or mutation chance
                            modified_yield *= 1.0 + (policy.strength * 0.08);
                        }
                        _ => {}
                    }
                }
            }
        }

        modified_yield
    }

    /// Enhanced harvest yield that includes policy effects
    pub fn modulate_harvest_yield(&self, base_yield: f32, pos: Vec3) -> f32 {
        let mut yield_mod = base_yield;

        // Existing biome influence
        if let Some(inf) = self.get_biome_influence_at(pos) {
            let mercy_mod = (self.mercy_flow_state.overall_mercy_flow * 0.25 + 0.75).clamp(0.8, 1.35);
            yield_mod = (base_yield * inf.resource_yield_mod * mercy_mod).max(0.1);
        }

        // Add policy effects
        yield_mod = self.apply_policy_effects_to_agent(0, pos, yield_mod);

        yield_mod
    }
}
