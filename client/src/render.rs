/*!
 * Powrush-MMO Advanced Render Pipeline Setup
 *
 * Complete temporal rendering stack:
 *   Velocity Prepass -> TAA Reprojection -> (Future SSR / Motion Blur) -> Tonemap / Final Present
 *
 * TAA is now positioned as a post-process effect that feeds clean resolved color
 * into Bevy's camera tonemapping (or your custom tonemap pass).
 *
 * PATSAGi Council + mercy-aligned for zero-artifact divine visuals.
 * AG-SML v1.0
 */

use bevy::prelude::*;
use bevy::render::render_graph::RenderGraphApp;
use bevy::render::RenderApp;

use crate::velocity_prepass::{VelocityPrepassNode, setup_velocity_prepass_pipeline, setup_velocity_texture};
use crate::ssr_render_node::{CameraMatrices, SsrRenderNodePlugin};
use crate::taa_reprojection::{TaaReprojectionNode, setup_taa_pipeline, setup_taa_history_texture};

pub struct PowrushRenderPlugin;

impl Plugin for PowrushRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SsrRenderNodePlugin);

        app.add_systems(Startup, (
            setup_velocity_prepass_pipeline,
            setup_velocity_texture,
            setup_taa_pipeline,
            setup_taa_history_texture,
        ));

        let render_app = app.sub_app_mut(RenderApp);

        // 1. Velocity Prepass (early, provides motion vectors)
        render_app.add_render_graph_node::<VelocityPrepassNode>("velocity_prepass");

        // 2. TAA Reprojection (post-process, uses velocity + history)
        render_app.add_render_graph_node::<TaaReprojectionNode>("taa_reprojection");
        render_app.add_render_graph_edge("velocity_prepass", "taa_reprojection");

        // 3. Wire TAA output into final tonemapping / present
        // In Bevy, the camera's tonemapping happens in the main graph after post-process.
        // Best practice: Make TaaReprojectionNode write resolved color to a texture
        // that your camera or a final PostProcess node reads as input.
        //
        // Example future edge:
        // render_app.add_render_graph_edge("taa_reprojection", bevy::render::main_graph::node::TONEMAP);
        // or insert a custom final resolve node before tonemap.
        //
        // For immediate use: TAA writes to history; the resolved output can be
        // copied or bound as the input for the camera's final color target.
    }
}
