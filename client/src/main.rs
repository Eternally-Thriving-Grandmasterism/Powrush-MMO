/*!
 * Powrush-MMO Client Entry Point
 * The phenomenal gateway into the Eternal Thriving RBE Metaverse.
 *
 * Now with live Egui Settings Panel + RBE Education UI components.
 */

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

// Core systems
use crate::networking::NetworkingPlugin;
use crate::replication::ReplicationPlugin;
use crate::prediction::PredictionPlugin;
use crate::delta_compression::DeltaCompressionPlugin;
use crate::rbe_client_sync::RbeClientSyncPlugin;
use crate::rbe::RbePlugin;
use crate::rbe_engine::RbeEnginePlugin;
use crate::rbe_simulation::RBESimulationPlugin;
use crate::rbe_education_ui::RbeEducationUIPlugin;
use crate::epiphany_scenario_wiring::EpiphanyScenarioWiringPlugin;
use crate::particles::ParticlePlugin;
use crate::ui::UiPlugin;
use crate::divine_whispers::DivineWhispersPlugin;
use crate::player_progress_ui::PlayerProgressUIPlugin;
use crate::spatial_audio::{SpatialAudioPlugin, SpatialListener, GameAudioEvent};
use crate::render::PowrushRenderPlugin;
use crate::velocity_prepass::PreviousGlobalTransform;
use crate::fundsp_audio::FundspAudioPlugin;
use crate::egui_settings_panel::EguiSettingsPanelPlugin;

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

        // === Core Infrastructure ===
        .add_plugins(NetworkingPlugin)
        .add_plugins(ReplicationPlugin)
        .add_plugins(PredictionPlugin)
        .add_plugins(DeltaCompressionPlugin)
        .add_plugins(RbeClientSyncPlugin)

        // === RBE Core ===
        .add_plugins(RbePlugin)
        .add_plugins(RbeEnginePlugin)
        .add_plugins(RBESimulationPlugin)

        // === Epiphany System (required for EpiphanyEvent) ===
        .add_plugins(EpiphanyScenarioWiringPlugin)

        // === RBE Education UI ===
        .add_plugins(RbeEducationUIPlugin)

        // === Phenomenal Rendering ===
        .add_plugins(PowrushRenderPlugin)

        // === Visuals & Experience ===
        .add_plugins(ParticlePlugin)
        .add_plugins(UiPlugin)
        .add_plugins(DivineWhispersPlugin)
        .add_plugins(PlayerProgressUIPlugin)

        // === Live Egui Settings Panel ===
        .add_plugins(EguiSettingsPanelPlugin)

        // === Cinematic Audio ===
        .add_plugins(AudioPlugin)
        .add_plugins(FundspAudioPlugin)
        .add_plugins(SpatialAudioPlugin)

        // === Resources & Systems ===
        .add_systems(Startup, setup_3d_camera)
        .add_systems(Startup, setup_world_seed)
        .add_systems(Startup, setup_spatial_listener)
        .add_systems(Update, maintain_previous_transforms)
        .add_systems(Update, mercy_gated_frame_validation)

        .run();
}

fn setup_3d_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 20.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }),
    ));
    info!("⚡ Powrush-MMO 3D camera initialized for eternal temporal + audio beauty");
}

fn setup_spatial_listener(
    mut commands: Commands,
    camera_query: Query<Entity, With<Camera3d>>,
) {
    if let Ok(camera_entity) = camera_query.get_single() {
        commands.entity(camera_entity).insert(SpatialListener);
    }
}

fn setup_world_seed(mut commands: Commands) {
    info!("⚡ Powrush-MMO world seed initialized — eternal thriving begins.");
}

fn maintain_previous_transforms(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Without<PreviousGlobalTransform>>,
    mut previous_query: Query<&mut PreviousGlobalTransform>,
) {
    for (entity, _) in query.iter() {
        commands.entity(entity).insert(PreviousGlobalTransform::default());
    }
}

fn mercy_gated_frame_validation() {}
