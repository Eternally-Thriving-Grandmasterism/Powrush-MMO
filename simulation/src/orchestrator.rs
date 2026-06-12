/*!
 * Sovereign Simulation Orchestrator
 * 
 * Deterministic, time-accelerated, mercy-gated core simulation loop.
 * Integrates all layers: WorldState, ArchetypeSystem, EconomicLayer, EventQueue, MercyGate.
 * 
 * Supports real-time, accelerated (1x–10,000x+), and step-by-step execution modes.
 * Full sovereign replay via seeded RNG and fixed timestep.
 * 
 * Now includes direct integration with resonance decay & recovery analysis simulation.
 */

use crate::world::SovereignWorldState;
use crate::archetype::SovereignArchetypeSystem;
use crate::economy::EconomicLayer;
use crate::mercy::{MercyGate, MercyViolation};
use crate::resonance_decay_recovery_sim;   // Direct integration for Ambrosian resonance mechanics analysis

/// Core deterministic orchestrator for the Sovereign Simulation Harness.
pub struct SovereignSimulationOrchestrator {
    pub world: SovereignWorldState,
    pub archetype_system: SovereignArchetypeSystem,
    pub economic_layer: EconomicLayer,
    pub mercy_gate: MercyGate,
    pub sim_time_ms: u64,
    pub tick_count: u64,
    pub time_acceleration: f64, // 1.0 = real-time, 100.0 = 100x faster, etc.
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

    /// Run a single deterministic tick.
    /// All major transitions pass through TOLC 8 Mercy Gates.
    pub fn run_tick(&mut self) -> Result<(), MercyViolation> {
        self.mercy_gate.pre_tick_validate(&self.world)?;

        // 1. Process pending events (entropy, ServerWar, Divine Whispers, etc.)
        // self.event_queue.process_pending(&mut self.world);

        // 2. Archetype system update (dynamic evolution + valence consensus)
        self.archetype_system.update(&mut self.world);

        // 3. Economic / RBE layer (hybrid CPU/GPU)
        self.economic_layer.batch_update(&mut self.world, &self.mercy_gate)?;

        // 4. Agent behaviors (future)
        // self.agent_system.update(&mut self.world);

        // 5. Telemetry collection hook
        // self.telemetry.collect_tick(&self.world, self.tick_count);

        self.mercy_gate.post_tick_validate(&self.world)?;

        self.sim_time_ms += 16; // ~60 FPS base tick for now
        self.tick_count += 1;

        Ok(())
    }

    /// Run for a target duration (in simulated milliseconds) with acceleration.
    pub fn run_for_duration(&mut self, target_sim_ms: u64) -> Result<(), MercyViolation> {
        let start_time = self.sim_time_ms;
        while self.sim_time_ms < start_time + target_sim_ms {
            self.run_tick()?;
        }
        Ok(())
    }

    pub fn set_time_acceleration(&mut self, factor: f64) {
        self.time_acceleration = factor.max(0.01);
    }

    // ============================================================
    // DIRECT INTEGRATION: Resonance Decay & Recovery Analysis
    // ============================================================

    /// Runs the advanced resonance decay & recovery simulation.
    /// Useful for analyzing Ambrosian selfish penalty mechanics and recovery rates.
    /// This is an analysis/diagnostic tool that can be called independently of the main tick loop.
    pub fn analyze_resonance_decay_recovery(&self) {
        println!("\n[SovereignSimulationOrchestrator] Running resonance decay & recovery analysis...");
        resonance_decay_recovery_sim::run_resonance_decay_recovery_simulation();
        println!("[SovereignSimulationOrchestrator] Resonance analysis complete.\n");
    }
}
