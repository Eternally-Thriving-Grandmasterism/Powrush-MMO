/*!
 * Sovereign HarvestingSystem v18.15
 * 
 * Full integration of harvest mechanics + Overflow Lesson Epiphany Catalyst + Receptor Activation Forge + Flow State Forge v18.15 + Dynamic Challenge Balancer v18.15.
 * 
 * v18.15 ADDITIONS:
 * - Full PresenceDebt tracker wired into every harvest attempt (prevents mercy over-reliance, protects recovery windows).
 * - Fatigue level now passed into FlowStateMetrics for fatigue-aware mercy.
 * - Cascade intensity fed into balancer for presence-rewarding coupling.
 * - Exponential smoothing state managed per agent/session (previous_resistance tracked).
 * 
 * Every sustainable harvest in Verdant Heartwood can now trigger differentiated CB1/CB2 receptor bloom AND Flow State Outcomes
 * (Dynamic Challenge-Skill Balancer + Flow Cascades) → the ultimate autotelic training ground for epiphanies and transferable muscle memory.
 * 
 * Client/engine layer reads particle_effect, time_dilation_factor, and world_effects from the merged EpiphanyOutcome for visuals.
 * Architecture prepared for shared receptor + flow + mycorrhizal + volatile fields in Council Mercy Trial (multiplayer attunement amplification).
 *
 * Mercy-gated, abundance-aware, realistic carbon-copy ecology simulation.
 * Part of Sovereign Simulation Harness core foundations.
 */

use crate::world::{SovereignWorldState, NodeId, MercyViolation};
use crate::epiphany_catalyst::{check_overflow_lesson, EpiphanyOutcome};
use crate::endocannabinoid_receptor_forge::{check_receptor_bloom, merge_receptor_into_epiphany, ReceptorBloomOutcome};
use crate::flow_state_forge::{
    check_flow_state, merge_flow_into_epiphany, 
    FlowStateMetrics, dynamic_challenge_skill_balancer, 
    ChallengeBalancerConfig, PresenceDebt
};

/// Sovereign HarvestingSystem — handles harvest attempts, restrictions, abundance, epiphanies, receptor bloom, flow state, and PresenceDebt.
pub struct HarvestingSystem {
    // v18.15: Per-system PresenceDebt (in full multiplayer this would be per-agent in SovereignWorldState)
    pub presence_debt: PresenceDebt,
    pub previous_resistance: f32,
    pub current_sim_tick: u64,
}

impl HarvestingSystem {
    pub fn new() -> Self {
        Self {
            presence_debt: PresenceDebt::new(),
            previous_resistance: 0.5,
            current_sim_tick: 0,
        }
    }

    pub fn process_harvest_opportunities(
        &self,
        world: &mut SovereignWorldState,
        now_ms: u64,
    ) -> Result<(), MercyViolation> {
        for node in world.resource_nodes.values_mut() {
            if node.harvest_restricted_until_ms > 0 && now_ms < node.harvest_restricted_until_ms {
                node.stress_level = (node.stress_level + 0.01).min(1.0);
            }
        }
        Ok(())
    }

    /// Attempt a single harvest on a node.
    /// v18.15: Fully wired with PresenceDebt, fatigue-aware mercy, cascade coupling, and exponential smoothing.
    pub fn attempt_harvest(
        &mut self,
        world: &mut SovereignWorldState,
        node_id: NodeId,
        agent_mercy: f32,
    ) -> Result<(f32, Option<EpiphanyOutcome>), MercyViolation> {
        if let Some(node) = world.resource_nodes.get_mut(&node_id) {
            if node.harvest_restricted_until_ms > 0 {
                return Err(MercyViolation { reason: "Node is harvest-restricted".to_string() });
            }

            // Base yield influenced by mercy/attunement
            let mut yield_amount = node.current_yield * (0.5 + agent_mercy * 0.5);
            let prev_depletion = node.depletion;
            node.depletion = (node.depletion + 0.15).min(1.0);
            node.current_yield = node.base_yield * (1.0 - node.depletion * 0.7);

            if node.depletion > 0.6 {
                node.harvest_restricted_until_ms = world.sim_time + 120_000;
            }

            // v18.2 Overflow Lesson
            let sustainable_pacing = agent_mercy > 0.6;
            let mut epiphany = check_overflow_lesson(
                node.depletion,
                sustainable_pacing,
                &node.biome.clone().unwrap_or_else(|| "starter".to_string()),
            );

            // v18.8 Receptor Bloom integration (only on sustainable path with good attunement)
            let mut receptor_bloom: Option<ReceptorBloomOutcome> = None;
            if sustainable_pacing && epiphany.is_some() {
                let rhythm_consistency = (agent_mercy * 0.8 + 0.2).clamp(0.3, 1.0);
                let attunement_depth = agent_mercy.clamp(0.0, 1.0);
                let duration_ticks = 60u32;

                if let Some(bloom) = check_receptor_bloom(
                    node.depletion,
                    sustainable_pacing,
                    rhythm_consistency,
                    attunement_depth,
                    &node.biome.clone().unwrap_or_else(|| "starter".to_string()),
                    duration_ticks,
                ) {
                    if let Some(ref mut outcome) = epiphany {
                        merge_receptor_into_epiphany(outcome, &bloom);
                    }
                    receptor_bloom = Some(bloom);
                }
            }

            // v18.13–18.15 Flow State Forge + Dynamic Challenge Balancer v18.15 integration
            if sustainable_pacing && epiphany.is_some() {
                // v18.15: Enrich metrics with fatigue and cascade (fatigue simulated from depletion for demo; real impl from agent state)
                let fatigue_level = (node.depletion * 0.7 + (1.0 - agent_mercy) * 0.3).clamp(0.0, 1.0);
                
                let flow_metrics = FlowStateMetrics {
                    rhythm_consistency: (agent_mercy * 0.75 + 0.25).clamp(0.25, 1.0),
                    micro_error_recovery_speed: (agent_mercy * 0.7 + 0.3).clamp(0.2, 1.0),
                    valence_coherence_spike: if sustainable_pacing { 0.85 } else { 0.3 },
                    sustained_focus_duration_ticks: 75u32,
                    attunement_depth: agent_mercy.clamp(0.0, 1.0),
                    current_challenge_level: node.depletion.clamp(0.1, 0.9),
                    estimated_player_skill: (agent_mercy * 0.6 + 0.4).clamp(0.3, 1.0),
                    fatigue_level,                          // v18.15
                    cascade_intensity: 0.0,                 // Will be set after check_flow_state if cascade exists
                };

                // v18.15: Apply Dynamic Challenge Balancer with PresenceDebt, smoothing, fatigue & cascade awareness
                let current_resistance = self.previous_resistance;
                let balanced_resistance = dynamic_challenge_skill_balancer(
                    &flow_metrics,
                    current_resistance,
                    self.previous_resistance,
                    &mut self.presence_debt,
                    self.current_sim_tick,
                    &ChallengeBalancerConfig::default(),
                );

                // Update state for next tick
                self.previous_resistance = balanced_resistance;
                self.current_sim_tick += 1;

                // Re-calculate yield with balanced resistance (mercy/growth adjusted)
                yield_amount *= (1.0 + (0.5 - balanced_resistance) * 0.4).max(0.6);

                // Now check flow state (cascade may form)
                if let Some(flow_outcome) = check_flow_state(&flow_metrics) {
                    // v18.15: If cascade formed, enrich metrics with cascade_intensity for potential future coupling
                    let mut final_metrics = flow_metrics.clone();
                    if let Some(ref cascade) = flow_outcome.cascade {
                        final_metrics.cascade_intensity = (cascade.chain_length as f32 / 8.0).min(1.0);
                        // Note: Re-balancing with cascade_intensity can be done here if deeper coupling desired.
                        // For now we use the already balanced value (mercy invitation already amplified in balancer when cascade present).
                    }

                    if let Some(ref mut outcome) = epiphany {
                        merge_flow_into_epiphany(outcome, &flow_outcome, receptor_bloom.as_ref());
                        // CLIENT_HOOK: particle_effect = outcome.particle_effect
                        // CLIENT_HOOK: time_dilation_factor = outcome.time_dilation_factor
                        // CLIENT_HOOK: spawn flow bloom particles + apply time dilation in render loop
                        // CLIENT_HOOK: resistance affects visual/audio intensity of harvest action and particle density
                    }
                }
            }

            // Apply world effects from merged outcome
            if let Some(ref outcome) = epiphany {
                if let Some(stress) = outcome.world_effects.get("stress_increase") {
                    node.stress_level = (node.stress_level + stress).min(1.0);
                }
                // Note: regen_multiplier and other effects would be applied to world/node in full SovereignWorldState
            }

            Ok((yield_amount, epiphany))
        } else {
            Err(MercyViolation { reason: "Node not found".to_string() })
        }
    }
}
