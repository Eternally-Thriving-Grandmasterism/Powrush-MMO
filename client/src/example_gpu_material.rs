/*!
 * example_gpu_material.rs
 *
 * Complete, plug-and-play GpuStateMaterial pipeline + easy test spawner.
 *
 * === Quick Test ===
 * Add GpuStateMaterialPlugin to your client app.
 * The example system below will spawn a glowing test sphere on startup
 * so you can immediately see the enriched mercy/council/RBE visuals.
 *
 * In a real game you would use this material on:
 *   - Resource nodes
 *   - Ships / infrastructure
 *   - Council structures
 *   - Any mesh that should react to simulation state
 *
 * AG-SML v1.0
 */

use bevy::{
    asset::Asset,
    ecs::system::SystemParam,
    pbr::Material,
    prelude::*,
    reflect::TypePath,
    render::{
        extract_component::ExtractComponent,
        mesh::MeshVertexBufferLayout,
        render_asset::RenderAssets,
        render_phase::{DrawFunctions, RenderPhase, SetItemPipeline},
        render_resource::*,
        renderer::RenderDevice,
        view::{ExtractedView, VisibleEntities},
        Extract, Render, RenderApp, RenderSet,
    },
    sprite::MaterialMesh2dBundle,
};

// ... (previous GpuStateMaterial, Pipeline, Draw, queue system, and Plugin definitions remain above this point)

// ============================================================================
// EASY TEST SPAWNER - run this to see the visuals immediately
// ============================================================================

/// Spawns a simple glowing test sphere using GpuStateMaterial.
/// Call this from a Startup system or debug command.
pub fn spawn_gpu_state_material_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GpuStateMaterial>>,
) {
    // Create a nice visible sphere
    let mesh = meshes.add(Sphere::new(1.5).mesh().ico(5));

    // Create the material with a nice base color
    // The shader will modulate it based on live GpuSimulationState
    let material = materials.add(GpuStateMaterial {
        base_color: Color::srgb(0.6, 0.85, 1.0),
    });

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(0.0, 3.0, 0.0),
        Name::new("GpuStateMaterial_Test_Sphere"),
        // Optional: add any tags your game uses for culling / interest management
    ));

    info!("[GpuStateMaterial] Test sphere spawned. You should see mercy/council/RBE driven visuals.");
}

// Example of how to register the test spawner:
// In your client app's Startup:
// app.add_systems(Startup, spawn_gpu_state_material_test);

// For even better testing you can also add:
// - bevy_inspector_egui to tweak base_color live
// - A system that updates GlobalConfidence / RbeGlobalState / CouncilValence every frame
//   so the sphere visibly reacts as real game systems come online.
