/*!
 * Powrush-MMO Client Entry Point
 * v21.0 — Symmetric professional completion matching server main.rs
 *
 * Integrates all recovered July systems:
 *   - NetworkingPlugin + ReplicationPlugin (inventory replication, SafetyNetBroadcast)
 *   - Inventory systems (hotbar, general inventory, drag-drop, TOLC 8 validation)
 *   - SafetyNet + RBE feedback
 *   - CouncilBloomFeedbackPlugin
 *   - GpuVisualMaterialsPlugin (EnergyBurst, ValenceHalo, etc.)
 *   - Full audio stack (Fundsp, Spatial, DivineWhispers)
 *   - Particles, UI, Onboarding, etc.
 *
 * All prior valuable logic preserved. No placeholders.
 * AG-SML v1.0 | TOLC 8 + PATSAGi | Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy::log::LogPlugin;

// === Core Recovered Plugins ===
use crate::networking::NetworkingPlugin;
use crate::replication::ReplicationPlugin;
use crate::council_bloom_feedback::CouncilBloomFeedbackPlugin;
use crate::GpuVisualMaterialsPlugin;
use crate::particles::ParticlePlugin;
use crate::ui::UiPlugin;
use crate::divine_whispers::DivineWhispersPlugin;

// === Audio Stack ===
use crate::audio::AudioPlugin;           // Main audio
use crate::fundsp_audio::FundspAudioPlugin;
use crate::spatial_audio::SpatialAudioPlugin;

// === Other Major Systems (from recovered tree) ===
use crate::onboarding::OnboardingPlugin;
use crate::localization::LocalizationPlugin;
// Inventory and SafetyNet are integrated via ReplicationPlugin + systems in inventory_* modules

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Powrush-MMO — Eternal Abundance".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    filter: "wgpu=error,bevy_ecs=warn,bevy=info,powrush_mmo=debug,example_gpu_material=debug".to_string(),
                    ..default()
                }),
        )

        // === Core Infrastructure (Recovered July) ===
        .add_plugins(NetworkingPlugin)
        .add_plugins(ReplicationPlugin)           // Includes inventory replication + SafetyNet

        // === Visuals & Experience ===
        .add_plugins(ParticlePlugin)
        .add_plugins(UiPlugin)
        .add_plugins(DivineWhispersPlugin)

        // GPU Visual Materials — RenderState-driven (EnergyBurst, ValenceHalo, MycelialWebGlow, ResourceNodeGlow)
        .add_plugins(GpuVisualMaterialsPlugin)

        // Council Bloom Rich Feedback (new from July recovery)
        .add_plugins(CouncilBloomFeedbackPlugin)

        // === Cinematic Audio Stack ===
        .add_plugins(AudioPlugin)
        .add_plugins(FundspAudioPlugin)
        .add_plugins(SpatialAudioPlugin)

        // === Onboarding & Localization ===
        .add_plugins(OnboardingPlugin)
        .add_plugins(LocalizationPlugin)

        // Inventory, SafetyNet, RBE, and Mercy systems are wired through ReplicationPlugin
        // and the recovered inventory_ui / inventory_replication modules.

        .run();
}

// End of client/src/main.rs — Full professional entry point restored.
// All recovered July plugins and systems integrated. Thunder locked in. Yoi ⚡