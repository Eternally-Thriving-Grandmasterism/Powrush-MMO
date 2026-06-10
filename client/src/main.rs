/*!
 * Powrush-MMO Client Entry Point
 */

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use simulation::cloud_sync::CloudSync;

use crate::networking::NetworkingPlugin;
use crate::replication::ReplicationPlugin;
use crate::prediction::PredictionPlugin;
use crate::delta_compression::DeltaCompressionPlugin;
use crate::rbe_client_sync::RbeClientSyncPlugin;
use crate::rbe::RbePlugin;
use crate::particles::ParticlePlugin;
use crate::ui::UiPlugin;
use crate::divine_whispers::DivineWhispersPlugin;
use crate::player_progress_ui::PlayerProgressUIPlugin;
use crate::spatial_audio::{SpatialAudioPlugin, SpatialListener};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO — Eternal Thriving Edition ⚡".to_string(),
                resizable: true,
                mode: bevy::window::WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))

        .add_plugins(NetworkingPlugin)
        .add_plugins(ReplicationPlugin)
        .add_plugins(PredictionPlugin)
        .add_plugins(DeltaCompressionPlugin)
        .add_plugins(RbeClientSyncPlugin)

        .add_plugins(RbePlugin)
        .add_plugins(ParticlePlugin)
        .add_plugins(UiPlugin)

        .add_plugins(AudioPlugin)
        .add_plugins(DivineWhispersPlugin)
        .add_plugins(PlayerProgressUIPlugin)

        // Performance-optimized Spatial Audio
        .add_plugins(SpatialAudioPlugin)

        .init_resource::<CloudSync>()
        .add_systems(Startup, init_cloud_sync)

        .add_systems(Startup, setup_spatial_listener)

        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_world_seed)
        .add_systems(Update, mercy_gated_frame_validation)

        .run();
}
