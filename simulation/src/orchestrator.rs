/*!
 * simulation/src/orchestrator.rs
 * Central Simulation Orchestrator (v21.12)
 *
 * Full TOLC 8 MercyGate + EconomicLayer batch_update
 * Integrated with robust PersistenceManager / PlayerSaveData
 * GPU PATSAGi foresight, Council bloom, Emergence, Harvest, Synergy
 * v21.12: Live EpiphanyEvent impact wired via apply_epiphany_policy_impact
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use crate::world::SovereignWorldState;
use crate::economy::EconomicLayer;
use crate::harvest::{HarvestEvent, HarvestingSystem};
use crate::emergence::{DynamicEmergenceEvent, EmergenceOrchestrator};
use crate::ability_tree::SynergyEffectEvent;
use crate::council_mercy_trial::CouncilSessionManager;
use crate::council::decision::{CouncilDecisions, apply_resource_policy_impact, apply_epiphany_policy_impact, PolicyType};
use crate::player_persistence::PlayerSaveData;
use crate::mercy::MercyGate;
use std::time::Instant;
use std::sync::Arc;
use tracing::{info, warn, debug};

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
    pub errors: Vec<String>,
}

pub struct SimulationOrchestrator {
    pub economic_layer: EconomicLayer,
    pub emergence_orchestrator: EmergenceOrchestrator,
    pub harvesting_system: HarvestingSystem,
    pub current_tick: u64,

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

        // Live ResourcePolicy + EpiphanyEvent impacts from CouncilDecisions
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
