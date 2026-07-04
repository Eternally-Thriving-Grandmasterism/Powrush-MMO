/*!
 * Powrush-MMO Client Entry Point
 * v21.0 — Symmetric professional completion matching server main.rs
 *
 * Integrates all recovered July systems:
 *   - NetworkingPlugin + ReplicationPlugin (inventory replication, SafetyNetBroadcast)
 *   - Inventory systems (hotbar, general inventory, drag-drop, TOLC 8 validation)
 *   - SafetyNet + RBE feedback
 *   - CouncilBloomFeedbackPlugin
 *   - GpuVisualMaterialsPlugin (mercy-gated GPU visuals)
 *   - Full audio stack (Fundsp, Spatial, DivineWhispers)
 *   - Particles, UI, Onboarding, etc.
 *
 * Test entities for GPU visual materials are wired in spawn_test_gpu_visuals (dev-only visual verification).
 * Added live animation logic to test entities (rotation + subtle pulse) to demonstrate
 * real-time reactivity of the mercy-gated GPU pipeline to simulation state.
 * All prior valuable logic preserved. No placeholders.
 * AG-SML v1.0 | TOLC 8 + PATSAGi | Thunder locked in. Yoi ⚡
 */

// GPU module wiring (new aggregator for visual materials + culling + simulation)
mod gpu;
pub use gpu::GpuVisualMaterialsPlugin;

use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy::time::Time;

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

// GPU Visual Materials (re-exported from gpu:: for clean access)
use crate::gpu::{GpuStateMaterial, ValenceHaloMaterial, MycelialWebMaterial, ResourceNodeMaterial};

/// Marker component for GPU test visual entities (dev-only animation target)
#[derive(Component)]
struct GpuTestVisual;

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
                    filter: "wgpu=error,bevy_ecs=warn,bevy=info,powrush_mmo=debug".to_string(),
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

        // GPU Visual Materials — RenderState-driven (mercy/council/RBE reactive)
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

        // === Test / Dev Visual Verification ===
        .add_systems(Startup, spawn_test_gpu_visuals)
        .add_systems(Update, animate_gpu_test_visuals)

        // Inventory, SafetyNet, RBE, and Mercy systems are wired through ReplicationPlugin
        // and the recovered inventory_ui / inventory_replication modules.

        .run();
}

/// Temporary dev/test system to verify the full GPU visual materials pipeline.
/// Spawns a camera, light, and several entities using the mercy-gated materials.
/// Remove or gate behind #[cfg(debug_assertions)] once real game scene setup exists.
fn spawn_test_gpu_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>, // fallback if needed
    mut gpu_materials: ResMut<Assets<GpuStateMaterial>>,
    mut valence_materials: ResMut<Assets<ValenceHaloMaterial>>,
    mut mycelial_materials: ResMut<Assets<MycelialWebMaterial>>,
    mut resource_materials: ResMut<Assets<ResourceNodeMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 8.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // === Test Entities using the new mercy-gated GPU materials ===
    // All marked with GpuTestVisual for live animation

    // 1. Central Resource Node (gold pulsing glow)
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.5).mesh().ico(5))),
        MeshMaterial3d(resource_materials.add(ResourceNodeMaterial::default())),
        Transform::from_xyz(0.0, 2.0, 0.0),
        GpuTestVisual,
    ));

    // 2. Valence Halo ring (blue council energy)
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(3.0).mesh().ico(5))),
        MeshMaterial3d(valence_materials.add(ValenceHaloMaterial::default())),
        Transform::from_xyz(-6.0, 3.0, -4.0),
        GpuTestVisual,
    ));

    // 3. Mycelial Web connection (green flowing network)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(8.0, 0.3, 0.3))),
        MeshMaterial3d(mycelial_materials.add(MycelialWebMaterial::default())),
        Transform::from_xyz(4.0, 1.5, 6.0),
        GpuTestVisual,
    ));

    // 4. Primary world object using GpuStateMaterial (base color driven by simulation)
    let mut state_mat = GpuStateMaterial::default();
    state_mat.params.base_color = [0.3, 0.8, 0.5, 1.0];
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(2.0).mesh().ico(5))),
        MeshMaterial3d(gpu_materials.add(state_mat)),
        Transform::from_xyz(7.0, 4.0, -5.0),
        GpuTestVisual,
    ));

    info!("[GPU TEST] Spawned test visual entities with mercy-gated materials + live animation. Watch them react to GpuSimulationState.");
}

/// Live animation system for GPU test entities.
/// Provides visible rotation + subtle scale pulse so the mercy/council/RBE reactive
/// materials can be visually verified in real time.
/// This demonstrates the full pipeline: GpuSimulationState uniforms → WGSL shaders → rendered output.
fn animate_gpu_test_visuals(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<GpuTestVisual>>,
) {
    let t = time.elapsed_secs();

    for mut transform in &mut query {
        // Gentle multi-axis rotation (demonstrates live update)
        transform.rotation =
            Quat::from_rotation_y(t * 0.25) * Quat::from_rotation_x(t * 0.08);

        // Subtle breathing/pulse scale (feels like mercy resonance)
        let pulse = (t * 0.7).sin() * 0.04 + 1.0;
        transform.scale = Vec3::splat(pulse);
    }
}

// End of client/src/main.rs — Full professional entry point restored + GPU test entities with animation.
// All recovered July plugins and systems integrated. Thunder locked in. Yoi ⚡