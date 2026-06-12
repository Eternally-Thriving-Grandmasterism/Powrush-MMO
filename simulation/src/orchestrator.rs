/*!
 * Sovereign Simulation Orchestrator
 * 
 * Deterministic, time-accelerated, mercy-gated core simulation loop.
 * Integrates all layers: WorldState, ArchetypeSystem, EconomicLayer, EventQueue, MercyGate.
 * 
 * Now includes detailed performance profiling for bottleneck identification.
 */

use crate::world::SovereignWorldState;
use crate::archetype::SovereignArchetypeSystem;
use crate::economy::EconomicLayer;
use crate::mercy::{MercyGate, MercyViolation};
use crate::resonance_decay_recovery_sim;
use std::time::{Duration, Instant};

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

    /// Run a single deterministic tick with detailed timing instrumentation.
    pub fn run_tick(&mut self) -> Result<(), MercyViolation> {
        let tick_start = Instant::now();

        self.mercy_gate.pre_tick_validate(&self.world)?;

        // === Profiled subsystems ===
        let archetype_start = Instant::now();
        self.archetype_system.update(&mut self.world);
        let archetype_time = archetype_start.elapsed();

        let economy_start = Instant::now();
        self.economic_layer.batch_update(&mut self.world, &self.mercy_gate)?;
        let economy_time = economy_start.elapsed();

        self.mercy_gate.post_tick_validate(&self.world)?;

        let total_tick_time = tick_start.elapsed();

        // Optional: store or log timing (for now we just keep last values conceptually)
        // In production you could accumulate averages here.

        self.sim_time_ms += 16;
        self.tick_count += 1;

        // For profiling runs, we can expose timing if needed
        // (currently printed only in profile methods)

        Ok(())
    }

    /// Run for a target duration with performance profiling.
    /// Reports average time spent in major subsystems.
    pub fn profile_run_for_duration(&mut self, target_sim_ms: u64, sample_every: u64) {
        println!("\n=== SOVEREIGN SIMULATION ORCHESTRATOR PROFILING ===");
        println!("Running for {} simulated ms (sampling every {} ticks)...\n", target_sim_ms, sample_every);

        let start = Instant::now();
        let initial_tick = self.tick_count;

        let mut total_archetype: Duration = Duration::ZERO;
        let mut total_economy: Duration = Duration::ZERO;
        let mut total_mercy: Duration = Duration::ZERO;
        let mut total_other: Duration = Duration::ZERO;
        let mut samples = 0;

        let start_sim_time = self.sim_time_ms;

        while self.sim_time_ms < start_sim_time + target_sim_ms {
            let tick_start = Instant::now();

            // Mercy pre
            let mercy_pre = Instant::now();
            if self.mercy_gate.pre_tick_validate(&self.world).is_err() {
                break;
            }
            let mercy_pre_time = mercy_pre.elapsed();

            // Archetype
            let arch_start = Instant::now();
            self.archetype_system.update(&mut self.world);
            let arch_time = arch_start.elapsed();

            // Economy
            let econ_start = Instant::now();
            let _ = self.economic_layer.batch_update(&mut self.world, &self.mercy_gate);
            let econ_time = econ_start.elapsed();

            // Mercy post
            let mercy_post = Instant::now();
            let _ = self.mercy_gate.post_tick_validate(&self.world);
            let mercy_post_time = mercy_post.elapsed();

            let tick_total = tick_start.elapsed();

            // Accumulate (only sample periodically to reduce overhead)
            if self.tick_count % sample_every == 0 {
                total_archetype += arch_time;
                total_economy += econ_time;
                total_mercy += mercy_pre_time + mercy_post_time;
                total_other += tick_total - (arch_time + econ_time + mercy_pre_time + mercy_post_time);
                samples += 1;
            }

            self.sim_time_ms += 16;
            self.tick_count += 1;
        }

        let elapsed = start.elapsed();

        println!("Profiling complete in {:?}", elapsed);
        println!("Ticks executed: {}", self.tick_count - initial_tick);
        if samples > 0 {
            println!("\nAverage time per sampled tick:");
            println!("  Archetype System : {:?}", total_archetype / samples);
            println!("  Economic Layer   : {:?}", total_economy / samples);
            println!("  Mercy Gates      : {:?}", total_mercy / samples);
            println!("  Other / Overhead : {:?}", total_other / samples);
        }
        println!();
    }

    pub fn set_time_acceleration(&mut self, factor: f64) {
        self.time_acceleration = factor.max(0.01);
    }

    /// Runs the resonance decay & recovery analysis simulation.
    pub fn analyze_resonance_decay_recovery(&self) {
        println!("\n[SovereignSimulationOrchestrator] Running resonance decay & recovery analysis...");
        resonance_decay_recovery_sim::run_resonance_decay_recovery_simulation();
        println!("[SovereignSimulationOrchestrator] Resonance analysis complete.\n");
    }
}
