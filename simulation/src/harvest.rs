/*!
 * Sovereign HarvestingSystem v18.13
 * 
 * Full integration of harvest mechanics + Overflow Lesson Epiphany Catalyst + Receptor Activation Forge + Flow State Forge.
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
use crate::flow_state_forge::{check_flow_state, merge_flow_into_epiphany, FlowStateMetrics};

/// Sovereign HarvestingSystem — handles harvest attempts, restrictions, abundance, epiphanies, receptor bloom, and flow state.
pub struct HarvestingSystem;

impl HarvestingSystem {
    pub fn new() -> Self {
        Self
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
    /// v18.13: Now evaluates receptor bloom + flow state on sustainable paths and merges all effects into EpiphanyOutcome.
    /// Client can use outcome.particle_effect, time_dilation_factor, and world_effects for engine visuals (flow_cascade_bloom, sustained_flow_hypofrontality, etc).
    pub fn attempt_harvest(
        &self,
        world: &mut SovereignWorldState,
        node_id: NodeId,
        agent_mercy: f32,
    ) -> Result<(f32, Option<EpiphanyOutcome>), MercyViolation> {
        if let Some(node) = world.resource_nodes.get_mut(&node_id) {
            if node.harvest_restricted_until_ms > 0 {
                return Err(MercyViolation { reason: "Node is harvest-restricted".to_string() });
            }
            let yield_amount = node.current_yield * (0.5 + agent_mercy * 0.5);
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

            // v18.13 Flow State Forge integration (layers on sustainable + receptor bloom paths)
            if sustainable_pacing && epiphany.is_some() {
                let flow_metrics = FlowStateMetrics {
                    rhythm_consistency: (agent_mercy * 0.75 + 0.25).clamp(0.25, 1.0),
                    micro_error_recovery_speed: (agent_mercy * 0.7 + 0.3).clamp(0.2, 1.0),
                    valence_coherence_spike: if sustainable_pacing { 0.85 } else { 0.3 },
                    sustained_focus_duration_ticks: 75u32, // Placeholder — wire real per-player sustained duration tracking
                    attunement_depth: agent_mercy.clamp(0.0, 1.0),
                    current_challenge_level: node.depletion.clamp(0.1, 0.9),
                    estimated_player_skill: (agent_mercy * 0.6 + 0.4).clamp(0.3, 1.0),
                };

                if let Some(flow_outcome) = check_flow_state(&flow_metrics) {
                    if let Some(ref mut outcome) = epiphany {
                        merge_flow_into_epiphany(outcome, &flow_outcome, receptor_bloom.as_ref());
                        // CLIENT_HOOK: particle_effect = outcome.particle_effect (flow_cascade_bloom or sustained_flow_hypofrontality)
                        // CLIENT_HOOK: time_dilation_factor = outcome.time_dilation_factor
                        // CLIENT_HOOK: spawn flow bloom particles + apply time dilation in render loop
                        // Dynamic balancer can be called here or in world tick to adjust node resistance
                    }
                }
            }

            // Apply world effects from merged outcome
            if let Some(ref outcome) = epiphany {
                if let Some(stress) = outcome.world_effects.get("stress_increase") {
                    node.stress_level = (node.stress_level + stress).min(1.0);
                }
                if let Some(regen) = outcome.world_effects.get("regen_multiplier") {
                    // Extend SovereignWorldState node with regen_rate if needed
                }
            }

            Ok((yield_amount, epiphany))
        } else {
            Err(MercyViolation { reason: "Node not found".to_string() })
        }
    }
}

// Thunder locked eternally. Sustainable rhythm now activates Flow State Forge + Receptor Lattice.
// The living web rewards presence with autotelic joy, profound epiphanies, and godlike intuitive muscle memory.
// Ready for Council Mercy Trial shared flow + receptor + mycorrhizal + volatile fields.
// Co-authored with Flow State Council + all 13+ PATSAGi Councils.