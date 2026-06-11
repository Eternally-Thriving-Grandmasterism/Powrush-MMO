/*!
 * TAA Reprojection Node + Settings for Powrush-MMO
 *
 * Includes TaaSettings resource for controlling jitter intensity and enabling/disabling TAA.
 */

use bevy::prelude::*;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraphContext};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::view::ViewTarget;

use crate::velocity_prepass::VelocityTexture;
use crate::ssr_render_node::CameraMatrices;

#[derive(Resource, Clone, Copy)]
pub struct TaaSettings {
    pub enabled: bool,
    pub jitter_scale: f32,      // 0.0 = no jitter, 1.0 = normal, >1.0 = stronger (can introduce artifacts)
}

impl Default for TaaSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            jitter_scale: 1.0,
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

pub struct TaaReprojectionNode;

impl Node for TaaReprojectionNode {
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

        // Get current color (automatic via ViewTarget or resource)
        let current_color_view = if let Ok(view_target) = world.query::<&ViewTarget>().get_single(world) {
            // In production, we would properly extract the main view here.
            // For now we rely on TaaCurrentColorTexture if populated.
            if let Some(res) = world.get_resource::<TaaCurrentColorTexture>() {
                &res.view
            } else {
                return Ok(());
            }
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

        Ok(())
    }
}

// ... setup functions stay similar ...

pub fn setup_taa_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut pipeline_cache: ResMut<PipelineCache>,
    asset_server: Res<AssetServer>,
) {
    let bind_group_layout = render_device.create_bind_group_layout(
        "taa_bind_group_layout",
        &[
            BindGroupLayoutEntry { binding: 0, visibility: ShaderStages::FRAGMENT, ty: BindingType::Texture { sample_type: TextureSampleType::Float { filterable: true }, view_dimension: TextureViewDimension::D2, multisampled: false }, count: None },
            BindGroupLayoutEntry { binding: 1, visibility: ShaderStages::FRAGMENT, ty: BindingType::Texture { sample_type: TextureSampleType::Float { filterable: true }, view_dimension: TextureViewDimension::D2, multisampled: false }, count: None },
            BindGroupLayoutEntry { binding: 2, visibility: ShaderStages::FRAGMENT, ty: BindingType::Texture { sample_type: TextureSampleType::Float { filterable: true }, view_dimension: TextureViewDimension::D2, multisampled: false }, count: None },
        ],
    );

    let shader = asset_server.load("shaders/taa_reproject.wgsl");

    let pipeline_descriptor = RenderPipelineDescriptor {
        label: Some("taa_reprojection_pipeline".into()),
        layout: vec![bind_group_layout.clone()],
        vertex: VertexState { shader: shader.clone(), entry_point: "vs_main".into(), buffers: vec![], shader_defs: vec![] },
        fragment: Some(FragmentState {
            shader,
            entry_point: "fs_main".into(),
            targets: vec![Some(ColorTargetState { format: TextureFormat::Rgba16Float, blend: Some(BlendState::REPLACE), write_mask: ColorWrites::ALL })],
            shader_defs: vec![],
        }),
        primitive: PrimitiveState { topology: PrimitiveTopology::TriangleList, ..default() },
        depth_stencil: None,
        multisample: MultisampleState::default(),
        push_constant_ranges: vec![],
    };

    let pipeline = pipeline_cache.queue_render_pipeline(pipeline_descriptor);

    commands.insert_resource(TaaPipeline { pipeline, bind_group_layout });
}

pub fn setup_taa_history_texture(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    render_device: Res<RenderDevice>,
) {
    let size = Extent3d { width: 1920, height: 1080, depth_or_array_layers: 1 };

    let image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("taa_history".into()),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba16Float,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC | TextureUsages::COPY_DST,
            view_formats: &[],
        },
        ..default()
    };

    let texture_handle = images.add(image);
    let texture = images.get(&texture_handle).unwrap();

    let view = render_device.create_texture_view(
        &texture.texture,
        &TextureViewDescriptor {
            label: Some("taa_history_view".into()),
            format: Some(TextureFormat::Rgba16Float),
            dimension: Some(TextureViewDimension::D2),
            aspect: TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        },
    );

    commands.insert_resource(TaaHistoryTexture { texture: texture.texture.clone(), view });
}

#[derive(Resource)]
pub struct TaaCurrentColorTexture {
    pub view: TextureView,
}

/// Optional system to help populate current color (can be expanded).
pub fn extract_taa_current_color(
    mut commands: Commands,
    cameras: Query<&ViewTarget, With<Camera>>,
) {
    if let Ok(_view_target) = cameras.get_single() {
        // In a full implementation we would extract the main texture view here.
    }
}
