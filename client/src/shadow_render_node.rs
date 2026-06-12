/*!
 * shadow_render_node.rs
 * Powrush-MMO — Wiring Bind Group for Poisson Disk PCF
 */

use bevy::prelude::*;
use bevy::render::{
    render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext},
    renderer::RenderContext,
    RenderApp,
};
use bevy::pbr::ShadowPass;
use bevy::render::render_resource::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, BufferBinding,
    ShaderType,
};
use bevy::render::renderer::RenderDevice;
use std::f32::consts::PI;

// ... (previous structs remain)

/// Resource holding the bind group for the Poisson Disk uniform
#[derive(Resource, Default)]
pub struct PoissonDiskBindGroup {
    pub bind_group: Option<BindGroup>,
}

/// System that creates/updates the bind group for the current Poisson Disk kernel
pub fn update_poisson_disk_bind_group(
    uniform_buffer: Res<PoissonDiskUniformBuffer>,
    mut bind_group: ResMut<PoissonDiskBindGroup>,
    render_device: Res<RenderDevice>,
    shadow_quality: Res<ShadowQualityState>,
) {
    if !shadow_quality.is_high_quality {
        bind_group.bind_group = None;
        return;
    }

    let Some(buffer) = &uniform_buffer.buffer else {
        return;
    };

    // Create bind group layout (in a real implementation this should be cached)
    let bind_group_layout = render_device.create_bind_group_layout(
        &bevy::render::render_resource::BindGroupLayoutDescriptor {
            label: Some("poisson_disk_bind_group_layout"),
            entries: &[bevy::render::render_resource::BindGroupLayoutEntry {
                binding: 0,
                visibility: bevy::render::render_resource::ShaderStages::FRAGMENT,
                ty: bevy::render::render_resource::BindingType::Buffer {
                    ty: bevy::render::render_resource::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        },
    );

    let group = render_device.create_bind_group(&BindGroupDescriptor {
        label: Some("poisson_disk_bind_group"),
        layout: &bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::Buffer(BufferBinding {
                buffer,
                offset: 0,
                size: None,
            }),
        }],
    });

    bind_group.bind_group = Some(group);
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

        let shader_handle = render_app.world.resource::<AssetServer>().load("shaders/poisson_disk_pcf.wgsl");
        render_app.world.insert_resource(PoissonDiskPcfShader(shader_handle));
        render_app.world.init_resource::<ShadowShaderSpecialization>();
        render_app.world.init_resource::<ShadowQualityState>();
        render_app.world.init_resource::<TemporalPoissonDisk>();
        render_app.world.init_resource::<PoissonDiskUniformBuffer>();
        render_app.world.init_resource::<PoissonDiskBindGroup>(); // <-- Added

        app.add_systems(Update, (
            finalize_shadow_specialization,
            update_temporal_poisson_disk_shadows,
            update_temporal_poisson_disk_uniform,
            update_poisson_disk_bind_group, // <-- New bind group system
        ));
    }
}
