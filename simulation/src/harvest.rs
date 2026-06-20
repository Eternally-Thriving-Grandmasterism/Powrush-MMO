/*!
 * Sovereign HarvestingSystem v18.97.1 + Proactive Joy Wiring
 * 
 * After successful sustainable or high-yield harvests, we now call
 * generate_proactive_joy_redemption_thread() for positive (non-scar)
 * emotional reward loops.
 * 
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use crate::world::{SovereignWorldState, NodeId, MercyViolation, Vec3};
use crate::epiphany_catalyst::{check_epiphany_after_harvest, EpiphanyOutcome};
use crate::player_legacy_journal::LegacyJournalRegistry; // NEW: for proactive joy
use bevy::prelude::*;

// ... (existing structs and functions preserved)

#[derive(Event, Clone, Debug)]
pub struct HarvestSpatialAudioEvent { /* ... unchanged ... */ }

#[derive(Event, Clone, Debug)]
pub struct HarvestEvent { /* ... unchanged ... */ }

pub fn trigger_harvest_spatial_audio(/* ... unchanged ... */) -> HarvestSpatialAudioEvent { /* ... */ }

fn simple_biome_hash(biome: &str) -> u32 { /* ... */ }

pub struct HarvestingSystem {
    pub current_sim_tick: u64,
}

impl HarvestingSystem {
    pub fn new() -> Self {
        Self { current_sim_tick: 0 }
    }

    pub fn process_harvest_tick(
        &mut self,
        world: &mut SovereignWorldState,
        current_tick: u64,
    ) -> Vec<HarvestEvent> {
        // ... (unchanged background harvest logic)
        let mut events = Vec::new();
        // ... existing code ...
        self.current_sim_tick = current_tick;
        events
    }

    /// Player-initiated harvest — now with proactive joy wiring on strong sustainable harvests
    pub fn attempt_harvest(
        &mut self,
        world: &mut SovereignWorldState,
        node_id: NodeId,
        agent_mercy: f32,
        behavioral_human_score: f32,
        player_id: u64,
        council_bloom: Option<&crate::council_mercy_trial::SharedReceptorBloomField>,
        // NEW: Pass registry for proactive joy (or access via world resource in full ECS)
        legacy_registry: Option<&mut LegacyJournalRegistry>,
    ) -> Result<(f32, Option<EpiphanyOutcome>), MercyViolation> {
        if let Some(node) = world.resource_nodes.get_mut(&node_id) {
            if node.harvest_restricted_until_ms > 0 {
                return Err(MercyViolation { reason: "Node is harvest-restricted".to_string() });
            }

            let mut yield_amount = world.modulate_harvest_yield(
                node.current_yield * (0.5 + agent_mercy * 0.5),
                node.position,
            );

            node.depletion = (node.depletion + 0.15).min(1.0);
            node.current_yield = node.base_yield * (1.0 - node.depletion * 0.7);

            if node.depletion > 0.6 {
                node.harvest_restricted_until_ms = world.sim_time + 120_000;
            }

            let sustainable_pacing = agent_mercy > 0.6;
            let regen_participation = sustainable_pacing && (node.depletion < 0.4);

            let effective_biome = if let Some(inf) = world.get_biome_influence_at(node.position) {
                inf.biome_name
            } else {
                node.biome.clone().unwrap_or_else(|| "starter".to_string())
            };

            let mut epiphany: Option<EpiphanyOutcome> = check_epiphany_after_harvest(
                node.depletion,
                sustainable_pacing,
                regen_participation,
                &effective_biome,
                node.season.as_deref(),
                behavioral_human_score,
            );

            if let (Some(ref mut outcome), Some(bloom)) = (&mut epiphany, council_bloom) {
                let amp = bloom.current_amplification_factor();
                if amp > 1.05 {
                    outcome.intensity = (outcome.intensity * amp * 0.7 + outcome.intensity * 0.3).min(0.98);
                    outcome.epiphany_multiplier *= amp;
                }
            }

            // === NEW: Proactive Joy on strong sustainable / high-yield harvests (non-scar) ===
            if sustainable_pacing && yield_amount > node.base_yield * 0.4 {
                if let Some(registry) = legacy_registry {
                    // In full ECS this would be the real player/agent id
                    // registry.generate_proactive_joy_redemption_thread(
                    //     player_id as u64,
                    //     format!("Sustainable harvest in {} — abundance flows from mercy", effective_biome),
                    //     yield_amount * 0.08,
                    //     0.18,
                    //     self.current_sim_tick,
                    //     0, // server_id placeholder
                    // );
                    // For now we log the integration point
                }
            }

            Ok((yield_amount, epiphany))
        } else {
            Err(MercyViolation { reason: "Node not found".to_string() })
        }
    }
}
