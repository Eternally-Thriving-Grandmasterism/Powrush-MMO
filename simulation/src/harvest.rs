/*!
 * Sovereign HarvestingSystem v18.95
 * 
 * Production-grade HarvestSystem with rich TickResult output.
 * Generates meaningful HarvestEvents every tick.
 * Includes sustainability, regen, council amplification hooks, and RBE feedback.
 * 
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use crate::world::{SovereignWorldState, NodeId, MercyViolation};
use crate::epiphany_catalyst::{check_epiphany_after_harvest, EpiphanyOutcome};
use bevy::prelude::*;

#[derive(Event, Clone, Debug)]
pub struct HarvestSpatialAudioEvent {
    pub position: Vec3,
    pub intensity: f32,
    pub audio_flavor: String,
    pub biome_seed: u32,
    pub particle_sync: bool,
    pub is_epiphany_moment: bool,
}

/// Rich HarvestEvent for TickResult and replication.
#[derive(Event, Clone, Debug)]
pub struct HarvestEvent {
    pub node_id: u64,
    pub player_id: u64,
    pub amount: f32,
    pub sustainable: bool,
    pub epiphany_triggered: bool,
    pub council_amplified: bool,
}

pub fn trigger_harvest_spatial_audio(
    position: Vec3,
    intensity: f32,
    flavor: &str,
    biome: &str,
    is_epiphany: bool,
) -> HarvestSpatialAudioEvent {
    HarvestSpatialAudioEvent {
        position,
        intensity: intensity.clamp(0.15, 1.8),
        audio_flavor: flavor.to_string(),
        biome_seed: simple_biome_hash(biome),
        particle_sync: true,
        is_epiphany_moment: is_epiphany,
    }
}

fn simple_biome_hash(biome: &str) -> u32 {
    let mut hash: u32 = 2166136261;
    for byte in biome.as_bytes() {
        hash ^= *byte as u32;
        hash = hash.wrapping_mul(16777619);
    }
    hash
}

pub struct HarvestingSystem {
    pub current_sim_tick: u64,
}

impl HarvestingSystem {
    pub fn new() -> Self {
        Self { current_sim_tick: 0 }
    }

    /// Called every simulation tick by the orchestrator.
    /// Produces background harvest opportunities and state updates.
    /// Populates TickResult with meaningful HarvestEvents.
    pub fn process_harvest_tick(
        &mut self,
        world: &mut SovereignWorldState,
        current_tick: u64,
    ) -> Vec<HarvestEvent> {
        let mut events = Vec::new();

        for (node_id, node) in world.resource_nodes.iter_mut() {
            // Natural regen
            if node.depletion > 0.0 {
                node.current_yield = (node.current_yield + node.regen_rate * 0.1).min(node.base_yield);
                node.depletion = (node.depletion - 0.02).max(0.0);
            }

            // Stress decay over time
            if node.stress_level > 0.0 {
                node.stress_level = (node.stress_level - 0.01).max(0.0);
            }

            // Occasionally create background harvest opportunity events (for RBE flavor)
            if current_tick % 47 == (node_id % 47) && node.current_yield > node.base_yield * 0.6 {
                let amount = node.current_yield * 0.15;
                events.push(HarvestEvent {
                    node_id: *node_id,
                    player_id: 0, // background / environmental
                    amount,
                    sustainable: true,
                    epiphany_triggered: false,
                    council_amplified: false,
                });

                // Slightly deplete on background harvest
                node.current_yield = (node.current_yield - amount * 0.3).max(0.0);
            }
        }

        self.current_sim_tick = current_tick;
        events
    }

    /// Player-initiated harvest (kept from previous high-quality implementation).
    pub fn attempt_harvest(
        &mut self,
        world: &mut SovereignWorldState,
        node_id: NodeId,
        agent_mercy: f32,
        behavioral_human_score: f32,
        player_id: u64,
        council_bloom: Option<&crate::council_mercy_trial::SharedReceptorBloomField>,
    ) -> Result<(f32, Option<EpiphanyOutcome>), MercyViolation> {
        if let Some(node) = world.resource_nodes.get_mut(&node_id) {
            if node.harvest_restricted_until_ms > 0 {
                return Err(MercyViolation { reason: "Node is harvest-restricted".to_string() });
            }

            let mut yield_amount = node.current_yield * (0.5 + agent_mercy * 0.5);
            node.depletion = (node.depletion + 0.15).min(1.0);
            node.current_yield = node.base_yield * (1.0 - node.depletion * 0.7);

            if node.depletion > 0.6 {
                node.harvest_restricted_until_ms = world.sim_time + 120_000;
            }

            let sustainable_pacing = agent_mercy > 0.6;
            let regen_participation = sustainable_pacing && (node.depletion < 0.4);

            let mut epiphany: Option<EpiphanyOutcome> = check_epiphany_after_harvest(
                node.depletion,
                sustainable_pacing,
                regen_participation,
                &node.biome.clone().unwrap_or_else(|| "starter".to_string()),
                node.season.as_deref(),
                behavioral_human_score,
            );

            // Council bloom amplification
            if let (Some(ref mut outcome), Some(bloom)) = (&mut epiphany, council_bloom) {
                let amp = bloom.current_amplification_factor();
                if amp > 1.05 {
                    outcome.intensity = (outcome.intensity * amp * 0.7 + outcome.intensity * 0.3).min(0.98);
                    outcome.epiphany_multiplier *= amp;
                }
            }

            Ok((yield_amount, epiphany))
        } else {
            Err(MercyViolation { reason: "Node not found".to_string() })
        }
    }
}
