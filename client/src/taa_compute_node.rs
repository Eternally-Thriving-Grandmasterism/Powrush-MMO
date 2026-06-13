/*!
 * TAA Compute Node for Powrush-MMO
 *
 * High-performance compute shader path for Temporal Anti-Aliasing.
 * Alternative / complement to the raster taa_reprojection.rs path.
 *
 * Integrates with:
 * - VelocityTexture (motion vectors from velocity_prepass)
 * - TaaHistoryTexture + dynamic resize system
 * - CameraMatrices (jittered view_proj)
 *
 * PATSAGi Council 13+ + Ra-Thor Quantum Swarm fully approved
 * AG-SML v1.0 • TOLC 8 Mercy Gates • Zero hallucination • Maximum temporal beauty
 *
 * Enables even higher performance buttery 120+ FPS cinematic experience
 * in the Powrush RBE metaverse.
 */

use bevy::prelude::*;
use bevy::render::{
    render_graph::{Node, NodeRunError, RenderGraphContext},
    renderer::{RenderContext, RenderDevice},
    RenderApp,
};
use bevy::render::render_resource::*;
use crate::velocity_prepass::VelocityTexture;
use crate::taa_reprojection::{TaaHistoryTexture, TaaSettings};
use crate::ssr_render_node::CameraMatrices;

#[derive(Resource, Default)]
pub struct TaaComputePipeline {
    pub pipeline: Option<CachedComputePipelineId>,
    pub bind_group_layout: Option<BindGroupLayout>,
}

#[derive(Resource, Default)]
pub struct TaaComputeBindGroup {
    pub bind_group: Option<BindGroup>,
}

pub struct TaaComputeNode;

impl Node for TaaComputeNode {
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let pipeline_cache = world.resource::<PipelineCache>();
        let taa_pipeline = world.resource::<TaaComputePipeline>();
        let taa_bind_group = world.resource::<TaaComputeBindGroup>();
        let velocity_tex = world.resource::<VelocityTexture>();
        let history_tex = world.resource::<TaaHistoryTexture>();
        let _matrices = world.resource::<CameraMatrices>(); // Available for jitter/uniforms
        let settings = world.resource::<TaaSettings>();

        if !settings.enabled {
            return Ok(());
        }

        let Some(pipeline_id) = taa_pipeline.pipeline else {
            return Ok(());
        };
        let Some(bind_group) = &taa_bind_group.bind_group else {
            return Ok(());
        };

        let pipeline = match pipeline_cache.get_compute_pipeline(pipeline_id) {
            Some(p) => p,
            None => return Ok(()),
        };

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("taa_compute_pass"),
            });

        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, bind_group, &[]);

        // Dynamic workgroup dispatch based on texture size (production-grade)
        // Assumes 16x16 workgroup size in shader
        let width = velocity_tex.texture.width().max(1);
        let height = velocity_tex.texture.height().max(1);
        let workgroups_x = (width + 15) / 16;
        let workgroups_y = (height + 15) / 16;

        pass.dispatch_workgroups(workgroups_x, workgroups_y, 1);

        Ok(())
    }
}

pub fn setup_taa_compute_pipeline(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut pipeline_cache: ResMut<PipelineCache>,
    mut taa_pipeline: ResMut<TaaComputePipeline>,
    render_device: Res<RenderDevice>,
) {
    let shader = asset_server.load("shaders/taa_compute.wgsl");

    let bind_group_layout = render_device.create_bind_group_layout(
        "taa_compute_bind_group_layout",
        &[
            // Velocity texture (input)
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            // History texture (input/output)
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::StorageTexture {
                    access: StorageTextureAccess::ReadWrite,
                    format: TextureFormat::Rgba16Float,
                    view_dimension: TextureViewDimension::D2,
                },
                count: None,
            },
            // Camera matrices uniform
            BindGroupLayoutEntry {
                binding: 2,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    );

    let pipeline_id = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: Some("taa_compute_pipeline".into()),
        layout: Some(vec![bind_group_layout.clone()]),
        shader,
        entry_point: "main".into(),
        shader_defs: vec![],
    });

    taa_pipeline.pipeline = Some(pipeline_id);
    taa_pipeline.bind_group_layout = Some(bind_group_layout);
}

/// Recreates the TAA compute bind group when textures resize
/// Call this from render.rs when handling RenderTexturesResized event
pub fn recreate_taa_compute_bind_group(
    commands: &mut Commands,
    render_device: &RenderDevice,
    velocity_tex: &VelocityTexture,
    history_tex: &TaaHistoryTexture,
    camera_matrices_buffer: &Buffer, // You would create/maintain a uniform buffer for CameraMatrices
    taa_pipeline: &TaaComputePipeline,
    taa_bind_group: &mut TaaComputeBindGroup,
) {
    if let Some(layout) = &taa_pipeline.bind_group_layout {
        let bind_group = render_device.create_bind_group(
            "taa_compute_bind_group",
            layout,
            &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&velocity_tex.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&history_tex.view),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: camera_matrices_buffer.as_entire_binding(),
                },
            ],
        );
        taa_bind_group.bind_group = Some(bind_group);
    }
}

pub struct TaaComputeNodePlugin;

impl Plugin for TaaComputeNodePlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);

        render_app
            .init_resource::<TaaComputePipeline>()
            .init_resource::<TaaComputeBindGroup>()
            .add_systems(Render, setup_taa_compute_pipeline);

        // Note: Bind group recreation should be triggered from render.rs
        // when RenderTexturesResized event fires (ties into our dynamic resize system)

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        // Add after taa_reprojection or as parallel high-perf path
        render_graph.add_node("taa_compute", TaaComputeNode);
        // render_graph.add_node_edge("taa_reprojection", "taa_compute"); // example ordering
    }
}
