//! client/src/taa_compute_node.rs
//! TAA Compute Node — Dispatches taa_compute.wgsl every frame
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v18.10+

use bevy::prelude::*;
use bevy::render::{
    render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext},
    renderer::{RenderContext, RenderDevice, RenderQueue},
    RenderApp,
};
use bevy::render::render_resource::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource,
    BufferBinding, BufferBindingType, CachedComputePipelineId, ComputePipelineDescriptor,
    PipelineCache, ShaderStages,
};
use bevy::render::texture::BevyDefault;
use crate::rbe_simulation::ShadowQuality; // or your TAA quality resource

#[derive(Resource, Default)]
pub struct TaaComputePipeline {
    pub pipeline: Option<CachedComputePipelineId>,
}

#[derive(Resource, Default)]
pub struct TaaBindGroup {
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
        let taa_bind_group = world.resource::<TaaBindGroup>();

        let Some(pipeline) = taa_pipeline.pipeline else {
            return Ok(());
        };
        let Some(bind_group) = &taa_bind_group.bind_group else {
            return Ok(());
        };

        let pipeline = match pipeline_cache.get_compute_pipeline(pipeline) {
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
        pass.dispatch_workgroups(64, 64, 1); // Adjust based on resolution

        Ok(())
    }
}

pub fn setup_taa_compute_pipeline(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut pipeline_cache: ResMut<PipelineCache>,
    mut taa_pipeline: ResMut<TaaComputePipeline>,
) {
    let shader = asset_server.load("shaders/taa_compute.wgsl");

    let pipeline_id = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: Some("taa_compute_pipeline".into()),
        layout: None,
        shader,
        entry_point: "main".into(),
        shader_defs: vec![],
    });

    taa_pipeline.pipeline = Some(pipeline_id);
}

pub fn update_taa_bind_group(
    mut taa_bind_group: ResMut<TaaBindGroup>,
    render_device: Res<RenderDevice>,
    // TODO: Add your actual texture handles here (current, history, velocity, output)
    // current_color: Res<CurrentColorTexture>,
    // history_color: Res<HistoryColorTexture>,
    // etc.
) {
    // This function should create the bind group with the correct textures
    // For now it's a placeholder — wire your actual textures here
}

pub struct TaaComputeNodePlugin;

impl Plugin for TaaComputeNodePlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);

        render_app
            .init_resource::<TaaComputePipeline>()
            .init_resource::<TaaBindGroup>()
            .add_systems(Render, (
                setup_taa_compute_pipeline,
                update_taa_bind_group,
            ));

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node("taa_compute", TaaComputeNode);
    }
}
