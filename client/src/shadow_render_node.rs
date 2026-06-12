/*!
 * shadow_render_node.rs
 * Powrush-MMO — Step 1: Resources & Texture Setup for Temporal Shadow Accumulation
 */

use bevy::prelude::*;
use bevy::render::{
    render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext},
    renderer::RenderContext,
    RenderApp,
    texture::BevyDefault,
};
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};

// ... (previous code remains)

/// Resource that holds the temporal accumulation textures for soft shadows
#[derive(Resource, Default)]
pub struct ShadowTemporalAccumulation {
    /// Accumulated soft shadow value (history)
    pub accumulation: Option<Handle<Image>>,
    /// Statistical moments for variance-guided filtering (mean + variance)
    pub moments: Option<Handle<Image>>,
}

/// System that creates the accumulation textures
pub fn setup_shadow_accumulation_textures(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    windows: Query<&Window>,
    mut accumulation: ResMut<ShadowTemporalAccumulation>,
) {
    let window = windows.single();
    let size = Extent3d {
        width: window.resolution.physical_width(),
        height: window.resolution.physical_height(),
        depth_or_array_layers: 1,
    };

    // Shadow Accumulation Texture (single channel)
    let accumulation_texture = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("shadow_accumulation"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::R16Float,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            mip_level_count: 1,
            sample_count: 1,
            view_formats: &[],
        },
        ..default()
    };

    // Shadow Moments Texture (mean + variance)
    let moments_texture = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("shadow_moments"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::RG16Float,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            mip_level_count: 1,
            sample_count: 1,
            view_formats: &[],
        },
        ..default()
    };

    let accumulation_handle = images.add(accumulation_texture);
    let moments_handle = images.add(moments_texture);

    accumulation.accumulation = Some(accumulation_handle);
    accumulation.moments = Some(moments_handle);
}

// ... (rest of file)

pub struct CustomShadowNodePlugin;

impl Plugin for CustomShadowNodePlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();

        render_graph.add_node(
            "poisson_disk_shadow_node",
            PoissonDiskShadowNode::new(&mut render_app.world),
        );

        // Initialize resources
        render_app.world.init_resource::<ShadowTemporalAccumulation>();

        let shader_handle = render_app.world.resource::<AssetServer>().load("shaders/poisson_disk_pcf.wgsl");
        render_app.world.insert_resource(PoissonDiskPcfShader(shader_handle));
        render_app.world.init_resource::<ShadowShaderSpecialization>();
        render_app.world.init_resource::<ShadowQualityState>();
        render_app.world.init_resource::<TemporalPoissonDisk>();
        render_app.world.init_resource::<PoissonDiskUniformBuffer>();
        render_app.world.init_resource::<PoissonDiskBindGroup>();
        render_app.world.init_resource::<ActivePoissonDiskBindGroup>();

        app.add_systems(Startup, setup_shadow_accumulation_textures);

        app.add_systems(Update, (
            finalize_shadow_specialization,
            update_temporal_poisson_disk_shadows,
            update_temporal_poisson_disk_uniform,
            update_poisson_disk_bind_group,
            integrate_poisson_disk_bind_group,
        ));
    }
}
