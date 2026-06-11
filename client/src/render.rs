/*!
 * Powrush-MMO Advanced Render Pipeline
 */

use bevy::prelude::*;
use bevy::render::render_graph::RenderGraphApp;
use bevy::render::RenderApp;

use crate::velocity_prepass::{VelocityPrepassNode, setup_velocity_prepass_pipeline, setup_velocity_texture};
use crate::ssr_render_node::SsrRenderNodePlugin;
use crate::taa_reprojection::{TaaReprojectionNode, TaaSettings, setup_taa_pipeline, setup_taa_history_texture};
use crate::motion_blur::{MotionBlurNode, MotionBlurSettings, setup_motion_blur_pipeline};

pub struct PowrushRenderPlugin;

impl Plugin for PowrushRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SsrRenderNodePlugin)
           .init_resource::<MotionBlurSettings>()
           .init_resource::<TaaSettings>();   // New tunable TAA settings

        app.add_systems(Startup, (
            setup_velocity_prepass_pipeline,
            setup_velocity_texture,
            setup_taa_pipeline,
            setup_taa_history_texture,
            setup_motion_blur_pipeline,
        ));

        let render_app = app.sub_app_mut(RenderApp);

        render_app.add_render_graph_node::<VelocityPrepassNode>("velocity_prepass");
        render_app.add_render_graph_node::<TaaReprojectionNode>("taa_reprojection");
        render_app.add_render_graph_node::<MotionBlurNode>("motion_blur");

        render_app.add_render_graph_edge("velocity_prepass", "taa_reprojection");
        render_app.add_render_graph_edge("taa_reprojection", "motion_blur");
    }
}
