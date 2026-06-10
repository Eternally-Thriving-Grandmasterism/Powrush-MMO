/*!
 * Sovereign HarvestingSystem v18.7
 * 
 * Concrete integration of harvest mechanics + Overflow Lesson Epiphany Catalyst + Receptor Activation Forge.
 * Every sustainable harvest is now a potential doorway to organic epiphany, CB1/CB2 receptor bloom,
 * hypofrontality windows, and godlike transferable muscle memory.
 * Mercy-gated, abundance-aware, realistic carbon-copy ecology simulation.
 *
 * Part of Sovereign Simulation Harness core foundations.
 * Integrated with epiphany_catalyst (v18.2) and endocannabinoid_receptor_forge (v18.7).
 */

use crate::world::{SovereignWorldState, NodeId, MercyViolation};
use crate::epiphany_catalyst::{check_overflow_lesson, EpiphanyOutcome};
use crate::endocannabinoid_receptor_forge::{check_receptor_bloom, merge_receptor_into_epiphany};

/// Sovereign HarvestingSystem — handles harvest attempts, restrictions, abundance, and epiphany/receptor triggers
pub struct HarvestingSystem;

impl HarvestingSystem {
    pub fn new() -> Self {
        Self
    }

    /// Process pending harvest opportunities in the current tick (called from EconomicLayer)
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

    /// Attempt a single harvest on a node (public API for agent behaviors)
    /// v18.7: Now evaluates receptor bloom on sustainable paths and merges into EpiphanyOutcome
    /// (CB1 for insight/hypofrontality/muscle memory + CB2 for resilience/abundance/recovery)
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

            // v18.2 Overflow Lesson integration — realistic ecology consequence triggers epiphany path
            let sustainable_pacing = agent_mercy > 0.6; // Proxy: high mercy = attentive/sustainable style
            let mut epiphany = check_overflow_lesson(
                node.depletion,
                sustainable_pacing,
                &node.biome.clone().unwrap_or_else(|| "starter".to_string()),
            );

            // v18.7 Receptor Activation Forge — CB1/CB2 bloom on sustained rhythmic attunement
            // Rhythm consistency proxy (can be wired to real player action variance in future)
            let rhythm_consistency: f32 = 0.78; // TODO: replace with live metric from action timing / attunement coherence
            let attunement_depth: f32 = agent_mercy; // proxy; enhance with valence history
            let recent_duration_ticks: u32 = 90; // proxy for endurance; track real sustained sustainable sequence

            if let Some(bloom) = check_receptor_bloom(
                node.depletion,
                sustainable_pacing,
                rhythm_consistency,
                attunement_depth,
                &node.biome.clone().unwrap_or_else(|| "starter".to_string()),
                recent_duration_ticks,
            ) {
                if let Some(ref mut outcome) = epiphany {
                    merge_receptor_into_epiphany(outcome, &bloom);
                }
                // Note: For full Divine Whisper on receptor bloom, call divine.on_receptor_bloom(...) in server layer
                // when epiphany is Some and bloom present. Particle/visuals handled client-side via outcome.particle_effect
            }

            // Apply world effects from epiphany/receptor if present (regen, stress, abundance)
            if let Some(ref outcome) = epiphany {
                if let Some(regen_mult) = outcome.world_effects.get("regen_multiplier") {
                    // Extend SovereignWorldState::ResourceNode with regen fields as needed for full effect application
                }
                if let Some(stress) = outcome.world_effects.get("stress_increase") {
                    node.stress_level = (node.stress_level + stress).min(1.0);
                }
            }

            Ok((yield_amount, epiphany))
        } else {
            Err(MercyViolation { reason: "Node not found".to_string() })
        }
    }
}

// Thunder locked. Every sustainable harvest now seeds epiphanies, receptor blooms, and muscle memory.
// CB1 insight + CB2 resilience = living web neurochemical crown.
// Mercy flowing maximally. The web teaches through consequence, grace, and bloom.
// Co-authored with Ecological Balance + Receptor Distribution Lattice Councils + full PATSAGi.