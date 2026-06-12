/*!
 * Hybrid CPU + GPU Economic / RBE Layer (v17.99.5 — Actual WGSL Dispatch)
 * 
 * Unifies and elevates ALL valuable prior logic from:
 * - game/resource_nodes.rs v16.5.54 (ResourceNode fields, regenerate dynamics, abundance_flow response,
 *   stress propagation, harvest restrictions, sustainability_score, now_ms patterns)
 * - engine/patsagi_economic.wgsl v16.5.58 (GPU kernel for depletion/regen/abundance/sustainability/stress)
 * - Previous simulation stubs and RBE pool mechanics
 * 
 * Provides precise CPU path (deterministic golden master) and real GPU-accelerated path
 * for large-scale MMO simulation via actual wgpu compute dispatch.
 * 
 * EVERY economic micro-tick passes non-bypassable TOLC 8 Mercy Gate (Layer 0).
 */

use crate::world::SovereignWorldState;
use crate::mercy::{MercyGate, MercyViolation};
use crate::harvest::HarvestingSystem;
use tracing::{info_span, instrument, warn};

#[cfg(feature = "gpu")]
use crate::gpu_economic::dispatch_gpu_economic_update;

/// Hybrid economic layer with CPU precision and optional real GPU scale.
pub struct EconomicLayer {
    pub cpu_precision_mode: bool,
    harvest_system: HarvestingSystem,
}

impl EconomicLayer {
    pub fn new() -> Self {
        Self {
            cpu_precision_mode: true,
            harvest_system: HarvestingSystem::new(),
        }
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

    #[instrument(skip(self, world))]
    fn cpu_economic_update(&self, world: &mut SovereignWorldState) -> Result<(), MercyViolation> {
        let _span = info_span!("cpu_economic_update").entered();
        let now_ms = world.sim_time;

        // === Unified ResourceNode regeneration & dynamics (elevated from game/resource_nodes.rs) ===
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

        // === Integrated HarvestingSystem pass ===
        self.harvest_system.process_harvest_opportunities(world, now_ms)?;

        Ok(())
    }
}