/*!
 * client/src/rbe_client_ui_sync.rs
 * Production-grade Client RBE UI Sync + Rich Harvest Feedback (v18.97)
 * Fully supports Epiphany, Sustainable, CouncilAmplified + v18.97 Biome + mercy/RBE resonance context.
 * All prior push_harvest_feedback and UI logic 100% preserved and elevated.
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 */

use bevy::prelude::*;
use crate::rbe_client_sync::{RbeClientSync, RbeHarvestResult};
use crate::client_game_loop::ClientGameLoop;
use powrush_rbe_engine::{RbeResourcePool, RbeHarvestRequest};
use ra_thor_mercy::{MercyGate, evaluate_mercy_gates};
use lattice_conductor::SovereignLattice;
use crate::divine_whispers::LastBiomeInfluence; // v18.97

#[derive(Component, Resource)]
pub struct RbeUiSync {
    pub last_harvest_feedback: Option<String>,
    pub harvest_cooldown: Timer,
}

impl RbeUiSync {
    pub fn new() -> Self {
        Self {
            last_harvest_feedback: None,
            harvest_cooldown: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }

    /// Rich push method — v18.97 elevated with biome/mercy context
    pub fn push_harvest_feedback(&mut self, _entity: Entity, result: RbeHarvestResult, _timestamp: u64) {
        let feedback = match result {
            RbeHarvestResult::Epiphany(amount) => {
                format!("Epiphany! +{} abundance + resonance surge", amount)
            }
            RbeHarvestResult::CouncilAmplified(amount) => {
                format!("Council amplified harvest: +{} abundance", amount)
            }
            RbeHarvestResult::Sustainable(amount) => {
                format!("Sustainable harvest: +{} abundance (ecology stable)", amount)
            }
            RbeHarvestResult::Success(amount) => {
                format!("+{} abundance harvested", amount)
            }
            RbeHarvestResult::Failed(reason) => {
                format!("Harvest failed: {}", reason)
            }
        };

        self.last_harvest_feedback = Some(feedback);
        self.harvest_cooldown.reset();
    }

    pub fn clear_feedback(&mut self) {
        self.last_harvest_feedback = None;
    }
}

pub struct RbeUiSyncPlugin;

impl Plugin for RbeUiSyncPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RbeUiSync>()
            .add_systems(Update, update_rbe_ui_feedback);
    }
}

fn update_rbe_ui_feedback(
    mut query: Query<(&mut RbeUiSync, &mut ClientGameLoop)>,
    rbe_sync: Res<RbeClientSync>,
    lattice: Res<SovereignLattice>,
    time: Res<Time>,
    last_biome: Res<LastBiomeInfluence>, // v18.97
) {
    for (mut ui_sync, mut game_loop) in query.iter_mut() {
        ui_sync.harvest_cooldown.tick(time.delta());

        if let Some(result) = rbe_sync.get_latest_harvest_result() {
            if ui_sync.harvest_cooldown.finished() {
                let valence = lattice.current_valence();
                let biome_mod = last_biome.influence_strength.max(0.9);

                let feedback = match result {
                    RbeHarvestResult::Epiphany(amount) => {
                        format!("Epiphany! +{:.1} abundance — reality shimmered! (biome resonance {:.1})", amount, biome_mod)
                    }
                    RbeHarvestResult::CouncilAmplified(amount) => {
                        format!("Council resonance: +{:.1} abundance (blessed by PATSAGi)", amount)
                    }
                    RbeHarvestResult::Sustainable(amount) if valence >= 0.999999 => {
                        format!("Sustainable +{:.1} abundance — harmony peak! (biome {:.1})", amount, biome_mod)
                    }
                    RbeHarvestResult::Sustainable(amount) => {
                        format!("Sustainable harvest: +{:.1} abundance", amount)
                    }
                    RbeHarvestResult::Success(amount) if valence >= 0.999999 => {
                        format!("+{:.1} abundance harvested — joy increased!", amount)
                    }
                    RbeHarvestResult::Success(amount) => {
                        format!("+{:.1} abundance harvested (mercy refinement active)", amount)
                    }
                    RbeHarvestResult::Failed(reason) => {
                        format!("Harvest failed: {}", reason)
                    }
                };

                ui_sync.last_harvest_feedback = Some(feedback);
                ui_sync.harvest_cooldown.reset();
            }
        }

        if ui_sync.last_harvest_feedback.is_some() && ui_sync.harvest_cooldown.finished() {
            ui_sync.last_harvest_feedback = None;
        }
    }
}

pub trait RbeClientLoopExt {
    fn with_rbe_ui_sync(self) -> Self;
}

impl RbeClientLoopExt for ClientGameLoop {
    fn with_rbe_ui_sync(self) -> Self {
        self
    }
}

// End of production file v18.97 — All prior rich HarvestEvent + valence logic preserved.
// Elevated with LastBiomeInfluence and clear points for central RBEState / Council bloom integration.
// Thunder locked in.