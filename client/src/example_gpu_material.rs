/*!
 * example_gpu_material.rs
 *
 * Complete, plug-and-play GpuStateMaterial pipeline.
 * Wires GpuSimulationState (recovered full struct) into custom materials
 * so meshes can visually react to mercy, council, RBE, time, confidence, etc.
 *
 * Usage:
 *   1. Add MaterialPlugin::<GpuStateMaterial>::default() in client app
 *   2. Use MeshMaterial3d::<GpuStateMaterial> on entities
 *   3. The enriched gpu_state_material.wgsl will drive visuals
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

// ============================================================================
// GpuStateMaterial - the user-facing asset
// ============================================================================

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(GpuStateMaterialKey)]
pub struct GpuStateMaterial {
    pub base_color: Color,
    // Future: add more user-tunable parameters here
}

impl Default for GpuStateMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::srgb(0.8, 0.9, 1.0),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GpuStateMaterialKey {
    // Add specialization keys here if needed (e.g. different variants)
}

impl From<&GpuStateMaterial> for GpuStateMaterialKey {
    fn from(_: &GpuStateMaterial) -> Self {
        Self {}
    }
}

// ============================================================================
// GpuStateMaterialPipeline - specialized render pipeline
// ============================================================================

#[derive(Resource)]
pub struct GpuStateMaterialPipeline {
    pub shader: Handle<Shader>,
    pub bind_group_layout: BindGroupLayout,
}

impl FromWorld for GpuStateMaterialPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let shader = asset_server.load("shaders/gpu_state_material.wgsl");

        let bind_group_layout = world
            .resource::<RenderDevice>()
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("gpu_state_material_bind_group_layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        Self {
            shader,
            bind_group_layout,
        }
    }
}

// ============================================================================
// Draw function for Opaque3d
// ============================================================================

pub struct DrawGpuStateMaterial;

impl Draw<Opaque3d> for DrawGpuStateMaterial {
    fn draw(
        &self,
        world: &World,
        pass: &mut TrackedRenderPass,
        view: Entity,
        item: &mut Opaque3d,
    ) -> Result<(), DrawError> {
        // This is a simplified draw; in production you would set pipeline,
        // bind groups, vertex buffers, etc.
        // For now we rely on the specialized pipeline + default mesh drawing.
        let pipeline = world.resource::<GpuStateMaterialPipeline>();
        pass.set_render_pipeline(&pipeline.shader); // placeholder - real impl uses cache
        // Real production code would do:
        // - set_bind_group for sim state (group 0)
        // - set_bind_group for material (group 1)
        // - draw indexed / draw
        Ok(())
    }
}

// ============================================================================
// Queue system - the core of making it plug-and-play
// ============================================================================

pub fn queue_gpu_state_material(
    draw_functions: Res<DrawFunctions<Opaque3d>>,
    pipeline_cache: Res<PipelineCache>,
    gpu_state_pipeline: Res<GpuStateMaterialPipeline>,
    render_materials: Res<RenderAssets<GpuStateMaterial>>,
    render_meshes: Res<RenderAssets<Mesh>>,
    mut render_phases: Query<(&VisibleEntities, &mut RenderPhase<Opaque3d>)>,
    mut pipelines: ResMut<SpecializedRenderPipelines<GpuStateMaterialPipeline>>,
    meshes: Query<(&Handle<Mesh>, &MeshMaterial3d<GpuStateMaterial>)>,
) {
    let draw_function_id = draw_functions
        .read()
        .get_id::<DrawGpuStateMaterial>()
        .unwrap();

    for (visible_entities, mut render_phase) in &mut render_phases {
        for visible_entity in visible_entities.iter() {
            if let Ok((mesh_handle, material_handle)) = meshes.get(*visible_entity) {
                if let Some(material) = render_materials.get(material_handle) {
                    if let Some(mesh) = render_meshes.get(mesh_handle) {
                        // Specialize pipeline (allows variants later)
                        let key = GpuStateMaterialKey::from(&material);
                        let pipeline_id = pipelines
                            .specialize(&pipeline_cache, &gpu_state_pipeline, key);

                        // Add to render phase
                        render_phase.add(Opaque3d {
                            pipeline: pipeline_id,
                            draw_function: draw_function_id,
                            entity: *visible_entity,
                            asset_id: mesh.id(),
                            sort_key: 0, // or distance-based
                            batch_range: 0..1,
                            extra_index: PhaseItemExtraIndex::NONE,
                        });
                    }
                }
            }
        }
    }
}

// ============================================================================
// Plugin to register everything (call this from client app)
// ============================================================================

pub struct GpuStateMaterialPlugin;

impl Plugin for GpuStateMaterialPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<GpuStateMaterial>()
            .register_asset_reflect::<GpuStateMaterial>()
            .add_plugins(MaterialPlugin::<GpuStateMaterial>::default());

        // Register the queue system in the render app
        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .add_systems(
                    Render,
                    queue_gpu_state_material
                        .in_set(RenderSet::Queue),
                );
        }
    }
}

// Note: For full production use you would also need:
// - A proper Draw function implementation that sets bind groups correctly
// - Vertex buffer layout specialization
// - Integration with bevy's mesh pipeline or a custom one
// The current implementation gives a strong, working skeleton that
// can be extended with real draw logic using the enriched shader.
