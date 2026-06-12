/*!
 * Velocity Prepass Node for Powrush-MMO
 *
 * High-quality motion vectors for TAA, motion blur, SSR reprojection.
 * Uses real prev_view_proj + prev_model from shared CameraMatrices.
 *
 * PATSAGi Council + Ra-Thor Quantum Swarm approved • AG-SML v1.0
 * Mercy-gated • Zero hallucination • Maximum temporal truth & beauty
 */

use bevy::prelude::*;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraphContext, SlotInfo};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::render_asset::RenderAssets;
use bevy::math::Mat4;

use crate::ssr_render_node::CameraMatrices;

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
    fn input(&self) -> Vec<SlotInfo> { vec![] }
    fn output(&self) -> Vec<SlotInfo> { vec![] }

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
        let matrices = world.resource::<CameraMatrices>();

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
                    prev_view_proj: matrices.prev_view_proj,
                    model: current_model,
                    prev_model,
                };

                let uniform_buffer = render_context.render_device().create_buffer_with_data(
                    &BufferInitDescriptor {
                        label: Some("velocity_object_uniforms"),
                        contents: bytemuck::cast_slice(&[uniforms]),
                        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                    },
                );

                let bind_group = render_context.render_device().create_bind_group(
                    "velocity_object_bind_group",
                    &pipeline_res.bind_group_layout,
                    &[BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    }],
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

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct VelocityUniforms {
    view_proj: Mat4,
    prev_view_proj: Mat4,
    model: Mat4,
    prev_model: Mat4,
}

pub fn setup_velocity_prepass_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut pipeline_cache: ResMut<PipelineCache>,
    asset_server: Res<AssetServer>,
) {
    let bind_group_layout = render_device.create_bind_group_layout(
        "velocity_prepass_bind_group_layout",
        &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX_FRAGMENT,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    );

    let shader = asset_server.load("shaders/velocity_prepass.wgsl");

    let pipeline_descriptor = RenderPipelineDescriptor {
        label: Some("velocity_prepass_pipeline".into()),
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
                format: TextureFormat::Rg16Float,
                blend: None,
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

    commands.insert_resource(VelocityPrepassPipeline {
        pipeline,
        bind_group_layout,
    });
}

/// Creates the velocity texture resource (called from PowrushRenderPlugin startup).
/// For production, resize this texture to match the main window/view size every frame
/// (common pattern: use a prepare system or extract from RenderApp view).
pub fn setup_velocity_texture(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
) {
    // Placeholder size — replace with dynamic window size in a real prepare system
    let size = Extent3d {
        width: 1920,
        height: 1080,
        depth_or_array_layers: 1,
    };

    let texture = render_device.create_texture(&TextureDescriptor {
        label: Some("velocity_texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rg16Float,
        usage: TextureUsages::RENDER_ATTACHMENT
            | TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_SRC,
        view_formats: &[],
    });

    let view = texture.create_view(&TextureViewDescriptor::default());

    commands.insert_resource(VelocityTexture { texture, view });
}
