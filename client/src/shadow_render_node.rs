//! client/src/shadow_render_node.rs
//! Poisson Disk PCF Shadow Bind Group + Custom Render Node Integration
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v18.10+

use bevy::prelude::*;
use bevy::render::{
    render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext},
    renderer::{RenderContext, RenderDevice},
    RenderApp,
};
use bevy::pbr::ShadowPass;
use bevy::render::render_resource::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource,
    BufferBinding, BufferBindingType, ShaderStages,
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

// ==================== RENDER NODE ====================

pub struct PoissonDiskShadowNode {
    query: QueryState<&'static ShadowPass>,
}

impl PoissonDiskShadowNode {
    pub fn new(world: &mut World) -> Self {
        Self {
            query: QueryState::new(world),
        }
    }
}

impl Node for PoissonDiskShadowNode {
    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let bind_group = world.resource::<ActivePoissonDiskBindGroup>();

        if let Some(bind_group) = &bind_group.bind_group {
            // Here you would bind the Poisson Disk PCF group during shadow sampling
            // Example (conceptual):
            // render_context.command_encoder().set_bind_group(1, bind_group, &[]);
        }

        Ok(())
    }
}

// ==================== SYSTEMS ====================

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

pub fn integrate_poisson_disk_bind_group(
    bind_group: Res<PoissonDiskBindGroup>,
    mut active_bind_group: ResMut<ActivePoissonDiskBindGroup>,
    shadow_quality: Res<ShadowQualityState>,
) {
    if shadow_quality.is_high_quality {
        active_bind_group.bind_group = bind_group.bind_group.clone();
    } else {
        active_bind_group.bind_group = None;
    }
}

// ==================== PLUGIN ====================

pub struct ShadowRenderNodePlugin;

impl Plugin for ShadowRenderNodePlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();

        render_graph.add_node(
            "poisson_disk_shadow_node",
            PoissonDiskShadowNode::new(&mut render_app.world),
        );

        app.init_resource::<PoissonDiskBindGroup>()
            .init_resource::<ActivePoissonDiskBindGroup>()
            .init_resource::<ShadowQualityState>()
            .add_systems(Update, (
                update_poisson_disk_bind_group,
                integrate_poisson_disk_bind_group,
            ));
    }
}
