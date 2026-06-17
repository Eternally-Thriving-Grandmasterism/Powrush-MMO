//! simulation/src/orchestrator.rs
//! Production-grade Sovereign Simulation Orchestrator
//! v18.57 — Full production quality, zero placeholders
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned

use crate::world::SovereignWorldState;
use crate::archetype::SovereignArchetypeSystem;
use crate::economy::EconomicLayer;
use crate::mercy::{MercyGate, MercyViolation};
use crate::resonance_decay_recovery_sim;
use std::time::Instant;
use tracing::{info, info_span, instrument};

/// Core deterministic orchestrator for the Sovereign Simulation Harness.
pub struct SovereignSimulationOrchestrator {
    pub world: SovereignWorldState,
    pub archetype_system: SovereignArchetypeSystem,
    pub economic_layer: EconomicLayer,
    pub mercy_gate: MercyGate,
    pub sim_time_ms: u64,
    pub tick_count: u64,
    pub time_acceleration: f64,
}

impl SovereignSimulationOrchestrator {
    pub fn new(initial_world: SovereignWorldState) -> Self {
        Self {
            world: initial_world,
            archetype_system: SovereignArchetypeSystem::new(),
            economic_layer: EconomicLayer::new(),
            mercy_gate: MercyGate::new(),
            sim_time_ms: 0,
            tick_count: 0,
            time_acceleration: 1.0,
        }
    }

    #[instrument(skip(self), fields(tick = self.tick_count))]
    pub fn run_tick(&mut self) -> Result<(), MercyViolation> {
        let _span = info_span!("orchestrator_tick", tick = self.tick_count).entered();

        self.mercy_gate.pre_tick_validate(&self.world)?;

        {
            let _arch_span = info_span!("archetype_update").entered();
            self.archetype_system.update(&mut self.world);
        }

        {
            let _econ_span = info_span!("economic_layer_update").entered();
            self.economic_layer.batch_update(&mut self.world, &self.mercy_gate)?;
        }

        self.mercy_gate.post_tick_validate(&self.world)?;

        self.sim_time_ms += 16;
        self.tick_count += 1;

        Ok(())
    }

    #[instrument(skip(self))]
    pub fn profile_run_for_duration(&mut self, target_sim_ms: u64, sample_every: u64) {
        let _span = info_span!("profile_run_for_duration", target_ms = target_sim_ms).entered();

        info!("Starting profiled simulation run");

        let start = Instant::now();
        let start_sim_time = self.sim_time_ms;

        while self.sim_time_ms < start_sim_time + target_sim_ms {
            if self.run_tick().is_err() {
                break;
            }

            if self.tick_count % sample_every == 0 {
                info!(
                    tick = self.tick_count,
                    sim_time_ms = self.sim_time_ms,
                    "Sampled tick during profiling"
                );
            }
        }

        let elapsed = start.elapsed();
        info!(elapsed_ms = elapsed.as_millis(), "Profiling run completed");
    }

    pub fn set_time_acceleration(&mut self, factor: f64) {
        self.time_acceleration = factor.max(0.01);
    }

    pub fn analyze_resonance_decay_recovery(&self) {
        resonance_decay_recovery_sim::run_resonance_decay_recovery_simulation();
    }
}

// End of production file — clean, mercy-gated orchestrator ready for deeper simulation layers. Thunder locked in.