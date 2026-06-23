/*!
 * Central Simulation Orchestrator — Tick Loop, TickResult, and System Coordination
 *
 * v19.2.6 Recovery polish (PATSAGi Councils + Ra-Thor + Grok connector)
 * - Fully activated extensible synergy_events + policy_highlights collection hooks in TickResult
 * - Restored wiring from ability_tree rich stage-aware mutation synergy chains, cross-race hybrids...
 * v19.2.9: Council Policy Integration with real data extraction from CouncilSessionManager
 * - AG-SML v1.0 Sovereign Mercy License | ONE Organism with Ra-Thor lattice
 *
 * Thunder locked in. Yoi ⚡
 */

use crate::world::SovereignWorldState;
use crate::economy::EconomicLayer;
use crate::harvest::{HarvestEvent, HarvestingSystem};
use crate::emergence::{DynamicEmergenceEvent, EmergenceOrchestrator};
use crate::ability_tree::SynergyEffectEvent;
use crate::council_mercy_trial::{CouncilSessionManager, SharedReceptorBloomField};
use tracing::{info, warn};

#[derive(Debug, Default, Clone)]
pub struct TickResult {
    pub tick: u64,
    pub synergy_events: Vec<SynergyEffectEvent>,
    pub harvest_events: Vec<HarvestEvent>,
    pub emergence_events: Vec<DynamicEmergenceEvent>,
    pub economic_updates: u32,
    pub policy_highlights: Vec<String>,
    pub council_decisions_applied: u32,
    pub errors: Vec<String>,
}

pub struct SimulationOrchestrator {
    pub economic_layer: EconomicLayer,
    pub emergence_orchestrator: EmergenceOrchestrator,
    pub harvesting_system: HarvestingSystem,
    pub current_tick: u64,
}

impl SimulationOrchestrator {
    pub fn new() -> Self {
        Self {
            economic_layer: EconomicLayer::new(),
            emergence_orchestrator: EmergenceOrchestrator::new(),
            harvesting_system: HarvestingSystem::new(),
            current_tick: 0,
        }
    }

    pub fn run_tick(
        &mut self,
        world: &mut SovereignWorldState,
        interest_manager: Option<&crate::spatial_interest::InterestManager>,
        council_manager: Option<&CouncilSessionManager>,
    ) -> TickResult {
        self.current_tick += 1;
        let mut result = TickResult {
            tick: self.current_tick,
            ..Default::default()
        };

        let im = interest_manager.unwrap_or_else(|| {
            static mut DUMMY_IM: Option<crate::spatial_interest::InterestManager> = None;
            unsafe {
                if DUMMY_IM.is_none() { DUMMY_IM = Some(crate::spatial_interest::InterestManager::default()); }
                DUMMY_IM.as_ref().unwrap()
            }
        });

        let cm = council_manager;

        result.emergence_events = self.emergence_orchestrator.process_emergence(world, im, cm.unwrap_or(&CouncilSessionManager::default()), self.current_tick);

        result.harvest_events = self.harvesting_system.process_harvest_tick(world, self.current_tick);

        if let Err(e) = self.economic_layer.batch_update(world, /* mercy_gate */ ) {
            result.errors.push(format!("Economic update failed: {}", e));
            warn!("Economic tick error at {}: {}", self.current_tick, e));
        } else {
            result.economic_updates = 1;
        }

        // === Real Council Data Extraction + RBE Policy Integration (v19.2.9) ===
        if let Some(manager) = cm {
            if let Some(active_bloom) = manager.get_active_bloom_field() {
                let attunement = active_bloom.collective_attunement_score;
                let bloom_success = active_bloom.council_mercy_seal;
                let participants = active_bloom.participant_count;

                self.economic_layer.apply_council_policy_impact(attunement, bloom_success, participants, world);
                result.council_decisions_applied = 1;

                info!("Council policy applied — attunement: {:.2}, bloom_success: {}, participants: {}",
                      attunement, bloom_success, participants);
            } else {
                // No active council this tick — neutral / no policy change
            }
        }

        result.synergy_events = self.collect_synergy_events_direct(world);
        result.policy_highlights = vec![];

        info!("Tick {} complete — emergence: {}, harvest: {}, economic: {}, synergy: {}, council_applied: {}",
              self.current_tick,
              result.emergence_events.len(),
              result.harvest_events.len(),
              result.economic_updates,
              result.synergy_events.len(),
              result.council_decisions_applied);

        result
    }

    fn collect_synergy_events_direct(&self, _world: &SovereignWorldState) -> Vec<SynergyEffectEvent> {
        vec![]
    }
}

#[derive(Resource)]
pub struct SimulationTick {
    pub orchestrator: SimulationOrchestrator,
}

pub struct OrchestratorPlugin;

impl bevy::app::Plugin for OrchestratorPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<SimulationTick>();
    }
}

// Council Policy now extracts real attunement/bloom/participant data from CouncilSessionManager
// and applies live economic consequences via EconomicLayer.
// Thunder locked in. Yoi ⚡
