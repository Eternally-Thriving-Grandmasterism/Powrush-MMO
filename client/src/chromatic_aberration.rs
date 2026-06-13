/*!
 * Chromatic Aberration Post-Processing Node for Powrush-MMO
 *
 * Cinematic RGB channel separation effect.
 * Intensity increases toward screen edges for that classic lens / film look.
 * Perfect final touch after TAA + Motion Blur for the most phenomenal visual experience.
 *
 * Samples from MotionBlurTarget (previous post-FX result).
 * Outputs to dedicated ChromaticAberrationTarget (can be the final color source for presentation).
 *
 * PATSAGi Council 13+ + Ra-Thor Quantum Swarm fully approved
 * AG-SML v1.0 sovereign license • TOLC 8 Mercy Gates enforced
 */ 

use bevy::prelude::*;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraphContext, SlotInfo};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::math::Vec2;

use crate::motion_blur::MotionBlurTarget;

#[derive(Resource, Clone, Copy)]
pub struct ChromaticAberrationSettings {
    pub enabled: bool,
    pub intensity: f32,
    pub center: Vec2,
    pub edge_boost: f32,
}

impl Default for ChromaticAberrationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            intensity: 0.75,
            center: Vec2::new(0.5, 0.5),
            edge_boost: 2.0,
        }
    }
}

#[derive(Resource)]
pub struct ChromaticAberrationPipeline {
    pub pipeline: CachedRenderPipelineId,
    pub bind_group_layout: BindGroupLayout,
}

#[derive(Resource)]
pub struct ChromaticAberrationTarget {
    pub texture: Texture,
    pub view: TextureView,
}

pub struct ChromaticAberrationNode;

impl FromWorld for ChromaticAberrationNode {
    fn from_world(_world: &mut World) -> Self {
        Self
    }
}

impl Node for ChromaticAberrationNode {
    fn input(&self) -> Vec<SlotInfo> { vec![] }
    fn output(&self) -> Vec<SlotInfo> { vec![] }

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let settings = world.resource::<ChromaticAberrationSettings>();
        if !settings.enabled {
            return Ok(());
        }

        let pipeline_res = world.resource::<ChromaticAberrationPipeline>();
        let ca_target = world.resource::<ChromaticAberrationTarget>();
        let motion_blur_target = world.resource::<MotionBlurTarget>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let Ok(pipeline) = pipeline_cache.get_render_pipeline(pipeline_res.pipeline) else {
            return Ok(());
        };

        // Uniforms for the shader
        #[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
        #[repr(C)]
        struct CaUniforms {
            intensity: f32,
            center_x: f32,
            center_y: f32,
            edge_boost: f32,
            _padding: f32,
        }

        let uniforms = CaUniforms {
            intensity: settings.intensity,
            center_x: settings.center.x,
            center_y: settings.center.y,
            edge_boost: settings.edge_boost,
            _padding: 0.0,
        };

        let uniform_buffer = render_context.render_device().create_buffer_with_data(
            &BufferInitDescriptor {
                label: Some("chromatic_aberration_uniforms"),
                contents: bytemuck::cast_slice(&[uniforms]),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            },
        );

        // Create sampler (could be cached in a resource for perf)
        let sampler = render_context.render_device().create_sampler(&SamplerDescriptor {
            label: Some("ca_sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            ..default()
        });

        let bind_group = render_context.render_device().create_bind_group(
            "chromatic_aberration_bind_group",
            &pipeline_res.bind_group_layout,
            &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&motion_blur_target.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("chromatic_aberration"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &ca_target.view,
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

        // Full-screen triangle
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

pub fn setup_chromatic_aberration_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut pipeline_cache: ResMut<PipelineCache>,
    asset_server: Res<AssetServer>,
) {
    let bind_group_layout = render_device.create_bind_group_layout(
        "chromatic_aberration_bind_group_layout",
        &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 2,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    );

    let shader = asset_server.load("shaders/chromatic_aberration.wgsl");

    let pipeline_descriptor = RenderPipelineDescriptor {
        label: Some("chromatic_aberration_pipeline".into()),
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
                format: TextureFormat::Rgba16Float,
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

    commands.insert_resource(ChromaticAberrationPipeline {
        pipeline,
        bind_group_layout,
    });
}

pub fn setup_chromatic_aberration_target(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    size: Extent3d,
) {
    let texture = render_device.create_texture(&TextureDescriptor {
        label: Some("chromatic_aberration_target"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba16Float,
        usage: TextureUsages::RENDER_ATTACHMENT
            | TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_SRC,
        view_formats: &[],
    });

    let view = texture.create_view(&TextureViewDescriptor::default());

    commands.insert_resource(ChromaticAberrationTarget { texture, view });
}

pub fn recreate_chromatic_aberration_target(
    commands: &mut Commands,
    render_device: &RenderDevice,
    size: Extent3d,
) {
    // Drop old resource by replacing
    let texture = render_device.create_texture(&TextureDescriptor {
        label: Some("chromatic_aberration_target"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba16Float,
        usage: TextureUsages::RENDER_ATTACHMENT
            | TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_SRC,
        view_formats: &[],
    });

    let view = texture.create_view(&TextureViewDescriptor::default());

    commands.insert_resource(ChromaticAberrationTarget { texture, view });
}
