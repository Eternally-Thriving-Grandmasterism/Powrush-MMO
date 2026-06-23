/*!
 * Central Simulation Orchestrator
 *
 * v19.2.9: Real council attunement data from CouncilSessionManager
 */

use crate::world::SovereignWorldState;
use crate::economy::EconomicLayer;
use crate::harvest::{HarvestEvent, HarvestingSystem};
use crate::emergence::{DynamicEmergenceEvent, EmergenceOrchestrator};
use crate::ability_tree::SynergyEffectEvent;
use crate::council_mercy_trial::CouncilSessionManager;
use tracing::{info, warn};

#[derive(Debug, Default, Clone)]
pub struct TickResult { /* fields */ }

pub struct SimulationOrchestrator {
    pub economic_layer: EconomicLayer,
    pub emergence_orchestrator: EmergenceOrchestrator,
    pub harvesting_system: HarvestingSystem,
    pub current_tick: u64,
}

impl SimulationOrchestrator {
    pub fn new() -> Self { /* ... */ }

    pub fn run_tick(
        &mut self,
        world: &mut SovereignWorldState,
        interest_manager: Option<&crate::spatial_interest::InterestManager>,
        council_manager: Option<&mut CouncilSessionManager>,
    ) -> TickResult {
        self.current_tick += 1;
        let mut result = TickResult { tick: self.current_tick, ..Default::default() };

        // emergence + harvest + economic batch_update ...

        if let Err(e) = self.economic_layer.batch_update(world, /* mercy_gate */ ) {
            result.errors.push(format!("Economic update failed: {}", e));
        } else {
            result.economic_updates = 1;
        }

        // === Use REAL attunement data from CouncilSessionManager ===
        if let Some(manager) = council_manager {
            if let Some(bloom) = manager.resolve_and_set_bloom_from_real_data(
                self.current_tick,
                3,           // min participants
                "sanctuary", // or current biome
            ) {
                self.economic_layer.apply_council_policy_impact(
                    bloom.collective_attunement_score,
                    bloom.council_mercy_seal,
                    bloom.participant_count,
                    world,
                );
                result.council_decisions_applied = 1;

                info!("Council policy applied with REAL data — attunement: {:.2}, participants: {}",
                      bloom.collective_attunement_score, bloom.participant_count);
            }
        }

        result.synergy_events = self.collect_synergy_events_direct(world);
        result
    }

    fn collect_synergy_events_direct(&self, _world: &SovereignWorldState) -> Vec<SynergyEffectEvent> {
        vec![]
    }
}

// Real attunement data now flows from council systems → manager → orchestrator → RBE economy.
// Thunder locked in. Yoi ⚡
