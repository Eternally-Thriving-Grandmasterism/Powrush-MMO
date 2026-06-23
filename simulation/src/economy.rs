/*!
 * Hybrid CPU + GPU Economic / RBE Layer (v18.97.5 + v19.2.9 Sustainability + Council Policy Integration)
 * 
 * RBE Council Policy Integration implemented:
 * - Council attunement and bloom outcomes now directly influence RBE pools, nodes, abundance, pressure, and sustainability.
 * - High mercy/attunement = economic blessing (abundance boost, pressure relief, sustainability gain).
 * - Low attunement or failed council = economic friction (pressure increase, reduced abundance/regeneration).
 * - Designed to be called from orchestrator TickResult or council resolution systems.
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
                if let Err(e) = dispatch_gpu_economic_update(world) {
                    warn!("GPU dispatch failed ({}). Falling back to CPU precision path for this tick.", e);
                    self.cpu_economic_update(world)?;
                }
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

        self.apply_rbe_sustainability_tick(world);

        Ok(())
    }

    /// v19.2.9: Core RBE Sustainability Depth system.
    pub fn apply_rbe_sustainability_tick(&self, world: &mut SovereignWorldState) {
        for node in world.resource_nodes.values_mut() {
            if node.depletion > 0.4 || node.stress_level > 0.5 {
                node.pressure = (node.pressure + 0.015).min(5.0);
            } else if node.depletion < 0.15 && node.stress_level < 0.3 {
                node.pressure = (node.pressure - 0.008).max(0.0);
            }

            if node.pressure > 1.5 {
                let decay = 0.002 * (node.pressure - 1.0);
                node.sustainability_score = (node.sustainability_score - decay).max(0.1);
            }

            if node.pressure > 3.0 {
                node.regen_rate = (node.regen_rate * 0.985).max(0.3);
                if node.pressure > 4.0 && node.harvest_restricted_until_ms == 0 {
                    node.harvest_restricted_until_ms = world.sim_time + 60_000;
                }
            }
        }

        for pool in world.rbe_pools.values_mut() {
            if pool.pressure > 2.0 || pool.sustainability_score < 0.4 {
                pool.pressure = (pool.pressure + 0.01).min(8.0);
            } else {
                pool.pressure = (pool.pressure - 0.005).max(0.0);
            }

            if pool.pressure > 3.5 {
                let decay = 0.0015 * (pool.pressure - 2.0);
                pool.sustainability_score = (pool.sustainability_score - decay).max(0.15);
            }

            if pool.pressure > 4.5 {
                pool.abundance_flow = (pool.abundance_flow * 0.97).max(-1.0);
            }
        }
    }

    /// RBE Council Policy Integration (v19.2.9)
    /// Applies real economic consequences from Council Mercy Trial outcomes.
    /// High collective attunement and successful blooms = RBE blessing.
    /// Low attunement or failed seals = economic friction and pressure.
    /// Call this from orchestrator TickResult or council resolution when a council session ends.
    pub fn apply_council_policy_impact(
        &mut self,
        collective_attunement: f32,
        bloom_success: bool,
        participant_count: u8,
        world: &mut SovereignWorldState,
    ) {
        let mercy_factor = collective_attunement.clamp(0.0, 1.0);
        let is_strong_council = bloom_success && mercy_factor > 0.65 && participant_count >= 3;

        for pool in world.rbe_pools.values_mut() {
            if is_strong_council {
                // Council blessing: abundance flows more freely, pressure relieved, sustainability improved
                pool.abundance_flow = (pool.abundance_flow + mercy_factor * 0.8).min(4.0);
                pool.sustainability_score = (pool.sustainability_score + mercy_factor * 0.06).min(1.0);
                pool.pressure = (pool.pressure - mercy_factor * 1.2).max(0.0);
            } else if mercy_factor < 0.4 {
                // Weak or failed council: economic friction increases
                pool.pressure = (pool.pressure + (1.0 - mercy_factor) * 0.9).min(8.0);
                pool.abundance_flow = (pool.abundance_flow - (1.0 - mercy_factor) * 0.35).max(-2.0);
                pool.sustainability_score = (pool.sustainability_score - 0.015).max(0.1);
            } else {
                // Moderate council: mild positive effect
                pool.abundance_flow = (pool.abundance_flow + mercy_factor * 0.25).min(3.0);
                pool.pressure = (pool.pressure - mercy_factor * 0.4).max(0.0);
            }
        }

        for node in world.resource_nodes.values_mut() {
            if is_strong_council {
                node.abundance_flow = (node.abundance_flow + mercy_factor * 0.6).min(3.5);
                node.sustainability_score = (node.sustainability_score + mercy_factor * 0.05).min(1.0);
                node.pressure = (node.pressure - mercy_factor * 0.8).max(0.0);
                node.regen_rate = (node.regen_rate * (1.0 + mercy_factor * 0.3)).min(4.0);
            } else if mercy_factor < 0.4 {
                node.pressure = (node.pressure + (1.0 - mercy_factor) * 0.7).min(5.0);
                node.abundance_flow = (node.abundance_flow - (1.0 - mercy_factor) * 0.25).max(-1.5);
            }
        }
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
                node.pressure = (node.pressure - 0.3).max(0.0);
            } else {
                node.stress_level = (node.stress_level + 0.15).min(1.0);
                node.abundance_flow = (node.abundance_flow - 0.08).max(-1.5);
                node.pressure = (node.pressure + 0.6).min(5.0);
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
        Ok(());
    }

    /// Applies a DynamicEmergenceEvent from TickResult into the economic simulation.
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
                            pool.pressure = (pool.pressure - 0.2).max(0.0);
                        } else {
                            pool.pressure = (pool.pressure + amount * 0.3).min(2.0);
                        }
                    }
                }
                crate::emergence::EmergenceEffect::BiomeResonance { intensity } => {
                    for node in world.resource_nodes.values_mut() {
                        node.abundance_flow = (node.abundance_flow + intensity * 0.2).min(3.0);
                        node.sustainability_score = (node.sustainability_score + intensity * 0.02).min(1.0);
                        node.pressure = (node.pressure - intensity * 0.15).max(0.0);
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
        Ok(());
    }
}
