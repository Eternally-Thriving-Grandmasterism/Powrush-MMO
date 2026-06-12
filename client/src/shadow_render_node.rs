//! client/src/shadow_render_node.rs
//! Poisson Disk PCF Shadow Bind Group Wiring
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v18.10+

use bevy::prelude::*;
use bevy::render::render_resource::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource,
    BufferBinding, BufferBindingType, ShaderStages,
};
use bevy::render::renderer::RenderDevice;
use crate::rbe_simulation::{PoissonDiskKernel, ShadowQuality};

/// Resource that holds the GPU bind group for the Poisson Disk PCF uniform
#[derive(Resource, Default)]
pub struct PoissonDiskBindGroup {
    pub bind_group: Option<BindGroup>,
}

/// System that creates or updates the Poisson Disk PCF bind group
pub fn update_poisson_disk_bind_group(
    mut poisson_bind_group: ResMut<PoissonDiskBindGroup>,
    kernel: Res<PoissonDiskKernel>,
    shadow_quality: Res<ShadowQuality>,
    render_device: Res<RenderDevice>,
) {
    // Only create bind group when using high quality shadows
    if *shadow_quality != ShadowQuality::HighQuality {
        poisson_bind_group.bind_group = None;
        return;
    }

    // Create the uniform buffer data from the kernel
    let uniform_data = crate::rbe_simulation::PoissonDiskUniform::from(&*kernel);

    // Create GPU buffer for the uniform
    let buffer = render_device.create_buffer_with_data(
        &bevy::render::render_resource::BufferInitDescriptor {
            label: Some("poisson_disk_pcf_uniform"),
            contents: bytemuck::cast_slice(&[uniform_data]),
            usage: bevy::render::render_resource::BufferUsages::UNIFORM
                | bevy::render::render_resource::BufferUsages::COPY_DST,
        },
    );

    // Create bind group layout (matches shader expectation)
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

    // Create the actual bind group
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

/// Plugin to register the Poisson Disk bind group system
pub struct ShadowRenderNodePlugin;

impl Plugin for ShadowRenderNodePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PoissonDiskBindGroup>()
            .add_systems(Update, update_poisson_disk_bind_group);
    }
}
