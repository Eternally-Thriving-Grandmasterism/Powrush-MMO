//! Main render graph setup including Velocity Prepass + Temporal SSR

use bevy::prelude::*;
use bevy::render::render_graph::RenderGraphApp;
use bevy::render::RenderApp;

use crate::ssr_render_node::{SSRNode, TemporalSSRNode, setup_ssr_pipeline, setup_temporal_ssr_pipeline, create_temporal_ssr_textures};
use crate::velocity_prepass::{VelocityPrepassNode, setup_velocity_prepass_pipeline};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                setup_ssr_pipeline,
                setup_temporal_ssr_pipeline,
                setup_velocity_prepass_pipeline,
                create_temporal_ssr_textures,
            ));

        let render_app = app.sub_app_mut(RenderApp);

        // Velocity Prepass - run early (before main color pass if possible)
        render_app.add_render_graph_node::<VelocityPrepassNode>("velocity_prepass");

        // Main SSR
        render_app.add_render_graph_node::<SSRNode>("ssr_post_process");
        render_app.add_render_graph_edge(
            bevy::render::main_graph::node::MAIN_PASS,
            "ssr_post_process",
        );

        // Temporal Accumulation after SSR
        render_app.add_render_graph_node::<TemporalSSRNode>("temporal_ssr");
        render_app.add_render_graph_edge("ssr_post_process", "temporal_ssr");

        // Optional: Velocity can be inserted earlier if it needs depth
        // render_app.add_render_graph_edge("velocity_prepass", "ssr_post_process");
    }
}
