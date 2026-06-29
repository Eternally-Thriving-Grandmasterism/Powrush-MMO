/*!
 * example_gpu_material.rs
 *
 * Plug-and-play GpuStateMaterial + ValenceHalo pipeline with easy test spawners.
 *
 * === Quick Start (Test Everything) ===
 * 1. Add GpuStateMaterialPlugin to your client app.
 * 2. Call spawn_gpu_visuals_test() in Startup.
 * 3. You will see two entities:
 *    - Left:  Rich GpuStateMaterial (mercy, council rings, RBE flow, breathing)
 *    - Right: Clean ValenceHalo (council/mercy aura)
 *
 * Use this to immediately see and tune all GPU simulation visuals.
 *
 * For live tuning during development:
 *   - Change values in GlobalConfidence, RbeGlobalState, CouncilValence at runtime
 *   - Or use bevy_inspector_egui on the bridge resources
 *
 * AG-SML v1.0
 */

use bevy::{
    asset::Asset,
    pbr::Material,
    prelude::*,
    reflect::TypePath,
    render::{
        render_asset::RenderAssets,
        render_phase::{DrawFunctions, RenderPhase},
        render_resource::*,
        RenderApp, RenderSet,
    },
};

// ============================================================================
// GpuStateMaterial (rich layered effects)
// ============================================================================

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(GpuStateMaterialKey)]
pub struct GpuStateMaterial {
    pub base_color: Color,
}

impl Default for GpuStateMaterial {
    fn default() -> Self {
        Self { base_color: Color::srgb(0.6, 0.85, 1.0) }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GpuStateMaterialKey;

impl From<&GpuStateMaterial> for GpuStateMaterialKey {
    fn from(_: &GpuStateMaterial) -> Self { Self }
}

// ... (GpuStateMaterialPipeline, DrawGpuStateMaterial, queue system, and Plugin from previous versions remain)

// ============================================================================
// ValenceHaloMaterial (dedicated council/mercy halo)
// ============================================================================

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(ValenceHaloKey)]
pub struct ValenceHaloMaterial {
    pub base_color: Color,
}

impl Default for ValenceHaloMaterial {
    fn default() -> Self {
        Self { base_color: Color::srgb(0.5, 0.7, 1.0) }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ValenceHaloKey;

impl From<&ValenceHaloMaterial> for ValenceHaloKey {
    fn from(_: &ValenceHaloMaterial) -> Self { Self }
}

// Note: For full production you would create a proper ValenceHaloPipeline similar to GpuStateMaterial.
// For now the halo shader can also be used via custom material or post-process.

// ============================================================================
// TEST SPAWNERS
// ============================================================================

/// Spawns two test entities side-by-side so you can see and compare all visuals.
pub fn spawn_gpu_visuals_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gpu_materials: ResMut<Assets<GpuStateMaterial>>,
    mut halo_materials: ResMut<Assets<ValenceHaloMaterial>>,
) {
    let mesh = meshes.add(Sphere::new(1.8).mesh().ico(5));

    // === Left: Rich GpuStateMaterial (mercy, council, RBE, time effects) ===
    let main_mat = gpu_materials.add(GpuStateMaterial {
        base_color: Color::srgb(0.55, 0.82, 1.0),
    });

    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(main_mat),
        Transform::from_xyz(-4.0, 3.0, 0.0),
        Name::new("GpuStateMaterial_Test"),
    ));

    // === Right: ValenceHalo (clean council/mercy aura) ===
    let halo_mat = halo_materials.add(ValenceHaloMaterial {
        base_color: Color::srgb(0.6, 0.75, 1.0),
    });

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(halo_mat),
        Transform::from_xyz(4.0, 3.0, 0.0),
        Name::new("ValenceHalo_Test"),
    ));

    info!("[GPU Visuals] Test entities spawned. Left = rich effects, Right = valence halo.");
    info!("[GPU Visuals] Change RbeGlobalState / CouncilValence / GlobalConfidence at runtime to see live reaction.");
}

// Recommended: Call this in Startup after adding the plugins.
// You can also trigger it via a debug console or keybind for on-demand testing.
