/*!
 * Powrush-MMO Advanced Render Pipeline
 *
 * Dynamic texture management for velocity prepass, TAA history, motion blur, chromatic aberration.
 * All textures automatically resize with the window for perfect temporal + cinematic fidelity
 * at any resolution. Includes RenderTexturesResized event.
 *
 * Full post-FX chain: Velocity Prepass → TAA Reprojection → Motion Blur → Chromatic Aberration
 *
 * + Anisotropic Filtering (16x default) for razor-sharp textures on all surfaces at any angle.
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
use bevy::log::info;

use crate::velocity_prepass::{
    VelocityPrepassNode, setup_velocity_prepass_pipeline, setup_velocity_texture,
    recreate_velocity_texture,
};
use crate::ssr_render_node::SsrRenderNodePlugin;
use crate::taa_reprojection::{
    TaaReprojectionNode, TaaSettings, setup_taa_pipeline, setup_taa_history_texture,
    recreate_taa_history_texture,
};
use crate::motion_blur::{MotionBlurNode, MotionBlurSettings, setup_motion_blur_pipeline, setup_motion_blur_target, recreate_motion_blur_target};
use crate::chromatic_aberration::{
    ChromaticAberrationNode, ChromaticAberrationSettings, setup_chromatic_aberration_pipeline,
    setup_chromatic_aberration_target, recreate_chromatic_aberration_target,
};
use crate::anisotropic_filtering::{AnisotropicFilteringPlugin, AnisotropicFilteringSettings};

/// Event fired whenever the render textures (velocity, TAA, motion blur, CA) are resized.
#[derive(Event, Debug, Clone, Copy)]
pub struct RenderTexturesResized {
    pub new_size: Extent3d,
};

pub struct PowrushRenderPlugin;

impl Plugin for PowrushRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SsrRenderNodePlugin,
            AnisotropicFilteringPlugin,
        ))
           .init_resource::<MotionBlurSettings>()
           .init_resource::<TaaSettings>()
           .init_resource::<ChromaticAberrationSettings>()
           .add_event::<RenderTexturesResized>();

        // Pipeline setups (size-independent)
        app.add_systems(Startup, (
            setup_velocity_prepass_pipeline,
            setup_taa_pipeline,
            setup_motion_blur_pipeline,
            setup_chromatic_aberration_pipeline,
        ));

        // Dynamic texture setup at startup
        app.add_systems(Startup, setup_dynamic_render_textures);

        // Runtime dynamic resizing
        app.add_systems(Update, handle_window_resize_for_render_textures);

        let render_app = app.sub_app_mut(RenderApp);

        render_app.add_render_graph_node::<VelocityPrepassNode>("velocity_prepass");
        render_app.add_render_graph_node::<TaaReprojectionNode>("taa_reprojection");
        render_app.add_render_graph_node::<MotionBlurNode>("motion_blur");
        render_app.add_render_graph_node::<ChromaticAberrationNode>("chromatic_aberration");

        render_app.add_render_graph_edge("velocity_prepass", "taa_reprojection");
        render_app.add_render_graph_edge("taa_reprojection", "motion_blur");
        render_app.add_render_graph_edge("motion_blur", "chromatic_aberration");
    }
}

/// Creates all post-FX textures sized to the current primary window.
fn setup_dynamic_render_textures(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    windows: Query<&Window>,
    mut resize_writer: EventWriter<RenderTexturesResized>,
) {
    let size = get_primary_window_size(&windows);

    setup_velocity_texture(&mut commands, &render_device, size);
    setup_taa_history_texture(&mut commands, &render_device, size);
    setup_motion_blur_target(&mut commands, &render_device, size);
    setup_chromatic_aberration_target(&mut commands, &render_device, size);

    resize_writer.send(RenderTexturesResized { new_size: size });
    info!("[Powrush] Initial render textures created at {}x{} (incl. Chromatic Aberration)", size.width, size.height);
}

/// Handles WindowResized events and recreates all temporal + post-FX textures.
fn handle_window_resize_for_render_textures(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut resize_events: EventReader<WindowResized>,
    mut resize_writer: EventWriter<RenderTexturesResized>,
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
            recreate_motion_blur_target(&mut commands, &render_device, size);
            recreate_chromatic_aberration_target(&mut commands, &render_device, size);

            resize_writer.send(RenderTexturesResized { new_size: size });
            info!("[Powrush] Render textures resized to {}x{} (temporal + CA reset recommended)", size.width, size.height);
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
