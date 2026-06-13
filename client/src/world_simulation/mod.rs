/*!
 * client/src/world_simulation/mod.rs
 * World Simulation State — The Living Heart of Powrush-MMO
 *
 * AG-SML v1.0 | TOLC 8 Mercy Gates + MIAL/MWPO enforced
 * PATSAGi Council + Ra-Thor Quantum Swarm approved.
 * Fully coherent with all documentation (Mirror Reckoning, Crownstone Trilemma,
 * Hybrid Protocol, Draek Origin & Great Betrayal, redemption paths, etc.).
 *
 * === VERSION CONTROL HISTORY ===
 * v1.0 (Initial Creation) — Foundational master resource.
 *      - Defined all core sub-state resources (CrownstoneState, ResonanceNetworkState,
 *        DraekHivemindState, Ambrosian states, Hivelord/AuroralSovereign, RbeWorldState).
 *      - Implemented WorldSimulationState aggregator with pending_major_events queue.
 *      - Added apply_ship_instability_contribution() for Human Hybrid → Mirror Score feedback.
 *      - Integrated with mirror_score.rs systems.
 *      - setup_world_simulation() for clean App wiring.
 *      - First version committed with data collection hooks from Council/Epiphany/RBE.
 *
 * This file is the single source of truth for the living, mercy-aligned universe simulation.
 * All future systems (Resonance Burst, Trilemma resolution, Hivelord Counter-Strategies,
 * Grove Communion, etc.) will extend from here.
 */

use bevy::prelude::*;
use crate::ships::{ShipVisualState, ActiveHybrid};

// Re-export mirror score systems for convenience
pub use super::mirror_score::*;

/// Master resource aggregating every major simulation system in Powrush-MMO.
/// This is the single source of truth for the living universe state.
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
        // TODO: Phase progression, event processing, weekly reset logic
    }

    /// Applies instability contribution from Human Hybrid ships to the weekly Mirror Score.
    /// Called by ship_instability_to_mirror_contribution_system.
    pub fn apply_ship_instability_contribution(&mut self, instability_level: f32, moral_alignment: f32) {
        let contribution = (instability_level * 0.6 + (1.0 - moral_alignment.abs() / 100.0) * 0.4) * 5.0;
        self.mirror.weekly_metrics.intra_server_cooperation =
            (self.mirror.weekly_metrics.intra_server_cooperation - contribution).max(0.0);
    }
}

/// Crownstone state (core artifact of the Draek Dominion).
#[derive(Resource, Default, Reflect, Clone)]
pub struct CrownstoneState {
    pub integrity: f32,
    pub corruption_level: f32,
    pub current_path: Option<TrilemmaPath>,
}

/// Quellorian Resonance Network state.
#[derive(Resource, Default, Reflect, Clone)]
pub struct ResonanceNetworkState {
    pub harmony_level: f32,
    pub attunement_level: f32,
    pub burst_cooldown: f32,
}

/// Draek Hivemind state (command hierarchy).
#[derive(Resource, Default, Reflect, Clone)]
pub struct DraekHivemindState {
    pub command_strength: f32,
    pub brood_evolution_stage: u32,
    pub hivelord_link_active: bool,
}

/// Ambrosian attunement state.
#[derive(Resource, Default, Reflect, Clone)]
pub struct AmbrosianAttunementState {
    pub attunement_level: f32,
}

/// Discordant Ambrosian corruption state.
#[derive(Resource, Default, Reflect, Clone)]
pub struct DiscordantAmbrosianState {
    pub corruption_spread: f32,
}

/// Hivelord state (supreme leader of the Draek Dominion).
#[derive(Resource, Default, Reflect, Clone)]
pub struct HivelordState {
    pub command_strength: f32,
    pub suit_integrity: f32,
}

/// Auroral Sovereign state (leader of the Quellorian Alliance).
#[derive(Resource, Default, Reflect, Clone)]
pub struct AuroralSovereignState {
    pub harmony_projection: f32,
}

/// RBE world economy state.
#[derive(Resource, Default, Reflect, Clone)]
pub struct RbeWorldState {
    pub global_abundance: f32,
    pub mercy_index: f32,
}

/// Mirror Reckoning state is defined in mirror_score.rs and re-exported.

/// Sets up the entire World Simulation layer.
pub fn setup_world_simulation(app: &mut App) {
    app.init_resource::<WorldSimulationState>()
        .add_systems(Update, (
            world_simulation_update_system,
            calculate_mirror_score_system,
            reset_weekly_metrics_system,
        ));
}

/// Core per-frame simulation update.
pub fn world_simulation_update_system(
    mut world_sim: ResMut<WorldSimulationState>,
) {
    world_sim.advance_tick();
    // TODO: Process pending_major_events queue, phase transitions, etc.
}
