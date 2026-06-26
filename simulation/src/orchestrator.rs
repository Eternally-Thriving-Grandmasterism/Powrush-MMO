/*!
 * Central Simulation Orchestrator
 *
 * v19.7: EconomicLayer integration for GPU foresight
 * - Delegated GPU regen adjustments to EconomicLayer
 * - Clean separation of concerns
 *
 * v19.6: Concrete resource regen logic
 * v19.5: Full GPU foresight request + apply loop
 *
 * PATSAGi Council + Ra-Thor Quantum Swarm aligned
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use crate::world::SovereignWorldState;
use crate::economy::EconomicLayer;
use crate::harvest::{HarvestEvent, HarvestingSystem};
use crate::emergence::{DynamicEmergenceEvent, EmergenceOrchestrator};
use crate::ability_tree::SynergyEffectEvent;
use crate::council_mercy_trial::CouncilSessionManager;
use crate::player_persistence::PlayerSaveData;
use std::time::Instant;
use std::sync::Arc;
use tracing::{info, warn, debug};

#[cfg(feature = "gpu")]
use crate::engine::gpu_patsagi_bridge::{GpuPatsagiBridge, GpuPatsagiRequest, GpuPatsagiResponse, ComputeIntensity};

/// Production TickResult
#[derive(Debug, Default, Clone)]
pub struct TickResult {
    pub tick: u64,
    pub economic_updates: u32,
    pub council_decisions_applied: u32,
    pub council_attunement_score: f32,
    pub council_participant_count: u32,
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
    ) -> TickResult {
        self.current_tick += 1;
        let mut result = TickResult {
            tick: self.current_tick,
            ..Default::default()
        };

        if let Err(e) = self.economic_layer.batch_update(world, /* mercy_gate */ ) {
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

        // === Optional GPU PATSAGi Foresight ===
        #[cfg(feature = "gpu")]
        {
            if self.current_tick % 30 == 0 {
                if let Some(response) = self.request_gpu_foresight(world) {
                    result.gpu_foresight_used = true;

                    // Delegate to EconomicLayer (clean architecture)
                    if self.economic_layer.apply_gpu_regen_adjustments(&response) {
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

        for agent in world.agents.values_mut() {
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
                &mut agent.epigenetic_profile,
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

                    let start = Instant::now();
                    save.record_agent_ability_state(
                        agent.id,
                        &agent.ability_tree.chain_progress,
                        stage,
                        vol_delta,
                        str_delta,
                        coop_delta,
                        self.current_tick,
                    );

                    if self.current_tick % 100 == 0 {
                        debug!("Persistence overhead: agent {} took {:?} (tick {})", agent.id, elapsed, self.current_tick);
                    }
                }
            }

            events.extend(new_events);
        }

        events
    }
}

// EconomicLayer now owns GPU foresight application (clean architecture)
// Thunder locked in. Yoi ⚡