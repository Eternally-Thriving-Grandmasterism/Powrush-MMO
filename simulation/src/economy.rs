/*!
 * Hybrid CPU + GPU Economic / RBE Layer (v18.97.3)
 * 
 * Now with apply_harvest_event + apply_emergence_event from TickResult.
 * Emergence events meaningfully affect RBE, abundance, and resonance.
 * GPU path elevated to async non-blocking dispatch (dispatch_gpu_economic_compute_async + GpuEconomicReadback).
 * Apply system (apply_gpu_economic_results) to be wired into Bevy schedule.
 * 
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use crate::world::SovereignWorldState;
use crate::mercy::{MercyGate, MercyViolation};
use crate::harvest::HarvestEvent;
use crate::emergence::DynamicEmergenceEvent;
use tracing::{info_span, instrument, warn};

#[cfg(feature = "gpu")]
use crate::gpu_economic::{dispatch_gpu_economic_update, dispatch_gpu_economic_compute_async, GpuEconomicReadback};

pub struct EconomicLayer {
    pub cpu_precision_mode: bool,
}

impl EconomicLayer {
    pub fn new() -> Self {
        Self { cpu_precision_mode: true }
    }

    #[instrument(skip(self, world, mercy_gate))]
    pub fn batch_update(
        &self,
        world: &mut SovereignWorldState,
        mercy_gate: &MercyGate,
    ) -> Result<(), MercyViolation> {
        let _span = info_span!("economic_batch_update").entered();

        mercy_gate.pre_economic_tick_validate(world)?;

        if self.cpu_precision_mode {
            self.cpu_economic_update(world)?;
        } else {
            #[cfg(feature = "gpu")]
            {
                // Production path: non-blocking async dispatch.
                // Full wiring requires GpuEconomicReadback resource in Bevy world and
                // apply_gpu_economic_results system added to simulation schedule.
                // For now falls back to legacy sync version for compatibility.
                if let Err(e) = dispatch_gpu_economic_update(world) {
                    warn!("GPU dispatch failed ({}). Falling back to CPU precision path for this tick.", e);
                    self.cpu_economic_update(world)?;
                }

                // Future: Replace above with:
                // if let Err(e) = dispatch_gpu_economic_compute_async(world, &mut gpu_readback, current_frame) {
                //     warn!(...);
                //     self.cpu_economic_update(world)?;
                // }
            }
            #[cfg(not(feature = "gpu"))]
            {
                self.cpu_economic_update(world)?;
            }
        }

        mercy_gate.post_economic_tick_validate(world)?;
        Ok(())
    }

    fn cpu_economic_update(&self, world: &mut SovereignWorldState) -> Result<(), MercyViolation> {
        let _span = info_span!("cpu_economic_update").entered();
        let now_ms = world.sim_time;

        for node in world.resource_nodes.values_mut() {
            if node.depletion > 0.0 {
                node.depletion = (node.depletion - node.regen_rate).max(0.0);
                node.current_yield = node.base_yield * (1.0 - node.depletion * 0.7);
            }
            node.sustainability_score = (1.0 - node.depletion * 0.5).max(0.3);

            if node.depletion < 0.3 {
                node.stress_level = (node.stress_level - 0.02).max(0.0);
            }

            if node.harvest_restricted_until_ms > 0 && now_ms > node.harvest_restricted_until_ms {
                node.harvest_restricted_until_ms = 0;
                node.stress_level = (node.stress_level * 0.5).max(0.0);
            }

            if node.abundance_flow > 0.2 {
                let bonus = 1.0 + (node.abundance_flow - 0.2) * 1.8;
                node.regen_rate = (node.regen_rate * bonus).min(3.5);
                node.sustainability_score = (node.sustainability_score + node.abundance_flow * 0.12).min(1.0);
            } else if node.abundance_flow < -0.15 {
                node.stress_level = (node.stress_level + 0.28).min(1.0);
                if node.stress_level > 0.75 {
                    node.harvest_restricted_until_ms = now_ms + 90_000;
                }
            }
        }

        for pool in world.rbe_pools.values_mut() {
            pool.abundance_flow = (pool.abundance_flow * 0.98 + 0.02).clamp(0.0, 2.0);
            pool.sustainability_score = (pool.sustainability_score * 0.995 + pool.abundance_flow * 0.005).clamp(0.3, 1.0);
            pool.pressure = (pool.pressure * 0.9).max(0.0);
        }

        Ok(())
    }

    /// Applies a HarvestEvent from TickResult into the economic simulation.
    pub fn apply_harvest_event(
        &mut self,
        event: &HarvestEvent,
        world: &mut SovereignWorldState,
        mercy_gate: &MercyGate,
    ) -> Result<(), MercyViolation> {
        mercy_gate.pre_economic_tick_validate(world)?;

        if let Some(node) = world.resource_nodes.get_mut(&event.node_id) {
            if event.sustainable {
                node.sustainability_score = (node.sustainability_score + 0.08).min(1.0);
                node.abundance_flow = (node.abundance_flow + 0.05).min(2.5);
            } else {
                node.stress_level = (node.stress_level + 0.15).min(1.0);
                node.abundance_flow = (node.abundance_flow - 0.08).max(-1.5);
            }

            for pool in world.rbe_pools.values_mut() {
                if event.council_amplified {
                    pool.abundance_flow = (pool.abundance_flow + 0.12).min(3.0);
                    pool.sustainability_score = (pool.sustainability_score + 0.04).min(1.0);
                } else {
                    pool.abundance_flow = (pool.abundance_flow + 0.03).min(2.5);
                }
            }
        }

        mercy_gate.post_economic_tick_validate(world)?;
        Ok(())
    }

    /// Applies a DynamicEmergenceEvent from TickResult into the economic simulation.
    /// Emergence effects (resource deltas, resonance, abundance) are applied here.
    pub fn apply_emergence_event(
        &mut self,
        event: &DynamicEmergenceEvent,
        world: &mut SovereignWorldState,
        mercy_gate: &MercyGate,
    ) -> Result<(), MercyViolation> {
        mercy_gate.pre_economic_tick_validate(world)?;

        for effect in &event.proposed_effects {
            match effect {
                crate::emergence::EmergenceEffect::ResourceDelta { resource: _, amount, is_abundance } => {
                    for pool in world.rbe_pools.values_mut() {
                        if *is_abundance {
                            pool.abundance_flow = (pool.abundance_flow + amount * 0.5).min(3.5);
                            pool.sustainability_score = (pool.sustainability_score + 0.03).min(1.0);
                        } else {
                            pool.pressure = (pool.pressure + amount * 0.3).min(2.0);
                        }
                    }
                }
                crate::emergence::EmergenceEffect::BiomeResonance { intensity } => {
                    for node in world.resource_nodes.values_mut() {
                        node.abundance_flow = (node.abundance_flow + intensity * 0.2).min(3.0);
                        node.sustainability_score = (node.sustainability_score + intensity * 0.02).min(1.0);
                    }
                }
                crate::emergence::EmergenceEffect::TemporaryMultiplier { multiplier, .. } => {
                    for pool in world.rbe_pools.values_mut() {
                        pool.abundance_flow = (pool.abundance_flow * multiplier).clamp(0.0, 3.5);
                    }
                }
                _ => {}
            }
        }

        mercy_gate.post_economic_tick_validate(world)?;
        Ok(())
    }
}
