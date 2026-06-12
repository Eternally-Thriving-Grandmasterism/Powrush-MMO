/*!
 * shadow_render_node.rs
 * Powrush-MMO — Step 2: Core Temporal Shadow Accumulation System
 */

use bevy::prelude::*;
use bevy::render::{
    render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext},
    renderer::RenderContext,
    RenderApp,
};
use bevy::render::render_resource::Shader;

// ... (previous resources remain)

/// Core system for variance-guided temporal shadow accumulation
///
/// This system handles:
/// - Checking if high-quality mode is active
/// - Reprojecting previous frame history
/// - Blending current Poisson Disk result with history
/// - Updating statistical moments
pub fn update_shadow_temporal_accumulation(
    shadow_quality: Res<ShadowQualityState>,
    temporal: Res<TemporalPoissonDisk>,
    accumulation: Res<ShadowTemporalAccumulation>,
    // In a full implementation we would also have:
    // - Motion vectors / velocity texture
    // - Current frame raw shadow result
    // - Previous frame accumulation + moments as inputs
    // - Render commands to execute a full-screen or compute pass
) {
    if !shadow_quality.is_high_quality {
        return;
    }

    // Get current rotated kernel (already advanced by previous system)
    let _current_kernel = temporal.current_kernel();

    // TODO in next steps:
    // 1. Reproject previous accumulation + moments using motion vectors
    // 2. Compute local variance of current shadow result
    // 3. Apply variance-guided clamping / weighting
    // 4. Blend current result with (clamped) history
    // 5. Update moments for next frame
    // 6. Write result to ShadowAccumulation texture

    // For now this system acts as the central coordination point.
    // The actual GPU work will be implemented in the WGSL pass (Step 3).
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
            update_shadow_temporal_accumulation, // <-- New core system
        ));
    }
}
