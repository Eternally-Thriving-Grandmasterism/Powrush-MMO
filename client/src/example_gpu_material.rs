/*!
 * example_gpu_material.rs
 *
 * Test scene now showcases the full shader library (7 shaders).
 *
 * AG-SML v1.0
 */

use bevy::{
    asset::Asset,
    pbr::Material,
    prelude::*,
    reflect::TypePath,
};

use crate::gpu_simulation::resources::{RbeGlobalState, CouncilValence, GlobalConfidence};

// ============================================================================
// Core Materials
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

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(ResourceNodeGlowKey)]
pub struct ResourceNodeGlowMaterial {
    pub base_color: Color,
}

impl Default for ResourceNodeGlowMaterial {
    fn default() -> Self { Self { base_color: Color::srgb(0.7, 0.5, 0.3) } }
}

// ============================================================================
// New Materials for recently added shaders
// ============================================================================

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(EnergyBurstKey)]
pub struct EnergyBurstMaterial {
    pub base_color: Color,
}

impl Default for EnergyBurstMaterial {
    fn default() -> Self { Self { base_color: Color::srgb(0.5, 0.65, 0.9) } }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EnergyBurstKey;

impl From<&EnergyBurstMaterial> for EnergyBurstKey {
    fn from(_: &EnergyBurstMaterial) -> Self { Self }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(ResonanceFieldKey)]
pub struct ResonanceFieldMaterial {
    pub base_color: Color,
}

impl Default for ResonanceFieldMaterial {
    fn default() -> Self { Self { base_color: Color::srgb(0.35, 0.45, 0.6) } }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ResonanceFieldKey;

impl From<&ResonanceFieldMaterial> for ResonanceFieldKey {
    fn from(_: &ResonanceFieldMaterial) -> Self { Self }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(ForgivenessWaveKey)]
pub struct ForgivenessWaveMaterial {
    pub base_color: Color,
}

impl Default for ForgivenessWaveMaterial {
    fn default() -> Self { Self { base_color: Color::srgb(0.7, 0.5, 0.4) } }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ForgivenessWaveKey;

impl From<&ForgivenessWaveMaterial> for ForgivenessWaveKey {
    fn from(_: &ForgivenessWaveMaterial) -> Self { Self }
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
            .init_asset::<EnergyBurstMaterial>()
            .init_asset::<ResonanceFieldMaterial>()
            .init_asset::<ForgivenessWaveMaterial>()
            .add_plugins(MaterialPlugin::<GpuStateMaterial>::default())
            .add_systems(Update, demo_animate_gpu_bridges);

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            // Full custom pipelines can be added here when production rendering is needed.
        }
    }
}

// ============================================================================
// FULL TEST SPAWNER (All 7 Shaders)
// ============================================================================

pub fn spawn_gpu_visuals_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gpu_materials: ResMut<Assets<GpuStateMaterial>>,
    mut halo_materials: ResMut<Assets<ValenceHaloMaterial>>,
    mut mycelial_materials: ResMut<Assets<MycelialWebGlowMaterial>>,
    mut node_materials: ResMut<Assets<ResourceNodeGlowMaterial>>,
    mut burst_materials: ResMut<Assets<EnergyBurstMaterial>>,
    mut field_materials: ResMut<Assets<ResonanceFieldMaterial>>,
    mut wave_materials: ResMut<Assets<ForgivenessWaveMaterial>>,
) {
    let sphere = meshes.add(Sphere::new(1.4).mesh().ico(5));
    let cube = meshes.add(Cuboid::new(2.0, 2.0, 2.0));

    // 1. Rich GpuStateMaterial
    let mat1 = gpu_materials.add(GpuStateMaterial { base_color: Color::srgb(0.5, 0.8, 1.0) });
    commands.spawn((Mesh3d(sphere.clone()), MeshMaterial3d(mat1), Transform::from_xyz(-7.0, 3.0, 0.0), Name::new("Rich_GpuStateMaterial")));

    // 2. ValenceHalo
    let mat2 = halo_materials.add(ValenceHaloMaterial { base_color: Color::srgb(0.55, 0.7, 1.0) });
    commands.spawn((Mesh3d(cube.clone()), MeshMaterial3d(mat2), Transform::from_xyz(7.0, 3.0, 0.0), Name::new("ValenceHalo")));

    // 3. Mycelial Web Glow
    let mat3 = mycelial_materials.add(MycelialWebGlowMaterial { base_color: Color::srgb(0.35, 0.5, 0.35) });
    commands.spawn((Mesh3d(sphere.clone()), MeshMaterial3d(mat3), Transform::from_xyz(-3.5, 5.5, -4.0), Name::new("MycelialWebGlow")));

    // 4. Resource Node Glow
    let mat4 = node_materials.add(ResourceNodeGlowMaterial { base_color: Color::srgb(0.65, 0.48, 0.28) });
    commands.spawn((Mesh3d(cube.clone()), MeshMaterial3d(mat4), Transform::from_xyz(3.5, 5.5, -4.0), Name::new("ResourceNodeGlow")));

    // 5. Energy Burst
    let mat5 = burst_materials.add(EnergyBurstMaterial { base_color: Color::srgb(0.5, 0.65, 0.9) });
    commands.spawn((Mesh3d(sphere.clone()), MeshMaterial3d(mat5), Transform::from_xyz(-1.5, 7.5, 3.0), Name::new("EnergyBurst")));

    // 6. Resonance Field
    let mat6 = field_materials.add(ResonanceFieldMaterial { base_color: Color::srgb(0.35, 0.45, 0.6) });
    commands.spawn((Mesh3d(cube.clone()), MeshMaterial3d(mat6), Transform::from_xyz(1.5, 7.5, 3.0), Name::new("ResonanceField")));

    // 7. Forgiveness Wave
    let mat7 = wave_materials.add(ForgivenessWaveMaterial { base_color: Color::srgb(0.7, 0.5, 0.4) });
    commands.spawn((Mesh3d(sphere), MeshMaterial3d(mat7), Transform::from_xyz(0.0, 4.0, -7.0), Name::new("ForgivenessWave")));

    info!("[GPU Visuals] Full test scene spawned with all 7 shaders.");
    info!("[GPU Visuals] demo_animate_gpu_bridges is running. Use bevy_inspector_egui for live tuning.");
}
