/*!
 * client/src/ssr_render_node.rs
 * Bevy RenderGraph Node for Screen Space Reflections (SSR)
 *
 * Option 1: Bind group recreated every frame with real depth + color textures.
 * Combined with persistent uniform buffer optimization.
 */

use bevy::prelude::*;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraphContext};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::view::{ViewTarget, ViewDepthTexture};

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

#[derive(Resource)]
pub struct SSRPipeline {
    pub pipeline: CachedRenderPipelineId,
    pub bind_group_layout: BindGroupLayout,
}

/// Holds the persistent uniform buffer
#[derive(Resource)]
pub struct SSRUniformBuffer {
    pub buffer: Buffer,
}

pub struct SSRNode {
    query: QueryState<(&'static ViewTarget, &'static ViewDepthTexture)>,
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
        let uniform_buffer = world.resource::<SSRUniformBuffer>();

        let Ok(pipeline) = pipeline_cache.get_render_pipeline(ssr_pipeline.pipeline) else {
            return Ok(());
        };

        // Update uniform buffer (persistent + efficient)
        let uniform_data = SSRUniforms {
            max_steps: settings.max_steps,
            step_size: settings.step_size,
            thickness: settings.thickness,
            max_distance: 50.0,
            fade_start: 5.0,
            fade_end: 80.0,
            intensity: settings.intensity,
            screen_lod_scale: 8.0,
            cheap_mode_threshold: 0.78,
            epiphany_boost: settings.epiphany_boost,
            view: Mat4::IDENTITY,
            inv_view: Mat4::IDENTITY,
            projection: Mat4::IDENTITY,
            inv_projection: Mat4::IDENTITY,
        };

        render_context.render_queue().write_buffer(
            &uniform_buffer.buffer,
            0,
            bytemuck::cast_slice(&[uniform_data]),
        );

        for (view_target, depth_texture) in self.query.iter_manual(world) {
            // === Create bind group with REAL textures every frame ===
            let bind_group = render_context.render_device.create_bind_group(
                "ssr_bind_group",
                &ssr_pipeline.bind_group_layout,
                &[
                    BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::TextureView(depth_texture.view()),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: BindingResource::TextureView(
                            &view_target.get_color_attachment().view,
                        ),
                    },
                ],
            );

            let color_attachment = view_target.get_color_attachment();

            let mut render_pass =
                render_context.begin_tracked_render_pass(RenderPassDescriptor {
                    label: Some("ssr_post_process"),
                    color_attachments: &[Some(color_attachment)],
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
struct SSRUniforms {
    max_steps: u32,
    step_size: f32,
    thickness: f32,
    max_distance: f32,
    fade_start: f32,
    fade_end: f32,
    intensity: f32,
    screen_lod_scale: f32,
    cheap_mode_threshold: f32,
    epiphany_boost: f32,
    view: Mat4,
    inv_view: Mat4,
    projection: Mat4,
    inv_projection: Mat4,
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
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Depth,
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 2,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
        ],
    );

    // Persistent uniform buffer
    let uniform_buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("ssr_uniform_buffer"),
        size: std::mem::size_of::<SSRUniforms>() as u64,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

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
                format: TextureFormat::Rgba8UnormSrgb,
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

    commands.insert_resource(SSRUniformBuffer {
        buffer: uniform_buffer,
    });
}
