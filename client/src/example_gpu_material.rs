/*!
 * example_gpu_material.rs
 *
 * Full pipeline specialization by AlphaBlendMode.
 *
 * AG-SML v1.0
 */

use bevy::{
    asset::Asset,
    pbr::Material,
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::*,
        renderer::RenderDevice,
        RenderApp, RenderSet,
    },
};

// ... (AlphaBlendMode, EnergyBurstMaterial, ValenceHaloMaterial definitions remain) ...

// ============================================================================
// ENERGY BURST PIPELINE WITH BLEND SPECIALIZATION
// ============================================================================

#[derive(Resource)]
pub struct EnergyBurstMaterialPipeline {
    pub shader: Handle<Shader>,
}

impl FromWorld for EnergyBurstMaterialPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            shader: asset_server.load("shaders/energy_burst.wgsl"),
        }
    }
}

impl SpecializedRenderPipeline for EnergyBurstMaterialPipeline {
    type Key = EnergyBurstKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let blend = key.blend_mode.blend_state();

        RenderPipelineDescriptor {
            label: Some("energy_burst_pipeline".into()),
            layout: vec![],
            vertex: VertexState {
                shader: self.shader.clone(),
                entry_point: "vertex_main".into(),
                shader_defs: vec![],
                buffers: vec![],
            },
            fragment: Some(FragmentState {
                shader: self.shader.clone(),
                entry_point: "fragment_main".into(),
                shader_defs: vec![],
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::Rgba8UnormSrgb, // or view format
                    blend: Some(blend),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
        }
    }
}

// ============================================================================
// VALENCE HALO PIPELINE WITH BLEND SPECIALIZATION
// ============================================================================

#[derive(Resource)]
pub struct ValenceHaloMaterialPipeline {
    pub shader: Handle<Shader>,
}

impl FromWorld for ValenceHaloMaterialPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            shader: asset_server.load("shaders/valence_halo.wgsl"),
        }
    }
}

impl SpecializedRenderPipeline for ValenceHaloMaterialPipeline {
    type Key = ValenceHaloKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let blend = key.blend_mode.blend_state();

        RenderPipelineDescriptor {
            label: Some("valence_halo_pipeline".into()),
            layout: vec![],
            vertex: VertexState {
                shader: self.shader.clone(),
                entry_point: "vertex_main".into(),
                shader_defs: vec![],
                buffers: vec![],
            },
            fragment: Some(FragmentState {
                shader: self.shader.clone(),
                entry_point: "fragment_main".into(),
                shader_defs: vec![],
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::Rgba8UnormSrgb,
                    blend: Some(blend),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
        }
    }
}

// ============================================================================
// QUEUE SYSTEMS (example - simplified)
// ============================================================================

pub fn queue_energy_burst_material(
    draw_functions: Res<DrawFunctions<Opaque3d>>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<EnergyBurstMaterialPipeline>,
    render_materials: Res<RenderAssets<EnergyBurstMaterial>>,
    mut render_phases: Query<(&VisibleEntities, &mut RenderPhase<Opaque3d>)>,
    mut specialized_pipelines: ResMut<SpecializedRenderPipelines<EnergyBurstMaterialPipeline>>,
) {
    // In a real implementation, we would iterate visible entities,
    // get their EnergyBurstMaterial, create the key with blend_mode,
    // specialize the pipeline, and add to the render phase.
}

// Similar queue function would exist for ValenceHaloMaterial.

// ============================================================================
// PLUGIN
// ============================================================================

pub struct GpuVisualMaterialsPlugin;

impl Plugin for GpuVisualMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<EnergyBurstMaterial>()
            .init_asset::<ValenceHaloMaterial>();

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .init_resource::<EnergyBurstMaterialPipeline>()
                .init_resource::<ValenceHaloMaterialPipeline>()
                .init_resource::<SpecializedRenderPipelines<EnergyBurstMaterialPipeline>>()
                .init_resource::<SpecializedRenderPipelines<ValenceHaloMaterialPipeline>>();

            // queue_energy_burst_material and queue_valence_halo_material
            // would be added to RenderSet::Queue here.
        }
    }
}
