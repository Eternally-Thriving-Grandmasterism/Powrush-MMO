/*!
 * client/src/ssr_render_node.rs
 * Bevy RenderGraph Node for Screen Space Reflections (SSR) + Temporal Accumulation
 *
 * Updated with safer default blend factor + comments about neighborhood clamping.
 */

use bevy::prelude::*;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraphContext};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::view::{ViewTarget, ViewDepthTexture};

// ... (previous resources remain)

#[derive(Resource, Default)]
pub struct SSRSettings {
    pub enabled: bool,
    pub intensity: f32,
    pub epiphany_boost: f32,
    pub max_steps: u32,
    pub step_size: f32,
    pub thickness: f32,
}

impl Default for SSRSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            intensity: 0.65,
            epiphany_boost: 1.0,
            max_steps: 32,
            step_size: 0.15,
            thickness: 0.08,
        }
    }
}

// ... other resources ...

pub struct TemporalSSRNode {
    query: QueryState<&'static ViewDepthTexture>,
}

impl FromWorld for TemporalSSRNode {
    fn from_world(world: &mut World) -> Self {
        Self {
            query: world.query_filtered(),
        }
    }
}

impl Node for TemporalSSRNode {
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
        let textures = world.resource::<TemporalSSRTextures>();
        let temporal_pipeline = world.resource::<TemporalSSRPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let matrices = world.resource::<CameraMatrices>();

        let Ok(pipeline) = pipeline_cache.get_render_pipeline(temporal_pipeline.pipeline) else {
            return Ok(());
        };

        for depth_texture in self.query.iter_manual(world) {
            // Build TemporalUniforms with real camera matrices
            let temporal_uniforms = TemporalUniforms {
                camera_position: matrices.camera_position,
                prev_camera_position: matrices.prev_camera_position,
                view: matrices.view,
                inv_view: matrices.inv_view,
                projection: matrices.projection,
                inv_projection: matrices.inv_projection,
                prev_view: matrices.prev_view,
                prev_projection: matrices.prev_projection,
                blend_factor: 0.83,   // Safer default with neighborhood clamping + depth rejection
            };

            let uniform_buffer = render_context.render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("temporal_ssr_uniforms"),
                    contents: bytemuck::cast_slice(&[temporal_uniforms]),
                    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                },
            );

            let bind_group = render_context.render_device.create_bind_group(
                "temporal_ssr_bind_group",
                &temporal_pipeline.bind_group_layout,
                &[
                    BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::TextureView(&textures.current),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: BindingResource::TextureView(&textures.history),
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: BindingResource::TextureView(depth_texture.view()),
                    },
                ],
            );

            let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
                label: Some("temporal_ssr_accumulation"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &textures.current,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_render_pipeline(pipeline);
            render_pass.set_bind_group(0, &bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        Ok(())
    }
}

// Note: The temporal shader now includes 3x3 neighborhood clamping + depth rejection.
// This allows a higher blend_factor (0.83) with much less ghosting than before.

// ... rest of the file (setup functions, CameraMatrices, etc.)
