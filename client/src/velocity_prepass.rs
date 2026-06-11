/*!
 * VelocityPrepassNode now uses prev_view_proj from CameraMatrices for better temporal accuracy.
 */

use bevy::prelude::*;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraphContext};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::mesh::RenderMesh;
use bevy::render::render_asset::RenderAssets;

// ... (other resources stay the same)

pub struct VelocityPrepassNode {
    query: QueryState<(
        &'static Handle<Mesh>,
        &'static GlobalTransform,
        Option<&'static PreviousGlobalTransform>,
    )>,
    camera_query: QueryState<(&'static Camera, &'static GlobalTransform)>,
}

impl FromWorld for VelocityPrepassNode {
    fn from_world(world: &mut World) -> Self {
        Self {
            query: world.query_filtered(),
            camera_query: world.query_filtered(),
        }
    }
}

impl Node for VelocityPrepassNode {
    fn input(&self) -> Vec<SlotInfo> {
        vec![]
    }

    fn output(&self) -> Vec<SlotInfo> {
        vec![]
    }

    fn update(&mut self, world: &mut World) {
        self.query.update_archetypes(world);
        self.camera_query.update_archetypes(world);
    }

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let pipeline_res = world.resource::<VelocityPrepassPipeline>();
        let velocity_tex = world.resource::<VelocityTexture>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let meshes = world.resource::<RenderAssets<Mesh>>();
        let matrices = world.resource::<crate::ssr_render_node::CameraMatrices>(); // Access shared camera data

        let Ok(pipeline) = pipeline_cache.get_render_pipeline(pipeline_res.pipeline) else {
            return Ok(());
        };

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("velocity_prepass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &velocity_tex.view,
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

        for (mesh_handle, global_transform, previous_transform) in self.query.iter_manual(world) {
            if let Some(mesh) = meshes.get(mesh_handle) {
                let current_model = global_transform.compute_matrix();
                let prev_model = previous_transform
                    .map(|p| p.0.compute_matrix())
                    .unwrap_or(current_model);

                let uniforms = VelocityUniforms {
                    view_proj: matrices.projection * matrices.view,
                    prev_view_proj: matrices.prev_view_proj, // Now using previous frame's view_proj
                    model: current_model,
                    prev_model,
                };

                let uniform_buffer = render_context.render_device.create_buffer_with_data(
                    &BufferInitDescriptor {
                        label: Some("velocity_object_uniforms"),
                        contents: bytemuck::cast_slice(&[uniforms]),
                        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                    },
                );

                let bind_group = render_context.render_device.create_bind_group(
                    "velocity_object_bind_group",
                    &pipeline_res.bind_group_layout,
                    &[
                        BindGroupEntry {
                            binding: 0,
                            resource: uniform_buffer.as_entire_binding(),
                        },
                    ],
                );

                render_pass.set_bind_group(0, &bind_group, &[]);

                if let Some(vertex_buffer) = mesh.vertex_buffer.as_ref() {
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

                    if let Some(index_buffer) = mesh.index_buffer.as_ref() {
                        render_pass.set_index_buffer(
                            index_buffer.slice(..),
                            0,
                            mesh.index_format.unwrap_or(IndexFormat::Uint32),
                        );
                        render_pass.draw_indexed(0..mesh.index_count, 0, 0..1);
                    } else {
                        render_pass.draw(0..mesh.vertex_count, 0..1);
                    }
                }
            }
        }

        Ok(())
    }
}

// ... (VelocityUniforms and setup function remain the same)
