//! client/src/shadow_render_node.rs
//! Poisson Disk PCF Shadow Bind Group + Temporal Shadow Accumulation
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v18.10+

use bevy::prelude::*;
use bevy::render::{
    render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext},
    renderer::{RenderContext, RenderDevice},
    texture::BevyDefault,
    RenderApp,
};
use bevy::render::render_resource::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource,
    BufferBinding, BufferBindingType, Extent3d, ShaderStages,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use crate::rbe_simulation::{PoissonDiskKernel, ShadowQuality};

// ==================== RESOURCES ====================

#[derive(Resource, Default)]
pub struct PoissonDiskBindGroup {
    pub bind_group: Option<BindGroup>,
}

#[derive(Resource, Default)]
pub struct ActivePoissonDiskBindGroup {
    pub bind_group: Option<BindGroup>,
}

#[derive(Resource, Default)]
pub struct ShadowQualityState {
    pub is_high_quality: bool,
}

/// Holds temporal accumulation textures for high-quality soft shadows
#[derive(Resource, Default)]
pub struct ShadowTemporalAccumulation {
    /// Accumulated soft shadow history
    pub accumulation: Option<Handle<Image>>,
    /// Statistical moments (mean + variance) for variance-guided filtering
    pub moments: Option<Handle<Image>>,
}

// ==================== RENDER NODE ====================

pub struct PoissonDiskShadowNode;

impl Node for PoissonDiskShadowNode {
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        _render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let _bind_group = world.resource::<ActivePoissonDiskBindGroup>();
        // TODO: Bind Poisson Disk group during custom shadow/lighting pass
        Ok(())
    }
}

// ==================== SYSTEMS ====================

/// Creates the temporal shadow accumulation textures (Startup)
pub fn setup_shadow_accumulation_textures(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    windows: Query<&Window>,
    mut accumulation: ResMut<ShadowTemporalAccumulation>,
    shadow_quality: Res<ShadowQuality>,
) {
    if *shadow_quality != ShadowQuality::HighQuality {
        return;
    }

    let window = windows.single();
    let size = Extent3d {
        width: window.resolution.physical_width(),
        height: window.resolution.physical_height(),
        depth_or_array_layers: 1,
    };

    // Accumulation texture (single channel history)
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

    // Moments texture (mean + variance)
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

    info!("Temporal shadow accumulation textures created for high-quality PCF");
}

/// Updates the Poisson Disk PCF bind group
pub fn update_poisson_disk_bind_group(
    mut poisson_bind_group: ResMut<PoissonDiskBindGroup>,
    kernel: Res<PoissonDiskKernel>,
    shadow_quality: Res<ShadowQuality>,
    render_device: Res<RenderDevice>,
) {
    if *shadow_quality != ShadowQuality::HighQuality {
        poisson_bind_group.bind_group = None;
        return;
    }

    let uniform_data = crate::rbe_simulation::PoissonDiskUniform::from(&*kernel);

    let buffer = render_device.create_buffer_with_data(
        &bevy::render::render_resource::BufferInitDescriptor {
            label: Some("poisson_disk_pcf_uniform"),
            contents: bytemuck::cast_slice(&[uniform_data]),
            usage: bevy::render::render_resource::BufferUsages::UNIFORM
                | bevy::render::render_resource::BufferUsages::COPY_DST,
        },
    );

    let bind_group_layout = render_device.create_bind_group_layout(
        &bevy::render::render_resource::BindGroupLayoutDescriptor {
            label: Some("poisson_disk_pcf_bind_group_layout"),
            entries: &[bevy::render::render_resource::BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: bevy::render::render_resource::BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        },
    );

    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: Some("poisson_disk_pcf_bind_group"),
        layout: &bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::Buffer(BufferBinding {
                buffer: &buffer,
                offset: 0,
                size: None,
            }),
        }],
    });

    poisson_bind_group.bind_group = Some(bind_group);
}

/// Core temporal shadow accumulation system (variance-guided)
pub fn update_shadow_temporal_accumulation(
    shadow_quality: Res<ShadowQualityState>,
    // TODO: Add motion vectors, current shadow result, etc. when implementing the WGSL pass
) {
    if !shadow_quality.is_high_quality {
        return;
    }

    // Placeholder for temporal accumulation logic:
    // 1. Reproject previous accumulation using motion vectors
    // 2. Compute variance of current frame
    // 3. Apply variance-guided clamping
    // 4. Blend current result with history
    // 5. Update moments for next frame
}

// ==================== PLUGIN ====================

pub struct ShadowRenderNodePlugin;

impl Plugin for ShadowRenderNodePlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();

        render_graph.add_node(
            "poisson_disk_shadow_node",
            PoissonDiskShadowNode,
        );

        app.init_resource::<PoissonDiskBindGroup>()
            .init_resource::<ActivePoissonDiskBindGroup>()
            .init_resource::<ShadowQualityState>()
            .init_resource::<ShadowTemporalAccumulation>()
            .add_systems(Startup, setup_shadow_accumulation_textures)
            .add_systems(Update, (
                update_poisson_disk_bind_group,
                integrate_poisson_disk_bind_group,
                update_shadow_temporal_accumulation,
            ));
    }
}
