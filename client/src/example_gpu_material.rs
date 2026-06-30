/*!
 * example_gpu_material.rs
 *
 * Refined test experience for all GPU simulation shaders.
 *
 * === Live Tuning Recommendation ===
 * Add bevy_inspector_egui to your Cargo.toml and app:
 *   .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
 *
 * Then you can live-edit RbeGlobalState, CouncilValence, GlobalConfidence, etc.
 * while the demo animation or real systems are running.
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

use crate::gpu_simulation::resources::{RbeGlobalState, CouncilValence, GlobalConfidence};

// ============================================================================
// Materials
// ============================================================================

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(GpuStateMaterialKey)]
pub struct GpuStateMaterial {
    pub base_color: Color,
}

impl Default for GpuStateMaterial {
    fn default() -> Self { Self { base_color: Color::srgb(0.6, 0.85, 1.0) } }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GpuStateMaterialKey;

impl From<&GpuStateMaterial> for GpuStateMaterialKey {
    fn from(_: &GpuStateMaterial) -> Self { Self }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(ValenceHaloKey)]
pub struct ValenceHaloMaterial {
    pub base_color: Color,
}

impl Default for ValenceHaloMaterial {
    fn default() -> Self { Self { base_color: Color::srgb(0.5, 0.75, 1.0) } }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(MycelialWebGlowKey)]
pub struct MycelialWebGlowMaterial {
    pub base_color: Color,
}

impl Default for MycelialWebGlowMaterial {
    fn default() -> Self { Self { base_color: Color::srgb(0.4, 0.55, 0.4) } }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MycelialWebGlowKey;

impl From<&MycelialWebGlowMaterial> for MycelialWebGlowKey {
    fn from(_: &MycelialWebGlowMaterial) -> Self { Self }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(ResourceNodeGlowKey)]
pub struct ResourceNodeGlowMaterial {
    pub base_color: Color,
}

impl Default for ResourceNodeGlowMaterial {
    fn default() -> Self { Self { base_color: Color::srgb(0.7, 0.5, 0.3) } }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ResourceNodeGlowKey;

impl From<&ResourceNodeGlowMaterial> for ResourceNodeGlowKey {
    fn from(_: &ResourceNodeGlowMaterial) -> Self { Self }
}

// ============================================================================
// Demo Animation
// ============================================================================

pub fn demo_animate_gpu_bridges(
    time: Res<Time>,
    mut rbe: ResMut<RbeGlobalState>,
    mut council: ResMut<CouncilValence>,
    mut confidence: ResMut<GlobalConfidence>,
) {
    let t = time.elapsed_seconds();
    rbe.flow_rate = (t.sin() * 0.5 + 0.5) * 2.5;
    council.value = ((t * 0.7).sin() * 0.5 + 0.5).max(0.1);
    confidence.value = 0.65 + (t * 0.4).sin() * 0.3;
}

// ============================================================================
// Plugin
// ============================================================================

pub struct GpuVisualMaterialsPlugin;

impl Plugin for GpuVisualMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<GpuStateMaterial>()
            .init_asset::<ValenceHaloMaterial>()
            .init_asset::<MycelialWebGlowMaterial>()
            .init_asset::<ResourceNodeGlowMaterial>()
            .add_plugins(MaterialPlugin::<GpuStateMaterial>::default())
            .add_systems(Update, demo_animate_gpu_bridges);

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            // Full custom pipelines for new materials can be added here when needed.
        }
    }
}

// ============================================================================
// TEST SPAWNER
// ============================================================================

pub fn spawn_gpu_visuals_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gpu_materials: ResMut<Assets<GpuStateMaterial>>,
    mut halo_materials: ResMut<Assets<ValenceHaloMaterial>>,
    mut mycelial_materials: ResMut<Assets<MycelialWebGlowMaterial>>,
    mut node_materials: ResMut<Assets<ResourceNodeGlowMaterial>>,
) {
    let sphere = meshes.add(Sphere::new(1.5).mesh().ico(5));
    let cube = meshes.add(Cuboid::new(2.2, 2.2, 2.2));

    // 1. Rich GpuStateMaterial
    let mat1 = gpu_materials.add(GpuStateMaterial {
        base_color: Color::srgb(0.5, 0.8, 1.0),
    });
    commands.spawn((
        Mesh3d(sphere.clone()),
        MeshMaterial3d(mat1),
        Transform::from_xyz(-6.0, 3.0, 0.0),
        Name::new("Rich_GpuStateMaterial"),
    ));

    // 2. ValenceHalo
    let mat2 = halo_materials.add(ValenceHaloMaterial {
        base_color: Color::srgb(0.55, 0.7, 1.0),
    });
    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(mat2),
        Transform::from_xyz(6.0, 3.0, 0.0),
        Name::new("ValenceHalo"),
    ));

    // 3. Mycelial Web Glow
    let mat3 = mycelial_materials.add(MycelialWebGlowMaterial {
        base_color: Color::srgb(0.35, 0.5, 0.35),
    });
    commands.spawn((
        Mesh3d(sphere.clone()),
        MeshMaterial3d(mat3),
        Transform::from_xyz(-2.0, 5.5, -5.0),
        Name::new("MycelialWebGlow"),
    ));

    // 4. Resource Node Glow
    let mat4 = node_materials.add(ResourceNodeGlowMaterial {
        base_color: Color::srgb(0.65, 0.48, 0.28),
    });
    commands.spawn((
        Mesh3d(cube),
        MeshMaterial3d(mat4),
        Transform::from_xyz(2.0, 5.5, -5.0),
        Name::new("ResourceNodeGlow"),
    ));

    // Extra variety object
    let extra = gpu_materials.add(GpuStateMaterial {
        base_color: Color::srgb(0.85, 0.65, 0.55),
    });
    commands.spawn((
        Mesh3d(sphere),
        MeshMaterial3d(extra),
        Transform::from_xyz(0.0, 7.0, 4.0),
        Name::new("Extra_GpuStateMaterial"),
    ));

    info!("[GPU Visuals] Test scene ready.");
    info!("[GPU Visuals] Use bevy_inspector_egui to live-edit bridge resources for best tuning experience.");
}
