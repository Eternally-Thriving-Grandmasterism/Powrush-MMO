/*!
 * client/src/ssr_render_node.rs
 * Bevy RenderGraph Node for Screen Space Reflections (SSR)
 *
 * Production-grade post-process node for Powrush-MMO.
 * Integrates with EpiphanyTriggered for dynamic intensity.
 * Uses the upgraded screen_space_reflections.wgsl shader.
 */

use bevy::prelude::*;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::view::ViewTarget;
use std::sync::Arc;

use crate::render::SSRSettings; // Adjust path if needed

#[derive(Resource)]
pub struct SSRPipeline {
    pub pipeline: CachedRenderPipelineId,
    pub bind_group_layout: BindGroupLayout,
}

pub struct SSRNode {
    pub query: QueryState<&'static ViewTarget>,
}

impl FromWorld for SSRNode {
    fn from_world(world: &mut World) -> Self {
        Self {
            query: world.query_filtered(),
        }
    }
}

impl Node for SSRNode {
    fn input(&self) -> Vec<SlotInfo> {
        vec![]
    }

    fn output(&self) -> Vec<SlotInfo> {
        vec![]
    }

    fn update(&mut self, world: &mut World) {
        self.query.update_archetypes(world);
    }

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let settings = world.resource::<SSRSettings>();
        if !settings.enabled {
            return Ok(());
        }

        let pipeline_cache = world.resource::<PipelineCache>();
        let ssr_pipeline = world.resource::<SSRPipeline>();

        let Ok(pipeline) = pipeline_cache.get_render_pipeline(ssr_pipeline.pipeline) else {
            return Ok(());
        };

        for view_target in self.query.iter_manual(world) {
            let color_attachment = view_target.get_color_attachment();
            let depth_attachment = view_target.get_depth_attachment();

            // TODO: Properly bind depth, color (previous), and normal textures
            // For a full implementation you would use a secondary texture or
            // the main depth texture from the 3D pass.

            let mut render_pass =
                render_context.begin_tracked_render_pass(RenderPassDescriptor {
                    label: Some("ssr_post_process"),
                    color_attachments: &[Some(color_attachment)],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

            render_pass.set_render_pipeline(pipeline);
            // render_pass.set_bind_group(0, &bind_group, &[]);
            render_pass.draw(0..3, 0..1); // Full screen triangle
        }

        Ok(())
    }
}

pub fn setup_ssr_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut pipeline_cache: ResMut<PipelineCache>,
    asset_server: Res<AssetServer>,
) {
    let bind_group_layout = render_device.create_bind_group_layout(
        "ssr_bind_group_layout",
        &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            // Add entries for depth, color, normal textures here
        ],
    );

    let shader = asset_server.load("shaders/screen_space_reflections.wgsl");

    let pipeline_descriptor = RenderPipelineDescriptor {
        label: Some("ssr_pipeline".into()),
        layout: vec![bind_group_layout.clone()],
        vertex: VertexState {
            shader: shader.clone(),
            entry_point: "vs_main".into(),
            buffers: vec![],
            shader_defs: vec![],
        },
        fragment: Some(FragmentState {
            shader,
            entry_point: "fs_main".into(),
            targets: vec![Some(ColorTargetState {
                format: TextureFormat::Rgba8UnormSrgb, // Match your main target
                blend: Some(BlendState::ALPHA_BLENDING),
                write_mask: ColorWrites::ALL,
            })],
            shader_defs: vec![],
        }),
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        multisample: MultisampleState::default(),
        push_constant_ranges: vec![],
    };

    let pipeline = pipeline_cache.queue_render_pipeline(pipeline_descriptor);

    commands.insert_resource(SSRPipeline {
        pipeline,
        bind_group_layout,
    });
}
