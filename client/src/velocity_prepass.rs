/*!
 * client/src/velocity_prepass.rs
 * Velocity Prepass - Now with better structure for drawing and previous transforms.
 */

use bevy::prelude::*;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraphContext};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;

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

#[derive(Resource, Default)]
pub struct VelocityUniforms {
    pub view_proj: Mat4,
    pub prev_view_proj: Mat4,
    pub model: Mat4,
    pub prev_model: Mat4,
}

pub struct VelocityPrepassNode {
    query: QueryState<&'static Camera>,
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
        let uniforms = world.resource::<VelocityUniforms>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let Ok(pipeline) = pipeline_cache.get_render_pipeline(pipeline_res.pipeline) else {
            return Ok(());
        };

        // Create uniform buffer with current values
        let uniform_buffer = render_context.render_device.create_buffer_with_data(
            &BufferInitDescriptor {
                label: Some("velocity_uniforms"),
                contents: bytemuck::cast_slice(&[*uniforms]),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            },
        );

        let bind_group = render_context.render_device.create_bind_group(
            "velocity_prepass_bind_group",
            &pipeline_res.bind_group_layout,
            &[
                BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
        );

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
            depth_stencil_attachment: None, // Can add shared depth later
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);

        // TODO: Draw meshes here
        // For testing, you can draw a simple quad or use existing mesh drawing systems.
        // In production, this node should draw all entities that have a VelocityPrepass component or similar.

        Ok(())
    }
}

pub fn setup_velocity_prepass_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut pipeline_cache: ResMut<PipelineCache>,
    asset_server: Res<AssetServer>,
) {
    let bind_group_layout = render_device.create_bind_group_layout(
        "velocity_prepass_bind_group_layout",
        &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX_FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
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
