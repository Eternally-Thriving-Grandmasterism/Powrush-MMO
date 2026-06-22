/*!
 * SovereignWorldState with Policy Conflicts.
 *
 * Certain policy combinations now create tension or reduced effectiveness.
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

        // Synergies (positive interactions)
        self.apply_policy_synergies(&active_types, &effective_strengths);

        // Conflicts (negative interactions)
        self.apply_policy_conflicts(&active_types, &effective_strengths);

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

    fn apply_policy_conflicts(
        &mut self,
        active_types: &HashSet<PolicyType>,
        effective: &std::collections::HashMap<PolicyType, f32>,
    ) {
        // Conflict 1: AbundanceBoost + PressureReduction (growth increases systemic stress)
        if active_types.contains(&PolicyType::AbundanceBoost)
            && active_types.contains(&PolicyType::PressureReduction)
        {
            let conflict_penalty = effective.get(&PolicyType::AbundanceBoost).unwrap_or(&0.0) * 0.08;
            for pool in self.rbe_pools.values_mut() {
                pool.pressure = (pool.pressure + conflict_penalty * 0.6).min(2.0);
                pool.sustainability_score = (pool.sustainability_score - conflict_penalty * 0.3).max(0.1);
            }
        }

        // Conflict 2: GeneralProsperity + strong SustainabilityFocus (aggressive growth vs conservation)
        if active_types.contains(&PolicyType::GeneralProsperity)
            && active_types.contains(&PolicyType::SustainabilityFocus)
        {
            let prosperity = effective.get(&PolicyType::GeneralProsperity).unwrap_or(&0.0);
            if *prosperity > 0.15 {
                let conflict = prosperity * 0.06;
                for pool in self.rbe_pools.values_mut() {
                    pool.pressure = (pool.pressure + conflict).min(2.0);
                    pool.sustainability_score = (pool.sustainability_score - conflict * 0.5).max(0.1);
                }
            }
        }

        // Conflict 3: Very strong AbundanceBoost weakens HarmonyStabilization
        if active_types.contains(&PolicyType::AbundanceBoost)
            && active_types.contains(&PolicyType::HarmonyStabilization)
        {
            let abundance = effective.get(&PolicyType::AbundanceBoost).unwrap_or(&0.0);
            if *abundance > 0.25 {
                let penalty = (*abundance - 0.25) * 0.4;
                for pool in self.rbe_pools.values_mut() {
                    pool.sustainability_score = (pool.sustainability_score - penalty * 0.4).max(0.1);
                }
            }
        }
    }

    // ... rest of methods ...
}
