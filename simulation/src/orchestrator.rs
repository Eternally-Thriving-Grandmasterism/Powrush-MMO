/*!
 * simulation/src/orchestrator.rs
 * Central Simulation Orchestrator (v21.88.13)
 *
 * Full TOLC 8 MercyGate + EconomicLayer batch_update
 * v21.88.5: Soft feedback loop hook (RaThorBridge::report_zone_grief)
 * v21.88.6: Soft feedback → telemetry custom metrics
 * v21.88.7: Soft feedback stress/purify aggregates → telemetry
 * v21.88.8: soft_feedback_health_score composite → telemetry
 * v21.88.9: ZoneHealthStatus counts → telemetry
 * v21.88.10: soft_feedback_critical_auto → telemetry
 * v21.88.11: valence histogram + mercy_ratio → telemetry
 * v21.88.12: soft_feedback_soft_remediates → telemetry
 * v21.88.13: grief-rate metrics → telemetry
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Contact: info@Rathor.ai
 * Thunder locked in. Yoi ⚡
 */

use crate::world::SovereignWorldState;
use crate::economy::EconomicLayer;
use crate::harvest::HarvestingSystem;
use crate::emergence::EmergenceOrchestrator;
use crate::ability_tree::SynergyEffectEvent;
use crate::council_mercy_trial::CouncilSessionManager;
use crate::council::decision::{CouncilDecisions, apply_resource_policy_impact, apply_epiphany_policy_impact, PolicyType};
use crate::player_persistence::PlayerSaveData;
use crate::mercy::MercyGate;
use crate::ra_thor_bridge::RaThorBridge;
use crate::telemetry::TelemetryCollector;
use std::sync::Arc;
use tracing::{info, warn};

#[cfg(feature = "gpu")]
use crate::engine::gpu_patsagi_bridge::{GpuPatsagiBridge, GpuPatsagiRequest, GpuPatsagiResponse, ComputeIntensity};

#[derive(Debug, Default, Clone)]
pub struct TickResult {
    pub tick: u64,
    pub economic_updates: u32,
    pub council_decisions_applied: u32,
    pub council_attunement_score: f32,
    pub council_participant_count: u32,
    pub resource_policy_impacts: u32,
    pub epiphany_policy_impacts: u32,
    pub harvest_nodes_processed: u32,
    pub emergence_events_triggered: u32,
    pub synergy_events: Vec<SynergyEffectEvent>,
    pub gpu_foresight_used: bool,
    pub gpu_foresight_applied: bool,
    pub soft_feedback_events: u32,
    pub errors: Vec<String>,
}

impl TickResult {
    pub fn estimated_mercy_flow(&self) -> f32 {
        let base = 0.85_f32;
        let council_boost = (self.council_attunement_score * 0.25).clamp(0.0, 0.4);
        let error_penalty = if self.errors.is_empty() { 0.0 } else { 0.15 };
        (base + council_boost - error_penalty).clamp(0.35, 1.85)
    }
}

pub struct SimulationOrchestrator {
    pub economic_layer: EconomicLayer,
    pub emergence_orchestrator: EmergenceOrchestrator,
    pub harvesting_system: HarvestingSystem,
    pub current_tick: u64,
    pub soft_feedback_bridge: Option<RaThorBridge>,

    #[cfg(feature = "gpu")]
    pub gpu_foresight: Option<Arc<dyn GpuPatsagiBridge + Send + Sync>>,
}

impl SimulationOrchestrator {
    pub fn new() -> Self {
        Self {
            economic_layer: EconomicLayer::default(),
            emergence_orchestrator: EmergenceOrchestrator::default(),
            harvesting_system: HarvestingSystem::default(),
            current_tick: 0,
            soft_feedback_bridge: Some(RaThorBridge::new_simulation(true)),

            #[cfg(feature = "gpu")]
            gpu_foresight: None,
        }
    }

    #[cfg(feature = "gpu")]
    pub fn set_gpu_foresight(&mut self, bridge: Arc<dyn GpuPatsagiBridge + Send + Sync>) {
        self.gpu_foresight = Some(bridge);
    }

    pub fn run_tick(
        &mut self,
        world: &mut SovereignWorldState,
        interest_manager: Option<&crate::spatial_interest::InterestManager>,
        council_manager: Option<&mut CouncilSessionManager>,
        player_save: Option<&mut PlayerSaveData>,
        council_decisions: Option<&CouncilDecisions>,
    ) -> TickResult {
        self.current_tick += 1;
        let mut result = TickResult {
            tick: self.current_tick,
            ..Default::default()
        };

        let mercy_gate = MercyGate;
        if let Err(e) = self.economic_layer.batch_update(world, &mercy_gate) {
            result.errors.push(format!("Economic update failed: {}", e));
        } else {
            result.economic_updates = 1;
        }

        if let Some(manager) = council_manager {
            if let Some(bloom) = manager.resolve_and_set_bloom_from_real_data(
                self.current_tick, 3, "sanctuary",
            ) {
                self.economic_layer.apply_council_policy_impact(
                    bloom.collective_attunement_score,
                    bloom.council_mercy_seal,
                    bloom.participant_count,
                    world,
                );
                result.council_decisions_applied = 1;
                result.council_attunement_score = bloom.collective_attunement_score;
                result.council_participant_count = bloom.participant_count;

                info!("Council policy applied with REAL data — attunement: {:.2}, participants: {}",
                      bloom.collective_attunement_score, bloom.participant_count);
            }
        }

        if let Some(decisions) = council_decisions {
            for policy in &decisions.active_policies {
                if policy.is_expired() {
                    continue;
                }

                let dummy_decision = crate::council::decision::CouncilDecision {
                    decision_id: policy.decision_id,
                    proposal_id: policy.decision_id,
                    proposal_type: match policy.policy_type {
                        PolicyType::ResourcePolicy => crate::council::proposal::ProposalType::ResourcePolicy,
                        PolicyType::EpiphanyEvent => crate::council::proposal::ProposalType::EpiphanyEvent,
                        PolicyType::KardashevAcceleration => crate::council::proposal::ProposalType::KardashevAcceleration,
                        PolicyType::HarmonyBoost => crate::council::proposal::ProposalType::HarmonyBoost,
                        PolicyType::General => crate::council::proposal::ProposalType::General,
                    },
                    title: policy.title.clone(),
                    effect_type: format!("{:?}", policy.policy_type),
                    mercy_factor: 0.75,
                    status: crate::council::proposal::ProposalStatus::Passed,
                    created_tick: policy.created_tick,
                    realm_id: 0,
                    proposer: 0,
                    target_interest_zone: policy.target_interest_zone,
                    strength: policy.strength,
                };

                match policy.policy_type {
                    PolicyType::ResourcePolicy => {
                        apply_resource_policy_impact(&dummy_decision, world);
                        result.resource_policy_impacts += 1;
                    }
                    PolicyType::EpiphanyEvent => {
                        apply_epiphany_policy_impact(&dummy_decision, world);
                        result.epiphany_policy_impacts += 1;
                    }
                    _ => {}
                }
            }
        }

        #[cfg(feature = "gpu")]
        {
            if self.current_tick % 30 == 0 {
                if let Some(response) = self.request_gpu_foresight(world) {
                    result.gpu_foresight_used = true;
                    if self.economic_layer.apply_gpu_regen_adjustments(&response, world) {
                        result.gpu_foresight_applied = true;
                        info!("GPU PATSAGi foresight applied via EconomicLayer at tick {}", self.current_tick);
                    }
                }
            }
        }

        result.synergy_events = self.collect_synergy_events_direct(world, player_save);
        let _ = interest_manager;

        if let Some(bridge) = self.soft_feedback_bridge.as_mut() {
            let valence = result.estimated_mercy_flow().clamp(0.0, 1.0);
            let raw_energy = (1.2 - valence as f64 * 0.8).max(0.05);
            let zone = (self.current_tick as usize) % 8;
            let _ev = bridge.report_zone_grief(zone, raw_energy, valence, self.current_tick as usize);
            result.soft_feedback_events = 1;
        }

        result
    }

    pub fn run_tick_with_telemetry(
        &mut self,
        world: &mut SovereignWorldState,
        interest_manager: Option<&crate::spatial_interest::InterestManager>,
        council_manager: Option<&mut CouncilSessionManager>,
        player_save: Option<&mut PlayerSaveData>,
        council_decisions: Option<&CouncilDecisions>,
        telemetry: &mut TelemetryCollector,
    ) -> TickResult {
        let result = self.run_tick(
            world,
            interest_manager,
            council_manager,
            player_save,
            council_decisions,
        );

        telemetry.record_tick_result(
            result.tick,
            result.estimated_mercy_flow(),
            result.council_participant_count,
            result.epiphany_policy_impacts,
            result.harvest_nodes_processed.max(result.resource_policy_impacts),
            !result.errors.is_empty(),
        );

        if result.soft_feedback_events > 0 {
            telemetry.current.custom_metrics.insert(
                "soft_feedback_events".into(),
                result.soft_feedback_events as f32,
            );
            if let Some(bridge) = self.soft_feedback_bridge.as_ref() {
                let snaps = bridge.soft_zone_snapshots();
                let total_grief: f64 = snaps.iter().map(|z| z.grief_absorbed).sum();
                let max_stress = snaps.iter().map(|z| z.stress_ema).fold(0.0_f64, f64::max);
                let total_purify: usize = snaps.iter().map(|z| z.purify_count).sum();
                let mean_period = if snaps.is_empty() {
                    0.0
                } else {
                    snaps.iter().map(|z| z.effective_period as f64).sum::<f64>() / snaps.len() as f64
                };
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_total_grief".into(),
                    total_grief as f32,
                );
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_max_stress".into(),
                    max_stress as f32,
                );
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_purify_count".into(),
                    total_purify as f32,
                );
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_mean_period".into(),
                    mean_period as f32,
                );
                let max_rho = snaps.iter().map(|z| z.last_rho).fold(0.0_f64, f64::max);
                let scale = 500.0_f64;
                let purity_term = 1.0 / (1.0 + max_rho * 1e12);
                let stress_term = 1.0 / (1.0 + max_stress / scale);
                let health_score = (purity_term * stress_term).clamp(0.0, 1.0);
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_health_score".into(),
                    health_score as f32,
                );
                let zones_healthy = snaps
                    .iter()
                    .filter(|z| z.status == crate::ra_thor_bridge::ZoneHealthStatus::Healthy)
                    .count();
                let zones_stressed = snaps
                    .iter()
                    .filter(|z| z.status == crate::ra_thor_bridge::ZoneHealthStatus::Stressed)
                    .count();
                let zones_critical = snaps
                    .iter()
                    .filter(|z| z.status == crate::ra_thor_bridge::ZoneHealthStatus::Critical)
                    .count();
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_zones_healthy".into(),
                    zones_healthy as f32,
                );
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_zones_stressed".into(),
                    zones_stressed as f32,
                );
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_zones_critical".into(),
                    zones_critical as f32,
                );
                let critical_auto: usize = snaps
                    .iter()
                    .map(|z| z.critical_auto_purify_count)
                    .sum();
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_critical_auto".into(),
                    critical_auto as f32,
                );
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_valence_high".into(),
                    bridge.valence_high_count as f32,
                );
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_valence_mid".into(),
                    bridge.valence_mid_count as f32,
                );
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_valence_low".into(),
                    bridge.valence_low_count as f32,
                );
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_mercy_ratio".into(),
                    bridge.valence_mercy_ratio() as f32,
                );
                let soft_rem: usize = snaps
                    .iter()
                    .map(|z| z.soft_remediate_count)
                    .sum();
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_soft_remediates".into(),
                    soft_rem as f32,
                );
                let tick = result.tick.max(1) as f64;
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_grief_per_tick".into(),
                    (total_grief / tick) as f32,
                );
                let total_vectors: usize = snaps.iter().map(|z| z.vectors_processed).sum();
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_vectors_per_tick".into(),
                    (total_vectors as f64 / tick) as f32,
                );
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_soft_remediate_rate".into(),
                    (soft_rem as f64 / tick) as f32,
                );
                telemetry.current.custom_metrics.insert(
                    "soft_feedback_critical_auto_rate".into(),
                    (critical_auto as f64 / tick) as f32,
                );
            }
        }

        result
    }

    #[cfg(feature = "gpu")]
    pub fn request_gpu_foresight(&self, world: &SovereignWorldState) -> Option<GpuPatsagiResponse> {
        let bridge = self.gpu_foresight.as_ref()?;
        let node_ids: Vec<u64> = world.agents.keys().copied().collect();
        let request = GpuPatsagiRequest {
            query: "economic_foresight".to_string(),
            intensity: ComputeIntensity::Medium,
            context: Default::default(),
            node_ids,
            harvesting_pressure: None,
        };
        match bridge.run_simulation(request) {
            Ok(response) => Some(response),
            Err(e) => {
                warn!("GPU PATSAGi foresight request failed: {}", e);
                None
            }
        }
    }

    fn collect_synergy_events_direct(
        &self,
        world: &SovereignWorldState,
        mut player_save: Option<&mut PlayerSaveData>,
    ) -> Vec<SynergyEffectEvent> {
        let mut events = Vec::new();
        for agent in world.agents.values() {
            let ability_tree = &agent.ability_tree;
            let active_mutations = agent.get_active_mutations();
            let unlocked_races = agent.get_unlocked_races();
            let mut synergies = ability_tree.calculate_mutation_synergy_chains(active_mutations);
            synergies.extend(
                ability_tree.calculate_cross_race_synergy_chains(active_mutations, unlocked_races)
            );
            if synergies.is_empty() {
                continue;
            }
            let new_events = ability_tree.apply_synergy_bonuses_to_profile(
                self.current_tick,
                agent.id,
                &mut agent.epigenetic_profile.clone(),
                &synergies,
            );
            if let Some(save) = &mut player_save {
                if self.current_tick % 5 == 0 {
                    let last_event = new_events.last();
                    let (vol_delta, str_delta, coop_delta, stage) = if let Some(ev) = last_event {
                        (ev.volatility_delta, ev.strength_delta, ev.cooperation_delta, ev.stage)
                    } else {
                        (0.0, 0.0, 0.0, 0)
                    };
                    save.record_agent_ability_state(
                        agent.id,
                        &agent.ability_tree.chain_progress,
                        stage,
                        vol_delta,
                        str_delta,
                        coop_delta,
                        self.current_tick,
                    );
                }
            }
            events.extend(new_events);
        }
        events
    }
}

// Thunder locked in. Yoi ⚡
