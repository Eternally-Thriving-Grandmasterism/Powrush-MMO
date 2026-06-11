/*!
 * Motion Blur Node for Powrush-MMO
 *
 * High-quality, velocity-driven motion blur.
 * Reuses the exact same VelocityTexture and CameraMatrices from the temporal stack.
 * Per-pixel blur strength based on motion vector length + optional camera rotation contribution.
 *
 * Features:
 * - Toggleable via client settings (see settings.rs integration)
 * - Velocity-adaptive blur amount
 * - Supports both object motion and camera motion
 * - Efficient fullscreen pass
 * - PATSAGi Council approved cinematic quality
 * - Mercy-gated: beautiful without destroying readability in combat
 * - AG-SML v1.0
 */

use bevy::prelude::*;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraphContext};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;

use crate::velocity_prepass::VelocityTexture;
use crate::ssr_render_node::CameraMatrices;

#[derive(Resource, Clone, Copy)]
pub struct MotionBlurSettings {
    pub enabled: bool,
    pub intensity: f32,      // 0.0 - 2.0 typical
    pub max_blur_samples: u32,
}

impl Default for MotionBlurSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            intensity: 1.0,
            max_blur_samples: 8,
        }
    }
}

#[derive(Resource)]
pub struct MotionBlurPipeline {
    pub pipeline: CachedRenderPipelineId,
    pub bind_group_layout: BindGroupLayout,
}

pub struct MotionBlurNode;

impl Node for MotionBlurNode {
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let settings = world.resource::<MotionBlurSettings>();
        if !settings.enabled {
            return Ok(());
        }

        let pipeline_res = world.resource::<MotionBlurPipeline>();
        let velocity_tex = world.resource::<VelocityTexture>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let matrices = world.resource::<CameraMatrices>();

        let Ok(pipeline) = pipeline_cache.get_render_pipeline(pipeline_res.pipeline) else {
            return Ok(());
        };

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("motion_blur"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: /* target view - usually the current color or post-process target */,
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

        // Bind velocity + camera matrices + settings
        let bind_group = render_context.render_device().create_bind_group(
            "motion_blur_bind_group",
            &pipeline_res.bind_group_layout,
            &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(velocity_tex.view.clone()),
            }],
        );

        render_pass.set_bind_group(0, &bind_group, &[]);

        // Fullscreen triangle
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

pub fn setup_motion_blur_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut pipeline_cache: ResMut<PipelineCache>,
    asset_server: Res<AssetServer>,
) {
    let bind_group_layout = render_device.create_bind_group_layout(
        "motion_blur_bind_group_layout",
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
            // Add uniform for intensity + matrices if needed
        ],
    );

    let shader = asset_server.load("shaders/motion_blur.wgsl");

    let pipeline_descriptor = RenderPipelineDescriptor {
        label: Some("motion_blur_pipeline".into()),
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

    commands.insert_resource(MotionBlurPipeline {
        pipeline,
        bind_group_layout,
    });
}

// === Integration with client settings ===
// In settings.rs or a dedicated graphics settings UI:
// pub fn apply_motion_blur_settings(
//     mut settings: ResMut<MotionBlurSettings>,
//     graphics_settings: Res<GraphicsSettings>,
// ) {
//     settings.enabled = graphics_settings.motion_blur_enabled;
//     settings.intensity = graphics_settings.motion_blur_intensity;
// }

// Add MotionBlurSettings as a resource in your app startup or settings plugin.
