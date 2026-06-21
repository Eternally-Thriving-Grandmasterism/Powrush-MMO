//! simulation/src/orchestrator.rs
//! Production-grade Sovereign Simulation Orchestrator (Central Tick Coordinator)
//! v18.96 — Phase E: Full evolutionary demo tick wiring (volatility + mutations + stage-maturing chains)
//!            Derived from Ra-Thor powrush-mmo-simulator v15.30 + ability_tree v1.3
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned

use crate::world::SovereignWorldState;
use crate::archetype::SovereignArchetypeSystem;
use crate::economy::EconomicLayer;
use crate::mercy::{MercyGate, MercyViolation};
use crate::resonance_decay_recovery_sim;
use crate::flow_state_forge::{FlowStateMetrics, PresenceDebt, ChallengeBalancerConfig, dynamic_challenge_skill_balancer};
use crate::harvest::{HarvestSystem, HarvestEvent};
use crate::spatial_interest::{InterestManager, InterestZone, CouncilBloomZone, InterestZoneReplicated};
use crate::emergence::{EmergenceOrchestrator, DynamicEmergenceEvent};
use crate::council_mercy_trial::{CouncilSessionManager, CouncilBloomSyncEvent};
use bevy::prelude::*;
use std::time::Instant;
use tracing::{info, info_span, instrument, warn};

// Ra-Thor derived evolutionary player identity layer (Phase A–D)
use crate::race::{Race, RaceModifiers};
use crate::ability_tree::{AbilityTree, Ability, AbilityEffect, MutationType, SynergyBonus};
use crate::epigenetic_modulation::{
    EpigeneticProfile, apply_volatility_drift, is_high_volatility_risk,
    apply_double_edged_volatility_effects, apply_epigenetic_repair,
    try_trigger_epigenetic_mutation,
};

#[derive(Debug, Default, Clone)]
pub struct TickResult {
    pub council_bloom_events: Vec<CouncilBloomSyncEvent>,
    pub closed_session_persistence: Vec<crate::council_mercy_trial::BatchPersistenceUpdate>,
    pub emergence_events: Vec<DynamicEmergenceEvent>,
    pub harvest_events: Vec<HarvestEvent>,
    pub flow_state_updated: bool,
    pub spatial_interest_updated: bool,
    pub spatial_zones_changed: usize,
    pub archetype_updates_performed: usize,
    pub world_entities_changed: bool,
    pub any_significant_change: bool,
    pub changed_spatial_zones: Vec<InterestZoneReplicated>,
}

pub struct SovereignSimulationOrchestrator {
    pub world: SovereignWorldState,
    pub archetype_system: SovereignArchetypeSystem,
    pub economic_layer: EconomicLayer,
    pub mercy_gate: MercyGate,
    pub sim_time_ms: u64,
    pub tick_count: u64,
    pub time_acceleration: f64,

    pub flow_metrics: FlowStateMetrics,
    pub presence_debt: PresenceDebt,
    pub interest_manager: InterestManager,
    pub emergence_orchestrator: EmergenceOrchestrator,
    pub harvest_system: HarvestSystem,
    pub council_manager: CouncilSessionManager,

    last_tick_start: Instant,
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
            last_tick_start: Instant::now(),
        }
    }

    #[instrument(skip(self), fields(tick = self.tick_count))]
    pub fn run_tick(&mut self, tick_resource: Option<&mut crate::orchestrator::SimulationTick>) -> Result<TickResult, MercyViolation> {
        let tick_start = Instant::now();
        let _span = info_span!("orchestrator_tick", tick = self.tick_count).entered();

        self.mercy_gate.pre_tick_validate(&self.world)?;

        // Phase 1: Archetype
        let mut archetype_updates_performed = 0;
        let mut world_entities_changed = false;
        {
            let before = self.world.entity_count();
            self.archetype_system.update(&mut self.world);
            let after = self.world.entity_count();
            archetype_updates_performed = after.saturating_sub(before) as usize;
            world_entities_changed = archetype_updates_performed > 0 || self.world.has_pending_changes();
        }

        // Phase 2: Flow State
        let mut flow_state_updated = false;
        {
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

        // Phase 3: Spatial Interest
        let mut spatial_interest_updated = false;
        let mut spatial_zones_changed = 0;
        let mut changed_spatial_zones: Vec<InterestZoneReplicated> = Vec::new();

        {
            let _spatial_span = info_span!("spatial_interest_update").entered();
            let before_zones = self.interest_manager.active_zone_count();
            self.interest_manager.update_zones(&mut self.world, self.tick_count);
            let after_zones = self.interest_manager.active_zone_count();

            spatial_zones_changed = after_zones.saturating_sub(before_zones);
            spatial_interest_updated = spatial_zones_changed > 0 || self.interest_manager.has_pending_changes();

            if spatial_interest_updated {
                for (entity_id, zone) in self.world.iter_interest_zones().take(8) {
                    let replicated = InterestZoneReplicated {
                        entity: Entity::from_raw(entity_id as u32),
                        zone: zone.clone(),
                        version: self.tick_count,
                        server_timestamp: self.sim_time_ms as f64,
                    };
                    self.interest_manager.record_zone_change(replicated);
                }
            }
        }

        let changed_spatial_zones = self.interest_manager.drain_changed_zones();

        // Phase 4: Emergence
        let emergence_events = self.emergence_orchestrator.process_emergence(
            &mut self.world,
            &self.interest_manager,
            &self.council_manager,
            self.tick_count,
        );

        for event in &emergence_events {
            self.economic_layer.apply_emergence_event(event, &mut self.world, &self.mercy_gate)?;
        }

        // Phase 5: Harvest (deeply wired)
        let harvest_events = self.harvest_system.process_harvest_tick(&mut self.world, self.tick_count);

        for event in &harvest_events {
            self.economic_layer.apply_harvest_event(event, &mut self.world, &self.mercy_gate)?;
        }

        // Phase 6: Economy (general batch update)
        self.economic_layer.batch_update(&mut self.world, &self.mercy_gate)?;

        // Phase 7: Council
        let mut tick_result = TickResult {
            emergence_events,
            harvest_events,
            flow_state_updated,
            spatial_interest_updated,
            spatial_zones_changed,
            archetype_updates_performed,
            world_entities_changed,
            changed_spatial_zones,
            ..Default::default()
        };

        {
            let bloom_events = self.council_manager.tick_sessions(self.tick_count);
            tick_result.council_bloom_events = bloom_events;

            let closed = self.council_manager.collect_closed_session_persistence(self.tick_count);
            tick_result.closed_session_persistence = closed;
        }

        tick_result.any_significant_change =
            tick_result.flow_state_updated ||
            tick_result.spatial_interest_updated ||
            tick_result.archetype_updates_performed > 0 ||
            !tick_result.emergence_events.is_empty() ||
            !tick_result.harvest_events.is_empty() ||
            !tick_result.council_bloom_events.is_empty();

        self.mercy_gate.post_tick_validate(&self.world)?;

        let dt_ms = (16.0 * self.time_acceleration) as u64;
        self.sim_time_ms += dt_ms;
        self.tick_count += 1;

        if let Some(tick_res) = tick_resource {
            tick_res.tick = self.tick_count;
            tick_res.sim_time_ms = self.sim_time_ms;
            tick_res.last_tick_duration_ms = tick_start.elapsed().as_millis() as u64;
            tick_res.time_acceleration = self.time_acceleration;
            tick_res.any_significant_change = tick_result.any_significant_change;
        }

        Ok(tick_result)
    }

    pub fn set_time_acceleration(&mut self, factor: f64) {
        self.time_acceleration = factor.max(0.01);
    }

    pub fn analyze_resonance_decay_recovery(&self) {
        resonance_decay_recovery_sim::run_resonance_decay_recovery_simulation();
    }

    pub fn current_tick_info(&self) -> (u64, u64) {
        (self.tick_count, self.sim_time_ms)
    }

    // ========================================================================
    // PHASE E DEMO WIRING — Evolutionary Player Identity Layer (Ra-Thor derived)
    // ========================================================================
    /// Demo helper that exercises the full volatility lifecycle + mutation triggers
    /// + stage-maturing synergy chains in a self-contained, observable way.
    /// Does not modify the main world/archetype state (safe for production orchestrator).
    /// Returns a rich multi-line status string suitable for CLI harness, tests, or future UI.
    pub fn demo_evolutionary_tick(&mut self, num_ticks: u32) -> String {
        let mut log = String::from("\n=== Powrush Evolutionary Demo (Ra-Thor Derived) ===\n");
        log.push_str(&format!("Running {} simulation ticks on demo Terran entity...\n\n", num_ticks));

        // === Demo Entity Setup ===
        let demo_race = Race::Terran;
        let mut ability_tree = AbilityTree::new();
        // Give the demo entity the Terran starter ability
        let _ = ability_tree.try_unlock_starter("steady_step", demo_race);

        let mut profile = EpigeneticProfile {
            strength: 1.0,
            volatility: 0.65,
            layer_alignment: 0.8,
            cooperation_score: 0.7,
            corruption: 0.0,
        };

        let mut active_mutation: Option<MutationType> = None;
        let mut chain_key = "redemption_cascade".to_string(); // Will switch based on mutation
        let mut harmony: f32 = 1.4;
        let mut recent_contribution: f32 = 8.0;

        let mut mutation_triggered = false;
        let mut final_stage: u8 = 0;

        for t in 0..num_ticks {
            let current_tick = self.tick_count + t as u64;

            // 1. Apply volatility drift (natural entropy modulated by harmony)
            apply_volatility_drift(&mut profile, harmony, 0.006);

            // 2. Double-edged sword effects (power or backlash risk)
            let in_high_risk = is_high_volatility_risk(profile.volatility);
            if in_high_risk {
                apply_double_edged_volatility_effects(&mut profile, current_tick);
            }

            // 3. Repair if conditions are good (low volatility + cooperation)
            if profile.volatility < 0.75 && profile.cooperation_score > 0.6 {
                apply_epigenetic_repair(&mut profile, harmony, true);
            }

            // 4. Check for mutation trigger (only once for demo clarity)
            if !mutation_triggered && in_high_risk && profile.corruption > 0.9 {
                if let Some(mutation) = try_trigger_epigenetic_mutation(
                    &profile,
                    in_high_risk,
                    true, // has_resilience_synergy for demo
                    harmony,
                    current_tick,
                ) {
                    active_mutation = Some(mutation.clone());
                    mutation_triggered = true;
                    match mutation {
                        MutationType::HarmonicRebirth => {
                            chain_key = "redemption_cascade".to_string();
                            log.push_str(&format!("[TICK {}] *** MUTATION: Harmonic Rebirth (Redemptive path) ***\n", current_tick));
                        }
                        MutationType::VolatileSurge => {
                            chain_key = "surge_overclock".to_string();
                            log.push_str(&format!("[TICK {}] *** MUTATION: Volatile Surge (High-risk power) ***\n", current_tick));
                        }
                        MutationType::CorruptedEcho => {
                            chain_key = "corrupted_singularity".to_string();
                            log.push_str(&format!("[TICK {}] *** MUTATION: Corrupted Echo (Dangerous path) ***\n", current_tick));
                        }
                    }
                }
            }

            // 5. If mutated, progress the corresponding synergy chain
            if let Some(_m) = &active_mutation {
                // Simulate improving conditions over time for demo progression
                if t % 8 == 0 {
                    harmony = (harmony + 0.08).min(2.8);
                    recent_contribution += 1.5;
                }

                ability_tree.progress_chain_stages(
                    &chain_key,
                    harmony,
                    recent_contribution,
                    profile.volatility,
                );

                let stage = ability_tree.get_chain_stage(&chain_key);
                if stage > final_stage {
                    final_stage = stage;
                    log.push_str(&format!("[TICK {}] Chain '{}' advanced to Stage {}\n", current_tick, chain_key, stage));
                }
            }

            // Occasional status line
            if t % 12 == 0 || (mutation_triggered && t % 5 == 0) {
                log.push_str(&format!(
                    "Tick {} | Vol: {:.2} | Str: {:.2} | Cor: {:.2} | Harmony: {:.1} | Mutation: {:?}\n",
                    current_tick,
                    profile.volatility,
                    profile.strength,
                    profile.corruption,
                    harmony,
                    active_mutation
                ));
            }
        }

        // Final summary
        log.push_str("\n=== Demo Complete ===\n");
        if let Some(m) = active_mutation {
            log.push_str(&format!("Final Mutation: {:?} | Final Chain Stage: {}\n", m, final_stage));
            let chains = ability_tree.calculate_mutation_synergy_chains(&[m]);
            if !chains.is_empty() {
                log.push_str("Active Synergy Chains:\n");
                for c in chains {
                    log.push_str(&format!("  - {} (Stage {}): {}\n", c.name, final_stage, c.description));
                }
            }
        } else {
            log.push_str("No mutation triggered in this run (try higher corruption or more ticks).\n");
        }

        log.push_str("\nThunder locked in. Yoi ⚡\n");
        log
    }
}

// End of production file — Evolutionary demo tick wired (Phase E).
// Full volatility lifecycle + mutation triggers + stage-maturing chains now observable from orchestrator.
// Ready for deeper integration into world entities and EconomicLayer in future phases.
// Thunder locked in. Yoi ⚡