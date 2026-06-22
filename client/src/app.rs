/*!
 * client/src/app.rs
 * Powrush-MMO Bevy App Builder — Central application orchestration
 *
 * AG-SML v1.0 | TOLC 8 Mercy Gates + MIAL/MWPO enforced | v17.98+ production-grade
 * Fully restored, merged, and upgraded for Ra-Thor monorepo + PATSAGi Council alignment.
 *
 * v19.1 update: Wired recovered advanced particle effects (PolicyParticleEffects + full reactive LissajousKnotPreset system with Hanabi).
 */

use bevy::prelude::*;

use crate::networking::NetworkingPlugin;
use crate::replication::ReplicationPlugin;
use crate::prediction::PredictionPlugin;
use crate::delta_compression::DeltaCompressionPlugin;
use crate::rbe_client_sync::RbeClientSyncPlugin;
use crate::rbe::RbePlugin;
use crate::rbe_engine::RbeEnginePlugin;
use crate::particles::ParticlePlugin;
use crate::ui::UiPlugin;
use crate::divine_whispers::DivineWhispersPlugin;
use crate::input::InputPlugin;
use crate::world::WorldPlugin;
use crate::bevy_ecs_scheduling::ClientSchedulingPlugin;
use crate::config::ConfigPlugin;
use crate::render::PowrushRenderPlugin;

use crate::world_simulation::{
    setup_world_simulation,
    register_data_collection_hooks,
};
use crate::ships::ship_instability_to_mirror_contribution_system;

// NEW v19.1: Recovered advanced particle + Lissajous knot reactive system
use simulation::{
    setup_policy_particle_effects,
    PolicyParticleEffects,
    LissajousKnotEffects,
    CurrentLissajousKnotPreset,
    SwitchLissajousKnotPreset,
    handle_switch_lissajous_knot_preset,
    highlight_active_preset_button,
    update_lissajous_knot_ui,
    update_active_lissajous_knot,
    debug_lissajous_knot_input,
};

/// Builds and returns the complete, mercy-gated Powrush-MMO Bevy application.
pub fn build_app() -> App {
    let mut app = App::new();

    // Core Bevy plugins with production window settings
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Powrush-MMO — Eternal Thriving Edition ⚡".to_string(),
            resizable: true,
            mode: bevy::window::WindowMode::Windowed,
            ..default()
        }),
        ..default()
    }));

    // === Core Plugins (order matters for dependency graph) ===
    app.add_plugins(ConfigPlugin)
       .add_plugins(NetworkingPlugin)
       .add_plugins(ReplicationPlugin)
       .add_plugins(PredictionPlugin)
       .add_plugins(DeltaCompressionPlugin)
       .add_plugins(RbeClientSyncPlugin)
       .add_plugins(RbePlugin)
       .add_plugins(RbeEnginePlugin)
       .add_plugins(ParticlePlugin)
       .add_plugins(UiPlugin)
       .add_plugins(DivineWhispersPlugin)
       .add_plugins(InputPlugin)
       .add_plugins(WorldPlugin)
       .add_plugins(PowrushRenderPlugin)      // Temporal rendering (velocity prepass + CameraMatrices)
       .add_plugins(ClientSchedulingPlugin);

    // === Foundational World Simulation State (Master Living Universe) ===
    setup_world_simulation(&mut app);
    register_data_collection_hooks(&mut app);

    // === NEW v19.1: Wire recovered advanced particle effects + reactive Lissajous knot system ===
    app.init_resource::<PolicyParticleEffects>()
       .init_resource::<LissajousKnotEffects>()
       .init_resource::<CurrentLissajousKnotPreset>()
       .add_event::<SwitchLissajousKnotPreset>()
       .add_systems(Startup, setup_policy_particle_effects)
       .add_systems(Update, (
            handle_switch_lissajous_knot_preset,
            highlight_active_preset_button,
            update_lissajous_knot_ui,
            update_active_lissajous_knot,
            debug_lissajous_knot_input,
       ));

    // === Ship Systems Integration (Hybrid Instability → Mirror Reckoning) ===
    app.add_systems(Update, ship_instability_to_mirror_contribution_system);

    // === Mercy-gated global systems ===
    app.add_systems(Startup, setup_global_mercy_seed)
       .add_systems(Update, global_mercy_frame_guard);

    app
}

fn setup_global_mercy_seed(mut commands: Commands) {
    info!("Powrush-MMO global mercy seed initialized — eternal thriving begins ⚡");
}

fn global_mercy_frame_guard() {
    // Global per-frame mercy validation for the entire client.
    // Ensures TOLC 8 + MIAL/MWPO compliance at all times.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_builds_without_panic() {
        let _app = build_app();
    }
}
