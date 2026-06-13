/*!
 * World Simulation State — The Living Heart of Powrush-MMO
 *
 * Master resource aggregating Crownstone, Resonance, Hivemind, Mirror Reckoning,
 * RBE, and all major simulation systems.
 * PATSAGi Council + Ra-Thor Quantum Swarm approved.
 */ 

use bevy::prelude::*;
use crate::ships::{ShipVisualState, ActiveHybrid};

// Re-export mirror score systems
pub use super::mirror_score::*;

#[derive(Resource, Default, Reflect)]
pub struct WorldSimulationState {
    pub crownstone: CrownstoneState,
    pub resonance: ResonanceNetworkState,
    pub draek_hivemind: DraekHivemindState,
    pub ambrosian: AmbrosianAttunementState,
    pub discordant: DiscordantAmbrosianState,
    pub hivelord: HivelordState,
    pub auroral_sovereign: AuroralSovereignState,
    pub mirror: MirrorReckoningState,
    pub rbe: RbeWorldState,
    pub pending_major_events: Vec<String>,
    pub current_tick: u64,
}

impl WorldSimulationState {
    pub fn advance_tick(&mut self) {
        self.current_tick += 1;
        // Phase progression logic can be expanded here
        if self.current_tick % 1000 == 0 {
            // Example: trigger minor simulation events
        }
    }

    /// Applies instability contribution from ships to the weekly Mirror Score
    pub fn apply_ship_instability_contribution(&mut self, instability_level: f32, moral_alignment: f32) {
        let contribution = (instability_level * 0.6 + (1.0 - moral_alignment.abs() / 100.0) * 0.4) * 5.0;
        self.mirror.weekly_metrics.intra_server_cooperation = (self.mirror.weekly_metrics.intra_server_cooperation - contribution).max(0.0);
        // This feeds into calculate_mirror_score
    }
}

#[derive(Resource, Default, Reflect, Clone)]
pub struct CrownstoneState { /* ... fields from previous */ pub integrity: f32, pub corruption_level: f32 }

#[derive(Resource, Default, Reflect, Clone)]
pub struct ResonanceNetworkState { pub harmony_level: f32 }

#[derive(Resource, Default, Reflect, Clone)]
pub struct DraekHivemindState { pub command_strength: f32 }

#[derive(Resource, Default, Reflect, Clone)]
pub struct AmbrosianAttunementState { pub attunement_level: f32 }

#[derive(Resource, Default, Reflect, Clone)]
pub struct DiscordantAmbrosianState { pub corruption_spread: f32 }

#[derive(Resource, Default, Reflect, Clone)]
pub struct HivelordState { pub command_strength: f32 }

#[derive(Resource, Default, Reflect, Clone)]
pub struct AuroralSovereignState { pub harmony_projection: f32 }

#[derive(Resource, Default, Reflect, Clone)]
pub struct RbeWorldState { pub global_abundance: f32, pub mercy_index: f32 }

// MirrorReckoningState is defined in mirror_score.rs and re-exported

pub fn setup_world_simulation(app: &mut App) {
    app.init_resource::<WorldSimulationState>()
        .add_systems(Update, (
            world_simulation_update_system,
            calculate_mirror_score_system,
            reset_weekly_metrics_system,
        ));
}

pub fn world_simulation_update_system(
    mut world_sim: ResMut<WorldSimulationState>,
) {
    world_sim.advance_tick();
    // Future: process pending_major_events queue
}
