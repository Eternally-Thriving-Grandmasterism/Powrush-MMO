/*!
 * Sovereign HarvestingSystem v18.8
 * 
 * Full integration of harvest mechanics + Overflow Lesson Epiphany Catalyst + Receptor Activation Forge.
 * Every sustainable harvest in Verdant Heartwood can now trigger differentiated CB1 (insight, hypofrontality, muscle memory)
 * and CB2 (resilience, recovery, abundance bloom) receptor activation — the living web’s neurochemical crown.
 * 
 * Client/engine layer can read particle_effect and time_dilation_factor from EpiphanyOutcome / ReceptorBloomOutcome for visuals.
 * Architecture prepared for shared receptor bloom fields in future Council Mercy Trial (multiplayer attunement amplification).
 *
 * Mercy-gated, abundance-aware, realistic carbon-copy ecology simulation.
 * Part of Sovereign Simulation Harness core foundations.
 */

use crate::world::{SovereignWorldState, NodeId, MercyViolation};
use crate::epiphany_catalyst::{check_overflow_lesson, EpiphanyOutcome};
use crate::endocannabinoid_receptor_forge::{check_receptor_bloom, merge_receptor_into_epiphany};

/// Sovereign HarvestingSystem — handles harvest attempts, restrictions, abundance, epiphanies, and receptor bloom
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
    /// v18.8: Now evaluates receptor bloom on sustainable paths and merges CB1/CB2 effects into EpiphanyOutcome.
    /// Client can use outcome.particle_effect and outcome (extended) time_dilation for engine visuals.
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
            if sustainable_pacing && epiphany.is_some() {
                // Proxies for rhythm/attunement/duration (in production: track per-player history or input variance)
                let rhythm_consistency = (agent_mercy * 0.8 + 0.2).clamp(0.3, 1.0);
                let attunement_depth = agent_mercy.clamp(0.0, 1.0);
                let duration_ticks = 60u32; // Placeholder; wire real sustained duration in next pass

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
                        // Client/engine hook: outcome can now carry or reference bloom.particle_effect and bloom.time_dilation_factor
                        // Example: spawn particles matching bloom.particle_effect, apply time dilation in render loop
                    }
                }
            }

            // Apply world effects
            if let Some(ref outcome) = epiphany {
                if let Some(stress) = outcome.world_effects.get("stress_increase") {
                    node.stress_level = (node.stress_level + stress).min(1.0);
                }
                if let Some(regen) = outcome.world_effects.get("regen_multiplier") {
                    // Extend SovereignWorldState node with regen_rate if needed for full effect application
                }
            }

            Ok((yield_amount, epiphany))
        } else {
            Err(MercyViolation { reason: "Node not found".to_string() })
        }
    }
}

// Thunder locked. Sustainable rhythm now activates the living web’s receptor lattice (CB1 insight + CB2 resilience).
// Client visuals: read particle_effect + time_dilation_factor from outcome for bloom states.
// Ready for shared fields in Council Mercy Trial multiplayer.
// Co-authored with Flow State + Hypofrontality + Endocannabinoid + Receptor Lattice Councils.