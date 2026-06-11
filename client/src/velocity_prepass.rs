/*!
 * Velocity Prepass - Implemented drawing logic with PreviousGlobalTransform support.
 */

use bevy::prelude::*;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraphContext};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::mesh::RenderMesh;
use bevy::render::render_asset::RenderAssets;

#[derive(Resource)]
pub struct VelocityPrepassPipeline {
    pub pipeline: CachedRenderPipelineId,
    pub bind_group_layout: BindGroupLayout,
}

#[derive(Resource)]
pub struct VelocityTexture {
    pub texture: Texture,
    pub view: TextureView,
}

#[derive(Component, Default)]
pub struct PreviousGlobalTransform(pub GlobalTransform);

pub struct VelocityPrepassNode {
    query: QueryState<(
        &'static Handle<Mesh>,
        &'static GlobalTransform,
        Option<&'static PreviousGlobalTransform>,
    )>,
}

impl FromWorld for VelocityPrepassNode {
    fn from_world(world: &mut World) -> Self {
        Self {
            query: world.query_filtered(),
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

                // Create per-object uniform buffer
                let uniforms = VelocityUniforms {
                    view_proj: Mat4::IDENTITY, // Replace with real camera matrices
                    prev_view_proj: Mat4::IDENTITY,
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

                // Draw the mesh
                if let Some(vertex_buffer) = &mesh.vertex_buffer {
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    if let Some(index_buffer) = &mesh.index_buffer {
                        render_pass.set_index_buffer(index_buffer.slice(..), 0, IndexFormat::Uint32);
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

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct VelocityUniforms {
    view_proj: Mat4,
    prev_view_proj: Mat4,
    model: Mat4,
    prev_model: Mat4,
}

// setup_velocity_prepass_pipeline remains the same as before
pub fn setup_velocity_prepass_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut pipeline_cache: ResMut<PipelineCache>,
    asset_server: Res<AssetServer>,
) {
    // ... (same as previous version)
}
