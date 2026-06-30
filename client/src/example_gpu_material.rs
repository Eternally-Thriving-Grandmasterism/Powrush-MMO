/*!
 * example_gpu_material.rs
 *
 * Full RenderState-driven visual materials + live egui tuning.
 * GPU compute shader integration (visual_compute.wgsl).
 * AG-SML v1.0
 */

use bevy::{
    asset::Asset,
    log::debug,
    pbr::{Material, MeshMaterial3d},
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::*,
        renderer::RenderDevice,
        RenderApp, RenderSet,
    },
};
use bevy_egui::EguiContexts;
use tracing::instrument;

// ... (previous code for AlphaBlendMode, RenderState, Materials, Pipelines, Settings, UI, etc. remains) ...

// ============================================================================
// GPU COMPUTE — Visual Compute Pipeline
// ============================================================================

#[derive(Resource)]
pub struct VisualComputePipeline {
    pub pipeline: CachedComputePipelineId,
    pub bind_group_layout: BindGroupLayout,
}

impl FromWorld for VisualComputePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let asset_server = world.resource::<AssetServer>();

        let shader = asset_server.load("shaders/visual_compute.wgsl");

        let bind_group_layout = render_device.create_bind_group_layout(
            "visual_compute_bind_group_layout",
            &[ 
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        );

        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("visual_compute_pipeline".into()),
            layout: vec![bind_group_layout.clone()],
            shader: shader.clone(),
            entry_point: "main".into(),
            shader_defs: vec![],
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

// Simple dispatch system for the visual compute shader
pub fn dispatch_visual_compute(
    compute_pipelines: Res<VisualComputePipeline>,
    pipeline_cache: Res<PipelineCache>,
    render_device: Res<RenderDevice>,
    mut commands: Commands,
) {
    if let Some(pipeline) = pipeline_cache.get_compute_pipeline(compute_pipelines.pipeline) {
        // In a real implementation we would create bind groups and dispatch here.
        // This is a placeholder showing the integration point.
        debug!("[VisualCompute] Compute pipeline ready — dispatch can be added here.");
    }
}

// ============================================================================
// PLUGIN (includes compute integration)
// ============================================================================

pub struct GpuVisualMaterialsPlugin;

impl Plugin for GpuVisualMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<EnergyBurstMaterial>()
            .init_asset::<ValenceHaloMaterial>()
            .init_asset::<MycelialWebGlowMaterial>()
            .init_asset::<ResourceNodeGlowMaterial>()
            .init_resource::<GpuVisualMaterialSettings>();

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .init_resource::<EnergyBurstMaterialPipeline>()
                .init_resource::<ValenceHaloMaterialPipeline>()
                .init_resource::<MycelialWebGlowMaterialPipeline>()
                .init_resource::<ResourceNodeGlowMaterialPipeline>()
                .init_resource::<VisualComputePipeline>()
                .init_resource::<SpecializedRenderPipelines<EnergyBurstMaterialPipeline>>()
                .init_resource::<SpecializedRenderPipelines<ValenceHaloMaterialPipeline>>()
                .init_resource::<SpecializedRenderPipelines<MycelialWebGlowMaterialPipeline>>()
                .init_resource::<SpecializedRenderPipelines<ResourceNodeGlowMaterialPipeline>>()
                .add_systems(
                    Render,
                    (
                        queue_energy_burst_material,
                        queue_valence_halo_material,
                        queue_mycelial_web_glow_material,
                        queue_resource_node_glow_material,
                        dispatch_visual_compute,
                    )
                        .in_set(RenderSet::Queue),
                );
        }

        app.add_systems(Update, tune_gpu_visual_materials);
    }
}
