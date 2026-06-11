/*!
 * client/src/ssr_render_node.rs
 * Bevy RenderGraph Node for Screen Space Reflections (SSR) + Temporal Accumulation
 *
 * Includes ping-pong texture management for temporal SSR.
 */

use bevy::prelude::*;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraphContext};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderContext;
use bevy::render::view::{ViewTarget, ViewDepthTexture};

// ... (SSRSettings, SSRPipeline, SSRUniformBuffer, SSRUniforms remain the same)

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

#[derive(Resource)]
pub struct SSRUniformBuffer {
    pub buffer: Buffer,
}

/// Ping-pong textures for temporal SSR accumulation
#[derive(Resource)]
pub struct TemporalSSRTextures {
    pub current: TextureView,
    pub history: TextureView,
    pub current_texture: Texture,
    pub history_texture: Texture,
    pub size: Extent3d,
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
    // ... (existing run method with real texture bind group)
    // For brevity, the main SSR node stays similar to previous version
}

// ==================== PING-PONG TEXTURE MANAGEMENT ====================

pub fn create_temporal_ssr_textures(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let size = Extent3d {
        width: window.resolution.width() as u32,
        height: window.resolution.height() as u32,
        depth_or_array_layers: 1,
    };

    let texture_descriptor = TextureDescriptor {
        label: Some("temporal_ssr_texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba16Float, // Good precision for accumulation
        usage: TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_DST,
        view_formats: &[],
    };

    let current_texture = render_device.create_texture(&texture_descriptor);
    let history_texture = render_device.create_texture(&texture_descriptor);

    let current_view = current_texture.create_view(&TextureViewDescriptor::default());
    let history_view = history_texture.create_view(&TextureViewDescriptor::default());

    commands.insert_resource(TemporalSSRTextures {
        current: current_view,
        history: history_view,
        current_texture,
        history_texture,
        size,
    });
}

/// Call this every frame (or in a render node) to swap current <-> history
pub fn swap_temporal_ssr_textures(textures: &mut TemporalSSRTextures) {
    std::mem::swap(&mut textures.current, &mut textures.history);
    std::mem::swap(&mut textures.current_texture, &mut textures.history_texture);
}

// ==================== TEMPORAL ACCUMULATION NODE (simplified) ====================

pub struct TemporalSSRNode;

impl Node for TemporalSSRNode {
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let textures = world.resource::<TemporalSSRTextures>();
        // In a full implementation you would:
        // 1. Run the temporal_ssr_accumulation.wgsl shader
        // 2. Bind textures.current as input and write to a temp target
        // 3. After pass, call swap_temporal_ssr_textures
        // For now this is a placeholder showing the management pattern
        Ok(())
    }
}

// Add this in your RenderPlugin setup:
// render_app.add_render_graph_node::<TemporalSSRNode>("temporal_ssr");
// render_app.add_render_graph_edge("ssr_post_process", "temporal_ssr");
