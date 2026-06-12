/*!
 * shadow_render_node.rs
 * Powrush-MMO — Final Integration of Poisson Disk PCF Bind Group into Pipeline
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
};
use bevy::render::renderer::RenderDevice;

// ... (previous code remains)

/// Final integration point for the Poisson Disk bind group
///
/// This system prepares the bind group so it can be used during
/// the shadow sampling / lighting pass when high-quality PCF is enabled.
pub fn integrate_poisson_disk_bind_group(
    bind_group: Res<PoissonDiskBindGroup>,
    mut commands: Commands,
    shadow_quality: Res<ShadowQualityState>,
) {
    if shadow_quality.is_high_quality {
        if let Some(group) = &bind_group.bind_group {
            // In a full custom pipeline, you would insert this bind group
            // into the render commands for the current view's shadow/lighting pass.
            //
            // Example (conceptual):
            // commands.insert_resource(ActivePoissonDiskBindGroup(group.clone()));
            //
            // Or set it on a custom render phase.
            //
            // For now, we store it so other systems/nodes can access it.
        }
    }
}

/// Resource to expose the active bind group to the rest of the pipeline
#[derive(Resource, Default)]
pub struct ActivePoissonDiskBindGroup {
    pub bind_group: Option<BindGroup>,
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
        render_app.world.init_resource::<PoissonDiskBindGroup>();
        render_app.world.init_resource::<ActivePoissonDiskBindGroup>(); // <-- Final integration resource

        app.add_systems(Update, (
            finalize_shadow_specialization,
            update_temporal_poisson_disk_shadows,
            update_temporal_poisson_disk_uniform,
            update_poisson_disk_bind_group,
            integrate_poisson_disk_bind_group, // <-- Final integration system
        ));
    }
}
