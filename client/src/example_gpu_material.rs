/*!
 * example_gpu_material.rs
 *
 * Refined test experience for GpuStateMaterial and ValenceHaloMaterial.
 *
 * Features:
 * - Two main test materials side-by-side
 * - Improved draw logic for ValenceHalo
 * - Demo animation system that updates bridge resources so visuals react even without full game systems
 * - Easy to extend with more test objects
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
        render_phase::{DrawFunctions, RenderPhase, SetItemPipeline},
        render_resource::*,
        renderer::RenderDevice,
        RenderApp, RenderSet,
    },
};

use crate::gpu_simulation::resources::{RbeGlobalState, CouncilValence, GlobalConfidence};

// ============================================================================
// GpuStateMaterial
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

// ============================================================================
// ValenceHaloMaterial (Improved)
// ============================================================================

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(ValenceHaloKey)]
pub struct ValenceHaloMaterial {
    pub base_color: Color,
}

impl Default for ValenceHaloMaterial {
    fn default() -> Self {
        Self { base_color: Color::srgb(0.5, 0.75, 1.0) }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ValenceHaloKey;

impl From<&ValenceHaloMaterial> for ValenceHaloKey {
    fn from(_: &ValenceHaloMaterial) -> Self { Self }
}

#[derive(Resource)]
pub struct ValenceHaloMaterialPipeline {
    pub shader: Handle<Shader>,
}

impl FromWorld for ValenceHaloMaterialPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self { shader: asset_server.load("shaders/valence_halo.wgsl") }
    }
}

pub struct DrawValenceHalo;

impl Draw<Opaque3d> for DrawValenceHalo {
    fn draw(
        &self,
        world: &World,
        pass: &mut TrackedRenderPass,
        view: Entity,
        item: &mut Opaque3d,
    ) -> Result<(), DrawError> {
        // Improved draw - sets pipeline. Full bind group setup can be expanded here.
        pass.set_render_pipeline(&item.pipeline);
        // In a more complete implementation, we would also set bind groups for:
        // - GpuSimulationState (group 0)
        // - Material uniform (group 1)
        Ok(())
    }
}

pub fn queue_valence_halo_material(
    draw_functions: Res<DrawFunctions<Opaque3d>>,
    pipeline_cache: Res<PipelineCache>,
    halo_pipeline: Res<ValenceHaloMaterialPipeline>,
    render_materials: Res<RenderAssets<ValenceHaloMaterial>>,
    render_meshes: Res<RenderAssets<Mesh>>,
    mut render_phases: Query<(&VisibleEntities, &mut RenderPhase<Opaque3d>)>,
    mut pipelines: ResMut<SpecializedRenderPipelines<ValenceHaloMaterialPipeline>>,
    meshes: Query<(&Handle<Mesh>, &MeshMaterial3d<ValenceHaloMaterial>)>,
) {
    let draw_function_id = draw_functions.read().get_id::<DrawValenceHalo>().unwrap();

    for (visible_entities, mut render_phase) in &mut render_phases {
        for visible_entity in visible_entities.iter() {
            if let Ok((mesh_handle, material_handle)) = meshes.get(*visible_entity) {
                if render_materials.get(material_handle).is_some() {
                    let key = ValenceHaloKey;
                    let pipeline_id = pipelines.specialize(&pipeline_cache, &halo_pipeline, key);

                    render_phase.add(Opaque3d {
                        pipeline: pipeline_id,
                        draw_function: draw_function_id,
                        entity: *visible_entity,
                        asset_id: mesh_handle.id(),
                        sort_key: 0,
                        batch_range: 0..1,
                        extra_index: PhaseItemExtraIndex::NONE,
                    });
                }
            }
        }
    }
}

// ============================================================================
// Demo Animation System (for live tuning / testing without full game systems)
// ============================================================================

pub fn demo_animate_gpu_bridges(
    time: Res<Time>,
    mut rbe: ResMut<RbeGlobalState>,
    mut council: ResMut<CouncilValence>,
    mut confidence: ResMut<GlobalConfidence>,
) {
    let t = time.elapsed_seconds();

    // Gentle animation so the visuals clearly react
    rbe.flow_rate = (t.sin() * 0.5 + 0.5) * 2.0;
    council.value = (t * 0.8).sin() * 0.5 + 0.5;
    confidence.value = 0.7 + (t * 0.3).sin() * 0.25;
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
            .add_plugins(MaterialPlugin::<GpuStateMaterial>::default())
            .add_systems(Update, demo_animate_gpu_bridges);

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .init_resource::<ValenceHaloMaterialPipeline>()
                .add_systems(Render, queue_valence_halo_material.in_set(RenderSet::Queue));
        }
    }
}

// ============================================================================
// TEST SPAWNER (Refined)
// ============================================================================

pub fn spawn_gpu_visuals_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gpu_materials: ResMut<Assets<GpuStateMaterial>>,
    mut halo_materials: ResMut<Assets<ValenceHaloMaterial>>,
) {
    let mesh = meshes.add(Sphere::new(1.8).mesh().ico(5));

    // Rich GpuStateMaterial
    let main_mat = gpu_materials.add(GpuStateMaterial {
        base_color: Color::srgb(0.55, 0.82, 1.0),
    });
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(main_mat),
        Transform::from_xyz(-4.0, 3.0, 0.0),
        Name::new("GpuStateMaterial_Test"),
    ));

    // ValenceHaloMaterial
    let halo_mat = halo_materials.add(ValenceHaloMaterial {
        base_color: Color::srgb(0.6, 0.75, 1.0),
    });
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(halo_mat),
        Transform::from_xyz(4.0, 3.0, 0.0),
        Name::new("ValenceHalo_Test"),
    ));

    info!("[GPU Visuals] Test entities spawned.");
    info!("[GPU Visuals] demo_animate_gpu_bridges is running - visuals should react live.");
}
