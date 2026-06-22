/*!
 * SovereignWorldState with ActivePolicy decay + application logic.
 */

use crate::council::decision::{ActivePolicy, PolicyType};

// ... existing code ...

impl SovereignWorldState {
    pub fn tick(&mut self, dt_ms: u64) -> Result<(), MercyViolation> {
        self.sim_time += dt_ms;

        let mercy_flow = self.mercy_flow_state.overall_mercy_flow;

        for state in self.active_biomes.values_mut() {
            let drift = 0.00008 * (mercy_flow - 0.5);
            state.epiphany_resonance = (state.epiphany_resonance + drift).clamp(0.35, 1.0);
            state.valence_harmony = (state.valence_harmony + drift * 0.7).clamp(0.25, 1.0);

            if mercy_flow > 0.6 {
                state.entropy_level = (state.entropy_level - 0.00005).max(0.1);
            }
        }

        // === Persistent Policy Decay + Application ===
        self.apply_and_decay_policies();

        Ok(())
    }

    fn apply_and_decay_policies(&mut self) {
        let mut i = 0;
        while i < self.active_policies.len() {
            let policy = &mut self.active_policies[i];

            // Apply this tick's effect
            self.apply_policy_effect(policy);

            // Decay
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

    fn apply_policy_effect(&mut self, policy: &ActivePolicy) {
        match policy.policy_type {
            PolicyType::AbundanceBoost => {
                for pool in self.rbe_pools.values_mut() {
                    if policy.target_faction.map_or(true, |f| /* match logic */ true) {
                        pool.abundance_flow = (pool.abundance_flow + policy.strength * 0.008).min(4.0);
                    }
                }
            }
            PolicyType::SustainabilityFocus => {
                for pool in self.rbe_pools.values_mut() {
                    pool.sustainability_score = (pool.sustainability_score + policy.strength * 0.006).min(1.0);
                }
            }
            PolicyType::PressureReduction => {
                for pool in self.rbe_pools.values_mut() {
                    pool.pressure = (pool.pressure - policy.strength * 0.01).max(0.0);
                }
            }
            PolicyType::HarmonyStabilization => {
                for pool in self.rbe_pools.values_mut() {
                    pool.sustainability_score = (pool.sustainability_score + policy.strength * 0.004).min(1.0);
                }
            }
            PolicyType::GeneralProsperity => {
                for pool in self.rbe_pools.values_mut() {
                    pool.abundance_flow = (pool.abundance_flow + policy.strength * 0.005).min(4.0);
                    pool.sustainability_score = (pool.sustainability_score + policy.strength * 0.003).min(1.0);
                }
            }
        }
    }

    // ... rest of impl ...
}
