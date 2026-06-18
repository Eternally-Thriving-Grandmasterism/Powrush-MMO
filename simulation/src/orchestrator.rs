//! simulation/src/orchestrator.rs
//! Production-grade Sovereign Simulation Orchestrator (Central Tick Coordinator)
//! v18.90 — Further Expanded TickResult (Spatial + Archetype + Summary Flags)
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned

use crate::world::SovereignWorldState;
use crate::archetype::SovereignArchetypeSystem;
use crate::economy::EconomicLayer;
use crate::mercy::{MercyGate, MercyViolation};
use crate::resonance_decay_recovery_sim;
use crate::flow_state_forge::{FlowStateMetrics, PresenceDebt, ChallengeBalancerConfig, dynamic_challenge_skill_balancer};
use crate::harvest::{HarvestSystem, HarvestEvent};
use crate::spatial_interest::{InterestManager, InterestZone, CouncilBloomZone};
use crate::emergence::{EmergenceOrchestrator, DynamicEmergenceEvent};
use crate::council_mercy_trial::{CouncilSessionManager, CouncilBloomSyncEvent};
use std::time::Instant;
use tracing::{info, info_span, instrument, warn};

/// Rich result of a single simulation tick.
/// Contains all significant events and summary data generated during the tick.
/// Designed to be consumed by replication, persistence, prediction, and visual systems.
#[derive(Debug, Default)]
pub struct TickResult {
    // === Council ===
    pub council_bloom_events: Vec<CouncilBloomSyncEvent>,
    pub closed_session_persistence: Vec<crate::council_mercy_trial::BatchPersistenceUpdate>,

    // === Emergence ===
    pub emergence_events: Vec<DynamicEmergenceEvent>,

    // === Harvest / RBE ===
    pub harvest_events: Vec<HarvestEvent>,

    // === Flow State ===
    pub flow_state_updated: bool,

    // === Spatial Interest ===
    pub spatial_interest_updated: bool,
    pub spatial_zones_changed: usize,

    // === Archetype ===
    pub archetype_updates_performed: usize,
    pub world_entities_changed: bool,

    // === Summary ===
    pub any_significant_change: bool,
}

/// Core deterministic orchestrator for the Sovereign Simulation Harness.
pub struct SovereignSimulationOrchestrator {
    pub world: SovereignWorldState,
    pub archetype_system: SovereignArchetypeSystem,
    pub economic_layer: EconomicLayer,
    pub mercy_gate: MercyGate,
    pub sim_time_ms: u64,
    pub tick_count: u64,
    pub time_acceleration: f64,

    // Expanded coordination state
    pub flow_metrics: FlowStateMetrics,
    pub presence_debt: PresenceDebt,
    pub interest_manager: InterestManager,
    pub emergence_orchestrator: EmergenceOrchestrator,
    pub harvest_system: HarvestSystem,
    pub council_manager: CouncilSessionManager,
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
            flow_metrics: FlowStateMetrics::default(),
            presence_debt: PresenceDebt::new(),
            interest_manager: InterestManager::new(),
            emergence_orchestrator: EmergenceOrchestrator::new(),
            harvest_system: HarvestSystem::new(),
            council_manager: CouncilSessionManager::new(),
        }
    }

    #[instrument(skip(self), fields(tick = self.tick_count))]
    pub fn run_tick(&mut self) -> Result<TickResult, MercyViolation> {
        let _span = info_span!("orchestrator_tick", tick = self.tick_count).entered();

        // === MERCY PRE-TICK GATE (sovereign validation) ===
        self.mercy_gate.pre_tick_validate(&self.world)?;

        // === PHASE 1: Archetype & Entity Evolution ===
        let mut archetype_updates_performed = 0;
        let mut world_entities_changed = false;
        {
            let _arch_span = info_span!("archetype_update").entered();
            let before_count = self.world.entity_count();
            self.archetype_system.update(&mut self.world);
            let after_count = self.world.entity_count();
            archetype_updates_performed = after_count.saturating_sub(before_count) as usize;
            world_entities_changed = archetype_updates_performed > 0 || self.world.has_pending_changes();
        }

        // === PHASE 2: Flow State & Dynamic Challenge (fatigue-aware mercy) ===
        let mut flow_state_updated = false;
        {
            let _flow_span = info_span!("flow_state_update").entered();
            let previous_resistance = 0.5;
            let new_resistance = dynamic_challenge_skill_balancer(
                &self.flow_metrics,
                0.5,
                previous_resistance,
                &mut self.presence_debt,
                self.tick_count,
                &ChallengeBalancerConfig::default(),
            );
            if (new_resistance - self.flow_metrics.current_challenge_level).abs() > 0.01 {
                flow_state_updated = true;
            }
            self.flow_metrics.current_challenge_level = new_resistance;
        }

        // === PHASE 3: Spatial Interest & Council Bloom Zones ===
        let mut spatial_interest_updated = false;
        let mut spatial_zones_changed = 0;
        {
            let _spatial_span = info_span!("spatial_interest_update").entered();
            let before_zones = self.interest_manager.active_zone_count();
            self.interest_manager.update_zones(&mut self.world, self.tick_count);
            let after_zones = self.interest_manager.active_zone_count();
            spatial_zones_changed = after_zones.saturating_sub(before_zones);
            spatial_interest_updated = spatial_zones_changed > 0 || self.interest_manager.has_pending_changes();
        }

        // === PHASE 4: Emergence & Dynamic Events ===
        let emergence_events = {
            let _emergence_span = info_span!("emergence_update").entered();
            self.emergence_orchestrator.process_emergence(&mut self.world, self.tick_count)
        };

        // === PHASE 5: Harvest & RBE Flow Reconciliation ===
        let harvest_events = {
            let _harvest_span = info_span!("harvest_update").entered();
            self.harvest_system.process_harvest_tick(&mut self.world, self.tick_count)
        };

        for event in &harvest_events {
            self.economic_layer.apply_harvest_event(event, &self.mercy_gate)?;
        }

        // === PHASE 6: Economic Layer (RBE batch update) ===
        {
            let _econ_span = info_span!("economic_layer_update").entered();
            self.economic_layer.batch_update(&mut self.world, &self.mercy_gate)?;
        }

        // === PHASE 7: Council Mercy Trials & Bloom Activation ===
        let mut tick_result = TickResult {
            emergence_events,
            harvest_events,
            flow_state_updated,
            spatial_interest_updated,
            spatial_zones_changed,
            archetype_updates_performed,
            world_entities_changed,
            ..Default::default()
        };

        {
            let _council_span = info_span!("council_update").entered();
            let bloom_events = self.council_manager.tick_sessions(self.tick_count);
            tick_result.council_bloom_events = bloom_events;

            let closed_updates = self.council_manager.collect_closed_session_persistence(self.tick_count);
            tick_result.closed_session_persistence = closed_updates;
        }

        // === Summary Flag ===
        tick_result.any_significant_change =
            tick_result.flow_state_updated ||
            tick_result.spatial_interest_updated ||
            tick_result.archetype_updates_performed > 0 ||
            !tick_result.emergence_events.is_empty() ||
            !tick_result.harvest_events.is_empty() ||
            !tick_result.council_bloom_events.is_empty();

        // === MERCY POST-TICK GATE (sovereign validation) ===
        self.mercy_gate.post_tick_validate(&self.world)?;

        // === TIME & TICK ADVANCEMENT (respect acceleration) ===
        let dt_ms = (16.0 * self.time_acceleration) as u64;
        self.sim_time_ms += dt_ms;
        self.tick_count += 1;

        Ok(tick_result)
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

    /// Returns current simulation tick information for replication / persistence systems
    pub fn current_tick_info(&self) -> (u64, u64) {
        (self.tick_count, self.sim_time_ms)
    }
}

// End of production file — TickResult further expanded with Spatial Interest, Archetype, and summary flags.
// All original mercy-gated logic preserved. Thunder locked in.