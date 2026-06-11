/*!
 * client/src/ssr_render_node.rs
 * Bevy RenderGraph Node for Screen Space Reflections (SSR) + Temporal Accumulation
 *
 * Includes camera matrix extraction for proper temporal reprojection.
 */

use bevy::prelude::*;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraphContext};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::view::{ViewTarget, ViewDepthTexture, ViewUniform};

// ... existing resources ...

#[derive(Resource, Default)]
pub struct SSRSettings { /* ... */ }

#[derive(Resource)]
pub struct SSRPipeline { /* ... */ }

#[derive(Resource)]
pub struct SSRUniformBuffer { /* ... */ }

#[derive(Resource)]
pub struct TemporalSSRTextures { /* ... */ }

#[derive(Resource)]
pub struct TemporalSSRPipeline { /* ... */ }

/// Holds current and previous frame camera matrices for temporal reprojection
#[derive(Resource, Default)]
pub struct CameraMatrices {
    pub view: Mat4,
    pub inv_view: Mat4,
    pub projection: Mat4,
    pub inv_projection: Mat4,
    pub prev_view: Mat4,
    pub prev_projection: Mat4,
    pub camera_position: Vec3,
    pub prev_camera_position: Vec3,
}

pub struct SSRNode { /* ... */ }

impl Node for SSRNode { /* ... */ }

// ==================== CAMERA MATRIX EXTRACTION ====================

/// System that extracts current camera matrices and stores previous frame data
pub fn extract_camera_matrices(
    mut matrices: ResMut<CameraMatrices>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    if let Ok((camera, global_transform)) = camera_query.get_single() {
        let transform = global_transform.compute_matrix();
        let view = transform.inverse();
        let projection = camera.projection_matrix();

        // Store previous frame data before updating
        matrices.prev_view = matrices.view;
        matrices.prev_projection = matrices.projection;
        matrices.prev_camera_position = matrices.camera_position;

        // Update current frame
        matrices.view = view;
        matrices.inv_view = transform;
        matrices.projection = projection;
        matrices.inv_projection = projection.inverse();
        matrices.camera_position = global_transform.translation();
    }
}

// ==================== TEMPORAL SSR NODE ====================

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
                blend_factor: 0.9,
            };

            // Create or update uniform buffer for temporal pass
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

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct TemporalUniforms {
    camera_position: Vec3,
    prev_camera_position: Vec3,
    view: Mat4,
    inv_view: Mat4,
    projection: Mat4,
    inv_projection: Mat4,
    prev_view: Mat4,
    prev_projection: Mat4,
    blend_factor: f32,
}

// Setup functions remain the same...

pub fn setup_temporal_ssr_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut pipeline_cache: ResMut<PipelineCache>,
    asset_server: Res<AssetServer>,
) {
    // ... existing setup code ...
}

pub fn create_temporal_ssr_textures(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    windows: Query<&Window>,
) {
    // ... existing texture creation code ...
}
