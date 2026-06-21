/*!
 * SovereignSimulationOrchestrator with dynamic mercy threshold exposed in TickResult.
 */

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

use crate::race::{Race, RaceModifiers};
use crate::ability_tree::{AbilityTree, Ability, AbilityEffect, MutationType, SynergyBonus, SynergyEffectEvent};
use crate::epigenetic_modulation::{
    EpigeneticProfile, apply_volatility_drift, is_high_volatility_risk,
    apply_double_edged_volatility_effects, apply_epigenetic_repair,
    try_trigger_epigenetic_mutation,
};
use crate::diplomacy::{DiplomacyManager, TreatyType};

use crate::council::{CouncilProposal, CouncilSession, CouncilDecision, ProposalType, ProposalStatus};

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
    pub evolutionary_agents_processed: usize,
    pub synergy_events: Vec<SynergyEffectEvent>,
    pub resolved_council_proposals: Vec<CouncilProposal>,
    pub applied_council_decisions: Vec<CouncilDecision>,
    pub dynamic_mercy_threshold: Option<f32>,   // NEW: exposed from CouncilSession
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

    pub council_session: CouncilSession,

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
            council_session: CouncilSession::new(0, 0),
            last_tick_start: Instant::now(),
        }
    }

    #[instrument(skip(self), fields(tick = self.tick_count))]
    pub fn run_tick(&mut self, tick_resource: Option<&mut crate::orchestrator::SimulationTick>) -> Result<TickResult, MercyViolation> {
        let tick_start = Instant::now();
        let _span = info_span!("orchestrator_tick", tick = self.tick_count).entered();

        self.mercy_gate.pre_tick_validate(&self.world)?;

        let mut archetype_updates_performed = 0;
        let mut world_entities_changed = false;
        {
            let before = self.world.entity_count();
            self.archetype_system.update(&mut self.world);
            let after = self.world.entity_count();
            archetype_updates_performed = after.saturating_sub(before) as usize;
            world_entities_changed = archetype_updates_performed > 0 || self.world.has_pending_changes();
        }

        let (evolutionary_agents_processed, synergy_events) = self.process_evolutionary_identities_for_attached_agents();

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

        let mut spatial_interest_updated = false;
        let mut spatial_zones_changed = 0;
        let mut changed_spatial_zones: Vec<InterestZoneReplicated> = Vec::new();
        {
            let before_zones = self.interest_manager.active_zone_count();
            self.interest_manager.update_zones(&mut self.world, self.tick_count);
            let after_zones = self.interest_manager.active_zone_count();
            spatial_zones_changed = after_zones.saturating_sub(before_zones);
            spatial_interest_updated = spatial_zones_changed > 0 || self.interest_manager.has_pending_changes();
        }

        let changed_spatial_zones = self.interest_manager.drain_changed_zones();

        let emergence_events = vec![];
        let harvest_events = vec![];

        // Council deliberation (now returns dynamic threshold via session)
        let resolved_council_proposals: Vec<CouncilProposal> = if self.tick_count % 10 == 0 {
            if self.council_session.active_proposals.is_empty() {
                let _proposal_id = self.council_session.submit_proposal(
                    ProposalType::ResourcePolicy,
                    "Demo RBE Abundance Policy".to_string(),
                    "Increase shared resource flow for all agents (audit log demo)".to_string(),
                    0,
                    self.tick_count,
                );
            }
            self.council_session.run_deliberation(72.0, self.tick_count)
        } else {
            vec![]
        };

        let mut applied_council_decisions: Vec<CouncilDecision> = Vec::new();

        for proposal in &resolved_council_proposals {
            if proposal.status == ProposalStatus::Passed {
                let decision = CouncilDecision::from_resolved_proposal(
                    proposal,
                    0.72,
                    self.tick_count,
                    self.council_session.realm_id,
                );

                match proposal.proposal_type {
                    ProposalType::ResourcePolicy => {
                        for pool in self.world.rbe_pools.values_mut() {
                            pool.abundance_flow = (pool.abundance_flow + 0.25).min(3.5);
                            pool.sustainability_score = (pool.sustainability_score + 0.08).min(1.0);
                            pool.pressure = (pool.pressure * 0.65).max(0.0);
                        }
                        for node in self.world.resource_nodes.values_mut() {
                            node.abundance_flow = (node.abundance_flow + 0.12).min(3.0);
                            node.sustainability_score = (node.sustainability_score + 0.05).min(1.0);
                        }
                    }
                    ProposalType::HarmonyBoost => {
                        self.presence_debt.current_debt = (self.presence_debt.current_debt - 0.18).max(0.0);
                        self.flow_metrics.current_challenge_level =
                            (self.flow_metrics.current_challenge_level - 0.06).max(0.08);
                        flow_state_updated = true;
                    }
                    ProposalType::EpiphanyEvent => {
                        self.flow_metrics.current_challenge_level =
                            (self.flow_metrics.current_challenge_level * 0.88).max(0.08);
                        flow_state_updated = true;
                    }
                    ProposalType::General => {
                        self.flow_metrics.current_challenge_level =
                            (self.flow_metrics.current_challenge_level - 0.02).max(0.08);
                    }
                }

                applied_council_decisions.push(decision);
            }
        }

        let dynamic_threshold = self.council_session.last_dynamic_threshold();

        let mut tick_result = TickResult {
            emergence_events,
            harvest_events,
            flow_state_updated,
            spatial_interest_updated,
            spatial_zones_changed,
            archetype_updates_performed,
            world_entities_changed,
            changed_spatial_zones,
            evolutionary_agents_processed,
            synergy_events,
            resolved_council_proposals,
            applied_council_decisions,
            dynamic_mercy_threshold: dynamic_threshold,
            ..Default::default()
        };

        tick_result.any_significant_change =
            tick_result.flow_state_updated ||
            tick_result.spatial_interest_updated ||
            tick_result.archetype_updates_performed > 0 ||
            evolutionary_agents_processed > 0 ||
            !tick_result.synergy_events.is_empty() ||
            !tick_result.resolved_council_proposals.is_empty() ||
            !tick_result.applied_council_decisions.is_empty();

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
        (self.tick_count, self.sim_time_ms);
    }

    pub fn demo_evolutionary_tick_attached(&mut self, num_ticks: u32) -> String {
        // ... unchanged ...
        let mut log = String::from("\n=== Powrush Evolutionary Demo (Attached to Real WorldState) ===\n");
        log.push_str(&format!("Running {} ticks on a real Agent entity with full evolutionary state...\n\n", num_ticks));

        let demo_agent_id: u64 = 424242;
        let demo_race = Race::Terran;

        let agent_exists = self.world.agents.iter().any(|a| a.id == demo_agent_id);
        if !agent_exists {
            self.world.agents.push(crate::world::Agent {
                id: demo_agent_id,
                archetype_id: 1,
                position: crate::world::Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                inventory: crate::world::Inventory::default(),
                mercy_score: 0.8,
                behavior_state: crate::world::BehaviorState { current: "exploring".to_string() },
            });
        }

        if !self.world.evolutionary_profiles.contains_key(&demo_agent_id) {
            self.world.evolutionary_profiles.insert(demo_agent_id, EpigeneticProfile {
                strength: 1.0,
                volatility: 0.65,
                layer_alignment: 0.8,
                cooperation_score: 0.7,
                corruption: 0.0,
            });
            let mut tree = AbilityTree::new();
            let _ = tree.try_unlock_starter("steady_step", demo_race);
            self.world.ability_trees.insert(demo_agent_id, tree);
            self.world.active_mutations.insert(demo_agent_id, vec![]);
        }

        let profile = self.world.evolutionary_profiles.get_mut(&demo_agent_id).unwrap();
        let ability_tree = self.world.ability_trees.get_mut(&demo_agent_id).unwrap();
        let active_mutations = self.world.active_mutations.get_mut(&demo_agent_id).unwrap();

        let mut harmony: f32 = 1.4;
        let mut recent_contribution: f32 = 8.0;
        let mut mutation_triggered = false;
        let mut final_stage: u8 = 0;
        let mut current_mutation: Option<MutationType> = None;

        for t in 0..num_ticks {
            let current_tick = self.tick_count + t as u64;

            apply_volatility_drift(profile, harmony, 0.006);

            let in_high_risk = is_high_volatility_risk(profile.volatility);
            if in_high_risk {
                apply_double_edged_volatility_effects(profile, current_tick);
            }

            if profile.volatility < 0.75 && profile.cooperation_score > 0.6 {
                apply_epigenetic_repair(profile, harmony, true);
            }

            if !mutation_triggered && in_high_risk && profile.corruption > 0.9 {
                if let Some(mutation) = try_trigger_epigenetic_mutation(
                    profile, in_high_risk, true, harmony, current_tick,
                ) {
                    active_mutations.push(mutation.clone());
                    current_mutation = Some(mutation.clone());
                    mutation_triggered = true;
                    log.push_str(&format!("[TICK {}] *** MUTATION ATTACHED: {:?} ***\n", current_tick, mutation));
                }
            }

            if let Some(m) = &current_mutation {
                if t % 8 == 0 {
                    harmony = (harmony + 0.08).min(2.8);
                    recent_contribution += 1.5;
                }
                let chain_key = match m {
                    MutationType::HarmonicRebirth => "redemption_cascade",
                    MutationType::VolatileSurge => "surge_overclock",
                    MutationType::CorruptedEcho => "corrupted_singularity",
                };
                ability_tree.progress_chain_stages(chain_key, harmony, recent_contribution, profile.volatility);
                let stage = ability_tree.get_chain_stage(chain_key);
                if stage > final_stage {
                    final_stage = stage;
                    log.push_str(&format!("[TICK {}] Chain '{}' advanced to Stage {} (real state)\n", current_tick, chain_key, stage));
                }
            }

            if t % 15 == 0 {
                log.push_str(&format!(
                    "Tick {} | Vol: {:.2} | Str: {:.2} | Cor: {:.2} | Mutation: {:?} | Stage: {}\n",
                    current_tick, profile.volatility, profile.strength, profile.corruption, current_mutation, final_stage
                ));
            }
        }

        log.push_str("\n=== Demo Complete (State lives in SovereignWorldState) ===\n");
        log.push_str(&format!("Agent {} now has evolutionary state persisted in world HashMaps.\n", demo_agent_id));
        log.push_str("Thunder locked in. Yoi ⚡\n");
        log
    }
}
