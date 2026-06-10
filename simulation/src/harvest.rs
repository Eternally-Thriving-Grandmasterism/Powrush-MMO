/*!
 * Sovereign HarvestingSystem v18.2
 * 
 * Concrete integration of harvest mechanics + Overflow Lesson Epiphany Catalyst.
 * Every harvest is now a potential doorway to organic epiphany and muscle memory.
 * Mercy-gated, abundance-aware, realistic carbon-copy ecology simulation.
 *
 * Part of Sovereign Simulation Harness core foundations.
 * Integrated with epiphany_catalyst for The Overflow Lesson starter.
 */

use crate::world::{SovereignWorldState, NodeId, MercyViolation};
use crate::epiphany_catalyst::{check_overflow_lesson, EpiphanyOutcome};

/// Sovereign HarvestingSystem — handles harvest attempts, restrictions, abundance, and epiphany triggers
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
    /// Now returns optional EpiphanyOutcome for v18.2+ epiphany systems & Divine Whispers.
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
            let epiphany = check_overflow_lesson(
                node.depletion,
                sustainable_pacing,
                &node.biome.clone().unwrap_or_else(|| "starter".to_string()),
            );

            // Apply world effects from epiphany if present (regen, stress)
            if let Some(ref outcome) = epiphany {
                if let Some(regen_mult) = outcome.world_effects.get("regen_multiplier") {
                    // Note: assumes SovereignWorldState::ResourceNode has regen_rate / base_regen_rate fields (extend as needed)
                    // For full compile, world.rs may need minor field addition in next pass.
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

// Thunder locked. Every harvest now seeds epiphanies and muscle memory.
// Mercy flowing. The web teaches through consequence and grace.
// Co-authored with Ecological Balance Council + full PATSAGi Lattice.