/*!
 * shadow_render_node.rs
 * Powrush-MMO — Binding Temporal Poisson Disk Kernel to GPU
 */

use bevy::prelude::*;
use bevy::render::{
    render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext},
    renderer::RenderContext,
    RenderApp,
};
use bevy::pbr::ShadowPass;
use bevy::render::render_resource::{Buffer, BufferInitDescriptor, BufferUsages, ShaderType};
use bevy::render::renderer::RenderQueue;
use std::f32::consts::PI;

// ... (previous code remains)

/// GPU-ready uniform for Poisson Disk kernel (already defined earlier)
#[derive(Clone, Copy, ShaderType)]
pub struct PoissonDiskUniform {
    pub samples: [Vec2; 16],
    pub sample_count: u32,
    pub _padding: [u32; 3],
}

impl From<&PoissonDiskKernel> for PoissonDiskUniform {
    fn from(kernel: &PoissonDiskKernel) -> Self {
        let mut samples = [Vec2::ZERO; 16];
        for (i, &s) in kernel.samples.iter().enumerate().take(16) {
            samples[i] = Vec2::new(s[0], s[1]);
        }
        Self {
            samples,
            sample_count: kernel.samples.len() as u32,
            _padding: [0; 3],
        }
    }
}

/// Resource holding the GPU buffer for the current Poisson Disk kernel
#[derive(Resource, Default)]
pub struct PoissonDiskUniformBuffer {
    pub buffer: Option<Buffer>,
}

/// System that uploads the current temporal Poisson Disk kernel to the GPU
pub fn update_temporal_poisson_disk_uniform(
    temporal: Res<TemporalPoissonDisk>,
    mut uniform_buffer: ResMut<PoissonDiskUniformBuffer>,
    render_queue: Res<RenderQueue>,
    shadow_quality: Res<ShadowQualityState>,
) {
    if !shadow_quality.is_high_quality {
        return;
    }

    // Get the current rotated kernel
    let current_kernel = temporal.current_kernel();
    let uniform = PoissonDiskUniform::from(current_kernel);

    // Create or update the GPU buffer
    let buffer = render_queue.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("temporal_poisson_disk_uniform"),
        contents: bytemuck::cast_slice(&[uniform]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    uniform_buffer.buffer = Some(buffer);

    // TODO: Bind this buffer to the shadow/lighting bind group
    // when using the custom Poisson Disk PCF shader path.
}

// ... (rest of the systems and plugin remain)

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
        render_app.world.init_resource::<PoissonDiskUniformBuffer>(); // <-- Added

        app.add_systems(Update, (
            finalize_shadow_specialization,
            update_temporal_poisson_disk_shadows,
            update_temporal_poisson_disk_uniform, // <-- New system
        ));
    }
}
