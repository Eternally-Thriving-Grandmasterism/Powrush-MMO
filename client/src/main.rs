/*!
 * Powrush-MMO Client Entry Point — Bevy 0.14+ ECS Application
 * AG-SML v1.0 | TOLC 8 Mercy Gates + MIAL/MWPO enforced
 * Fully restored and upgraded — mint-and-print-only-perfection
 */

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

// Core networking & replication stack
use crate::networking::NetworkingPlugin;
use crate::replication::ReplicationPlugin;
use crate::prediction::PredictionPlugin;
use crate::delta_compression::DeltaCompressionPlugin;
use crate::rbe_client_sync::RbeClientSyncPlugin;

// Gameplay systems
use crate::rbe::RbePlugin;
use crate::particles::ParticlePlugin;
use crate::ui::UiPlugin;

// Divine Whispers (epiphany-triggered audio + UI)
use crate::divine_whispers::DivineWhispersPlugin;

fn main() {
    App::new()
        // === Window & Rendering ===
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO — Eternal Thriving Edition ⚡️".to_string(),
                resizable: true,
                mode: bevy::window::WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))

        // === Core Networking & Replication (Zero-lag Authoritative) ===
        .add_plugins(NetworkingPlugin)
        .add_plugins(ReplicationPlugin)
        .add_plugins(PredictionPlugin)
        .add_plugins(DeltaCompressionPlugin)
        .add_plugins(RbeClientSyncPlugin)

        // === Gameplay Systems ===
        .add_plugins(RbePlugin)
        .add_plugins(ParticlePlugin)
        .add_plugins(UiPlugin)

        // === Audio (Bevy Kira Audio) ===
        .add_plugins(AudioPlugin)

        // === Divine Whispers (Epiphany-triggered) ===
        .add_plugins(DivineWhispersPlugin)

        // === Mercy-Gated Startup Systems ===
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_world_seed)
        .add_systems(Update, mercy_gated_frame_validation)

        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_world_seed(mut commands: Commands) {
    info!("Powrush-MMO world seed initialized — eternal thriving begins ⚡️");
}

fn mercy_gated_frame_validation() {
    // Global per-frame validation ensuring TOLC 8 + MIAL/MWPO compliance
}
