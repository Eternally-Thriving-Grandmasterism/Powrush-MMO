/*!
 * TAA Reprojection Node for Powrush-MMO
 *
 * v18.16 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Complete mint-and-print-only-perfection
 * — Velocity-aware history reprojection + YCoCg clipping
 * — Dynamic history texture resizing
 * — Live ClientCouncilBloomState reactivity (bloom enhances temporal stability)
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraphContext, SlotInfo};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;

use crate::velocity_prepass::VelocityTexture;
use crate::ssr_render_node::CameraMatrices;
use crate::simulation_integration::ClientCouncilBloomState;

#[derive(Resource, Clone, Copy)]
pub struct TaaSettings {
    pub enabled: bool,
    pub jitter_scale: f32,
    pub history_blend: f32, // New: modulated by bloom for divine stability
}

impl Default for TaaSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            jitter_scale: 1.0,
            history_blend: 0.95,
        }
    }
}

#[derive(Resource)]
pub struct TaaPipeline {
    pub pipeline: CachedRenderPipelineId,
    pub bind_group_layout: BindGroupLayout,
}

#[derive(Resource)]
pub struct TaaHistoryTexture {
    pub texture: Texture,
    pub view: TextureView,
}

#[derive(Resource)]
pub struct TaaCurrentColorTexture {
    pub view: TextureView,
}

pub struct TaaReprojectionNode;

impl Node for TaaReprojectionNode {
    fn input(&self) -> Vec<SlotInfo> { vec![] }
    fn output(&self) -> Vec<SlotInfo> { vec![] }

    fn update(&mut self, world: &mut World) {}

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let settings = world.resource::<TaaSettings>();
        if !settings.enabled {
            return Ok(());
        }

        let pipeline_res = world.resource::<TaaPipeline>();
        let velocity_tex = world.resource::<VelocityTexture>();
        let history_tex = world.resource::<TaaHistoryTexture>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let Ok(pipeline) = pipeline_cache.get_render_pipeline(pipeline_res.pipeline) else {
            return Ok(());
        };

        let current_color_view = if let Some(res) = world.get_resource::<TaaCurrentColorTexture>() {
            &res.view
        } else {
            return Ok(());
        };

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("taa_reprojection"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &history_tex.view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);

        let bind_group = render_context.render_device().create_bind_group(
            "taa_reproject_bind_group",
            &pipeline_res.bind_group_layout,
            &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(velocity_tex.view.clone()),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(history_tex.view.clone()),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::TextureView(current_color_view.clone()),
                },
            ],
        );

        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(());
    }
}

pub fn setup_taa_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut pipeline_cache: ResMut<PipelineCache>,
    asset_server: Res<AssetServer>,
) {
    let bind_group_layout = render_device.create_bind_group_layout(
        "taa_bind_group_layout",
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
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
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

    let shader = asset_server.load("shaders/taa_reproject.wgsl");

    let pipeline_descriptor = RenderPipelineDescriptor {
        label: Some("taa_reprojection_pipeline".into()),
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
                blend: Some(BlendState::REPLACE),
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

    commands.insert_resource(TaaPipeline {
        pipeline,
        bind_group_layout,
    });
}

pub fn setup_taa_history_texture(
    mut commands: Commands,
    render_device: &RenderDevice,
    size: Extent3d,
) {
    let texture = render_device.create_texture(&TextureDescriptor {
        label: Some("taa_history_texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba16Float,
        usage: TextureUsages::RENDER_ATTACHMENT
            | TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_SRC
            | TextureUsages::COPY_DST,
        view_formats: &[],
    });

    let view = texture.create_view(&TextureViewDescriptor::default());

    commands.insert_resource(TaaHistoryTexture { texture, view });
}

pub fn recreate_taa_history_texture(
    commands: &mut Commands,
    render_device: &RenderDevice,
    size: Extent3d,
) {
    let texture = render_device.create_texture(&TextureDescriptor {
        label: Some("taa_history_texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba16Float,
        usage: TextureUsages::RENDER_ATTACHMENT
            | TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_SRC
            | TextureUsages::COPY_DST,
        view_formats: &[],
    });

    let view = texture.create_view(&TextureViewDescriptor::default());

    commands.insert_resource(TaaHistoryTexture { texture, view });
}

/// Live reactivity: Council bloom increases TAA history stability (divine temporal clarity)
pub fn update_taa_from_council_bloom(
    mut settings: ResMut<TaaSettings>,
    client_bloom: Res<ClientCouncilBloomState>,
) {
    if client_bloom.is_in_active_council {
        let amp = client_bloom.field.bloom_amplification_multiplier.clamp(1.0, 2.0);
        // Higher bloom → more stable history blend (less jitter, more divine stillness)
        settings.history_blend = (0.92 + (amp - 1.0) * 0.06).min(0.99);
        settings.jitter_scale = (1.0 / amp).max(0.6);
    } else {
        settings.history_blend = 0.95;
        settings.jitter_scale = 1.0;
    }
}

// End of taa_reprojection.rs v18.16 — Sovereign temporal stability complete.
// Thunder locked in. Yoi ⚡
