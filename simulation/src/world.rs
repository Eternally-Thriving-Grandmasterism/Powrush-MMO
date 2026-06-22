/*!
 * SovereignWorldState with Policy Stacking Rules.
 *
 * Same-type policies now use diminishing returns + soft caps.
 */

use std::collections::HashMap;
use crate::council::decision::{ActivePolicy, PolicyType};

// ... existing code ...

impl SovereignWorldState {
    // ... existing tick and methods ...

    fn apply_and_decay_policies(&mut self) {
        // === Policy Stacking Rules ===
        let effective_strengths = self.calculate_effective_policy_strengths();

        // Apply using effective (stacked) strengths
        for (policy_type, strength) in effective_strengths {
            self.apply_policy_effect_with_strength(policy_type, strength);
        }

        // Decay all policies
        let mut i = 0;
        while i < self.active_policies.len() {
            let policy = &mut self.active_policies[i];
            if policy.remaining_ticks > 0 {
                policy.remaining_ticks = policy.remaining_ticks.saturating_sub(1);
            }
            if policy.remaining_ticks == 0 {
                self.active_policies.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    /// Calculates effective strength per PolicyType using diminishing returns.
    /// This implements basic stacking rules.
    fn calculate_effective_policy_strengths(&self) -> HashMap<PolicyType, f32> {
        let mut totals: HashMap<PolicyType, f32> = HashMap::new();

        for policy in &self.active_policies {
            *totals.entry(policy.policy_type).or_insert(0.0) += policy.strength;
        }

        let mut effective = HashMap::new();
        for (ptype, total) in totals {
            // Diminishing returns: square root scaling + soft cap
            let scaled = (total.sqrt() * 0.7).min(0.8); // soft cap at 0.8 effective
            effective.insert(ptype, scaled);
        }

        effective
    }

    fn apply_policy_effect_with_strength(&mut self, policy_type: PolicyType, strength: f32) {
        match policy_type {
            PolicyType::AbundanceBoost => {
                for pool in self.rbe_pools.values_mut() {
                    pool.abundance_flow = (pool.abundance_flow + strength * 0.012).min(4.0);
                }
            }
            PolicyType::SustainabilityFocus => {
                for pool in self.rbe_pools.values_mut() {
                    pool.sustainability_score = (pool.sustainability_score + strength * 0.009).min(1.0);
                }
            }
            PolicyType::PressureReduction => {
                for pool in self.rbe_pools.values_mut() {
                    pool.pressure = (pool.pressure - strength * 0.015).max(0.0);
                }
            }
            PolicyType::HarmonyStabilization => {
                for pool in self.rbe_pools.values_mut() {
                    pool.sustainability_score = (pool.sustainability_score + strength * 0.006).min(1.0);
                }
            }
            PolicyType::GeneralProsperity => {
                for pool in self.rbe_pools.values_mut() {
                    pool.abundance_flow = (pool.abundance_flow + strength * 0.007).min(4.0);
                    pool.sustainability_score = (pool.sustainability_score + strength * 0.004).min(1.0);
                }
            }
        }
    }

    // Keep old method for backward compatibility if needed, or remove later
    fn apply_policy_effect(&mut self, _policy: &ActivePolicy) {
        // Deprecated - now handled via calculate_effective_policy_strengths
    }
}
