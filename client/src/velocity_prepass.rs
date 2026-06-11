/*!
 * client/src/velocity_prepass.rs
 * Velocity / Motion Vector Prepass for Temporal Techniques
 *
 * Phase 2 of improving temporal SSR quality.
 * Renders per-pixel velocity (current pos - previous pos) into a texture.
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

pub struct VelocityPrepassNode;

impl Node for VelocityPrepassNode {
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        // TODO: Implement actual velocity rendering pass
        // This node should render all objects with a velocity shader that outputs
        // (current_world_position - previous_world_position) as RG channels.
        //
        // For a minimal version:
        // - Use a separate render pass that writes to VelocityTexture
        // - Requires storing previous frame transforms per entity (or per mesh)
        // - Can start with rigid bodies only
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
            // Add entries for model matrix, previous model matrix, etc.
        ],
    );

    let shader = asset_server.load("shaders/velocity_prepass.wgsl"); // To be created

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
                format: TextureFormat::Rg16Float, // Velocity stored in RG
                blend: None,
                write_mask: ColorWrites::ALL,
            })],
            shader_defs: vec![],
        }),
        primitive: PrimitiveState::default(),
        depth_stencil: Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::Less,
            stencil: StencilState::default(),
            bias: DepthBiasState::default(),
        }),
        multisample: MultisampleState::default(),
        push_constant_ranges: vec![],
    };

    let pipeline = pipeline_cache.queue_render_pipeline(pipeline_descriptor);

    commands.insert_resource(VelocityPrepassPipeline {
        pipeline,
        bind_group_layout,
    });
}

// TODO: Create shaders/velocity_prepass.wgsl
// It should output velocity in RG channels and write depth normally.
