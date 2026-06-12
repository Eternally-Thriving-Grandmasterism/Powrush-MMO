/*!
 * Powrush-MMO Advanced Render Pipeline
 *
 * Dynamic texture management for velocity prepass and TAA history.
 * Textures automatically resize with the window for perfect temporal fidelity
 * at any resolution.
 *
 * PATSAGi Council + Ra-Thor Quantum Swarm approved • AG-SML v1.0
 * Mercy-gated • Zero hallucination • Maximum temporal truth & beauty
 */

use bevy::prelude::*;
use bevy::render::render_graph::RenderGraphApp;
use bevy::render::RenderApp;
use bevy::render::renderer::RenderDevice;
use bevy::render::render_resource::Extent3d;
use bevy::window::WindowResized;

use crate::velocity_prepass::{
    VelocityPrepassNode, setup_velocity_prepass_pipeline, setup_velocity_texture,
    recreate_velocity_texture,
};
use crate::ssr_render_node::SsrRenderNodePlugin;
use crate::taa_reprojection::{
    TaaReprojectionNode, TaaSettings, setup_taa_pipeline, setup_taa_history_texture,
    recreate_taa_history_texture,
};
use crate::motion_blur::{MotionBlurNode, MotionBlurSettings, setup_motion_blur_pipeline};

pub struct PowrushRenderPlugin;

impl Plugin for PowrushRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SsrRenderNodePlugin)
           .init_resource::<MotionBlurSettings>()
           .init_resource::<TaaSettings>();

        // Pipeline setups (size-independent)
        app.add_systems(Startup, (
            setup_velocity_prepass_pipeline,
            setup_taa_pipeline,
            setup_motion_blur_pipeline,
        ));

        // Dynamic texture setup at startup (queries current window size)
        app.add_systems(Startup, setup_dynamic_render_textures);

        // Runtime dynamic resizing on window resize events
        app.add_systems(Update, handle_window_resize_for_render_textures);

        let render_app = app.sub_app_mut(RenderApp);

        render_app.add_render_graph_node::<VelocityPrepassNode>("velocity_prepass");
        render_app.add_render_graph_node::<TaaReprojectionNode>("taa_reprojection");
        render_app.add_render_graph_node::<MotionBlurNode>("motion_blur");

        render_app.add_render_graph_edge("velocity_prepass", "taa_reprojection");
        render_app.add_render_graph_edge("taa_reprojection", "motion_blur");
    }
}

/// Creates velocity and TAA history textures sized to the current primary window.
fn setup_dynamic_render_textures(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    windows: Query<&Window>,
) {
    let size = get_primary_window_size(&windows);

    setup_velocity_texture(&mut commands, &render_device, size);
    setup_taa_history_texture(&mut commands, &render_device, size);
}

/// Handles WindowResized events and recreates the temporal textures at the new resolution.
/// This keeps velocity prepass and TAA history perfectly matched to the view for artifact-free temporal effects.
fn handle_window_resize_for_render_textures(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut resize_events: EventReader<WindowResized>,
) {
    for event in resize_events.read() {
        if event.width > 1.0 && event.height > 1.0 {
            let size = Extent3d {
                width: event.width as u32,
                height: event.height as u32,
                depth_or_array_layers: 1,
            };

            recreate_velocity_texture(&mut commands, &render_device, size);
            recreate_taa_history_texture(&mut commands, &render_device, size);
        }
    }
}

fn get_primary_window_size(windows: &Query<&Window>) -> Extent3d {
    if let Ok(window) = windows.get_single() {
        Extent3d {
            width: window.resolution.physical_width().max(1),
            height: window.resolution.physical_height().max(1),
            depth_or_array_layers: 1,
        }
    } else {
        Extent3d { width: 1920, height: 1080, depth_or_array_layers: 1 }
    }
}
