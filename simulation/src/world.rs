/*!
 * SovereignWorldState with Cross-Type Policy Synergies.
 */

use std::collections::HashSet;
use crate::council::decision::PolicyType;

// ... existing code ...

impl SovereignWorldState {
    fn apply_and_decay_policies(&mut self) {
        let active_types: HashSet<PolicyType> = self.active_policies
            .iter()
            .map(|p| p.policy_type)
            .collect();

        let effective_strengths = self.calculate_effective_policy_strengths();

        // Apply base effects
        for (policy_type, strength) in &effective_strengths {
            self.apply_policy_effect_with_strength(*policy_type, *strength);
        }

        // === Cross-Type Synergies ===
        self.apply_policy_synergies(&active_types, &effective_strengths);

        // Decay
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

    fn apply_policy_synergies(
        &mut self,
        active_types: &HashSet<PolicyType>,
        effective: &std::collections::HashMap<PolicyType, f32>,
    ) {
        // Synergy 1: Abundance + Sustainability (strong economic harmony)
        if active_types.contains(&PolicyType::AbundanceBoost)
            && active_types.contains(&PolicyType::SustainabilityFocus)
        {
            let bonus = effective.get(&PolicyType::AbundanceBoost).unwrap_or(&0.0) * 0.15;
            for pool in self.rbe_pools.values_mut() {
                pool.sustainability_score = (pool.sustainability_score + bonus * 0.6).min(1.0);
                pool.abundance_flow = (pool.abundance_flow + bonus * 0.4).min(4.0);
            }
        }

        // Synergy 2: Harmony + Pressure Reduction (systemic stability)
        if active_types.contains(&PolicyType::HarmonyStabilization)
            && active_types.contains(&PolicyType::PressureReduction)
        {
            let bonus = effective.get(&PolicyType::HarmonyStabilization).unwrap_or(&0.0) * 0.12;
            for pool in self.rbe_pools.values_mut() {
                pool.sustainability_score = (pool.sustainability_score + bonus).min(1.0);
                pool.pressure = (pool.pressure - bonus * 0.5).max(0.0);
            }
        }

        // Synergy 3: GeneralProsperity amplifies other policies slightly
        if active_types.contains(&PolicyType::GeneralProsperity) {
            let prosperity = effective.get(&PolicyType::GeneralProsperity).unwrap_or(&0.0);
            if prosperity > &0.1 {
                for pool in self.rbe_pools.values_mut() {
                    pool.abundance_flow = (pool.abundance_flow + prosperity * 0.006).min(4.0);
                    pool.sustainability_score = (pool.sustainability_score + prosperity * 0.004).min(1.0);
                }
            }
        }
    }

    // ... rest of the methods ...
}
