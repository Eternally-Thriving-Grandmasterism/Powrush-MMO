/*!
 * Wiring epigenetic policy effects into the main tick loop + events.
 */

use crate::epigenetic_modulation::EpigeneticProfile;

// Add this event (or reuse existing event system)
#[derive(Event, Clone, Debug)]
pub struct EpigeneticPolicyEffectApplied {
    pub agent_id: AgentId,
    pub policy_type: crate::council::decision::PolicyType,
    pub strength: f32,
}

impl SovereignWorldState {
    pub fn tick(&mut self, dt_ms: u64) -> Result<(), MercyViolation> {
        self.sim_time += dt_ms;

        // ... existing biome + policy decay logic ...

        // === Apply epigenetic effects from policies to all agents ===
        // Run every 20 ticks for performance
        if self.sim_time % 20 == 0 {
            for agent in &self.agents {
                self.apply_policy_epigenetic_effects(agent.id, agent.position);
            }
        }

        // Periodic cache rebuild
        if self.sim_time % 200 == 0 {
            self.rebuild_zone_node_cache();
        }

        Ok(())
    }
}
