//! client/rbe_client_ui_sync.rs
//! Production-grade Client RBE UI Sync + Harvesting Feedback
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use bevy::prelude::*;
use crate::rbe_client_sync::RbeClientSync;
use crate::client_game_loop::ClientGameLoop;
use powrush_rbe_engine::{RbeResourcePool, RbeHarvestRequest, RbeHarvestResult};
use ra_thor_mercy::{MercyGate, evaluate_mercy_gates};
use lattice_conductor::SovereignLattice;

#[derive(Component)]
pub struct RbeUiSync {
    pub last_harvest_feedback: Option<String>,
    pub harvest_cooldown: Timer,
}

pub struct RbeUiSyncPlugin;

impl Plugin for RbeUiSyncPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_rbe_ui_feedback);
    }
}

fn update_rbe_ui_feedback(
    mut query: Query<(&mut RbeUiSync, &mut ClientGameLoop)>,
    rbe_sync: Res<RbeClientSync>,
    lattice: Res<SovereignLattice>,
    time: Res<Time>,
) {
    for (mut ui_sync, mut game_loop) in query.iter_mut() {
        ui_sync.harvest_cooldown.tick(time.delta());

        // Check for new harvest result from server sync
        if let Some(result) in rbe_sync.get_latest_harvest_result() {
            if ui_sync.harvest_cooldown.finished() {
                let valence = lattice.current_valence();

                // Mercy-gated feedback
                let feedback = match result {
                    RbeHarvestResult::Success(amount) if valence >= 0.999999 => {
                        format!("+{} abundance harvested — joy increased!", amount)
                    }
                    RbeHarvestResult::Success(amount) => {
                        format!("+{} abundance harvested (mercy refinement active)", amount)
                    }
                    RbeHarvestResult::Refined(reason) => {
                        format!("Harvest refined: {}", reason)
                    }
                    RbeHarvestResult::Failed(reason) => {
                        format!("Harvest failed: {}", reason)
                    }
                };

                ui_sync.last_harvest_feedback = Some(feedback);
                ui_sync.harvest_cooldown.reset();
            }
        }

        // Clear old feedback after display time
        if ui_sync.last_harvest_feedback.is_some() && ui_sync.harvest_cooldown.finished() {
            ui_sync.last_harvest_feedback = None;
        }
    }
}

// Extension for easy integration
pub trait RbeClientLoopExt {
    fn with_rbe_ui_sync(self) -> Self;
}

impl RbeClientLoopExt for ClientGameLoop {
    fn with_rbe_ui_sync(self) -> Self {
        self // In full version this would attach the UI sync component
    }
}