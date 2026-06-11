/*!
 * Powrush-MMO Advanced Render Pipeline Setup
 *
 * Wires Velocity Prepass + TAA Reprojection + CameraMatrices into the Bevy render graph.
 * Complete temporal rendering stack for artifact-free anti-aliasing and motion coherence.
 *
 * Upgraded with TAA reprojection logic for the most phenomenal cinematic experience.
 * PATSAGi Council + Quantum Swarm ready. Mercy-gated temporal stability.
 * AG-SML v1.0
 */

use bevy::prelude::*;
use bevy::render::render_graph::RenderGraphApp;
use bevy::render::RenderApp;

use crate::velocity_prepass::{VelocityPrepassNode, setup_velocity_prepass_pipeline, setup_velocity_texture};
use crate::ssr_render_node::{CameraMatrices, SsrRenderNodePlugin};
use crate::taa_reprojection::{TaaReprojectionNode, setup_taa_pipeline, setup_taa_history_texture};

/// Main render plugin for Powrush-MMO.
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

        // Velocity Prepass (motion vectors)
        render_app.add_render_graph_node::<VelocityPrepassNode>("velocity_prepass");

        // TAA Reprojection (uses velocity + history)
        render_app.add_render_graph_node::<TaaReprojectionNode>("taa_reprojection");
        render_app.add_render_graph_edge("velocity_prepass", "taa_reprojection");

        // Future expansions:
        // - SSR after TAA or interleaved
        // - Motion blur using velocity
        // - Final tonemap / post-process
    }
}
