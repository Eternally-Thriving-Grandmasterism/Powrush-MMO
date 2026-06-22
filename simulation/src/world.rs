/*!
 * Epigenetic mutation effects from active policies.
 */

use crate::epigenetic_modulation::{
    apply_volatility_drift, is_high_volatility_risk,
    apply_double_edged_volatility_effects, apply_epigenetic_repair,
    try_trigger_epigenetic_mutation,
};

impl SovereignWorldState {
    /// Applies epigenetic effects from local active policies to an agent.
    pub fn apply_policy_epigenetic_effects(&mut self, agent_id: AgentId, position: Vec3) {
        if let Some(profile) = self.evolutionary_profiles.get_mut(&agent_id) {
            // Find zones the agent is currently in
            for (zone_id, zone) in &self.interest_zones {
                let dx = position.x - zone.center.x;
                let dz = position.z - zone.center.z;
                let dist_sq = dx * dx + dz * dz;

                if dist_sq <= zone.radius * zone.radius {
                    let policies = self.get_active_policies_for_zone(*zone_id);

                    for policy in policies {
                        let intensity = policy.strength;

                        match policy.policy_type {
                            PolicyType::AbundanceBoost => {
                                // Abundance encourages positive drift and lower volatility
                                profile.volatility = (profile.volatility - intensity * 0.08).max(0.1);
                                profile.strength = (profile.strength + intensity * 0.05).min(2.0);

                                // Slightly higher chance of beneficial mutation
                                if rand::random::<f32>() < intensity * 0.04 {
                                    let _ = try_trigger_epigenetic_mutation(
                                        profile, false, true, 1.2, self.sim_time
                                    );
                                }
                            }
                            PolicyType::SustainabilityFocus => {
                                // Sustainability improves stability and repair
                                profile.volatility = (profile.volatility - intensity * 0.12).max(0.1);
                                apply_epigenetic_repair(profile, 1.1, true);
                            }
                            PolicyType::HarmonyStabilization => {
                                profile.cooperation_score = (profile.cooperation_score + intensity * 0.1).min(2.0);
                                profile.layer_alignment = (profile.layer_alignment + intensity * 0.06).min(2.0);
                            }
                            PolicyType::GeneralProsperity => {
                                // Broad positive effect
                                profile.strength = (profile.strength + intensity * 0.04).min(2.0);
                                if rand::random::<f32>() < intensity * 0.03 {
                                    let _ = try_trigger_epigenetic_mutation(
                                        profile, false, true, 1.0, self.sim_time
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
